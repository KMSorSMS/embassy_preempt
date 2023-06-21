#![no_std]

use control::Control;
use embassy_futures::select::{select3, Either3};
use embassy_net_driver_channel as ch;
use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use embedded_hal_async::spi::SpiDevice;
use ioctl::Shared;
use proto::CtrlMsg;

use crate::ioctl::PendingIoctl;
use crate::proto::CtrlMsgPayload;

mod proto;

// must be first
mod fmt;

mod control;
mod ioctl;

const MTU: usize = 1514;

macro_rules! impl_bytes {
    ($t:ident) => {
        impl $t {
            pub const SIZE: usize = core::mem::size_of::<Self>();

            #[allow(unused)]
            pub fn to_bytes(&self) -> [u8; Self::SIZE] {
                unsafe { core::mem::transmute(*self) }
            }

            #[allow(unused)]
            pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> &Self {
                let alignment = core::mem::align_of::<Self>();
                assert_eq!(
                    bytes.as_ptr().align_offset(alignment),
                    0,
                    "{} is not aligned",
                    core::any::type_name::<Self>()
                );
                unsafe { core::mem::transmute(bytes) }
            }

            #[allow(unused)]
            pub fn from_bytes_mut(bytes: &mut [u8; Self::SIZE]) -> &mut Self {
                let alignment = core::mem::align_of::<Self>();
                assert_eq!(
                    bytes.as_ptr().align_offset(alignment),
                    0,
                    "{} is not aligned",
                    core::any::type_name::<Self>()
                );

                unsafe { core::mem::transmute(bytes) }
            }
        }
    };
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default)]
struct PayloadHeader {
    /// InterfaceType on lower 4 bits, number on higher 4 bits.
    if_type_and_num: u8,

    /// Flags.
    ///
    /// bit 0: more fragments.
    flags: u8,

    len: u16,
    offset: u16,
    checksum: u16,
    seq_num: u16,
    reserved2: u8,

    /// Packet type for HCI or PRIV interface, reserved otherwise
    hci_priv_packet_type: u8,
}
impl_bytes!(PayloadHeader);

#[allow(unused)]
#[repr(u8)]
enum InterfaceType {
    Sta = 0,
    Ap = 1,
    Serial = 2,
    Hci = 3,
    Priv = 4,
    Test = 5,
}

const MAX_SPI_BUFFER_SIZE: usize = 1600;

pub struct State {
    shared: Shared,
    ch: ch::State<MTU, 4, 4>,
}

impl State {
    pub fn new() -> Self {
        Self {
            shared: Shared::new(),
            ch: ch::State::new(),
        }
    }
}

pub type NetDriver<'a> = ch::Device<'a, MTU>;

pub async fn new<'a, SPI, IN, OUT>(
    state: &'a mut State,
    spi: SPI,
    handshake: IN,
    ready: IN,
    reset: OUT,
) -> (NetDriver<'a>, Control<'a>, Runner<'a, SPI, IN, OUT>)
where
    SPI: SpiDevice,
    IN: InputPin + Wait,
    OUT: OutputPin,
{
    let (ch_runner, device) = ch::new(&mut state.ch, [0; 6]);
    let state_ch = ch_runner.state_runner();

    let mut runner = Runner {
        ch: ch_runner,
        shared: &state.shared,
        next_seq: 1,
        handshake,
        ready,
        reset,
        spi,
    };
    runner.init().await;

    (device, Control::new(state_ch, &state.shared), runner)
}

pub struct Runner<'a, SPI, IN, OUT> {
    ch: ch::Runner<'a, MTU>,
    shared: &'a Shared,

    next_seq: u16,

    spi: SPI,
    handshake: IN,
    ready: IN,
    reset: OUT,
}

