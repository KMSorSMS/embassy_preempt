use core::sync::atomic::{fence, Ordering};
use core::task::Waker;

use embassy::interrupt::{Interrupt, InterruptExt};
use embassy::waitqueue::AtomicWaker;

use crate::interrupt;
use crate::pac;
use crate::pac::dma::{regs, vals};
use crate::rcc::sealed::RccPeripheral;

use super::{Request, Word, WordSize};

const CH_COUNT: usize = pac::peripheral_count!(DMA) * 8;

impl From<WordSize> for vals::Size {
    fn from(raw: WordSize) -> Self {
        match raw {
            WordSize::OneByte => Self::BITS8,
            WordSize::TwoBytes => Self::BITS16,
            WordSize::FourBytes => Self::BITS32,
        }
    }
}

struct State {
    ch_wakers: [AtomicWaker; CH_COUNT],
}

impl State {
    const fn new() -> Self {
        const AW: AtomicWaker = AtomicWaker::new();
        Self {
            ch_wakers: [AW; CH_COUNT],
        }
    }
}

static STATE: State = State::new();

macro_rules! dma_num {
    (DMA1) => {
        0
    };
    (DMA2) => {
        1
    };
}

unsafe fn on_irq() {
    pac::peripherals! {
        (dma, $dma:ident) => {
            for isrn in 0..2 {
                let isr = pac::$dma.isr(isrn).read();

                for chn in 0..4 {
                    let cr = pac::$dma.st(isrn * 4 + chn).cr();

                    if isr.tcif(chn) && cr.read().tcie() {
                        cr.write(|_| ()); // Disable channel interrupts with the default value.
                        let n = dma_num!($dma) * 8 + isrn * 4 + chn;
                        STATE.ch_wakers[n].wake();
                    }
                }
            }
        };
    }
}

/// safety: must be called only once
pub(crate) unsafe fn init() {
    pac::interrupts! {
        ($peri:ident, dma, $block:ident, $signal_name:ident, $irq:ident) => {
            interrupt::$irq::steal().enable();
        };
    }
    pac::peripherals! {
        (dma, $peri:ident) => {
            crate::peripherals::$peri::enable();
        };
    }
}

pac::dma_channels! {
    ($channel_peri:ident, $dma_peri:ident, dma, $channel_num:expr, $dmamux:tt) => {
        impl crate::dma::sealed::Channel for crate::peripherals::$channel_peri {
            unsafe fn start_write<W: Word>(&mut self, request: Request, buf: &[W], reg_addr: *mut u32) {
                let isrn = $channel_num as usize / 4;
                let isrbit = $channel_num as usize % 4;
                low_level_api::reset_status(&crate::pac::$dma_peri, isrn, isrbit);
                low_level_api::start_transfer(
                    request,
                    vals::Dir::MEMORYTOPERIPHERAL,
                    reg_addr as *const u32,
                    buf.as_ptr() as *mut u32,
                    buf.len(),
                    true,
                    crate::pac::$dma_peri.st($channel_num as _),
                    vals::Size::from(W::bits()),
                    #[cfg(dmamux)]
                    <Self as super::dmamux::sealed::MuxChannel>::DMAMUX_REGS,
                    #[cfg(dmamux)]
                    <Self as super::dmamux::sealed::MuxChannel>::DMAMUX_CH_NUM,
                )
            }

            unsafe fn start_write_repeated<W: Word>(&mut self, request: Request, repeated: W, count: usize, reg_addr: *mut u32) {
                let buf = [repeated];
                let isrn = $channel_num as usize / 4;
                let isrbit = $channel_num as usize % 4;
                low_level_api::reset_status(&crate::pac::$dma_peri, isrn, isrbit);
                low_level_api::start_transfer(
                    request,
                    vals::Dir::MEMORYTOPERIPHERAL,
                    reg_addr as *const u32,
                    buf.as_ptr() as *mut u32,
                    count,
                    false,
                    crate::pac::$dma_peri.st($channel_num as _),
                    vals::Size::from(W::bits()),
                    #[cfg(dmamux)]
                    <Self as super::dmamux::sealed::MuxChannel>::DMAMUX_REGS,
                    #[cfg(dmamux)]
                    <Self as super::dmamux::sealed::MuxChannel>::DMAMUX_CH_NUM,
                )
            }

            unsafe fn start_read<W: Word>(&mut self, request: Request, reg_addr: *mut u32, buf: &mut [W]) {
                let isrn = $channel_num as usize / 4;
                let isrbit = $channel_num as usize % 4;
                low_level_api::reset_status(&crate::pac::$dma_peri, isrn, isrbit);
                low_level_api::start_transfer(
                    request,
                    vals::Dir::PERIPHERALTOMEMORY,
                    reg_addr as *const u32,
                    buf.as_ptr() as *mut u32,
                    buf.len(),
                    true,
                    crate::pac::$dma_peri.st($channel_num as _),
                    vals::Size::from(W::bits()),
                    #[cfg(dmamux)]
                    <Self as super::dmamux::sealed::MuxChannel>::DMAMUX_REGS,
                    #[cfg(dmamux)]
                    <Self as super::dmamux::sealed::MuxChannel>::DMAMUX_CH_NUM,
                );
            }

            fn request_stop(&mut self) {
                unsafe {low_level_api::request_stop(&crate::pac::$dma_peri, $channel_num);}
            }

            fn is_running(&self) -> bool {
                unsafe {low_level_api::is_running(&crate::pac::$dma_peri, $channel_num)}
            }

            fn remaining_transfers(&mut self) -> u16 {
                unsafe {low_level_api::get_remaining_transfers(&crate::pac::$dma_peri, $channel_num)}
            }

            fn set_waker(&mut self, waker: &Waker) {
                unsafe {low_level_api::set_waker(&crate::pac::$dma_peri,  $channel_num, waker )}
            }
        }

        impl crate::dma::Channel for crate::peripherals::$channel_peri { }
    };
}