impl<'a, SPI, IN, OUT> Runner<'a, SPI, IN, OUT>
where
    SPI: SpiDevice,
    IN: InputPin + Wait,
    OUT: OutputPin,
{
    async fn init(&mut self) {}

    pub async fn run(mut self) -> ! {
        debug!("resetting...");
        self.reset.set_low().unwrap();
        Timer::after(Duration::from_millis(100)).await;
        self.reset.set_high().unwrap();
        Timer::after(Duration::from_millis(1000)).await;

        let mut tx_buf = [0u8; MAX_SPI_BUFFER_SIZE];
        let mut rx_buf = [0u8; MAX_SPI_BUFFER_SIZE];

        loop {
            self.handshake.wait_for_high().await.unwrap();

            let ioctl = self.shared.ioctl_wait_pending();
            let tx = self.ch.tx_buf();
            let ev = async { self.ready.wait_for_high().await.unwrap() };

            match select3(ioctl, tx, ev).await {
                Either3::First(PendingIoctl { buf, req_len }) => {
                    tx_buf[12..24].copy_from_slice(b"\x01\x08\x00ctrlResp\x02");
                    tx_buf[24..26].copy_from_slice(&(req_len as u16).to_le_bytes());
                    tx_buf[26..][..req_len].copy_from_slice(&unsafe { &*buf }[..req_len]);

                    let mut header = PayloadHeader {
                        if_type_and_num: InterfaceType::Serial as _,
                        len: (req_len + 14) as _,
                        offset: PayloadHeader::SIZE as _,
                        seq_num: self.next_seq,
                        ..Default::default()
                    };
                    self.next_seq = self.next_seq.wrapping_add(1);

                    // Calculate checksum
                    tx_buf[0..12].copy_from_slice(&header.to_bytes());
                    header.checksum = checksum(&tx_buf[..26 + req_len]);
                    tx_buf[0..12].copy_from_slice(&header.to_bytes());
                }
                Either3::Second(packet) => {
                    tx_buf[12..][..packet.len()].copy_from_slice(packet);

                    let mut header = PayloadHeader {
                        if_type_and_num: InterfaceType::Sta as _,
                        len: packet.len() as _,
                        offset: PayloadHeader::SIZE as _,
                        seq_num: self.next_seq,
                        ..Default::default()
                    };
                    self.next_seq = self.next_seq.wrapping_add(1);

                    // Calculate checksum
                    tx_buf[0..12].copy_from_slice(&header.to_bytes());
                    header.checksum = checksum(&tx_buf[..12 + packet.len()]);
                    tx_buf[0..12].copy_from_slice(&header.to_bytes());

                    self.ch.tx_done();
                }
                Either3::Third(()) => {
                    tx_buf[..PayloadHeader::SIZE].fill(0);
                }
            }

            if tx_buf[0] != 0 {
                trace!("tx: {:02x}", &tx_buf[..40]);
            }

            self.spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
            let delay_until = Instant::now() + Duration::from_millis(1);
            self.handle_rx(&mut rx_buf);
            Timer::at(delay_until).await;
        }
    }

    fn handle_rx(&mut self, buf: &mut [u8]) {
        trace!("rx: {:02x}", &buf[..40]);

        let buf_len = buf.len();
        let h = PayloadHeader::from_bytes_mut((&mut buf[..PayloadHeader::SIZE]).try_into().unwrap());

        if h.len == 0 || h.offset as usize != PayloadHeader::SIZE {
            return;
        }

        let payload_len = h.len as usize;
        if buf_len < PayloadHeader::SIZE + payload_len {
            warn!("rx: len too big");
            return;
        }

        let if_type_and_num = h.if_type_and_num;
        let want_checksum = h.checksum;
        h.checksum = 0;
        let got_checksum = checksum(&buf[..PayloadHeader::SIZE + payload_len]);
        if want_checksum != got_checksum {
            warn!("rx: bad checksum. Got {:04x}, want {:04x}", got_checksum, want_checksum);
            return;
        }

        let payload = &mut buf[PayloadHeader::SIZE..][..payload_len];

        match if_type_and_num & 0x0f {
            // STA
            0 => match self.ch.try_rx_buf() {
                Some(buf) => {
                    buf[..payload.len()].copy_from_slice(payload);
                    self.ch.rx_done(payload.len())
                }
                None => warn!("failed to push rxd packet to the channel."),
            },
            // serial
            2 => {
                trace!("serial rx: {:02x}", payload);
                if payload.len() < 14 {
                    warn!("serial rx: too short");
                    return;
                }

                let is_event = match &payload[..12] {
                    b"\x01\x08\x00ctrlResp\x02" => false,
                    b"\x01\x08\x00ctrlEvnt\x02" => true,
                    _ => {
                        warn!("serial rx: bad tlv");
                        return;
                    }
                };

                let len = u16::from_le_bytes(payload[12..14].try_into().unwrap()) as usize;
                if payload.len() < 14 + len {
                    warn!("serial rx: too short 2");
                    return;
                }
                let data = &payload[14..][..len];

                if is_event {
                    self.handle_event(data);
                } else {
                    self.shared.ioctl_done(data);
                }
            }
            _ => warn!("unknown iftype {}", if_type_and_num),
        }
    }

    fn handle_event(&self, data: &[u8]) {
        let Ok(event) = noproto::read::<CtrlMsg>(data) else {
            warn!("failed to parse event");
            return
        };

        debug!("event: {:?}", &event);

        let Some(payload) = &event.payload else {
            warn!("event without payload?");
            return
        };

        match payload {
            CtrlMsgPayload::EventEspInit(_) => self.shared.init_done(),
            _ => {}
        }
    }
}

fn checksum(buf: &[u8]) -> u16 {
    let mut res = 0u16;
    for &b in buf {
        res = res.wrapping_add(b as _);
    }
    res
}