pac::interrupts! {
    ($peri:ident, dma, $block:ident, $signal_name:ident, $irq:ident) => {
        #[crate::interrupt]
        unsafe fn $irq () {
            on_irq()
        }
    };
}

mod low_level_api {
    use super::*;

    pub unsafe fn start_transfer(
        request: Request,
        dir: vals::Dir,
        peri_addr: *const u32,
        mem_addr: *mut u32,
        mem_len: usize,
        incr_mem: bool,
        ch: crate::pac::dma::St,
        data_size: vals::Size,
        #[cfg(dmamux)] dmamux_regs: pac::dmamux::Dmamux,
        #[cfg(dmamux)] dmamux_ch_num: u8,
    ) {
        #[cfg(dmamux)]
        super::super::dmamux::configure_dmamux(dmamux_regs, dmamux_ch_num, request);

        // "Preceding reads and writes cannot be moved past subsequent writes."
        fence(Ordering::SeqCst);

        ch.par().write_value(peri_addr as u32);
        ch.m0ar().write_value(mem_addr as u32);
        ch.ndtr().write_value(regs::Ndtr(mem_len as _));
        ch.cr().write(|w| {
            w.set_dir(dir);
            w.set_msize(data_size);
            w.set_psize(data_size);
            w.set_pl(vals::Pl::VERYHIGH);
            if incr_mem {
                w.set_minc(vals::Inc::INCREMENTED);
            } else {
                w.set_minc(vals::Inc::FIXED);
            }
            w.set_pinc(vals::Inc::FIXED);
            w.set_teie(true);
            w.set_tcie(true);
            #[cfg(dma_v1)]
            w.set_trbuff(true);

            #[cfg(dma_v2)]
            w.set_chsel(request);

            w.set_en(true);
        });
    }

    /// Stops the DMA channel.
    pub unsafe fn request_stop(dma: &pac::dma::Dma, channel_number: u8) {
        // Reset status
        let isrn = channel_number as usize / 4;
        let isrbit = channel_number as usize % 4;
        reset_status(dma, isrn, isrbit);

        // get a handle on the channel itself
        let ch = dma.st(channel_number as _);

        // Disable the channel and interrupts with the default value.
        ch.cr().write(|_| ());

        // "Subsequent reads and writes cannot be moved ahead of preceding reads."
        fence(Ordering::SeqCst);
    }

    /// Gets the running status of the channel
    pub unsafe fn is_running(dma: &pac::dma::Dma, ch: u8) -> bool {
        // get a handle on the channel itself
        let ch = dma.st(ch as _);

        // Wait for the transfer to complete when it was ongoing.
        ch.cr().read().en()
    }

    /// Gets the total remaining transfers for the channel
    /// Note: this will be zero for transfers that completed without cancellation.
    pub unsafe fn get_remaining_transfers(dma: &pac::dma::Dma, ch: u8) -> u16 {
        // get a handle on the channel itself
        let ch = dma.st(ch as _);
        // read the remaining transfer count. If this is zero, the transfer completed fully.
        ch.ndtr().read().ndt()
    }

    /// Sets the waker for the specified DMA channel
    pub unsafe fn set_waker(_dma: &pac::dma::Dma, state_number: u8, waker: &Waker) {
        let n = state_number as usize;
        STATE.ch_wakers[n].register(waker);
    }

    pub unsafe fn reset_status(dma: &crate::pac::dma::Dma, isrn: usize, isrbit: usize) {
        dma.ifcr(isrn).write(|w| {
            w.set_tcif(isrbit, true);
            w.set_teif(isrbit, true);
        });
    }
}
