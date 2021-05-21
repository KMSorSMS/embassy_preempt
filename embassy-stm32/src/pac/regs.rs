#![no_std]
#![doc = "Peripheral access API (generated using svd2rust v0.17.0 (22741fa 2021-04-20))"]
pub mod usart_v1 {
    use crate::generic::*;
    #[doc = "Universal synchronous asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Usart(pub *mut u8);
    unsafe impl Send for Usart {}
    unsafe impl Sync for Usart {}
    impl Usart {
        #[doc = "Status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Baud rate register"]
        pub fn brr(self) -> Reg<regs::Brr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Usart, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Control register 3"]
        pub fn cr3(self) -> Reg<regs::Cr3Usart, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Guard time and prescaler register"]
        pub fn gtpr(self) -> Reg<regs::Gtpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    #[doc = "Universal asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Uart(pub *mut u8);
    unsafe impl Send for Uart {}
    unsafe impl Sync for Uart {}
    impl Uart {
        #[doc = "Status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Baud rate register"]
        pub fn brr(self) -> Reg<regs::Brr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Control register 3"]
        pub fn cr3(self) -> Reg<regs::Cr3, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SrUsart(pub u32);
        impl SrUsart {
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Noise error flag"]
            pub const fn ne(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Noise error flag"]
            pub fn set_ne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE line detected"]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE line detected"]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete"]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete"]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "LIN break detection flag"]
            pub const fn lbd(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection flag"]
            pub fn set_lbd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "CTS flag"]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "CTS flag"]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for SrUsart {
            fn default() -> SrUsart {
                SrUsart(0)
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Noise error flag"]
            pub const fn ne(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Noise error flag"]
            pub fn set_ne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE line detected"]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE line detected"]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete"]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete"]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "LIN break detection flag"]
            pub const fn lbd(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection flag"]
            pub fn set_lbd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Send break"]
            pub const fn sbk(&self) -> super::vals::Sbk {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Sbk(val as u8)
            }
            #[doc = "Send break"]
            pub fn set_sbk(&mut self, val: super::vals::Sbk) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Receiver wakeup"]
            pub const fn rwu(&self) -> super::vals::Rwu {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rwu(val as u8)
            }
            #[doc = "Receiver wakeup"]
            pub fn set_rwu(&mut self, val: super::vals::Rwu) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Receiver enable"]
            pub const fn re(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver enable"]
            pub fn set_re(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmitter enable"]
            pub const fn te(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Transmitter enable"]
            pub fn set_te(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE interrupt enable"]
            pub const fn idleie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE interrupt enable"]
            pub fn set_idleie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "RXNE interrupt enable"]
            pub const fn rxneie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "RXNE interrupt enable"]
            pub fn set_rxneie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete interrupt enable"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete interrupt enable"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "TXE interrupt enable"]
            pub const fn txeie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "TXE interrupt enable"]
            pub fn set_txeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "PE interrupt enable"]
            pub const fn peie(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "PE interrupt enable"]
            pub fn set_peie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Parity selection"]
            pub const fn ps(&self) -> super::vals::Ps {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Ps(val as u8)
            }
            #[doc = "Parity selection"]
            pub fn set_ps(&mut self, val: super::vals::Ps) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Parity control enable"]
            pub const fn pce(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Parity control enable"]
            pub fn set_pce(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Wakeup method"]
            pub const fn wake(&self) -> super::vals::Wake {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Wake(val as u8)
            }
            #[doc = "Wakeup method"]
            pub fn set_wake(&mut self, val: super::vals::Wake) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "Word length"]
            pub const fn m(&self) -> super::vals::M {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::M(val as u8)
            }
            #[doc = "Word length"]
            pub fn set_m(&mut self, val: super::vals::M) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "USART enable"]
            pub const fn ue(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "USART enable"]
            pub fn set_ue(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr3(pub u32);
        impl Cr3 {
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "IrDA mode enable"]
            pub const fn iren(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "IrDA mode enable"]
            pub fn set_iren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Half-duplex selection"]
            pub const fn hdsel(&self) -> super::vals::Hdsel {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hdsel(val as u8)
            }
            #[doc = "Half-duplex selection"]
            pub fn set_hdsel(&mut self, val: super::vals::Hdsel) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DMA enable transmitter"]
            pub const fn dmat(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable transmitter"]
            pub fn set_dmat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cr3 {
            fn default() -> Cr3 {
                Cr3(0)
            }
        }
        #[doc = "Guard time and prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gtpr(pub u32);
        impl Gtpr {
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Guard time value"]
            pub const fn gt(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Guard time value"]
            pub fn set_gt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Gtpr {
            fn default() -> Gtpr {
                Gtpr(0)
            }
        }
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2Usart(pub u32);
        impl Cr2Usart {
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "lin break detection length"]
            pub const fn lbdl(&self) -> super::vals::Lbdl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lbdl(val as u8)
            }
            #[doc = "lin break detection length"]
            pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Last bit clock pulse"]
            pub const fn lbcl(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Last bit clock pulse"]
            pub fn set_lbcl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Clock enable"]
            pub const fn clken(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Clock enable"]
            pub fn set_clken(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "STOP bits"]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Stop(val as u8)
            }
            #[doc = "STOP bits"]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cr2Usart {
            fn default() -> Cr2Usart {
                Cr2Usart(0)
            }
        }
        #[doc = "Baud rate register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "fraction of USARTDIV"]
            pub const fn div_fraction(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "fraction of USARTDIV"]
            pub fn set_div_fraction(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "mantissa of USARTDIV"]
            pub const fn div_mantissa(&self) -> u16 {
                let val = (self.0 >> 4usize) & 0x0fff;
                val as u16
            }
            #[doc = "mantissa of USARTDIV"]
            pub fn set_div_mantissa(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "lin break detection length"]
            pub const fn lbdl(&self) -> super::vals::Lbdl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lbdl(val as u8)
            }
            #[doc = "lin break detection length"]
            pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "STOP bits"]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Stop(val as u8)
            }
            #[doc = "STOP bits"]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr3Usart(pub u32);
        impl Cr3Usart {
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "IrDA mode enable"]
            pub const fn iren(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "IrDA mode enable"]
            pub fn set_iren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Half-duplex selection"]
            pub const fn hdsel(&self) -> super::vals::Hdsel {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hdsel(val as u8)
            }
            #[doc = "Half-duplex selection"]
            pub fn set_hdsel(&mut self, val: super::vals::Hdsel) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Smartcard NACK enable"]
            pub const fn nack(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Smartcard NACK enable"]
            pub fn set_nack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Smartcard mode enable"]
            pub const fn scen(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Smartcard mode enable"]
            pub fn set_scen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DMA enable transmitter"]
            pub const fn dmat(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable transmitter"]
            pub fn set_dmat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "RTS enable"]
            pub const fn rtse(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "RTS enable"]
            pub fn set_rtse(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "CTS enable"]
            pub const fn ctse(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "CTS enable"]
            pub fn set_ctse(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "CTS interrupt enable"]
            pub const fn ctsie(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "CTS interrupt enable"]
            pub fn set_ctsie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Cr3Usart {
            fn default() -> Cr3Usart {
                Cr3Usart(0)
            }
        }
        #[doc = "Data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data value"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Data value"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ps(pub u8);
        impl Ps {
            #[doc = "Even parity"]
            pub const EVEN: Self = Self(0);
            #[doc = "Odd parity"]
            pub const ODD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Wake(pub u8);
        impl Wake {
            #[doc = "USART wakeup on idle line"]
            pub const IDLELINE: Self = Self(0);
            #[doc = "USART wakeup on address mark"]
            pub const ADDRESSMARK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct M(pub u8);
        impl M {
            #[doc = "8 data bits"]
            pub const M8: Self = Self(0);
            #[doc = "9 data bits"]
            pub const M9: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRST: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECOND: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "Steady low value on CK pin outside transmission window"]
            pub const LOW: Self = Self(0);
            #[doc = "Steady high value on CK pin outside transmission window"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Irlp(pub u8);
        impl Irlp {
            #[doc = "Normal mode"]
            pub const NORMAL: Self = Self(0);
            #[doc = "Low-power mode"]
            pub const LOWPOWER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hdsel(pub u8);
        impl Hdsel {
            #[doc = "Half duplex mode is not selected"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Half duplex mode is selected"]
            pub const HALFDUPLEX: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sbk(pub u8);
        impl Sbk {
            #[doc = "No break character is transmitted"]
            pub const NOBREAK: Self = Self(0);
            #[doc = "Break character transmitted"]
            pub const BREAK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lbdl(pub u8);
        impl Lbdl {
            #[doc = "10-bit break detection"]
            pub const LBDL10: Self = Self(0);
            #[doc = "11-bit break detection"]
            pub const LBDL11: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Stop(pub u8);
        impl Stop {
            #[doc = "1 stop bit"]
            pub const STOP1: Self = Self(0);
            #[doc = "0.5 stop bits"]
            pub const STOP0P5: Self = Self(0x01);
            #[doc = "2 stop bits"]
            pub const STOP2: Self = Self(0x02);
            #[doc = "1.5 stop bits"]
            pub const STOP1P5: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rwu(pub u8);
        impl Rwu {
            #[doc = "Receiver in active mode"]
            pub const ACTIVE: Self = Self(0);
            #[doc = "Receiver in mute mode"]
            pub const MUTE: Self = Self(0x01);
        }
    }
}
pub mod dma_v1 {
    use crate::generic::*;
    #[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[derive(Copy, Clone)]
    pub struct Ch(pub *mut u8);
    unsafe impl Send for Ch {}
    unsafe impl Sync for Ch {}
    impl Ch {
        #[doc = "DMA channel configuration register (DMA_CCR)"]
        pub fn cr(self) -> Reg<regs::Cr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "DMA channel 1 number of data register"]
        pub fn ndtr(self) -> Reg<regs::Ndtr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "DMA channel 1 peripheral address register"]
        pub fn par(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA channel 1 memory address register"]
        pub fn mar(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
    }
    #[doc = "DMA controller"]
    #[derive(Copy, Clone)]
    pub struct Dma(pub *mut u8);
    unsafe impl Send for Dma {}
    unsafe impl Sync for Dma {}
    impl Dma {
        #[doc = "DMA interrupt status register (DMA_ISR)"]
        pub fn isr(self) -> Reg<regs::Isr, R> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
        pub fn ifcr(self) -> Reg<regs::Ifcr, W> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
        pub fn ch(self, n: usize) -> Ch {
            assert!(n < 7usize);
            unsafe { Ch(self.0.add(8usize + n * 20usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "DMA channel configuration register (DMA_CCR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Channel enable"]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Channel enable"]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Transfer complete interrupt enable"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer complete interrupt enable"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Half Transfer interrupt enable"]
            pub const fn htie(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Half Transfer interrupt enable"]
            pub fn set_htie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Transfer error interrupt enable"]
            pub const fn teie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer error interrupt enable"]
            pub fn set_teie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Data transfer direction"]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Dir(val as u8)
            }
            #[doc = "Data transfer direction"]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Circular mode"]
            pub const fn circ(&self) -> super::vals::Circ {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Circ(val as u8)
            }
            #[doc = "Circular mode"]
            pub fn set_circ(&mut self, val: super::vals::Circ) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "Peripheral increment mode"]
            pub const fn pinc(&self) -> super::vals::Inc {
                let val = (self.0 >> 6usize) & 0x01;
                super::vals::Inc(val as u8)
            }
            #[doc = "Peripheral increment mode"]
            pub fn set_pinc(&mut self, val: super::vals::Inc) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
            }
            #[doc = "Memory increment mode"]
            pub const fn minc(&self) -> super::vals::Inc {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Inc(val as u8)
            }
            #[doc = "Memory increment mode"]
            pub fn set_minc(&mut self, val: super::vals::Inc) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Peripheral size"]
            pub const fn psize(&self) -> super::vals::Size {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Size(val as u8)
            }
            #[doc = "Peripheral size"]
            pub fn set_psize(&mut self, val: super::vals::Size) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.0 as u32) & 0x03) << 8usize);
            }
            #[doc = "Memory size"]
            pub const fn msize(&self) -> super::vals::Size {
                let val = (self.0 >> 10usize) & 0x03;
                super::vals::Size(val as u8)
            }
            #[doc = "Memory size"]
            pub fn set_msize(&mut self, val: super::vals::Size) {
                self.0 = (self.0 & !(0x03 << 10usize)) | (((val.0 as u32) & 0x03) << 10usize);
            }
            #[doc = "Channel Priority level"]
            pub const fn pl(&self) -> super::vals::Pl {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Pl(val as u8)
            }
            #[doc = "Channel Priority level"]
            pub fn set_pl(&mut self, val: super::vals::Pl) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "Memory to memory mode"]
            pub const fn mem2mem(&self) -> super::vals::Memmem {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Memmem(val as u8)
            }
            #[doc = "Memory to memory mode"]
            pub fn set_mem2mem(&mut self, val: super::vals::Memmem) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ifcr(pub u32);
        impl Ifcr {
            #[doc = "Channel 1 Global interrupt clear"]
            pub fn cgif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Global interrupt clear"]
            pub fn set_cgif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Channel 1 Transfer Complete clear"]
            pub fn ctcif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 1usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Transfer Complete clear"]
            pub fn set_ctcif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 1usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Channel 1 Half Transfer clear"]
            pub fn chtif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 2usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Half Transfer clear"]
            pub fn set_chtif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 2usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Channel 1 Transfer Error clear"]
            pub fn cteif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 3usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Transfer Error clear"]
            pub fn set_cteif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 3usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Ifcr {
            fn default() -> Ifcr {
                Ifcr(0)
            }
        }
        #[doc = "DMA channel 1 number of data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ndtr(pub u32);
        impl Ndtr {
            #[doc = "Number of data to transfer"]
            pub const fn ndt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of data to transfer"]
            pub fn set_ndt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Ndtr {
            fn default() -> Ndtr {
                Ndtr(0)
            }
        }
        #[doc = "DMA interrupt status register (DMA_ISR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Isr(pub u32);
        impl Isr {
            #[doc = "Channel 1 Global interrupt flag"]
            pub fn gif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Global interrupt flag"]
            pub fn set_gif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Channel 1 Transfer Complete flag"]
            pub fn tcif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 1usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Transfer Complete flag"]
            pub fn set_tcif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 1usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Channel 1 Half Transfer Complete flag"]
            pub fn htif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 2usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Half Transfer Complete flag"]
            pub fn set_htif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 2usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Channel 1 Transfer Error flag"]
            pub fn teif(&self, n: usize) -> bool {
                assert!(n < 7usize);
                let offs = 3usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Channel 1 Transfer Error flag"]
            pub fn set_teif(&mut self, n: usize, val: bool) {
                assert!(n < 7usize);
                let offs = 3usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Isr {
            fn default() -> Isr {
                Isr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Inc(pub u8);
        impl Inc {
            #[doc = "Increment mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Increment mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Memmem(pub u8);
        impl Memmem {
            #[doc = "Memory to memory mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Memory to memory mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Size(pub u8);
        impl Size {
            #[doc = "8-bit size"]
            pub const BITS8: Self = Self(0);
            #[doc = "16-bit size"]
            pub const BITS16: Self = Self(0x01);
            #[doc = "32-bit size"]
            pub const BITS32: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dir(pub u8);
        impl Dir {
            #[doc = "Read from peripheral"]
            pub const FROMPERIPHERAL: Self = Self(0);
            #[doc = "Read from memory"]
            pub const FROMMEMORY: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Circ(pub u8);
        impl Circ {
            #[doc = "Circular buffer disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Circular buffer enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pl(pub u8);
        impl Pl {
            #[doc = "Low priority"]
            pub const LOW: Self = Self(0);
            #[doc = "Medium priority"]
            pub const MEDIUM: Self = Self(0x01);
            #[doc = "High priority"]
            pub const HIGH: Self = Self(0x02);
            #[doc = "Very high priority"]
            pub const VERYHIGH: Self = Self(0x03);
        }
    }
}
pub mod timer_v1 {
    use crate::generic::*;
    #[doc = "Basic timer"]
    #[derive(Copy, Clone)]
    pub struct TimBasic(pub *mut u8);
    unsafe impl Send for TimBasic {}
    unsafe impl Sync for TimBasic {}
    impl TimBasic {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Basic, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Basic, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierBasic, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrBasic, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrBasic, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt16, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr16, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
    }
    #[doc = "Advanced-timers"]
    #[derive(Copy, Clone)]
    pub struct TimAdv(pub *mut u8);
    unsafe impl Send for TimAdv {}
    unsafe impl Sync for TimAdv {}
    impl TimAdv {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Adv, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "slave mode control register"]
        pub fn smcr(self) -> Reg<regs::Smcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierAdv, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrAdv, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrAdv, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        pub fn ccmr_input(self, n: usize) -> Reg<regs::CcmrInput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare mode register 1 (output mode)"]
        pub fn ccmr_output(self, n: usize) -> Reg<regs::CcmrOutput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare enable register"]
        pub fn ccer(self) -> Reg<regs::CcerAdv, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt16, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr16, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "repetition counter register"]
        pub fn rcr(self) -> Reg<regs::Rcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "capture/compare register"]
        pub fn ccr(self, n: usize) -> Reg<regs::Ccr16, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(52usize + n * 4usize)) }
        }
        #[doc = "break and dead-time register"]
        pub fn bdtr(self) -> Reg<regs::Bdtr, RW> {
            unsafe { Reg::from_ptr(self.0.add(68usize)) }
        }
        #[doc = "DMA control register"]
        pub fn dcr(self) -> Reg<regs::Dcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "DMA address for full transfer"]
        pub fn dmar(self) -> Reg<regs::Dmar, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    #[doc = "General purpose 16-bit timer"]
    #[derive(Copy, Clone)]
    pub struct TimGp16(pub *mut u8);
    unsafe impl Send for TimGp16 {}
    unsafe impl Sync for TimGp16 {}
    impl TimGp16 {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "slave mode control register"]
        pub fn smcr(self) -> Reg<regs::Smcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrGp, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        pub fn ccmr_input(self, n: usize) -> Reg<regs::CcmrInput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare mode register 1 (output mode)"]
        pub fn ccmr_output(self, n: usize) -> Reg<regs::CcmrOutput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare enable register"]
        pub fn ccer(self) -> Reg<regs::CcerGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt16, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr16, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "capture/compare register"]
        pub fn ccr(self, n: usize) -> Reg<regs::Ccr16, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(52usize + n * 4usize)) }
        }
        #[doc = "DMA control register"]
        pub fn dcr(self) -> Reg<regs::Dcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "DMA address for full transfer"]
        pub fn dmar(self) -> Reg<regs::Dmar, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    #[doc = "General purpose 32-bit timer"]
    #[derive(Copy, Clone)]
    pub struct TimGp32(pub *mut u8);
    unsafe impl Send for TimGp32 {}
    unsafe impl Sync for TimGp32 {}
    impl TimGp32 {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "slave mode control register"]
        pub fn smcr(self) -> Reg<regs::Smcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrGp, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        pub fn ccmr_input(self, n: usize) -> Reg<regs::CcmrInput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare mode register 1 (output mode)"]
        pub fn ccmr_output(self, n: usize) -> Reg<regs::CcmrOutput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare enable register"]
        pub fn ccer(self) -> Reg<regs::CcerGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt32, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr32, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "capture/compare register"]
        pub fn ccr(self, n: usize) -> Reg<regs::Ccr32, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(52usize + n * 4usize)) }
        }
        #[doc = "DMA control register"]
        pub fn dcr(self) -> Reg<regs::Dcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "DMA address for full transfer"]
        pub fn dmar(self) -> Reg<regs::Dmar, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "DMA/Interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DierGp(pub u32);
        impl DierGp {
            #[doc = "Update interrupt enable"]
            pub const fn uie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update interrupt enable"]
            pub fn set_uie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn ccie(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn set_ccie(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Trigger interrupt enable"]
            pub const fn tie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger interrupt enable"]
            pub fn set_tie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Update DMA request enable"]
            pub const fn ude(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Update DMA request enable"]
            pub fn set_ude(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn ccde(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn set_ccde(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Trigger DMA request enable"]
            pub const fn tde(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger DMA request enable"]
            pub fn set_tde(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for DierGp {
            fn default() -> DierGp {
                DierGp(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2Gp(pub u32);
        impl Cr2Gp {
            #[doc = "Capture/compare DMA selection"]
            pub const fn ccds(&self) -> super::vals::Ccds {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Ccds(val as u8)
            }
            #[doc = "Capture/compare DMA selection"]
            pub fn set_ccds(&mut self, val: super::vals::Ccds) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Master mode selection"]
            pub const fn mms(&self) -> super::vals::Mms {
                let val = (self.0 >> 4usize) & 0x07;
                super::vals::Mms(val as u8)
            }
            #[doc = "Master mode selection"]
            pub fn set_mms(&mut self, val: super::vals::Mms) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val.0 as u32) & 0x07) << 4usize);
            }
            #[doc = "TI1 selection"]
            pub const fn ti1s(&self) -> super::vals::Tis {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Tis(val as u8)
            }
            #[doc = "TI1 selection"]
            pub fn set_ti1s(&mut self, val: super::vals::Tis) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cr2Gp {
            fn default() -> Cr2Gp {
                Cr2Gp(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SrBasic(pub u32);
        impl SrBasic {
            #[doc = "Update interrupt flag"]
            pub const fn uif(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update interrupt flag"]
            pub fn set_uif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for SrBasic {
            fn default() -> SrBasic {
                SrBasic(0)
            }
        }
        #[doc = "event generation register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EgrBasic(pub u32);
        impl EgrBasic {
            #[doc = "Update generation"]
            pub const fn ug(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update generation"]
            pub fn set_ug(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for EgrBasic {
            fn default() -> EgrBasic {
                EgrBasic(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2Basic(pub u32);
        impl Cr2Basic {
            #[doc = "Master mode selection"]
            pub const fn mms(&self) -> super::vals::Mms {
                let val = (self.0 >> 4usize) & 0x07;
                super::vals::Mms(val as u8)
            }
            #[doc = "Master mode selection"]
            pub fn set_mms(&mut self, val: super::vals::Mms) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val.0 as u32) & 0x07) << 4usize);
            }
        }
        impl Default for Cr2Basic {
            fn default() -> Cr2Basic {
                Cr2Basic(0)
            }
        }
        #[doc = "DMA/Interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DierBasic(pub u32);
        impl DierBasic {
            #[doc = "Update interrupt enable"]
            pub const fn uie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update interrupt enable"]
            pub fn set_uie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Update DMA request enable"]
            pub const fn ude(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Update DMA request enable"]
            pub fn set_ude(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for DierBasic {
            fn default() -> DierBasic {
                DierBasic(0)
            }
        }
        #[doc = "DMA control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcr(pub u32);
        impl Dcr {
            #[doc = "DMA base address"]
            pub const fn dba(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "DMA base address"]
            pub fn set_dba(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "DMA burst length"]
            pub const fn dbl(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "DMA burst length"]
            pub fn set_dbl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
            }
        }
        impl Default for Dcr {
            fn default() -> Dcr {
                Dcr(0)
            }
        }
        #[doc = "capture/compare register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ccr32(pub u32);
        impl Ccr32 {
            #[doc = "Capture/Compare 1 value"]
            pub const fn ccr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Capture/Compare 1 value"]
            pub fn set_ccr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Ccr32 {
            fn default() -> Ccr32 {
                Ccr32(0)
            }
        }
        #[doc = "capture/compare register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ccr16(pub u32);
        impl Ccr16 {
            #[doc = "Capture/Compare 1 value"]
            pub const fn ccr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Capture/Compare 1 value"]
            pub fn set_ccr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Ccr16 {
            fn default() -> Ccr16 {
                Ccr16(0)
            }
        }
        #[doc = "counter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cnt32(pub u32);
        impl Cnt32 {
            #[doc = "counter value"]
            pub const fn cnt(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "counter value"]
            pub fn set_cnt(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cnt32 {
            fn default() -> Cnt32 {
                Cnt32(0)
            }
        }
        #[doc = "auto-reload register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Arr16(pub u32);
        impl Arr16 {
            #[doc = "Auto-reload value"]
            pub const fn arr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Auto-reload value"]
            pub fn set_arr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Arr16 {
            fn default() -> Arr16 {
                Arr16(0)
            }
        }
        #[doc = "repetition counter register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rcr(pub u32);
        impl Rcr {
            #[doc = "Repetition counter value"]
            pub const fn rep(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Repetition counter value"]
            pub fn set_rep(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rcr {
            fn default() -> Rcr {
                Rcr(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SrGp(pub u32);
        impl SrGp {
            #[doc = "Update interrupt flag"]
            pub const fn uif(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update interrupt flag"]
            pub fn set_uif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn ccif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn set_ccif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "COM interrupt flag"]
            pub const fn comif(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "COM interrupt flag"]
            pub fn set_comif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Trigger interrupt flag"]
            pub const fn tif(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger interrupt flag"]
            pub fn set_tif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Break interrupt flag"]
            pub const fn bif(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Break interrupt flag"]
            pub fn set_bif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn ccof(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn set_ccof(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for SrGp {
            fn default() -> SrGp {
                SrGp(0)
            }
        }
        #[doc = "break and dead-time register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bdtr(pub u32);
        impl Bdtr {
            #[doc = "Dead-time generator setup"]
            pub const fn dtg(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Dead-time generator setup"]
            pub fn set_dtg(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Lock configuration"]
            pub const fn lock(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x03;
                val as u8
            }
            #[doc = "Lock configuration"]
            pub fn set_lock(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
            }
            #[doc = "Off-state selection for Idle mode"]
            pub const fn ossi(&self) -> super::vals::Ossi {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Ossi(val as u8)
            }
            #[doc = "Off-state selection for Idle mode"]
            pub fn set_ossi(&mut self, val: super::vals::Ossi) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Off-state selection for Run mode"]
            pub const fn ossr(&self) -> super::vals::Ossr {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Ossr(val as u8)
            }
            #[doc = "Off-state selection for Run mode"]
            pub fn set_ossr(&mut self, val: super::vals::Ossr) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "Break enable"]
            pub const fn bke(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Break enable"]
            pub fn set_bke(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Break polarity"]
            pub const fn bkp(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Break polarity"]
            pub fn set_bkp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Automatic output enable"]
            pub const fn aoe(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Automatic output enable"]
            pub fn set_aoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Main output enable"]
            pub const fn moe(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Main output enable"]
            pub fn set_moe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Bdtr {
            fn default() -> Bdtr {
                Bdtr(0)
            }
        }
        #[doc = "event generation register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EgrAdv(pub u32);
        impl EgrAdv {
            #[doc = "Update generation"]
            pub const fn ug(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update generation"]
            pub fn set_ug(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn ccg(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn set_ccg(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare control update generation"]
            pub const fn comg(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare control update generation"]
            pub fn set_comg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Trigger generation"]
            pub const fn tg(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger generation"]
            pub fn set_tg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Break generation"]
            pub const fn bg(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Break generation"]
            pub fn set_bg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for EgrAdv {
            fn default() -> EgrAdv {
                EgrAdv(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1Gp(pub u32);
        impl Cr1Gp {
            #[doc = "Counter enable"]
            pub const fn cen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Counter enable"]
            pub fn set_cen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Update disable"]
            pub const fn udis(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Update disable"]
            pub fn set_udis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Update request source"]
            pub const fn urs(&self) -> super::vals::Urs {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Urs(val as u8)
            }
            #[doc = "Update request source"]
            pub fn set_urs(&mut self, val: super::vals::Urs) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "One-pulse mode"]
            pub const fn opm(&self) -> super::vals::Opm {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Opm(val as u8)
            }
            #[doc = "One-pulse mode"]
            pub fn set_opm(&mut self, val: super::vals::Opm) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Direction"]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Dir(val as u8)
            }
            #[doc = "Direction"]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Center-aligned mode selection"]
            pub const fn cms(&self) -> super::vals::Cms {
                let val = (self.0 >> 5usize) & 0x03;
                super::vals::Cms(val as u8)
            }
            #[doc = "Center-aligned mode selection"]
            pub fn set_cms(&mut self, val: super::vals::Cms) {
                self.0 = (self.0 & !(0x03 << 5usize)) | (((val.0 as u32) & 0x03) << 5usize);
            }
            #[doc = "Auto-reload preload enable"]
            pub const fn arpe(&self) -> super::vals::Arpe {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Arpe(val as u8)
            }
            #[doc = "Auto-reload preload enable"]
            pub fn set_arpe(&mut self, val: super::vals::Arpe) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Clock division"]
            pub const fn ckd(&self) -> super::vals::Ckd {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Ckd(val as u8)
            }
            #[doc = "Clock division"]
            pub fn set_ckd(&mut self, val: super::vals::Ckd) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.0 as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Cr1Gp {
            fn default() -> Cr1Gp {
                Cr1Gp(0)
            }
        }
        #[doc = "capture/compare enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CcerAdv(pub u32);
        impl CcerAdv {
            #[doc = "Capture/Compare 1 output enable"]
            pub fn cce(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output enable"]
            pub fn set_cce(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 complementary output enable"]
            pub fn ccne(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 2usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 complementary output enable"]
            pub fn set_ccne(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 2usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccnp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 3usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccnp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 3usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for CcerAdv {
            fn default() -> CcerAdv {
                CcerAdv(0)
            }
        }
        #[doc = "slave mode control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Smcr(pub u32);
        impl Smcr {
            #[doc = "Slave mode selection"]
            pub const fn sms(&self) -> super::vals::Sms {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Sms(val as u8)
            }
            #[doc = "Slave mode selection"]
            pub fn set_sms(&mut self, val: super::vals::Sms) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
            }
            #[doc = "Trigger selection"]
            pub const fn ts(&self) -> super::vals::Ts {
                let val = (self.0 >> 4usize) & 0x07;
                super::vals::Ts(val as u8)
            }
            #[doc = "Trigger selection"]
            pub fn set_ts(&mut self, val: super::vals::Ts) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val.0 as u32) & 0x07) << 4usize);
            }
            #[doc = "Master/Slave mode"]
            pub const fn msm(&self) -> super::vals::Msm {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Msm(val as u8)
            }
            #[doc = "Master/Slave mode"]
            pub fn set_msm(&mut self, val: super::vals::Msm) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "External trigger filter"]
            pub const fn etf(&self) -> super::vals::Etf {
                let val = (self.0 >> 8usize) & 0x0f;
                super::vals::Etf(val as u8)
            }
            #[doc = "External trigger filter"]
            pub fn set_etf(&mut self, val: super::vals::Etf) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val.0 as u32) & 0x0f) << 8usize);
            }
            #[doc = "External trigger prescaler"]
            pub const fn etps(&self) -> super::vals::Etps {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Etps(val as u8)
            }
            #[doc = "External trigger prescaler"]
            pub fn set_etps(&mut self, val: super::vals::Etps) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "External clock enable"]
            pub const fn ece(&self) -> super::vals::Ece {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Ece(val as u8)
            }
            #[doc = "External clock enable"]
            pub fn set_ece(&mut self, val: super::vals::Ece) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "External trigger polarity"]
            pub const fn etp(&self) -> super::vals::Etp {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Etp(val as u8)
            }
            #[doc = "External trigger polarity"]
            pub fn set_etp(&mut self, val: super::vals::Etp) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Smcr {
            fn default() -> Smcr {
                Smcr(0)
            }
        }
        #[doc = "DMA address for full transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmar(pub u32);
        impl Dmar {
            #[doc = "DMA register for burst accesses"]
            pub const fn dmab(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "DMA register for burst accesses"]
            pub fn set_dmab(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dmar {
            fn default() -> Dmar {
                Dmar(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1Basic(pub u32);
        impl Cr1Basic {
            #[doc = "Counter enable"]
            pub const fn cen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Counter enable"]
            pub fn set_cen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Update disable"]
            pub const fn udis(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Update disable"]
            pub fn set_udis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Update request source"]
            pub const fn urs(&self) -> super::vals::Urs {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Urs(val as u8)
            }
            #[doc = "Update request source"]
            pub fn set_urs(&mut self, val: super::vals::Urs) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "One-pulse mode"]
            pub const fn opm(&self) -> super::vals::Opm {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Opm(val as u8)
            }
            #[doc = "One-pulse mode"]
            pub fn set_opm(&mut self, val: super::vals::Opm) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Auto-reload preload enable"]
            pub const fn arpe(&self) -> super::vals::Arpe {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Arpe(val as u8)
            }
            #[doc = "Auto-reload preload enable"]
            pub fn set_arpe(&mut self, val: super::vals::Arpe) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cr1Basic {
            fn default() -> Cr1Basic {
                Cr1Basic(0)
            }
        }
        #[doc = "counter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cnt16(pub u32);
        impl Cnt16 {
            #[doc = "counter value"]
            pub const fn cnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "counter value"]
            pub fn set_cnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Cnt16 {
            fn default() -> Cnt16 {
                Cnt16(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2Adv(pub u32);
        impl Cr2Adv {
            #[doc = "Capture/compare preloaded control"]
            pub const fn ccpc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare preloaded control"]
            pub fn set_ccpc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/compare control update selection"]
            pub const fn ccus(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare control update selection"]
            pub fn set_ccus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Capture/compare DMA selection"]
            pub const fn ccds(&self) -> super::vals::Ccds {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Ccds(val as u8)
            }
            #[doc = "Capture/compare DMA selection"]
            pub fn set_ccds(&mut self, val: super::vals::Ccds) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Master mode selection"]
            pub const fn mms(&self) -> super::vals::Mms {
                let val = (self.0 >> 4usize) & 0x07;
                super::vals::Mms(val as u8)
            }
            #[doc = "Master mode selection"]
            pub fn set_mms(&mut self, val: super::vals::Mms) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val.0 as u32) & 0x07) << 4usize);
            }
            #[doc = "TI1 selection"]
            pub const fn ti1s(&self) -> super::vals::Tis {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Tis(val as u8)
            }
            #[doc = "TI1 selection"]
            pub fn set_ti1s(&mut self, val: super::vals::Tis) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Output Idle state 1"]
            pub fn ois(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 8usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 1"]
            pub fn set_ois(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 8usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Output Idle state 1"]
            pub const fn ois1n(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 1"]
            pub fn set_ois1n(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Output Idle state 2"]
            pub const fn ois2n(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 2"]
            pub fn set_ois2n(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Output Idle state 3"]
            pub const fn ois3n(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 3"]
            pub fn set_ois3n(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Cr2Adv {
            fn default() -> Cr2Adv {
                Cr2Adv(0)
            }
        }
        #[doc = "DMA/Interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DierAdv(pub u32);
        impl DierAdv {
            #[doc = "Update interrupt enable"]
            pub const fn uie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update interrupt enable"]
            pub fn set_uie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn ccie(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn set_ccie(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "COM interrupt enable"]
            pub const fn comie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "COM interrupt enable"]
            pub fn set_comie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Trigger interrupt enable"]
            pub const fn tie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger interrupt enable"]
            pub fn set_tie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Break interrupt enable"]
            pub const fn bie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Break interrupt enable"]
            pub fn set_bie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Update DMA request enable"]
            pub const fn ude(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Update DMA request enable"]
            pub fn set_ude(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn ccde(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn set_ccde(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "COM DMA request enable"]
            pub const fn comde(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "COM DMA request enable"]
            pub fn set_comde(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Trigger DMA request enable"]
            pub const fn tde(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger DMA request enable"]
            pub fn set_tde(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for DierAdv {
            fn default() -> DierAdv {
                DierAdv(0)
            }
        }
        #[doc = "event generation register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EgrGp(pub u32);
        impl EgrGp {
            #[doc = "Update generation"]
            pub const fn ug(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update generation"]
            pub fn set_ug(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn ccg(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn set_ccg(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare control update generation"]
            pub const fn comg(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare control update generation"]
            pub fn set_comg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Trigger generation"]
            pub const fn tg(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger generation"]
            pub fn set_tg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Break generation"]
            pub const fn bg(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Break generation"]
            pub fn set_bg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for EgrGp {
            fn default() -> EgrGp {
                EgrGp(0)
            }
        }
        #[doc = "capture/compare enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CcerGp(pub u32);
        impl CcerGp {
            #[doc = "Capture/Compare 1 output enable"]
            pub fn cce(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output enable"]
            pub fn set_cce(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccnp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 3usize + n * 4usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccnp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 3usize + n * 4usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for CcerGp {
            fn default() -> CcerGp {
                CcerGp(0)
            }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CcmrInput(pub u32);
        impl CcmrInput {
            #[doc = "Capture/Compare 1 selection"]
            pub fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
                assert!(n < 2usize);
                let offs = 0usize + n * 8usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::CcmrInputCcs(val as u8)
            }
            #[doc = "Capture/Compare 1 selection"]
            pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
                assert!(n < 2usize);
                let offs = 0usize + n * 8usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
            #[doc = "Input capture 1 prescaler"]
            pub fn icpsc(&self, n: usize) -> u8 {
                assert!(n < 2usize);
                let offs = 2usize + n * 8usize;
                let val = (self.0 >> offs) & 0x03;
                val as u8
            }
            #[doc = "Input capture 1 prescaler"]
            pub fn set_icpsc(&mut self, n: usize, val: u8) {
                assert!(n < 2usize);
                let offs = 2usize + n * 8usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
            }
            #[doc = "Input capture 1 filter"]
            pub fn icf(&self, n: usize) -> super::vals::Icf {
                assert!(n < 2usize);
                let offs = 4usize + n * 8usize;
                let val = (self.0 >> offs) & 0x0f;
                super::vals::Icf(val as u8)
            }
            #[doc = "Input capture 1 filter"]
            pub fn set_icf(&mut self, n: usize, val: super::vals::Icf) {
                assert!(n < 2usize);
                let offs = 4usize + n * 8usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val.0 as u32) & 0x0f) << offs);
            }
        }
        impl Default for CcmrInput {
            fn default() -> CcmrInput {
                CcmrInput(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SrAdv(pub u32);
        impl SrAdv {
            #[doc = "Update interrupt flag"]
            pub const fn uif(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Update interrupt flag"]
            pub fn set_uif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn ccif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn set_ccif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "COM interrupt flag"]
            pub const fn comif(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "COM interrupt flag"]
            pub fn set_comif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Trigger interrupt flag"]
            pub const fn tif(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Trigger interrupt flag"]
            pub fn set_tif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Break interrupt flag"]
            pub const fn bif(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Break interrupt flag"]
            pub fn set_bif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn ccof(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn set_ccof(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 9usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for SrAdv {
            fn default() -> SrAdv {
                SrAdv(0)
            }
        }
        #[doc = "capture/compare mode register 2 (output mode)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CcmrOutput(pub u32);
        impl CcmrOutput {
            #[doc = "Capture/Compare 3 selection"]
            pub fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
                assert!(n < 2usize);
                let offs = 0usize + n * 8usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::CcmrOutputCcs(val as u8)
            }
            #[doc = "Capture/Compare 3 selection"]
            pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
                assert!(n < 2usize);
                let offs = 0usize + n * 8usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
            #[doc = "Output compare 3 fast enable"]
            pub fn ocfe(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 2usize + n * 8usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Output compare 3 fast enable"]
            pub fn set_ocfe(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 2usize + n * 8usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Output compare 3 preload enable"]
            pub fn ocpe(&self, n: usize) -> super::vals::Ocpe {
                assert!(n < 2usize);
                let offs = 3usize + n * 8usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ocpe(val as u8)
            }
            #[doc = "Output compare 3 preload enable"]
            pub fn set_ocpe(&mut self, n: usize, val: super::vals::Ocpe) {
                assert!(n < 2usize);
                let offs = 3usize + n * 8usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Output compare 3 mode"]
            pub fn ocm(&self, n: usize) -> super::vals::Ocm {
                assert!(n < 2usize);
                let offs = 4usize + n * 8usize;
                let val = (self.0 >> offs) & 0x07;
                super::vals::Ocm(val as u8)
            }
            #[doc = "Output compare 3 mode"]
            pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
                assert!(n < 2usize);
                let offs = 4usize + n * 8usize;
                self.0 = (self.0 & !(0x07 << offs)) | (((val.0 as u32) & 0x07) << offs);
            }
            #[doc = "Output compare 3 clear enable"]
            pub fn occe(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 7usize + n * 8usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Output compare 3 clear enable"]
            pub fn set_occe(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 7usize + n * 8usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for CcmrOutput {
            fn default() -> CcmrOutput {
                CcmrOutput(0)
            }
        }
        #[doc = "auto-reload register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Arr32(pub u32);
        impl Arr32 {
            #[doc = "Auto-reload value"]
            pub const fn arr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Auto-reload value"]
            pub fn set_arr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Arr32 {
            fn default() -> Arr32 {
                Arr32(0)
            }
        }
        #[doc = "prescaler"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psc(pub u32);
        impl Psc {
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Psc {
            fn default() -> Psc {
                Psc(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct CcmrOutputCcs(pub u8);
        impl CcmrOutputCcs {
            #[doc = "CCx channel is configured as output"]
            pub const OUTPUT: Self = Self(0);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ts(pub u8);
        impl Ts {
            #[doc = "Internal Trigger 0 (ITR0)"]
            pub const ITR0: Self = Self(0);
            #[doc = "Internal Trigger 1 (ITR1)"]
            pub const ITR1: Self = Self(0x01);
            #[doc = "Internal Trigger 2 (ITR2)"]
            pub const ITR2: Self = Self(0x02);
            #[doc = "TI1 Edge Detector (TI1F_ED)"]
            pub const TI1F_ED: Self = Self(0x04);
            #[doc = "Filtered Timer Input 1 (TI1FP1)"]
            pub const TI1FP1: Self = Self(0x05);
            #[doc = "Filtered Timer Input 2 (TI2FP2)"]
            pub const TI2FP2: Self = Self(0x06);
            #[doc = "External Trigger input (ETRF)"]
            pub const ETRF: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cms(pub u8);
        impl Cms {
            #[doc = "The counter counts up or down depending on the direction bit"]
            pub const EDGEALIGNED: Self = Self(0);
            #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
            pub const CENTERALIGNED1: Self = Self(0x01);
            #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
            pub const CENTERALIGNED2: Self = Self(0x02);
            #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
            pub const CENTERALIGNED3: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ece(pub u8);
        impl Ece {
            #[doc = "External clock mode 2 disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Urs(pub u8);
        impl Urs {
            #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
            pub const ANYEVENT: Self = Self(0);
            #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
            pub const COUNTERONLY: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Opm(pub u8);
        impl Opm {
            #[doc = "Counter is not stopped at update event"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ocpe(pub u8);
        impl Ocpe {
            #[doc = "Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Preload register on CCR2 enabled. Preload value is loaded into active register on each update event"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ccds(pub u8);
        impl Ccds {
            #[doc = "CCx DMA request sent when CCx event occurs"]
            pub const ONCOMPARE: Self = Self(0);
            #[doc = "CCx DMA request sent when update event occurs"]
            pub const ONUPDATE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct CcmrInputCcs(pub u8);
        impl CcmrInputCcs {
            #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
            pub const TI4: Self = Self(0x01);
            #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
            pub const TI3: Self = Self(0x02);
            #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
            pub const TRC: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Arpe(pub u8);
        impl Arpe {
            #[doc = "TIMx_APRR register is not buffered"]
            pub const DISABLED: Self = Self(0);
            #[doc = "TIMx_APRR register is buffered"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mms(pub u8);
        impl Mms {
            #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
            pub const RESET: Self = Self(0);
            #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
            pub const ENABLE: Self = Self(0x01);
            #[doc = "The update event is selected as trigger output"]
            pub const UPDATE: Self = Self(0x02);
            #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
            pub const COMPAREPULSE: Self = Self(0x03);
            #[doc = "OC1REF signal is used as trigger output"]
            pub const COMPAREOC1: Self = Self(0x04);
            #[doc = "OC2REF signal is used as trigger output"]
            pub const COMPAREOC2: Self = Self(0x05);
            #[doc = "OC3REF signal is used as trigger output"]
            pub const COMPAREOC3: Self = Self(0x06);
            #[doc = "OC4REF signal is used as trigger output"]
            pub const COMPAREOC4: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Msm(pub u8);
        impl Msm {
            #[doc = "No action"]
            pub const NOSYNC: Self = Self(0);
            #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
            pub const SYNC: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Etf(pub u8);
        impl Etf {
            #[doc = "No filter, sampling is done at fDTS"]
            pub const NOFILTER: Self = Self(0);
            #[doc = "fSAMPLING=fCK_INT, N=2"]
            pub const FCK_INT_N2: Self = Self(0x01);
            #[doc = "fSAMPLING=fCK_INT, N=4"]
            pub const FCK_INT_N4: Self = Self(0x02);
            #[doc = "fSAMPLING=fCK_INT, N=8"]
            pub const FCK_INT_N8: Self = Self(0x03);
            #[doc = "fSAMPLING=fDTS/2, N=6"]
            pub const FDTS_DIV2_N6: Self = Self(0x04);
            #[doc = "fSAMPLING=fDTS/2, N=8"]
            pub const FDTS_DIV2_N8: Self = Self(0x05);
            #[doc = "fSAMPLING=fDTS/4, N=6"]
            pub const FDTS_DIV4_N6: Self = Self(0x06);
            #[doc = "fSAMPLING=fDTS/4, N=8"]
            pub const FDTS_DIV4_N8: Self = Self(0x07);
            #[doc = "fSAMPLING=fDTS/8, N=6"]
            pub const FDTS_DIV8_N6: Self = Self(0x08);
            #[doc = "fSAMPLING=fDTS/8, N=8"]
            pub const FDTS_DIV8_N8: Self = Self(0x09);
            #[doc = "fSAMPLING=fDTS/16, N=5"]
            pub const FDTS_DIV16_N5: Self = Self(0x0a);
            #[doc = "fSAMPLING=fDTS/16, N=6"]
            pub const FDTS_DIV16_N6: Self = Self(0x0b);
            #[doc = "fSAMPLING=fDTS/16, N=8"]
            pub const FDTS_DIV16_N8: Self = Self(0x0c);
            #[doc = "fSAMPLING=fDTS/32, N=5"]
            pub const FDTS_DIV32_N5: Self = Self(0x0d);
            #[doc = "fSAMPLING=fDTS/32, N=6"]
            pub const FDTS_DIV32_N6: Self = Self(0x0e);
            #[doc = "fSAMPLING=fDTS/32, N=8"]
            pub const FDTS_DIV32_N8: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Icf(pub u8);
        impl Icf {
            #[doc = "No filter, sampling is done at fDTS"]
            pub const NOFILTER: Self = Self(0);
            #[doc = "fSAMPLING=fCK_INT, N=2"]
            pub const FCK_INT_N2: Self = Self(0x01);
            #[doc = "fSAMPLING=fCK_INT, N=4"]
            pub const FCK_INT_N4: Self = Self(0x02);
            #[doc = "fSAMPLING=fCK_INT, N=8"]
            pub const FCK_INT_N8: Self = Self(0x03);
            #[doc = "fSAMPLING=fDTS/2, N=6"]
            pub const FDTS_DIV2_N6: Self = Self(0x04);
            #[doc = "fSAMPLING=fDTS/2, N=8"]
            pub const FDTS_DIV2_N8: Self = Self(0x05);
            #[doc = "fSAMPLING=fDTS/4, N=6"]
            pub const FDTS_DIV4_N6: Self = Self(0x06);
            #[doc = "fSAMPLING=fDTS/4, N=8"]
            pub const FDTS_DIV4_N8: Self = Self(0x07);
            #[doc = "fSAMPLING=fDTS/8, N=6"]
            pub const FDTS_DIV8_N6: Self = Self(0x08);
            #[doc = "fSAMPLING=fDTS/8, N=8"]
            pub const FDTS_DIV8_N8: Self = Self(0x09);
            #[doc = "fSAMPLING=fDTS/16, N=5"]
            pub const FDTS_DIV16_N5: Self = Self(0x0a);
            #[doc = "fSAMPLING=fDTS/16, N=6"]
            pub const FDTS_DIV16_N6: Self = Self(0x0b);
            #[doc = "fSAMPLING=fDTS/16, N=8"]
            pub const FDTS_DIV16_N8: Self = Self(0x0c);
            #[doc = "fSAMPLING=fDTS/32, N=5"]
            pub const FDTS_DIV32_N5: Self = Self(0x0d);
            #[doc = "fSAMPLING=fDTS/32, N=6"]
            pub const FDTS_DIV32_N6: Self = Self(0x0e);
            #[doc = "fSAMPLING=fDTS/32, N=8"]
            pub const FDTS_DIV32_N8: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dir(pub u8);
        impl Dir {
            #[doc = "Counter used as upcounter"]
            pub const UP: Self = Self(0);
            #[doc = "Counter used as downcounter"]
            pub const DOWN: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Etps(pub u8);
        impl Etps {
            #[doc = "Prescaler OFF"]
            pub const DIV1: Self = Self(0);
            #[doc = "ETRP frequency divided by 2"]
            pub const DIV2: Self = Self(0x01);
            #[doc = "ETRP frequency divided by 4"]
            pub const DIV4: Self = Self(0x02);
            #[doc = "ETRP frequency divided by 8"]
            pub const DIV8: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ossr(pub u8);
        impl Ossr {
            #[doc = "When inactive, OC/OCN outputs are disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
            pub const IDLELEVEL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ocm(pub u8);
        impl Ocm {
            #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
            pub const FROZEN: Self = Self(0);
            #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
            pub const ACTIVEONMATCH: Self = Self(0x01);
            #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
            pub const INACTIVEONMATCH: Self = Self(0x02);
            #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
            pub const TOGGLE: Self = Self(0x03);
            #[doc = "OCyREF is forced low"]
            pub const FORCEINACTIVE: Self = Self(0x04);
            #[doc = "OCyREF is forced high"]
            pub const FORCEACTIVE: Self = Self(0x05);
            #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
            pub const PWMMODE1: Self = Self(0x06);
            #[doc = "Inversely to PwmMode1"]
            pub const PWMMODE2: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Tis(pub u8);
        impl Tis {
            #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
            pub const NORMAL: Self = Self(0);
            #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
            pub const XOR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Etp(pub u8);
        impl Etp {
            #[doc = "ETR is noninverted, active at high level or rising edge"]
            pub const NOTINVERTED: Self = Self(0);
            #[doc = "ETR is inverted, active at low level or falling edge"]
            pub const INVERTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ckd(pub u8);
        impl Ckd {
            #[doc = "t_DTS = t_CK_INT"]
            pub const DIV1: Self = Self(0);
            #[doc = "t_DTS = 2 × t_CK_INT"]
            pub const DIV2: Self = Self(0x01);
            #[doc = "t_DTS = 4 × t_CK_INT"]
            pub const DIV4: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sms(pub u8);
        impl Sms {
            #[doc = "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
            pub const DISABLED: Self = Self(0);
            #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
            pub const ENCODER_MODE_1: Self = Self(0x01);
            #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
            pub const ENCODER_MODE_2: Self = Self(0x02);
            #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
            pub const ENCODER_MODE_3: Self = Self(0x03);
            #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
            pub const RESET_MODE: Self = Self(0x04);
            #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
            pub const GATED_MODE: Self = Self(0x05);
            #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
            pub const TRIGGER_MODE: Self = Self(0x06);
            #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
            pub const EXT_CLOCK_MODE: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ossi(pub u8);
        impl Ossi {
            #[doc = "When inactive, OC/OCN outputs are disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
            pub const IDLELEVEL: Self = Self(0x01);
        }
    }
}
pub mod exti_v1 {
    use crate::generic::*;
    #[doc = "External interrupt/event controller"]
    #[derive(Copy, Clone)]
    pub struct Exti(pub *mut u8);
    unsafe impl Send for Exti {}
    unsafe impl Sync for Exti {}
    impl Exti {
        #[doc = "Interrupt mask register (EXTI_IMR)"]
        pub fn imr(self) -> Reg<regs::Imr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Event mask register (EXTI_EMR)"]
        pub fn emr(self) -> Reg<regs::Emr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Rising Trigger selection register (EXTI_RTSR)"]
        pub fn rtsr(self) -> Reg<regs::Rtsr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Falling Trigger selection register (EXTI_FTSR)"]
        pub fn ftsr(self) -> Reg<regs::Ftsr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Software interrupt event register (EXTI_SWIER)"]
        pub fn swier(self) -> Reg<regs::Swier, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Pending register (EXTI_PR)"]
        pub fn pr(self) -> Reg<regs::Pr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mr(pub u8);
        impl Mr {
            #[doc = "Interrupt request line is masked"]
            pub const MASKED: Self = Self(0);
            #[doc = "Interrupt request line is unmasked"]
            pub const UNMASKED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prw(pub u8);
        impl Prw {
            #[doc = "Clears pending bit"]
            pub const CLEAR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Tr(pub u8);
        impl Tr {
            #[doc = "Falling edge trigger is disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Falling edge trigger is enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prr(pub u8);
        impl Prr {
            #[doc = "No trigger request occurred"]
            pub const NOTPENDING: Self = Self(0);
            #[doc = "Selected trigger request occurred"]
            pub const PENDING: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Swierw(pub u8);
        impl Swierw {
            #[doc = "Generates an interrupt request"]
            pub const PEND: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Falling Trigger selection register (EXTI_FTSR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ftsr(pub u32);
        impl Ftsr {
            #[doc = "Falling trigger event configuration of line 0"]
            pub fn tr(&self, n: usize) -> super::vals::Tr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Tr(val as u8)
            }
            #[doc = "Falling trigger event configuration of line 0"]
            pub fn set_tr(&mut self, n: usize, val: super::vals::Tr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Ftsr {
            fn default() -> Ftsr {
                Ftsr(0)
            }
        }
        #[doc = "Rising Trigger selection register (EXTI_RTSR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rtsr(pub u32);
        impl Rtsr {
            #[doc = "Rising trigger event configuration of line 0"]
            pub fn tr(&self, n: usize) -> super::vals::Tr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Tr(val as u8)
            }
            #[doc = "Rising trigger event configuration of line 0"]
            pub fn set_tr(&mut self, n: usize, val: super::vals::Tr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Rtsr {
            fn default() -> Rtsr {
                Rtsr(0)
            }
        }
        #[doc = "Software interrupt event register (EXTI_SWIER)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Swier(pub u32);
        impl Swier {
            #[doc = "Software Interrupt on line 0"]
            pub fn swier(&self, n: usize) -> bool {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Software Interrupt on line 0"]
            pub fn set_swier(&mut self, n: usize, val: bool) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Swier {
            fn default() -> Swier {
                Swier(0)
            }
        }
        #[doc = "Event mask register (EXTI_EMR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Emr(pub u32);
        impl Emr {
            #[doc = "Event Mask on line 0"]
            pub fn mr(&self, n: usize) -> super::vals::Mr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Mr(val as u8)
            }
            #[doc = "Event Mask on line 0"]
            pub fn set_mr(&mut self, n: usize, val: super::vals::Mr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Emr {
            fn default() -> Emr {
                Emr(0)
            }
        }
        #[doc = "Interrupt mask register (EXTI_IMR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Imr(pub u32);
        impl Imr {
            #[doc = "Interrupt Mask on line 0"]
            pub fn mr(&self, n: usize) -> super::vals::Mr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Mr(val as u8)
            }
            #[doc = "Interrupt Mask on line 0"]
            pub fn set_mr(&mut self, n: usize, val: super::vals::Mr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Imr {
            fn default() -> Imr {
                Imr(0)
            }
        }
        #[doc = "Pending register (EXTI_PR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pr(pub u32);
        impl Pr {
            #[doc = "Pending bit 0"]
            pub fn pr(&self, n: usize) -> bool {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pending bit 0"]
            pub fn set_pr(&mut self, n: usize, val: bool) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Pr {
            fn default() -> Pr {
                Pr(0)
            }
        }
    }
}
pub mod spi_v2 {
    use crate::generic::*;
    #[doc = "Serial peripheral interface"]
    #[derive(Copy, Clone)]
    pub struct Spi(pub *mut u8);
    unsafe impl Send for Spi {}
    unsafe impl Sync for Spi {}
    impl Spi {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "CRC polynomial register"]
        pub fn crcpr(self) -> Reg<regs::Crcpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "RX CRC register"]
        pub fn rxcrcr(self) -> Reg<regs::Rxcrcr, R> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "TX CRC register"]
        pub fn txcrcr(self) -> Reg<regs::Txcrcr, R> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "RX CRC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrcr(pub u32);
        impl Rxcrcr {
            #[doc = "Rx CRC register"]
            pub const fn rx_crc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Rx CRC register"]
            pub fn set_rx_crc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Rxcrcr {
            fn default() -> Rxcrcr {
                Rxcrcr(0)
            }
        }
        #[doc = "TX CRC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txcrcr(pub u32);
        impl Txcrcr {
            #[doc = "Tx CRC register"]
            pub const fn tx_crc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Tx CRC register"]
            pub fn set_tx_crc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Txcrcr {
            fn default() -> Txcrcr {
                Txcrcr(0)
            }
        }
        #[doc = "data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data register"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Data register"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Receive buffer not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Receive buffer not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit buffer empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit buffer empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "CRC error flag"]
            pub const fn crcerr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "CRC error flag"]
            pub fn set_crcerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Mode fault"]
            pub const fn modf(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Mode fault"]
            pub fn set_modf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Overrun flag"]
            pub const fn ovr(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun flag"]
            pub fn set_ovr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Busy flag"]
            pub const fn bsy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Busy flag"]
            pub fn set_bsy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Frame format error"]
            pub const fn fre(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Frame format error"]
            pub fn set_fre(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "FIFO reception level"]
            pub const fn frlvl(&self) -> u8 {
                let val = (self.0 >> 9usize) & 0x03;
                val as u8
            }
            #[doc = "FIFO reception level"]
            pub fn set_frlvl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
            }
            #[doc = "FIFO Transmission Level"]
            pub const fn ftlvl(&self) -> u8 {
                let val = (self.0 >> 11usize) & 0x03;
                val as u8
            }
            #[doc = "FIFO Transmission Level"]
            pub fn set_ftlvl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Rx buffer DMA enable"]
            pub const fn rxdmaen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Rx buffer DMA enable"]
            pub fn set_rxdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Tx buffer DMA enable"]
            pub const fn txdmaen(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Tx buffer DMA enable"]
            pub fn set_txdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SS output enable"]
            pub const fn ssoe(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SS output enable"]
            pub fn set_ssoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "NSS pulse management"]
            pub const fn nssp(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "NSS pulse management"]
            pub fn set_nssp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Frame format"]
            pub const fn frf(&self) -> super::vals::Frf {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Frf(val as u8)
            }
            #[doc = "Frame format"]
            pub fn set_frf(&mut self, val: super::vals::Frf) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Error interrupt enable"]
            pub const fn errie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_errie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "RX buffer not empty interrupt enable"]
            pub const fn rxneie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer not empty interrupt enable"]
            pub fn set_rxneie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Tx buffer empty interrupt enable"]
            pub const fn txeie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Tx buffer empty interrupt enable"]
            pub fn set_txeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Data size"]
            pub const fn ds(&self) -> super::vals::Ds {
                let val = (self.0 >> 8usize) & 0x0f;
                super::vals::Ds(val as u8)
            }
            #[doc = "Data size"]
            pub fn set_ds(&mut self, val: super::vals::Ds) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val.0 as u32) & 0x0f) << 8usize);
            }
            #[doc = "FIFO reception threshold"]
            pub const fn frxth(&self) -> super::vals::Frxth {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Frxth(val as u8)
            }
            #[doc = "FIFO reception threshold"]
            pub fn set_frxth(&mut self, val: super::vals::Frxth) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Last DMA transfer for reception"]
            pub const fn ldma_rx(&self) -> super::vals::LdmaRx {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::LdmaRx(val as u8)
            }
            #[doc = "Last DMA transfer for reception"]
            pub fn set_ldma_rx(&mut self, val: super::vals::LdmaRx) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val.0 as u32) & 0x01) << 13usize);
            }
            #[doc = "Last DMA transfer for transmission"]
            pub const fn ldma_tx(&self) -> super::vals::LdmaTx {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::LdmaTx(val as u8)
            }
            #[doc = "Last DMA transfer for transmission"]
            pub fn set_ldma_tx(&mut self, val: super::vals::LdmaTx) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "CRC polynomial register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpr(pub u32);
        impl Crcpr {
            #[doc = "CRC polynomial register"]
            pub const fn crcpoly(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "CRC polynomial register"]
            pub fn set_crcpoly(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Crcpr {
            fn default() -> Crcpr {
                Crcpr(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Master selection"]
            pub const fn mstr(&self) -> super::vals::Mstr {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Mstr(val as u8)
            }
            #[doc = "Master selection"]
            pub fn set_mstr(&mut self, val: super::vals::Mstr) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Baud rate control"]
            pub const fn br(&self) -> super::vals::Br {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Br(val as u8)
            }
            #[doc = "Baud rate control"]
            pub fn set_br(&mut self, val: super::vals::Br) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.0 as u32) & 0x07) << 3usize);
            }
            #[doc = "SPI enable"]
            pub const fn spe(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SPI enable"]
            pub fn set_spe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Frame format"]
            pub const fn lsbfirst(&self) -> super::vals::Lsbfirst {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Lsbfirst(val as u8)
            }
            #[doc = "Frame format"]
            pub fn set_lsbfirst(&mut self, val: super::vals::Lsbfirst) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Internal slave select"]
            pub const fn ssi(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Internal slave select"]
            pub fn set_ssi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Software slave management"]
            pub const fn ssm(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Software slave management"]
            pub fn set_ssm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Receive only"]
            pub const fn rxonly(&self) -> super::vals::Rxonly {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Rxonly(val as u8)
            }
            #[doc = "Receive only"]
            pub fn set_rxonly(&mut self, val: super::vals::Rxonly) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "CRC length"]
            pub const fn crcl(&self) -> super::vals::Crcl {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Crcl(val as u8)
            }
            #[doc = "CRC length"]
            pub fn set_crcl(&mut self, val: super::vals::Crcl) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "CRC transfer next"]
            pub const fn crcnext(&self) -> super::vals::Crcnext {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Crcnext(val as u8)
            }
            #[doc = "CRC transfer next"]
            pub fn set_crcnext(&mut self, val: super::vals::Crcnext) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Hardware CRC calculation enable"]
            pub const fn crcen(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware CRC calculation enable"]
            pub fn set_crcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Output enable in bidirectional mode"]
            pub const fn bidioe(&self) -> super::vals::Bidioe {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Bidioe(val as u8)
            }
            #[doc = "Output enable in bidirectional mode"]
            pub fn set_bidioe(&mut self, val: super::vals::Bidioe) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "Bidirectional data mode enable"]
            pub const fn bidimode(&self) -> super::vals::Bidimode {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Bidimode(val as u8)
            }
            #[doc = "Bidirectional data mode enable"]
            pub fn set_bidimode(&mut self, val: super::vals::Bidimode) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Br(pub u8);
        impl Br {
            #[doc = "f_PCLK / 2"]
            pub const DIV2: Self = Self(0);
            #[doc = "f_PCLK / 4"]
            pub const DIV4: Self = Self(0x01);
            #[doc = "f_PCLK / 8"]
            pub const DIV8: Self = Self(0x02);
            #[doc = "f_PCLK / 16"]
            pub const DIV16: Self = Self(0x03);
            #[doc = "f_PCLK / 32"]
            pub const DIV32: Self = Self(0x04);
            #[doc = "f_PCLK / 64"]
            pub const DIV64: Self = Self(0x05);
            #[doc = "f_PCLK / 128"]
            pub const DIV128: Self = Self(0x06);
            #[doc = "f_PCLK / 256"]
            pub const DIV256: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct LdmaRx(pub u8);
        impl LdmaRx {
            #[doc = "Number of data to transfer for receive is even"]
            pub const EVEN: Self = Self(0);
            #[doc = "Number of data to transfer for receive is odd"]
            pub const ODD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "CK to 0 when idle"]
            pub const IDLELOW: Self = Self(0);
            #[doc = "CK to 1 when idle"]
            pub const IDLEHIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ftlvlr(pub u8);
        impl Ftlvlr {
            #[doc = "Tx FIFO Empty"]
            pub const EMPTY: Self = Self(0);
            #[doc = "Tx 1/4 FIFO"]
            pub const QUARTER: Self = Self(0x01);
            #[doc = "Tx 1/2 FIFO"]
            pub const HALF: Self = Self(0x02);
            #[doc = "Tx FIFO full"]
            pub const FULL: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bidioe(pub u8);
        impl Bidioe {
            #[doc = "Output disabled (receive-only mode)"]
            pub const OUTPUTDISABLED: Self = Self(0);
            #[doc = "Output enabled (transmit-only mode)"]
            pub const OUTPUTENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bidimode(pub u8);
        impl Bidimode {
            #[doc = "2-line unidirectional data mode selected"]
            pub const UNIDIRECTIONAL: Self = Self(0);
            #[doc = "1-line bidirectional data mode selected"]
            pub const BIDIRECTIONAL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct LdmaTx(pub u8);
        impl LdmaTx {
            #[doc = "Number of data to transfer for transmit is even"]
            pub const EVEN: Self = Self(0);
            #[doc = "Number of data to transfer for transmit is odd"]
            pub const ODD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRSTEDGE: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECONDEDGE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frxth(pub u8);
        impl Frxth {
            #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
            pub const HALF: Self = Self(0);
            #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
            pub const QUARTER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frer(pub u8);
        impl Frer {
            #[doc = "No frame format error"]
            pub const NOERROR: Self = Self(0);
            #[doc = "A frame format error occurred"]
            pub const ERROR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ds(pub u8);
        impl Ds {
            #[doc = "4-bit"]
            pub const FOURBIT: Self = Self(0x03);
            #[doc = "5-bit"]
            pub const FIVEBIT: Self = Self(0x04);
            #[doc = "6-bit"]
            pub const SIXBIT: Self = Self(0x05);
            #[doc = "7-bit"]
            pub const SEVENBIT: Self = Self(0x06);
            #[doc = "8-bit"]
            pub const EIGHTBIT: Self = Self(0x07);
            #[doc = "9-bit"]
            pub const NINEBIT: Self = Self(0x08);
            #[doc = "10-bit"]
            pub const TENBIT: Self = Self(0x09);
            #[doc = "11-bit"]
            pub const ELEVENBIT: Self = Self(0x0a);
            #[doc = "12-bit"]
            pub const TWELVEBIT: Self = Self(0x0b);
            #[doc = "13-bit"]
            pub const THIRTEENBIT: Self = Self(0x0c);
            #[doc = "14-bit"]
            pub const FOURTEENBIT: Self = Self(0x0d);
            #[doc = "15-bit"]
            pub const FIFTEENBIT: Self = Self(0x0e);
            #[doc = "16-bit"]
            pub const SIXTEENBIT: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frf(pub u8);
        impl Frf {
            #[doc = "SPI Motorola mode"]
            pub const MOTOROLA: Self = Self(0);
            #[doc = "SPI TI mode"]
            pub const TI: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lsbfirst(pub u8);
        impl Lsbfirst {
            #[doc = "Data is transmitted/received with the MSB first"]
            pub const MSBFIRST: Self = Self(0);
            #[doc = "Data is transmitted/received with the LSB first"]
            pub const LSBFIRST: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crcl(pub u8);
        impl Crcl {
            #[doc = "8-bit CRC length"]
            pub const EIGHTBIT: Self = Self(0);
            #[doc = "16-bit CRC length"]
            pub const SIXTEENBIT: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crcnext(pub u8);
        impl Crcnext {
            #[doc = "Next transmit value is from Tx buffer"]
            pub const TXBUFFER: Self = Self(0);
            #[doc = "Next transmit value is from Tx CRC register"]
            pub const CRC: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frlvlr(pub u8);
        impl Frlvlr {
            #[doc = "Rx FIFO Empty"]
            pub const EMPTY: Self = Self(0);
            #[doc = "Rx 1/4 FIFO"]
            pub const QUARTER: Self = Self(0x01);
            #[doc = "Rx 1/2 FIFO"]
            pub const HALF: Self = Self(0x02);
            #[doc = "Rx FIFO full"]
            pub const FULL: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxonly(pub u8);
        impl Rxonly {
            #[doc = "Full duplex (Transmit and receive)"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Output disabled (Receive-only mode)"]
            pub const OUTPUTDISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mstr(pub u8);
        impl Mstr {
            #[doc = "Slave configuration"]
            pub const SLAVE: Self = Self(0);
            #[doc = "Master configuration"]
            pub const MASTER: Self = Self(0x01);
        }
    }
}
pub mod spi_v1 {
    use crate::generic::*;
    #[doc = "Serial peripheral interface"]
    #[derive(Copy, Clone)]
    pub struct Spi(pub *mut u8);
    unsafe impl Send for Spi {}
    unsafe impl Sync for Spi {}
    impl Spi {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "CRC polynomial register"]
        pub fn crcpr(self) -> Reg<regs::Crcpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "RX CRC register"]
        pub fn rxcrcr(self) -> Reg<regs::Rxcrcr, R> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "TX CRC register"]
        pub fn txcrcr(self) -> Reg<regs::Txcrcr, R> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Receive buffer not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Receive buffer not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit buffer empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit buffer empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "CRC error flag"]
            pub const fn crcerr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "CRC error flag"]
            pub fn set_crcerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Mode fault"]
            pub const fn modf(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Mode fault"]
            pub fn set_modf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Overrun flag"]
            pub const fn ovr(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun flag"]
            pub fn set_ovr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Busy flag"]
            pub const fn bsy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Busy flag"]
            pub fn set_bsy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TI frame format error"]
            pub const fn fre(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TI frame format error"]
            pub fn set_fre(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Rx buffer DMA enable"]
            pub const fn rxdmaen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Rx buffer DMA enable"]
            pub fn set_rxdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Tx buffer DMA enable"]
            pub const fn txdmaen(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Tx buffer DMA enable"]
            pub fn set_txdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SS output enable"]
            pub const fn ssoe(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SS output enable"]
            pub fn set_ssoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Frame format"]
            pub const fn frf(&self) -> super::vals::Frf {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Frf(val as u8)
            }
            #[doc = "Frame format"]
            pub fn set_frf(&mut self, val: super::vals::Frf) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Error interrupt enable"]
            pub const fn errie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_errie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "RX buffer not empty interrupt enable"]
            pub const fn rxneie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer not empty interrupt enable"]
            pub fn set_rxneie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Tx buffer empty interrupt enable"]
            pub const fn txeie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Tx buffer empty interrupt enable"]
            pub fn set_txeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data register"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Data register"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
        #[doc = "RX CRC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrcr(pub u32);
        impl Rxcrcr {
            #[doc = "Rx CRC register"]
            pub const fn rx_crc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Rx CRC register"]
            pub fn set_rx_crc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Rxcrcr {
            fn default() -> Rxcrcr {
                Rxcrcr(0)
            }
        }
        #[doc = "CRC polynomial register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpr(pub u32);
        impl Crcpr {
            #[doc = "CRC polynomial register"]
            pub const fn crcpoly(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "CRC polynomial register"]
            pub fn set_crcpoly(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Crcpr {
            fn default() -> Crcpr {
                Crcpr(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Master selection"]
            pub const fn mstr(&self) -> super::vals::Mstr {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Mstr(val as u8)
            }
            #[doc = "Master selection"]
            pub fn set_mstr(&mut self, val: super::vals::Mstr) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Baud rate control"]
            pub const fn br(&self) -> super::vals::Br {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Br(val as u8)
            }
            #[doc = "Baud rate control"]
            pub fn set_br(&mut self, val: super::vals::Br) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.0 as u32) & 0x07) << 3usize);
            }
            #[doc = "SPI enable"]
            pub const fn spe(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SPI enable"]
            pub fn set_spe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Frame format"]
            pub const fn lsbfirst(&self) -> super::vals::Lsbfirst {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Lsbfirst(val as u8)
            }
            #[doc = "Frame format"]
            pub fn set_lsbfirst(&mut self, val: super::vals::Lsbfirst) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Internal slave select"]
            pub const fn ssi(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Internal slave select"]
            pub fn set_ssi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Software slave management"]
            pub const fn ssm(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Software slave management"]
            pub fn set_ssm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Receive only"]
            pub const fn rxonly(&self) -> super::vals::Rxonly {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Rxonly(val as u8)
            }
            #[doc = "Receive only"]
            pub fn set_rxonly(&mut self, val: super::vals::Rxonly) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Data frame format"]
            pub const fn dff(&self) -> super::vals::Dff {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Dff(val as u8)
            }
            #[doc = "Data frame format"]
            pub fn set_dff(&mut self, val: super::vals::Dff) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "CRC transfer next"]
            pub const fn crcnext(&self) -> super::vals::Crcnext {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Crcnext(val as u8)
            }
            #[doc = "CRC transfer next"]
            pub fn set_crcnext(&mut self, val: super::vals::Crcnext) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Hardware CRC calculation enable"]
            pub const fn crcen(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware CRC calculation enable"]
            pub fn set_crcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Output enable in bidirectional mode"]
            pub const fn bidioe(&self) -> super::vals::Bidioe {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Bidioe(val as u8)
            }
            #[doc = "Output enable in bidirectional mode"]
            pub fn set_bidioe(&mut self, val: super::vals::Bidioe) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "Bidirectional data mode enable"]
            pub const fn bidimode(&self) -> super::vals::Bidimode {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Bidimode(val as u8)
            }
            #[doc = "Bidirectional data mode enable"]
            pub fn set_bidimode(&mut self, val: super::vals::Bidimode) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "TX CRC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txcrcr(pub u32);
        impl Txcrcr {
            #[doc = "Tx CRC register"]
            pub const fn tx_crc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Tx CRC register"]
            pub fn set_tx_crc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Txcrcr {
            fn default() -> Txcrcr {
                Txcrcr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxonly(pub u8);
        impl Rxonly {
            #[doc = "Full duplex (Transmit and receive)"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Output disabled (Receive-only mode)"]
            pub const OUTPUTDISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Iscfg(pub u8);
        impl Iscfg {
            #[doc = "Slave - transmit"]
            pub const SLAVETX: Self = Self(0);
            #[doc = "Slave - receive"]
            pub const SLAVERX: Self = Self(0x01);
            #[doc = "Master - transmit"]
            pub const MASTERTX: Self = Self(0x02);
            #[doc = "Master - receive"]
            pub const MASTERRX: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dff(pub u8);
        impl Dff {
            #[doc = "8-bit data frame format is selected for transmission/reception"]
            pub const EIGHTBIT: Self = Self(0);
            #[doc = "16-bit data frame format is selected for transmission/reception"]
            pub const SIXTEENBIT: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bidioe(pub u8);
        impl Bidioe {
            #[doc = "Output disabled (receive-only mode)"]
            pub const OUTPUTDISABLED: Self = Self(0);
            #[doc = "Output enabled (transmit-only mode)"]
            pub const OUTPUTENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRSTEDGE: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECONDEDGE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crcnext(pub u8);
        impl Crcnext {
            #[doc = "Next transmit value is from Tx buffer"]
            pub const TXBUFFER: Self = Self(0);
            #[doc = "Next transmit value is from Tx CRC register"]
            pub const CRC: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "CK to 0 when idle"]
            pub const IDLELOW: Self = Self(0);
            #[doc = "CK to 1 when idle"]
            pub const IDLEHIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lsbfirst(pub u8);
        impl Lsbfirst {
            #[doc = "Data is transmitted/received with the MSB first"]
            pub const MSBFIRST: Self = Self(0);
            #[doc = "Data is transmitted/received with the LSB first"]
            pub const LSBFIRST: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Br(pub u8);
        impl Br {
            #[doc = "f_PCLK / 2"]
            pub const DIV2: Self = Self(0);
            #[doc = "f_PCLK / 4"]
            pub const DIV4: Self = Self(0x01);
            #[doc = "f_PCLK / 8"]
            pub const DIV8: Self = Self(0x02);
            #[doc = "f_PCLK / 16"]
            pub const DIV16: Self = Self(0x03);
            #[doc = "f_PCLK / 32"]
            pub const DIV32: Self = Self(0x04);
            #[doc = "f_PCLK / 64"]
            pub const DIV64: Self = Self(0x05);
            #[doc = "f_PCLK / 128"]
            pub const DIV128: Self = Self(0x06);
            #[doc = "f_PCLK / 256"]
            pub const DIV256: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bidimode(pub u8);
        impl Bidimode {
            #[doc = "2-line unidirectional data mode selected"]
            pub const UNIDIRECTIONAL: Self = Self(0);
            #[doc = "1-line bidirectional data mode selected"]
            pub const BIDIRECTIONAL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mstr(pub u8);
        impl Mstr {
            #[doc = "Slave configuration"]
            pub const SLAVE: Self = Self(0);
            #[doc = "Master configuration"]
            pub const MASTER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frer(pub u8);
        impl Frer {
            #[doc = "No frame format error"]
            pub const NOERROR: Self = Self(0);
            #[doc = "A frame format error occurred"]
            pub const ERROR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frf(pub u8);
        impl Frf {
            #[doc = "SPI Motorola mode"]
            pub const MOTOROLA: Self = Self(0);
            #[doc = "SPI TI mode"]
            pub const TI: Self = Self(0x01);
        }
    }
}
pub mod i2c_v3 {
    use crate::generic::*;
    #[doc = "I2C"]
    #[derive(Copy, Clone)]
    pub struct I2c1(pub *mut u8);
    unsafe impl Send for I2c1 {}
    unsafe impl Sync for I2c1 {}
    impl I2c1 {
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        pub fn oar1(self) -> Reg<regs::Oar1, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        pub fn oar2(self) -> Reg<regs::Oar2, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Access: No wait states"]
        pub fn timingr(self) -> Reg<regs::Timingr, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        pub fn timeoutr(self) -> Reg<regs::Timeoutr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Access: No wait states"]
        pub fn isr(self) -> Reg<regs::Isr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Access: No wait states"]
        pub fn icr(self) -> Reg<regs::Icr, W> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "Access: No wait states"]
        pub fn pecr(self) -> Reg<regs::Pecr, R> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "Access: No wait states"]
        pub fn rxdr(self) -> Reg<regs::Rxdr, R> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "Access: No wait states"]
        pub fn txdr(self) -> Reg<regs::Txdr, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Reload(pub u8);
        impl Reload {
            #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
            pub const COMPLETED: Self = Self(0);
            #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
            pub const NOTCOMPLETED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Add(pub u8);
        impl Add {
            #[doc = "The master operates in 7-bit addressing mode"]
            pub const BIT7: Self = Self(0);
            #[doc = "The master operates in 10-bit addressing mode"]
            pub const BIT10: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Oamsk(pub u8);
        impl Oamsk {
            #[doc = "No mask"]
            pub const NOMASK: Self = Self(0);
            #[doc = "OA2[1]
is masked and don’t care. Only OA2[7:2]
are compared"]
            pub const MASK1: Self = Self(0x01);
            #[doc = "OA2[2:1]
are masked and don’t care. Only OA2[7:3]
are compared"]
            pub const MASK2: Self = Self(0x02);
            #[doc = "OA2[3:1]
are masked and don’t care. Only OA2[7:4]
are compared"]
            pub const MASK3: Self = Self(0x03);
            #[doc = "OA2[4:1]
are masked and don’t care. Only OA2[7:5]
are compared"]
            pub const MASK4: Self = Self(0x04);
            #[doc = "OA2[5:1]
are masked and don’t care. Only OA2[7:6]
are compared"]
            pub const MASK5: Self = Self(0x05);
            #[doc = "OA2[6:1]
are masked and don’t care. Only OA2[7]
is compared."]
            pub const MASK6: Self = Self(0x06);
            #[doc = "OA2[7:1]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
            pub const MASK7: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Nack(pub u8);
        impl Nack {
            #[doc = "an ACK is sent after current received byte"]
            pub const ACK: Self = Self(0);
            #[doc = "a NACK is sent after current received byte"]
            pub const NACK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Headr(pub u8);
        impl Headr {
            #[doc = "The master sends the complete 10 bit slave address read sequence"]
            pub const COMPLETE: Self = Self(0);
            #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
            pub const PARTIAL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct RdWrn(pub u8);
        impl RdWrn {
            #[doc = "Master requests a write transfer"]
            pub const WRITE: Self = Self(0);
            #[doc = "Master requests a read transfer"]
            pub const READ: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pecerr(pub u8);
        impl Pecerr {
            #[doc = "Received PEC does match with PEC register"]
            pub const MATCH: Self = Self(0);
            #[doc = "Received PEC does not match with PEC register"]
            pub const NOMATCH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dnf(pub u8);
        impl Dnf {
            #[doc = "Digital filter disabled"]
            pub const NOFILTER: Self = Self(0);
            #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
            pub const FILTER1: Self = Self(0x01);
            #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
            pub const FILTER2: Self = Self(0x02);
            #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
            pub const FILTER3: Self = Self(0x03);
            #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
            pub const FILTER4: Self = Self(0x04);
            #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
            pub const FILTER5: Self = Self(0x05);
            #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
            pub const FILTER6: Self = Self(0x06);
            #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
            pub const FILTER7: Self = Self(0x07);
            #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
            pub const FILTER8: Self = Self(0x08);
            #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
            pub const FILTER9: Self = Self(0x09);
            #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
            pub const FILTER10: Self = Self(0x0a);
            #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
            pub const FILTER11: Self = Self(0x0b);
            #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
            pub const FILTER12: Self = Self(0x0c);
            #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
            pub const FILTER13: Self = Self(0x0d);
            #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
            pub const FILTER14: Self = Self(0x0e);
            #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
            pub const FILTER15: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Start(pub u8);
        impl Start {
            #[doc = "No Start generation"]
            pub const NOSTART: Self = Self(0);
            #[doc = "Restart/Start generation"]
            pub const START: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dir(pub u8);
        impl Dir {
            #[doc = "Write transfer, slave enters receiver mode"]
            pub const WRITE: Self = Self(0);
            #[doc = "Read transfer, slave enters transmitter mode"]
            pub const READ: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Oamode(pub u8);
        impl Oamode {
            #[doc = "Own address 1 is a 7-bit address"]
            pub const BIT7: Self = Self(0);
            #[doc = "Own address 1 is a 10-bit address"]
            pub const BIT10: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Stop(pub u8);
        impl Stop {
            #[doc = "No Stop generation"]
            pub const NOSTOP: Self = Self(0);
            #[doc = "Stop generation after current byte transfer"]
            pub const STOP: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Autoend(pub u8);
        impl Autoend {
            #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
            pub const SOFTWARE: Self = Self(0);
            #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
            pub const AUTOMATIC: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pecbyte(pub u8);
        impl Pecbyte {
            #[doc = "No PEC transfer"]
            pub const NOPEC: Self = Self(0);
            #[doc = "PEC transmission/reception is requested"]
            pub const PEC: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Slave address bit 0 (master mode) In 7-bit addressing mode (ADD10 = 0): This bit is dont care In 10-bit addressing mode (ADD10 = 1): This bit should be written with bit 0 of the slave address to be sent Note: Changing these bits when the START bit is set is not allowed."]
            pub const fn sadd(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Slave address bit 0 (master mode) In 7-bit addressing mode (ADD10 = 0): This bit is dont care In 10-bit addressing mode (ADD10 = 1): This bit should be written with bit 0 of the slave address to be sent Note: Changing these bits when the START bit is set is not allowed."]
            pub fn set_sadd(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
            #[doc = "Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
            pub const fn rd_wrn(&self) -> super::vals::RdWrn {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::RdWrn(val as u8)
            }
            #[doc = "Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
            pub fn set_rd_wrn(&mut self, val: super::vals::RdWrn) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
            pub const fn add10(&self) -> super::vals::Add {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Add(val as u8)
            }
            #[doc = "10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
            pub fn set_add10(&mut self, val: super::vals::Add) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
            pub const fn head10r(&self) -> super::vals::Headr {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Headr(val as u8)
            }
            #[doc = "10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
            pub fn set_head10r(&mut self, val: super::vals::Headr) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
            pub const fn start(&self) -> super::vals::Start {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Start(val as u8)
            }
            #[doc = "Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
            pub fn set_start(&mut self, val: super::vals::Start) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val.0 as u32) & 0x01) << 13usize);
            }
            #[doc = "Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect."]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Stop(val as u8)
            }
            #[doc = "Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect."]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
            pub const fn nack(&self) -> super::vals::Nack {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Nack(val as u8)
            }
            #[doc = "NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
            pub fn set_nack(&mut self, val: super::vals::Nack) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
            pub const fn nbytes(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
            pub fn set_nbytes(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "NBYTES reload mode This bit is set and cleared by software."]
            pub const fn reload(&self) -> super::vals::Reload {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Reload(val as u8)
            }
            #[doc = "NBYTES reload mode This bit is set and cleared by software."]
            pub fn set_reload(&mut self, val: super::vals::Reload) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
            }
            #[doc = "Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
            pub const fn autoend(&self) -> super::vals::Autoend {
                let val = (self.0 >> 25usize) & 0x01;
                super::vals::Autoend(val as u8)
            }
            #[doc = "Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
            pub fn set_autoend(&mut self, val: super::vals::Autoend) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val.0 as u32) & 0x01) << 25usize);
            }
            #[doc = "Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn pecbyte(&self) -> super::vals::Pecbyte {
                let val = (self.0 >> 26usize) & 0x01;
                super::vals::Pecbyte(val as u8)
            }
            #[doc = "Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_pecbyte(&mut self, val: super::vals::Pecbyte) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val.0 as u32) & 0x01) << 26usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Access: No wait states"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Timingr(pub u32);
        impl Timingr {
            #[doc = "SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings."]
            pub const fn scll(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings."]
            pub fn set_scll(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing."]
            pub const fn sclh(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing."]
            pub fn set_sclh(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Data hold time This field is used to generate the delay tSDADEL between SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSDADEL. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing."]
            pub const fn sdadel(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Data hold time This field is used to generate the delay tSDADEL between SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSDADEL. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing."]
            pub fn set_sdadel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSCLDEL. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing."]
            pub const fn scldel(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x0f;
                val as u8
            }
            #[doc = "Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSCLDEL. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing."]
            pub fn set_scldel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
            }
            #[doc = "Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters (refer to I2C timings on page9) and for SCL high and low level counters (refer to I2C master initialization on page24). tPRESC = (PRESC+1) x tI2CCLK"]
            pub const fn presc(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x0f;
                val as u8
            }
            #[doc = "Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters (refer to I2C timings on page9) and for SCL high and low level counters (refer to I2C master initialization on page24). tPRESC = (PRESC+1) x tI2CCLK"]
            pub fn set_presc(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
            }
        }
        impl Default for Timingr {
            fn default() -> Timingr {
                Timingr(0)
            }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Oar2(pub u32);
        impl Oar2 {
            #[doc = "Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
            pub const fn oa2(&self) -> u8 {
                let val = (self.0 >> 1usize) & 0x7f;
                val as u8
            }
            #[doc = "Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
            pub fn set_oa2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
            }
            #[doc = "Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
            pub const fn oa2msk(&self) -> super::vals::Oamsk {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Oamsk(val as u8)
            }
            #[doc = "Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
            pub fn set_oa2msk(&mut self, val: super::vals::Oamsk) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.0 as u32) & 0x07) << 8usize);
            }
            #[doc = "Own Address 2 enable"]
            pub const fn oa2en(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Own Address 2 enable"]
            pub fn set_oa2en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Oar2 {
            fn default() -> Oar2 {
                Oar2(0)
            }
        }
        #[doc = "Access: No wait states"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Isr(pub u32);
        impl Isr {
            #[doc = "Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to 1 by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0."]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to 1 by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0."]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to 1 by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0."]
            pub const fn txis(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to 1 by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0."]
            pub fn set_txis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE=0."]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub const fn addr(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub const fn nackf(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_nackf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Stop detection flag This flag is set by hardware when a Stop condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub const fn stopf(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Stop detection flag This flag is set by hardware when a Stop condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_stopf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE=0."]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE=0. This flag is only for master mode, or for slave mode when the SBC bit is set."]
            pub const fn tcr(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE=0. This flag is only for master mode, or for slave mode when the SBC bit is set."]
            pub fn set_tcr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Bus error This flag is set by hardware when a misplaced Start or Stop condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub const fn berr(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Bus error This flag is set by hardware when a misplaced Start or Stop condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_berr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub const fn arlo(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_arlo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH=1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub const fn ovr(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH=1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE=0."]
            pub fn set_ovr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn pecerr(&self) -> super::vals::Pecerr {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Pecerr(val as u8)
            }
            #[doc = "PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_pecerr(&mut self, val: super::vals::Pecerr) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "Timeout or tLOW detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Timeout or tLOW detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_timeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn alert(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_alert(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a Stop condition is detected, or when PE=0."]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a Stop condition is detected, or when PE=0."]
            pub fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1)."]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Dir(val as u8)
            }
            #[doc = "Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1)."]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
            #[doc = "Address match code (Slave mode) These bits are updated with the received address when an address match event occurs (ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed by the 2 MSBs of the address."]
            pub const fn addcode(&self) -> u8 {
                let val = (self.0 >> 17usize) & 0x7f;
                val as u8
            }
            #[doc = "Address match code (Slave mode) These bits are updated with the received address when an address match event occurs (ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed by the 2 MSBs of the address."]
            pub fn set_addcode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
            }
        }
        impl Default for Isr {
            fn default() -> Isr {
                Isr(0)
            }
        }
        #[doc = "Access: No wait states"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txdr(pub u32);
        impl Txdr {
            #[doc = "8-bit transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1."]
            pub const fn txdata(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "8-bit transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1."]
            pub fn set_txdata(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txdr {
            fn default() -> Txdr {
                Txdr(0)
            }
        }
        #[doc = "Access: No wait states"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icr(pub u32);
        impl Icr {
            #[doc = "Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
            pub const fn addrcf(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
            pub fn set_addrcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
            pub const fn nackcf(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
            pub fn set_nackcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
            pub const fn stopcf(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
            pub fn set_stopcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
            pub const fn berrcf(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
            pub fn set_berrcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
            pub const fn arlocf(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
            pub fn set_arlocf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
            pub const fn ovrcf(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
            pub fn set_ovrcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn peccf(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_peccf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn timoutcf(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_timoutcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn alertcf(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_alertcf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Icr {
            fn default() -> Icr {
                Icr(0)
            }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Timeoutr(pub u32);
        impl Timeoutr {
            #[doc = "Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
            pub const fn timeouta(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
            pub fn set_timeouta(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
            pub const fn tidle(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
            pub fn set_tidle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Clock timeout enable"]
            pub const fn timouten(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Clock timeout enable"]
            pub fn set_timouten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
            pub const fn timeoutb(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
            pub fn set_timeoutb(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
            #[doc = "Extended clock timeout enable"]
            pub const fn texten(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Extended clock timeout enable"]
            pub fn set_texten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Timeoutr {
            fn default() -> Timeoutr {
                Timeoutr(0)
            }
        }
        #[doc = "Access: No wait states"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxdr(pub u32);
        impl Rxdr {
            #[doc = "8-bit receive data Data byte received from the I2C bus."]
            pub const fn rxdata(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "8-bit receive data Data byte received from the I2C bus."]
            pub fn set_rxdata(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxdr {
            fn default() -> Rxdr {
                Rxdr(0)
            }
        }
        #[doc = "Access: No wait states"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pecr(pub u32);
        impl Pecr {
            #[doc = "Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
            pub const fn pec(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
            pub fn set_pec(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Pecr {
            fn default() -> Pecr {
                Pecr(0)
            }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Oar1(pub u32);
        impl Oar1 {
            #[doc = "Interface address 7-bit addressing mode: dont care 10-bit addressing mode: bits 9:8 of address Note: These bits can be written only when OA1EN=0. OA1[7:1]: Interface address Bits 7:1 of address Note: These bits can be written only when OA1EN=0. OA1[0]: Interface address 7-bit addressing mode: dont care 10-bit addressing mode: bit 0 of address Note: This bit can be written only when OA1EN=0."]
            pub const fn oa1(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Interface address 7-bit addressing mode: dont care 10-bit addressing mode: bits 9:8 of address Note: These bits can be written only when OA1EN=0. OA1[7:1]: Interface address Bits 7:1 of address Note: These bits can be written only when OA1EN=0. OA1[0]: Interface address 7-bit addressing mode: dont care 10-bit addressing mode: bit 0 of address Note: This bit can be written only when OA1EN=0."]
            pub fn set_oa1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
            #[doc = "Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
            pub const fn oa1mode(&self) -> super::vals::Oamode {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Oamode(val as u8)
            }
            #[doc = "Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
            pub fn set_oa1mode(&mut self, val: super::vals::Oamode) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Own Address 1 enable"]
            pub const fn oa1en(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Own Address 1 enable"]
            pub fn set_oa1en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Oar1 {
            fn default() -> Oar1 {
                Oar1(0)
            }
        }
        #[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TX Interrupt enable"]
            pub const fn txie(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "TX Interrupt enable"]
            pub fn set_txie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "RX Interrupt enable"]
            pub const fn rxie(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "RX Interrupt enable"]
            pub fn set_rxie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Address match Interrupt enable (slave only)"]
            pub const fn addrie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Address match Interrupt enable (slave only)"]
            pub fn set_addrie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Not acknowledge received Interrupt enable"]
            pub const fn nackie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Not acknowledge received Interrupt enable"]
            pub fn set_nackie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "STOP detection Interrupt enable"]
            pub const fn stopie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "STOP detection Interrupt enable"]
            pub fn set_stopie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transfer Complete interrupt enable Note: Any of these events will generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer Complete interrupt enable Note: Any of these events will generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
            pub const fn errie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
            pub fn set_errie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF[3:0]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
            pub const fn dnf(&self) -> super::vals::Dnf {
                let val = (self.0 >> 8usize) & 0x0f;
                super::vals::Dnf(val as u8)
            }
            #[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF[3:0]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
            pub fn set_dnf(&mut self, val: super::vals::Dnf) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val.0 as u32) & 0x0f) << 8usize);
            }
            #[doc = "Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
            pub const fn anfoff(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
            pub fn set_anfoff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "DMA transmission requests enable"]
            pub const fn txdmaen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "DMA transmission requests enable"]
            pub fn set_txdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "DMA reception requests enable"]
            pub const fn rxdmaen(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "DMA reception requests enable"]
            pub fn set_rxdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Slave byte control This bit is used to enable hardware byte control in slave mode."]
            pub const fn sbc(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Slave byte control This bit is used to enable hardware byte control in slave mode."]
            pub fn set_sbc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
            pub const fn nostretch(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
            pub fn set_nostretch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation. Note: WUPEN can be set only when DNF = 0000"]
            pub const fn wupen(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation. Note: WUPEN can be set only when DNF = 0000"]
            pub fn set_wupen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "General call enable"]
            pub const fn gcen(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "General call enable"]
            pub fn set_gcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "SMBus Host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn smbhen(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "SMBus Host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_smbhen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "SMBus Device Default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn smbden(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "SMBus Device Default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_smbden(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "SMBus alert enable Device mode (SMBHEN=0): Host mode (SMBHEN=1): Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn alerten(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "SMBus alert enable Device mode (SMBHEN=0): Host mode (SMBHEN=1): Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_alerten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub const fn pecen(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
            pub fn set_pecen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
    }
}
pub mod gpio_v2 {
    use crate::generic::*;
    #[doc = "General-purpose I/Os"]
    #[derive(Copy, Clone)]
    pub struct Gpio(pub *mut u8);
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[doc = "GPIO port mode register"]
        pub fn moder(self) -> Reg<regs::Moder, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "GPIO port output type register"]
        pub fn otyper(self) -> Reg<regs::Otyper, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "GPIO port output speed register"]
        pub fn ospeedr(self) -> Reg<regs::Ospeedr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "GPIO port pull-up/pull-down register"]
        pub fn pupdr(self) -> Reg<regs::Pupdr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "GPIO port input data register"]
        pub fn idr(self) -> Reg<regs::Idr, R> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "GPIO port output data register"]
        pub fn odr(self) -> Reg<regs::Odr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "GPIO port bit set/reset register"]
        pub fn bsrr(self) -> Reg<regs::Bsrr, W> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "GPIO port configuration lock register"]
        pub fn lckr(self) -> Reg<regs::Lckr, RW> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "GPIO alternate function register (low, high)"]
        pub fn afr(self, n: usize) -> Reg<regs::Afr, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ospeedr(pub u8);
        impl Ospeedr {
            #[doc = "Low speed"]
            pub const LOWSPEED: Self = Self(0);
            #[doc = "Medium speed"]
            pub const MEDIUMSPEED: Self = Self(0x01);
            #[doc = "High speed"]
            pub const HIGHSPEED: Self = Self(0x02);
            #[doc = "Very high speed"]
            pub const VERYHIGHSPEED: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pupdr(pub u8);
        impl Pupdr {
            #[doc = "No pull-up, pull-down"]
            pub const FLOATING: Self = Self(0);
            #[doc = "Pull-up"]
            pub const PULLUP: Self = Self(0x01);
            #[doc = "Pull-down"]
            pub const PULLDOWN: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lck(pub u8);
        impl Lck {
            #[doc = "Port configuration not locked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "Port configuration locked"]
            pub const LOCKED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bsw(pub u8);
        impl Bsw {
            #[doc = "Sets the corresponding ODRx bit"]
            pub const SET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lckk(pub u8);
        impl Lckk {
            #[doc = "Port configuration lock key not active"]
            pub const NOTACTIVE: Self = Self(0);
            #[doc = "Port configuration lock key active"]
            pub const ACTIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Brw(pub u8);
        impl Brw {
            #[doc = "Resets the corresponding ODRx bit"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ot(pub u8);
        impl Ot {
            #[doc = "Output push-pull (reset state)"]
            pub const PUSHPULL: Self = Self(0);
            #[doc = "Output open-drain"]
            pub const OPENDRAIN: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Afr(pub u8);
        impl Afr {
            #[doc = "AF0"]
            pub const AF0: Self = Self(0);
            #[doc = "AF1"]
            pub const AF1: Self = Self(0x01);
            #[doc = "AF2"]
            pub const AF2: Self = Self(0x02);
            #[doc = "AF3"]
            pub const AF3: Self = Self(0x03);
            #[doc = "AF4"]
            pub const AF4: Self = Self(0x04);
            #[doc = "AF5"]
            pub const AF5: Self = Self(0x05);
            #[doc = "AF6"]
            pub const AF6: Self = Self(0x06);
            #[doc = "AF7"]
            pub const AF7: Self = Self(0x07);
            #[doc = "AF8"]
            pub const AF8: Self = Self(0x08);
            #[doc = "AF9"]
            pub const AF9: Self = Self(0x09);
            #[doc = "AF10"]
            pub const AF10: Self = Self(0x0a);
            #[doc = "AF11"]
            pub const AF11: Self = Self(0x0b);
            #[doc = "AF12"]
            pub const AF12: Self = Self(0x0c);
            #[doc = "AF13"]
            pub const AF13: Self = Self(0x0d);
            #[doc = "AF14"]
            pub const AF14: Self = Self(0x0e);
            #[doc = "AF15"]
            pub const AF15: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Moder(pub u8);
        impl Moder {
            #[doc = "Input mode (reset state)"]
            pub const INPUT: Self = Self(0);
            #[doc = "General purpose output mode"]
            pub const OUTPUT: Self = Self(0x01);
            #[doc = "Alternate function mode"]
            pub const ALTERNATE: Self = Self(0x02);
            #[doc = "Analog mode"]
            pub const ANALOG: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "GPIO port output data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Odr(pub u32);
        impl Odr {
            #[doc = "Port output data (y = 0..15)"]
            pub fn odr(&self, n: usize) -> super::vals::Odr {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Odr(val as u8)
            }
            #[doc = "Port output data (y = 0..15)"]
            pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Odr {
            fn default() -> Odr {
                Odr(0)
            }
        }
        #[doc = "GPIO alternate function register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Afr(pub u32);
        impl Afr {
            #[doc = "Alternate function selection for port x bit y (y = 0..15)"]
            pub fn afr(&self, n: usize) -> super::vals::Afr {
                assert!(n < 8usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                super::vals::Afr(val as u8)
            }
            #[doc = "Alternate function selection for port x bit y (y = 0..15)"]
            pub fn set_afr(&mut self, n: usize, val: super::vals::Afr) {
                assert!(n < 8usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val.0 as u32) & 0x0f) << offs);
            }
        }
        impl Default for Afr {
            fn default() -> Afr {
                Afr(0)
            }
        }
        #[doc = "GPIO port bit set/reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn bs(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn set_bs(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Bsrr {
            fn default() -> Bsrr {
                Bsrr(0)
            }
        }
        #[doc = "GPIO port output speed register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ospeedr(pub u32);
        impl Ospeedr {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn ospeedr(&self, n: usize) -> super::vals::Ospeedr {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Ospeedr(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_ospeedr(&mut self, n: usize, val: super::vals::Ospeedr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Ospeedr {
            fn default() -> Ospeedr {
                Ospeedr(0)
            }
        }
        #[doc = "GPIO port mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Moder(pub u32);
        impl Moder {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn moder(&self, n: usize) -> super::vals::Moder {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Moder(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_moder(&mut self, n: usize, val: super::vals::Moder) {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Moder {
            fn default() -> Moder {
                Moder(0)
            }
        }
        #[doc = "GPIO port output type register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Otyper(pub u32);
        impl Otyper {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn ot(&self, n: usize) -> super::vals::Ot {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ot(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_ot(&mut self, n: usize, val: super::vals::Ot) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Otyper {
            fn default() -> Otyper {
                Otyper(0)
            }
        }
        #[doc = "GPIO port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn lck(&self, n: usize) -> super::vals::Lck {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Lck(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lck(&mut self, n: usize, val: super::vals::Lck) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
        #[doc = "GPIO port pull-up/pull-down register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pupdr(pub u32);
        impl Pupdr {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn pupdr(&self, n: usize) -> super::vals::Pupdr {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Pupdr(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_pupdr(&mut self, n: usize, val: super::vals::Pupdr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Pupdr {
            fn default() -> Pupdr {
                Pupdr(0)
            }
        }
        #[doc = "GPIO port input data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idr(pub u32);
        impl Idr {
            #[doc = "Port input data (y = 0..15)"]
            pub fn idr(&self, n: usize) -> super::vals::Idr {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Idr(val as u8)
            }
            #[doc = "Port input data (y = 0..15)"]
            pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Idr {
            fn default() -> Idr {
                Idr(0)
            }
        }
    }
}
pub mod syscfg_h7 {
    use crate::generic::*;
    #[doc = "System configuration controller"]
    #[derive(Copy, Clone)]
    pub struct Syscfg(pub *mut u8);
    unsafe impl Send for Syscfg {}
    unsafe impl Sync for Syscfg {}
    impl Syscfg {
        #[doc = "peripheral mode configuration register"]
        pub fn pmcr(self) -> Reg<regs::Pmcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "external interrupt configuration register 1"]
        pub fn exticr(self, n: usize) -> Reg<regs::Exticr, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "compensation cell control/status register"]
        pub fn cccsr(self) -> Reg<regs::Cccsr, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "SYSCFG compensation cell value register"]
        pub fn ccvr(self) -> Reg<regs::Ccvr, R> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "SYSCFG compensation cell code register"]
        pub fn cccr(self) -> Reg<regs::Cccr, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "SYSCFG power control register"]
        pub fn pwrcr(self) -> Reg<regs::Pwrcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "SYSCFG package register"]
        pub fn pkgr(self) -> Reg<regs::Pkgr, R> {
            unsafe { Reg::from_ptr(self.0.add(292usize)) }
        }
        #[doc = "SYSCFG user register 0"]
        pub fn ur0(self) -> Reg<regs::Ur0, R> {
            unsafe { Reg::from_ptr(self.0.add(768usize)) }
        }
        #[doc = "SYSCFG user register 2"]
        pub fn ur2(self) -> Reg<regs::Ur2, RW> {
            unsafe { Reg::from_ptr(self.0.add(776usize)) }
        }
        #[doc = "SYSCFG user register 3"]
        pub fn ur3(self) -> Reg<regs::Ur3, RW> {
            unsafe { Reg::from_ptr(self.0.add(780usize)) }
        }
        #[doc = "SYSCFG user register 4"]
        pub fn ur4(self) -> Reg<regs::Ur4, R> {
            unsafe { Reg::from_ptr(self.0.add(784usize)) }
        }
        #[doc = "SYSCFG user register 5"]
        pub fn ur5(self) -> Reg<regs::Ur5, R> {
            unsafe { Reg::from_ptr(self.0.add(788usize)) }
        }
        #[doc = "SYSCFG user register 6"]
        pub fn ur6(self) -> Reg<regs::Ur6, R> {
            unsafe { Reg::from_ptr(self.0.add(792usize)) }
        }
        #[doc = "SYSCFG user register 7"]
        pub fn ur7(self) -> Reg<regs::Ur7, R> {
            unsafe { Reg::from_ptr(self.0.add(796usize)) }
        }
        #[doc = "SYSCFG user register 8"]
        pub fn ur8(self) -> Reg<regs::Ur8, R> {
            unsafe { Reg::from_ptr(self.0.add(800usize)) }
        }
        #[doc = "SYSCFG user register 9"]
        pub fn ur9(self) -> Reg<regs::Ur9, R> {
            unsafe { Reg::from_ptr(self.0.add(804usize)) }
        }
        #[doc = "SYSCFG user register 10"]
        pub fn ur10(self) -> Reg<regs::Ur10, R> {
            unsafe { Reg::from_ptr(self.0.add(808usize)) }
        }
        #[doc = "SYSCFG user register 11"]
        pub fn ur11(self) -> Reg<regs::Ur11, R> {
            unsafe { Reg::from_ptr(self.0.add(812usize)) }
        }
        #[doc = "SYSCFG user register 12"]
        pub fn ur12(self) -> Reg<regs::Ur12, R> {
            unsafe { Reg::from_ptr(self.0.add(816usize)) }
        }
        #[doc = "SYSCFG user register 13"]
        pub fn ur13(self) -> Reg<regs::Ur13, R> {
            unsafe { Reg::from_ptr(self.0.add(820usize)) }
        }
        #[doc = "SYSCFG user register 14"]
        pub fn ur14(self) -> Reg<regs::Ur14, RW> {
            unsafe { Reg::from_ptr(self.0.add(824usize)) }
        }
        #[doc = "SYSCFG user register 15"]
        pub fn ur15(self) -> Reg<regs::Ur15, R> {
            unsafe { Reg::from_ptr(self.0.add(828usize)) }
        }
        #[doc = "SYSCFG user register 16"]
        pub fn ur16(self) -> Reg<regs::Ur16, R> {
            unsafe { Reg::from_ptr(self.0.add(832usize)) }
        }
        #[doc = "SYSCFG user register 17"]
        pub fn ur17(self) -> Reg<regs::Ur17, R> {
            unsafe { Reg::from_ptr(self.0.add(836usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "SYSCFG user register 13"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur13(pub u32);
        impl Ur13 {
            #[doc = "Secured DTCM RAM Size"]
            pub const fn sdrs(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "Secured DTCM RAM Size"]
            pub fn set_sdrs(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "D1 Standby reset"]
            pub const fn d1sbrst(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "D1 Standby reset"]
            pub fn set_d1sbrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur13 {
            fn default() -> Ur13 {
                Ur13(0)
            }
        }
        #[doc = "SYSCFG user register 9"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur9(pub u32);
        impl Ur9 {
            #[doc = "Write protection for flash bank 2"]
            pub const fn wrpn_2(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Write protection for flash bank 2"]
            pub fn set_wrpn_2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Protected area start address for bank 2"]
            pub const fn pa_beg_2(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "Protected area start address for bank 2"]
            pub fn set_pa_beg_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for Ur9 {
            fn default() -> Ur9 {
                Ur9(0)
            }
        }
        #[doc = "SYSCFG compensation cell code register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cccr(pub u32);
        impl Cccr {
            #[doc = "NMOS compensation code"]
            pub const fn ncc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "NMOS compensation code"]
            pub fn set_ncc(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "PMOS compensation code"]
            pub const fn pcc(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "PMOS compensation code"]
            pub fn set_pcc(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Cccr {
            fn default() -> Cccr {
                Cccr(0)
            }
        }
        #[doc = "SYSCFG compensation cell value register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ccvr(pub u32);
        impl Ccvr {
            #[doc = "NMOS compensation value"]
            pub const fn ncv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "NMOS compensation value"]
            pub fn set_ncv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "PMOS compensation value"]
            pub const fn pcv(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "PMOS compensation value"]
            pub fn set_pcv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Ccvr {
            fn default() -> Ccvr {
                Ccvr(0)
            }
        }
        #[doc = "SYSCFG user register 10"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur10(pub u32);
        impl Ur10 {
            #[doc = "Protected area end address for bank 2"]
            pub const fn pa_end_2(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Protected area end address for bank 2"]
            pub fn set_pa_end_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "Secured area start address for bank 2"]
            pub const fn sa_beg_2(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "Secured area start address for bank 2"]
            pub fn set_sa_beg_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for Ur10 {
            fn default() -> Ur10 {
                Ur10(0)
            }
        }
        #[doc = "SYSCFG user register 12"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur12(pub u32);
        impl Ur12 {
            #[doc = "Secure mode"]
            pub const fn secure(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Secure mode"]
            pub fn set_secure(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur12 {
            fn default() -> Ur12 {
                Ur12(0)
            }
        }
        #[doc = "SYSCFG user register 17"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur17(pub u32);
        impl Ur17 {
            #[doc = "I/O high speed / low voltage"]
            pub const fn io_hslv(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "I/O high speed / low voltage"]
            pub fn set_io_hslv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ur17 {
            fn default() -> Ur17 {
                Ur17(0)
            }
        }
        #[doc = "SYSCFG power control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pwrcr(pub u32);
        impl Pwrcr {
            #[doc = "Overdrive enable"]
            pub const fn oden(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Overdrive enable"]
            pub fn set_oden(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Pwrcr {
            fn default() -> Pwrcr {
                Pwrcr(0)
            }
        }
        #[doc = "SYSCFG user register 5"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur5(pub u32);
        impl Ur5 {
            #[doc = "Mass erase secured area disabled for bank 1"]
            pub const fn mesad_1(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Mass erase secured area disabled for bank 1"]
            pub fn set_mesad_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write protection for flash bank 1"]
            pub const fn wrpn_1(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Write protection for flash bank 1"]
            pub fn set_wrpn_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
        }
        impl Default for Ur5 {
            fn default() -> Ur5 {
                Ur5(0)
            }
        }
        #[doc = "external interrupt configuration register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Exticr(pub u32);
        impl Exticr {
            #[doc = "EXTI x configuration (x = 4 to 7)"]
            pub fn exti(&self, n: usize) -> u8 {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                val as u8
            }
            #[doc = "EXTI x configuration (x = 4 to 7)"]
            pub fn set_exti(&mut self, n: usize, val: u8) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
            }
        }
        impl Default for Exticr {
            fn default() -> Exticr {
                Exticr(0)
            }
        }
        #[doc = "SYSCFG user register 4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur4(pub u32);
        impl Ur4 {
            #[doc = "Mass Erase Protected Area Disabled for bank 1"]
            pub const fn mepad_1(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Mass Erase Protected Area Disabled for bank 1"]
            pub fn set_mepad_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur4 {
            fn default() -> Ur4 {
                Ur4(0)
            }
        }
        #[doc = "SYSCFG user register 7"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur7(pub u32);
        impl Ur7 {
            #[doc = "Secured area start address for bank 1"]
            pub const fn sa_beg_1(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Secured area start address for bank 1"]
            pub fn set_sa_beg_1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "Secured area end address for bank 1"]
            pub const fn sa_end_1(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "Secured area end address for bank 1"]
            pub fn set_sa_end_1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for Ur7 {
            fn default() -> Ur7 {
                Ur7(0)
            }
        }
        #[doc = "SYSCFG user register 14"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur14(pub u32);
        impl Ur14 {
            #[doc = "D1 Stop Reset"]
            pub const fn d1stprst(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "D1 Stop Reset"]
            pub fn set_d1stprst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ur14 {
            fn default() -> Ur14 {
                Ur14(0)
            }
        }
        #[doc = "peripheral mode configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pmcr(pub u32);
        impl Pmcr {
            #[doc = "I2C1 Fm+"]
            pub const fn i2c1fmp(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "I2C1 Fm+"]
            pub fn set_i2c1fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "I2C2 Fm+"]
            pub const fn i2c2fmp(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "I2C2 Fm+"]
            pub fn set_i2c2fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "I2C3 Fm+"]
            pub const fn i2c3fmp(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "I2C3 Fm+"]
            pub fn set_i2c3fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "I2C4 Fm+"]
            pub const fn i2c4fmp(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "I2C4 Fm+"]
            pub fn set_i2c4fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PB(6) Fm+"]
            pub const fn pb6fmp(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PB(6) Fm+"]
            pub fn set_pb6fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "PB(7) Fast Mode Plus"]
            pub const fn pb7fmp(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "PB(7) Fast Mode Plus"]
            pub fn set_pb7fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "PB(8) Fast Mode Plus"]
            pub const fn pb8fmp(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "PB(8) Fast Mode Plus"]
            pub fn set_pb8fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "PB(9) Fm+"]
            pub const fn pb9fmp(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "PB(9) Fm+"]
            pub fn set_pb9fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Booster Enable"]
            pub const fn booste(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Booster Enable"]
            pub fn set_booste(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Analog switch supply voltage selection"]
            pub const fn boostvddsel(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Analog switch supply voltage selection"]
            pub fn set_boostvddsel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Ethernet PHY Interface Selection"]
            pub const fn epis(&self) -> u8 {
                let val = (self.0 >> 21usize) & 0x07;
                val as u8
            }
            #[doc = "Ethernet PHY Interface Selection"]
            pub fn set_epis(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
            }
            #[doc = "PA0 Switch Open"]
            pub const fn pa0so(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "PA0 Switch Open"]
            pub fn set_pa0so(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "PA1 Switch Open"]
            pub const fn pa1so(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "PA1 Switch Open"]
            pub fn set_pa1so(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "PC2 Switch Open"]
            pub const fn pc2so(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PC2 Switch Open"]
            pub fn set_pc2so(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "PC3 Switch Open"]
            pub const fn pc3so(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "PC3 Switch Open"]
            pub fn set_pc3so(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
        }
        impl Default for Pmcr {
            fn default() -> Pmcr {
                Pmcr(0)
            }
        }
        #[doc = "SYSCFG user register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur2(pub u32);
        impl Ur2 {
            #[doc = "BOR_LVL Brownout Reset Threshold Level"]
            pub const fn borh(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "BOR_LVL Brownout Reset Threshold Level"]
            pub fn set_borh(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "Boot Address 0"]
            pub const fn boot_add0(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "Boot Address 0"]
            pub fn set_boot_add0(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Ur2 {
            fn default() -> Ur2 {
                Ur2(0)
            }
        }
        #[doc = "SYSCFG user register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur0(pub u32);
        impl Ur0 {
            #[doc = "Bank Swap"]
            pub const fn bks(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Bank Swap"]
            pub fn set_bks(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Readout protection"]
            pub const fn rdp(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Readout protection"]
            pub fn set_rdp(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
        }
        impl Default for Ur0 {
            fn default() -> Ur0 {
                Ur0(0)
            }
        }
        #[doc = "SYSCFG user register 6"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur6(pub u32);
        impl Ur6 {
            #[doc = "Protected area start address for bank 1"]
            pub const fn pa_beg_1(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Protected area start address for bank 1"]
            pub fn set_pa_beg_1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "Protected area end address for bank 1"]
            pub const fn pa_end_1(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "Protected area end address for bank 1"]
            pub fn set_pa_end_1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for Ur6 {
            fn default() -> Ur6 {
                Ur6(0)
            }
        }
        #[doc = "SYSCFG user register 11"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur11(pub u32);
        impl Ur11 {
            #[doc = "Secured area end address for bank 2"]
            pub const fn sa_end_2(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Secured area end address for bank 2"]
            pub fn set_sa_end_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "Independent Watchdog 1 mode"]
            pub const fn iwdg1m(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Independent Watchdog 1 mode"]
            pub fn set_iwdg1m(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur11 {
            fn default() -> Ur11 {
                Ur11(0)
            }
        }
        #[doc = "SYSCFG user register 16"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur16(pub u32);
        impl Ur16 {
            #[doc = "Freeze independent watchdog in Stop mode"]
            pub const fn fziwdgstp(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Freeze independent watchdog in Stop mode"]
            pub fn set_fziwdgstp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Private key programmed"]
            pub const fn pkp(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Private key programmed"]
            pub fn set_pkp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur16 {
            fn default() -> Ur16 {
                Ur16(0)
            }
        }
        #[doc = "compensation cell control/status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cccsr(pub u32);
        impl Cccsr {
            #[doc = "enable"]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "enable"]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Code selection"]
            pub const fn cs(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Code selection"]
            pub fn set_cs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Compensation cell ready flag"]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Compensation cell ready flag"]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "High-speed at low-voltage"]
            pub const fn hslv(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "High-speed at low-voltage"]
            pub fn set_hslv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Cccsr {
            fn default() -> Cccsr {
                Cccsr(0)
            }
        }
        #[doc = "SYSCFG user register 15"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur15(pub u32);
        impl Ur15 {
            #[doc = "Freeze independent watchdog in Standby mode"]
            pub const fn fziwdgstb(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Freeze independent watchdog in Standby mode"]
            pub fn set_fziwdgstb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur15 {
            fn default() -> Ur15 {
                Ur15(0)
            }
        }
        #[doc = "SYSCFG package register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pkgr(pub u32);
        impl Pkgr {
            #[doc = "Package"]
            pub const fn pkg(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Package"]
            pub fn set_pkg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Pkgr {
            fn default() -> Pkgr {
                Pkgr(0)
            }
        }
        #[doc = "SYSCFG user register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur3(pub u32);
        impl Ur3 {
            #[doc = "Boot Address 1"]
            pub const fn boot_add1(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "Boot Address 1"]
            pub fn set_boot_add1(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Ur3 {
            fn default() -> Ur3 {
                Ur3(0)
            }
        }
        #[doc = "SYSCFG user register 8"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ur8(pub u32);
        impl Ur8 {
            #[doc = "Mass erase protected area disabled for bank 2"]
            pub const fn mepad_2(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Mass erase protected area disabled for bank 2"]
            pub fn set_mepad_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Mass erase secured area disabled for bank 2"]
            pub const fn mesad_2(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Mass erase secured area disabled for bank 2"]
            pub fn set_mesad_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Ur8 {
            fn default() -> Ur8 {
                Ur8(0)
            }
        }
    }
}
pub mod rng_v1 {
    use crate::generic::*;
    #[doc = "Random number generator"]
    #[derive(Copy, Clone)]
    pub struct Rng(pub *mut u8);
    unsafe impl Send for Rng {}
    unsafe impl Sync for Rng {}
    impl Rng {
        #[doc = "control register"]
        pub fn cr(self) -> Reg<regs::Cr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "data register"]
        pub fn dr(self) -> Reg<u32, R> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Random number generator enable"]
            pub const fn rngen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Random number generator enable"]
            pub fn set_rngen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Interrupt enable"]
            pub const fn ie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Interrupt enable"]
            pub fn set_ie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Data ready"]
            pub const fn drdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Data ready"]
            pub fn set_drdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Clock error current status"]
            pub const fn cecs(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Clock error current status"]
            pub fn set_cecs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Seed error current status"]
            pub const fn secs(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Seed error current status"]
            pub fn set_secs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Clock error interrupt status"]
            pub const fn ceis(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Clock error interrupt status"]
            pub fn set_ceis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Seed error interrupt status"]
            pub const fn seis(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Seed error interrupt status"]
            pub fn set_seis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
    }
}
pub mod syscfg_f4 {
    use crate::generic::*;
    #[doc = "System configuration controller"]
    #[derive(Copy, Clone)]
    pub struct Syscfg(pub *mut u8);
    unsafe impl Send for Syscfg {}
    unsafe impl Sync for Syscfg {}
    impl Syscfg {
        #[doc = "memory remap register"]
        pub fn memrm(self) -> Reg<regs::Memrm, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "peripheral mode configuration register"]
        pub fn pmc(self) -> Reg<regs::Pmc, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "external interrupt configuration register"]
        pub fn exticr(self, n: usize) -> Reg<regs::Exticr, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "Compensation cell control register"]
        pub fn cmpcr(self) -> Reg<regs::Cmpcr, R> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "memory remap register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Memrm(pub u32);
        impl Memrm {
            #[doc = "Memory mapping selection"]
            pub const fn mem_mode(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Memory mapping selection"]
            pub fn set_mem_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
            #[doc = "Flash bank mode selection"]
            pub const fn fb_mode(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Flash bank mode selection"]
            pub fn set_fb_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "FMC memory mapping swap"]
            pub const fn swp_fmc(&self) -> u8 {
                let val = (self.0 >> 10usize) & 0x03;
                val as u8
            }
            #[doc = "FMC memory mapping swap"]
            pub fn set_swp_fmc(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
            }
        }
        impl Default for Memrm {
            fn default() -> Memrm {
                Memrm(0)
            }
        }
        #[doc = "Compensation cell control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cmpcr(pub u32);
        impl Cmpcr {
            #[doc = "Compensation cell power-down"]
            pub const fn cmp_pd(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Compensation cell power-down"]
            pub fn set_cmp_pd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "READY"]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "READY"]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Cmpcr {
            fn default() -> Cmpcr {
                Cmpcr(0)
            }
        }
        #[doc = "peripheral mode configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pmc(pub u32);
        impl Pmc {
            #[doc = "ADC1DC2"]
            pub const fn adc1dc2(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "ADC1DC2"]
            pub fn set_adc1dc2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "ADC2DC2"]
            pub const fn adc2dc2(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "ADC2DC2"]
            pub fn set_adc2dc2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "ADC3DC2"]
            pub const fn adc3dc2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "ADC3DC2"]
            pub fn set_adc3dc2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Ethernet PHY interface selection"]
            pub const fn mii_rmii_sel(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Ethernet PHY interface selection"]
            pub fn set_mii_rmii_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
        }
        impl Default for Pmc {
            fn default() -> Pmc {
                Pmc(0)
            }
        }
        #[doc = "external interrupt configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Exticr(pub u32);
        impl Exticr {
            #[doc = "EXTI x configuration"]
            pub fn exti(&self, n: usize) -> u8 {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                val as u8
            }
            #[doc = "EXTI x configuration"]
            pub fn set_exti(&mut self, n: usize, val: u8) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
            }
        }
        impl Default for Exticr {
            fn default() -> Exticr {
                Exticr(0)
            }
        }
    }
}
pub mod usart_v2 {
    use crate::generic::*;
    #[doc = "Universal synchronous asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Usart(pub *mut u8);
    unsafe impl Send for Usart {}
    unsafe impl Sync for Usart {}
    impl Usart {
        #[doc = "Control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Control register 3"]
        pub fn cr3(self) -> Reg<regs::Cr3, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Baud rate register"]
        pub fn brr(self) -> Reg<regs::Brr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Guard time and prescaler register"]
        pub fn gtpr(self) -> Reg<regs::Gtpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Receiver timeout register"]
        pub fn rtor(self) -> Reg<regs::Rtor, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Request register"]
        pub fn rqr(self) -> Reg<regs::Rqr, W> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Interrupt & status register"]
        pub fn isr(self) -> Reg<regs::Ixr, R> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "Interrupt flag clear register"]
        pub fn icr(self) -> Reg<regs::Ixr, W> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "Receive data register"]
        pub fn rdr(self) -> Reg<regs::Dr, R> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "Transmit data register"]
        pub fn tdr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Abrmod(pub u8);
        impl Abrmod {
            #[doc = "Measurement of the start bit is used to detect the baud rate"]
            pub const START: Self = Self(0);
            #[doc = "Falling edge to falling edge measurement"]
            pub const EDGE: Self = Self(0x01);
            #[doc = "0x7F frame detection"]
            pub const FRAME7F: Self = Self(0x02);
            #[doc = "0x55 frame detection"]
            pub const FRAME55: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Wake(pub u8);
        impl Wake {
            #[doc = "Idle line"]
            pub const IDLE: Self = Self(0);
            #[doc = "Address mask"]
            pub const ADDRESS: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Wus(pub u8);
        impl Wus {
            #[doc = "WUF active on address match"]
            pub const ADDRESS: Self = Self(0);
            #[doc = "WuF active on Start bit detection"]
            pub const START: Self = Self(0x02);
            #[doc = "WUF active on RXNE"]
            pub const RXNE: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxfrq(pub u8);
        impl Rxfrq {
            #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
            pub const DISCARD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lbdl(pub u8);
        impl Lbdl {
            #[doc = "10-bit break detection"]
            pub const BIT10: Self = Self(0);
            #[doc = "11-bit break detection"]
            pub const BIT11: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sbkrq(pub u8);
        impl Sbkrq {
            #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
            pub const BREAK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRST: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECOND: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Onebit(pub u8);
        impl Onebit {
            #[doc = "Three sample bit method"]
            pub const SAMPLE3: Self = Self(0);
            #[doc = "One sample bit method"]
            pub const SAMPLE1: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Datainv(pub u8);
        impl Datainv {
            #[doc = "Logical data from the data register are send/received in positive/direct logic"]
            pub const POSITIVE: Self = Self(0);
            #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
            pub const NEGATIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxinv(pub u8);
        impl Rxinv {
            #[doc = "RX pin signal works using the standard logic levels"]
            pub const STANDARD: Self = Self(0);
            #[doc = "RX pin signal values are inverted"]
            pub const INVERTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Over(pub u8);
        impl Over {
            #[doc = "Oversampling by 16"]
            pub const OVERSAMPLING16: Self = Self(0);
            #[doc = "Oversampling by 8"]
            pub const OVERSAMPLING8: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Stop(pub u8);
        impl Stop {
            #[doc = "1 stop bit"]
            pub const STOP1: Self = Self(0);
            #[doc = "0.5 stop bit"]
            pub const STOP0P5: Self = Self(0x01);
            #[doc = "2 stop bit"]
            pub const STOP2: Self = Self(0x02);
            #[doc = "1.5 stop bit"]
            pub const STOP1P5: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Txinv(pub u8);
        impl Txinv {
            #[doc = "TX pin signal works using the standard logic levels"]
            pub const STANDARD: Self = Self(0);
            #[doc = "TX pin signal values are inverted"]
            pub const INVERTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Addm(pub u8);
        impl Addm {
            #[doc = "4-bit address detection"]
            pub const BIT4: Self = Self(0);
            #[doc = "7-bit address detection"]
            pub const BIT7: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ddre(pub u8);
        impl Ddre {
            #[doc = "DMA is not disabled in case of reception error"]
            pub const NOTDISABLED: Self = Self(0);
            #[doc = "DMA is disabled following a reception error"]
            pub const DISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ovrdis(pub u8);
        impl Ovrdis {
            #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
            pub const ENABLED: Self = Self(0);
            #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
            pub const DISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct M0(pub u8);
        impl M0 {
            #[doc = "1 start bit, 8 data bits, n stop bits"]
            pub const BIT8: Self = Self(0);
            #[doc = "1 start bit, 9 data bits, n stop bits"]
            pub const BIT9: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dep(pub u8);
        impl Dep {
            #[doc = "DE signal is active high"]
            pub const HIGH: Self = Self(0);
            #[doc = "DE signal is active low"]
            pub const LOW: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Irlp(pub u8);
        impl Irlp {
            #[doc = "Normal mode"]
            pub const NORMAL: Self = Self(0);
            #[doc = "Low-power mode"]
            pub const LOWPOWER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hdsel(pub u8);
        impl Hdsel {
            #[doc = "Half duplex mode is not selected"]
            pub const NOTSELECTED: Self = Self(0);
            #[doc = "Half duplex mode is selected"]
            pub const SELECTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Msbfirst(pub u8);
        impl Msbfirst {
            #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
            pub const LSB: Self = Self(0);
            #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
            pub const MSB: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Txfrq(pub u8);
        impl Txfrq {
            #[doc = "Set the TXE flags. This allows to discard the transmit data"]
            pub const DISCARD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ps(pub u8);
        impl Ps {
            #[doc = "Even parity"]
            pub const EVEN: Self = Self(0);
            #[doc = "Odd parity"]
            pub const ODD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct M1(pub u8);
        impl M1 {
            #[doc = "Use M0 to set the data bits"]
            pub const M0: Self = Self(0);
            #[doc = "1 start bit, 7 data bits, n stop bits"]
            pub const BIT7: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lbcl(pub u8);
        impl Lbcl {
            #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
            pub const NOTOUTPUT: Self = Self(0);
            #[doc = "The clock pulse of the last data bit is output to the CK pin"]
            pub const OUTPUT: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "Steady low value on CK pin outside transmission window"]
            pub const LOW: Self = Self(0);
            #[doc = "Steady high value on CK pin outside transmission window"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Swap(pub u8);
        impl Swap {
            #[doc = "TX/RX pins are used as defined in standard pinout"]
            pub const STANDARD: Self = Self(0);
            #[doc = "The TX and RX pins functions are swapped"]
            pub const SWAPPED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Abrrq(pub u8);
        impl Abrrq {
            #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
            pub const REQUEST: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mmrq(pub u8);
        impl Mmrq {
            #[doc = "Puts the USART in mute mode and sets the RWU flag"]
            pub const MUTE: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Request register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rqr(pub u32);
        impl Rqr {
            #[doc = "Auto baud rate request"]
            pub const fn abrrq(&self) -> super::vals::Abrrq {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Abrrq(val as u8)
            }
            #[doc = "Auto baud rate request"]
            pub fn set_abrrq(&mut self, val: super::vals::Abrrq) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Send break request"]
            pub const fn sbkrq(&self) -> super::vals::Sbkrq {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Sbkrq(val as u8)
            }
            #[doc = "Send break request"]
            pub fn set_sbkrq(&mut self, val: super::vals::Sbkrq) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Mute mode request"]
            pub const fn mmrq(&self) -> super::vals::Mmrq {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Mmrq(val as u8)
            }
            #[doc = "Mute mode request"]
            pub fn set_mmrq(&mut self, val: super::vals::Mmrq) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Receive data flush request"]
            pub const fn rxfrq(&self) -> super::vals::Rxfrq {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Rxfrq(val as u8)
            }
            #[doc = "Receive data flush request"]
            pub fn set_rxfrq(&mut self, val: super::vals::Rxfrq) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Transmit data flush request"]
            pub const fn txfrq(&self) -> super::vals::Txfrq {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Txfrq(val as u8)
            }
            #[doc = "Transmit data flush request"]
            pub fn set_txfrq(&mut self, val: super::vals::Txfrq) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Rqr {
            fn default() -> Rqr {
                Rqr(0)
            }
        }
        #[doc = "Receiver timeout register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rtor(pub u32);
        impl Rtor {
            #[doc = "Receiver timeout value"]
            pub const fn rto(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Receiver timeout value"]
            pub fn set_rto(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
            #[doc = "Block Length"]
            pub const fn blen(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Block Length"]
            pub fn set_blen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Rtor {
            fn default() -> Rtor {
                Rtor(0)
            }
        }
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "USART enable"]
            pub const fn ue(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "USART enable"]
            pub fn set_ue(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "USART enable in Stop mode"]
            pub const fn uesm(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "USART enable in Stop mode"]
            pub fn set_uesm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Receiver enable"]
            pub const fn re(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver enable"]
            pub fn set_re(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmitter enable"]
            pub const fn te(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Transmitter enable"]
            pub fn set_te(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE interrupt enable"]
            pub const fn idleie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE interrupt enable"]
            pub fn set_idleie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "RXNE interrupt enable"]
            pub const fn rxneie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "RXNE interrupt enable"]
            pub fn set_rxneie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete interrupt enable"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete interrupt enable"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "interrupt enable"]
            pub const fn txeie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "interrupt enable"]
            pub fn set_txeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "PE interrupt enable"]
            pub const fn peie(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "PE interrupt enable"]
            pub fn set_peie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Parity selection"]
            pub const fn ps(&self) -> super::vals::Ps {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Ps(val as u8)
            }
            #[doc = "Parity selection"]
            pub fn set_ps(&mut self, val: super::vals::Ps) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Parity control enable"]
            pub const fn pce(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Parity control enable"]
            pub fn set_pce(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Receiver wakeup method"]
            pub const fn wake(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver wakeup method"]
            pub fn set_wake(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Word length"]
            pub const fn m0(&self) -> super::vals::M0 {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::M0(val as u8)
            }
            #[doc = "Word length"]
            pub fn set_m0(&mut self, val: super::vals::M0) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Word length"]
            pub const fn m1(&self) -> super::vals::M1 {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::M1(val as u8)
            }
            #[doc = "Word length"]
            pub fn set_m1(&mut self, val: super::vals::M1) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Mute mode enable"]
            pub const fn mme(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Mute mode enable"]
            pub fn set_mme(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Character match interrupt enable"]
            pub const fn cmie(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Character match interrupt enable"]
            pub fn set_cmie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Oversampling mode"]
            pub fn over(&self, n: usize) -> super::vals::Over {
                assert!(n < 1usize);
                let offs = 15usize + n * 0usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Over(val as u8)
            }
            #[doc = "Oversampling mode"]
            pub fn set_over(&mut self, n: usize, val: super::vals::Over) {
                assert!(n < 1usize);
                let offs = 15usize + n * 0usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Driver Enable deassertion time"]
            pub const fn dedt(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x1f;
                val as u8
            }
            #[doc = "Driver Enable deassertion time"]
            pub fn set_dedt(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
            }
            #[doc = "Driver Enable assertion time"]
            pub const fn deat(&self) -> u8 {
                let val = (self.0 >> 21usize) & 0x1f;
                val as u8
            }
            #[doc = "Driver Enable assertion time"]
            pub fn set_deat(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
            }
            #[doc = "Receiver timeout interrupt enable"]
            pub const fn rtoie(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver timeout interrupt enable"]
            pub fn set_rtoie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "End of Block interrupt enable"]
            pub const fn eobie(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "End of Block interrupt enable"]
            pub fn set_eobie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr3(pub u32);
        impl Cr3 {
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "IrDA mode enable"]
            pub const fn iren(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "IrDA mode enable"]
            pub fn set_iren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Half-duplex selection"]
            pub const fn hdsel(&self) -> super::vals::Hdsel {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hdsel(val as u8)
            }
            #[doc = "Half-duplex selection"]
            pub fn set_hdsel(&mut self, val: super::vals::Hdsel) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Smartcard NACK enable"]
            pub const fn nack(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Smartcard NACK enable"]
            pub fn set_nack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Smartcard mode enable"]
            pub const fn scen(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Smartcard mode enable"]
            pub fn set_scen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DMA enable transmitter"]
            pub const fn dmat(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable transmitter"]
            pub fn set_dmat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "RTS enable"]
            pub const fn rtse(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "RTS enable"]
            pub fn set_rtse(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "CTS enable"]
            pub const fn ctse(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "CTS enable"]
            pub fn set_ctse(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "CTS interrupt enable"]
            pub const fn ctsie(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "CTS interrupt enable"]
            pub fn set_ctsie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "One sample bit method enable"]
            pub const fn onebit(&self) -> super::vals::Onebit {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Onebit(val as u8)
            }
            #[doc = "One sample bit method enable"]
            pub fn set_onebit(&mut self, val: super::vals::Onebit) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "Overrun Disable"]
            pub const fn ovrdis(&self) -> super::vals::Ovrdis {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Ovrdis(val as u8)
            }
            #[doc = "Overrun Disable"]
            pub fn set_ovrdis(&mut self, val: super::vals::Ovrdis) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "DMA Disable on Reception Error"]
            pub const fn ddre(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "DMA Disable on Reception Error"]
            pub fn set_ddre(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Driver enable mode"]
            pub const fn dem(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Driver enable mode"]
            pub fn set_dem(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Driver enable polarity selection"]
            pub const fn dep(&self) -> super::vals::Dep {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Dep(val as u8)
            }
            #[doc = "Driver enable polarity selection"]
            pub fn set_dep(&mut self, val: super::vals::Dep) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "Smartcard auto-retry count"]
            pub const fn scarcnt(&self) -> u8 {
                let val = (self.0 >> 17usize) & 0x07;
                val as u8
            }
            #[doc = "Smartcard auto-retry count"]
            pub fn set_scarcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
            }
            #[doc = "Wakeup from Stop mode interrupt flag selection"]
            pub const fn wus(&self) -> super::vals::Wus {
                let val = (self.0 >> 20usize) & 0x03;
                super::vals::Wus(val as u8)
            }
            #[doc = "Wakeup from Stop mode interrupt flag selection"]
            pub fn set_wus(&mut self, val: super::vals::Wus) {
                self.0 = (self.0 & !(0x03 << 20usize)) | (((val.0 as u32) & 0x03) << 20usize);
            }
            #[doc = "Wakeup from Stop mode interrupt enable"]
            pub const fn wufie(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Wakeup from Stop mode interrupt enable"]
            pub fn set_wufie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Cr3 {
            fn default() -> Cr3 {
                Cr3(0)
            }
        }
        #[doc = "Interrupt & status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ixr(pub u32);
        impl Ixr {
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Noise detected flag"]
            pub const fn nf(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Noise detected flag"]
            pub fn set_nf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Idle line detected"]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Idle line detected"]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete"]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete"]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "LIN break detection flag"]
            pub const fn lbdf(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection flag"]
            pub fn set_lbdf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "CTS interrupt flag"]
            pub const fn ctsif(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "CTS interrupt flag"]
            pub fn set_ctsif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "CTS flag"]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "CTS flag"]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Receiver timeout"]
            pub const fn rtof(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver timeout"]
            pub fn set_rtof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "End of block flag"]
            pub const fn eobf(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "End of block flag"]
            pub fn set_eobf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Auto baud rate error"]
            pub const fn abre(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Auto baud rate error"]
            pub fn set_abre(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Auto baud rate flag"]
            pub const fn abrf(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Auto baud rate flag"]
            pub fn set_abrf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Busy flag"]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Busy flag"]
            pub fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "character match flag"]
            pub const fn cmf(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "character match flag"]
            pub fn set_cmf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Send break flag"]
            pub const fn sbkf(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Send break flag"]
            pub fn set_sbkf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Receiver wakeup from Mute mode"]
            pub const fn rwu(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver wakeup from Mute mode"]
            pub fn set_rwu(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Wakeup from Stop mode flag"]
            pub const fn wuf(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Wakeup from Stop mode flag"]
            pub fn set_wuf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Transmit enable acknowledge flag"]
            pub const fn teack(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit enable acknowledge flag"]
            pub fn set_teack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Receive enable acknowledge flag"]
            pub const fn reack(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Receive enable acknowledge flag"]
            pub fn set_reack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Ixr {
            fn default() -> Ixr {
                Ixr(0)
            }
        }
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "7-bit Address Detection/4-bit Address Detection"]
            pub fn addm(&self, n: usize) -> super::vals::Addm {
                assert!(n < 1usize);
                let offs = 4usize + n * 0usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Addm(val as u8)
            }
            #[doc = "7-bit Address Detection/4-bit Address Detection"]
            pub fn set_addm(&mut self, n: usize, val: super::vals::Addm) {
                assert!(n < 1usize);
                let offs = 4usize + n * 0usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "LIN break detection length"]
            pub const fn lbdl(&self) -> super::vals::Lbdl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lbdl(val as u8)
            }
            #[doc = "LIN break detection length"]
            pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Last bit clock pulse"]
            pub const fn lbcl(&self) -> super::vals::Lbcl {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Lbcl(val as u8)
            }
            #[doc = "Last bit clock pulse"]
            pub fn set_lbcl(&mut self, val: super::vals::Lbcl) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Clock enable"]
            pub const fn clken(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Clock enable"]
            pub fn set_clken(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "STOP bits"]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Stop(val as u8)
            }
            #[doc = "STOP bits"]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Swap TX/RX pins"]
            pub const fn swap(&self) -> super::vals::Swap {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Swap(val as u8)
            }
            #[doc = "Swap TX/RX pins"]
            pub fn set_swap(&mut self, val: super::vals::Swap) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "RX pin active level inversion"]
            pub const fn rxinv(&self) -> super::vals::Rxinv {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Rxinv(val as u8)
            }
            #[doc = "RX pin active level inversion"]
            pub fn set_rxinv(&mut self, val: super::vals::Rxinv) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
            #[doc = "TX pin active level inversion"]
            pub const fn txinv(&self) -> super::vals::Txinv {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Txinv(val as u8)
            }
            #[doc = "TX pin active level inversion"]
            pub fn set_txinv(&mut self, val: super::vals::Txinv) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
            }
            #[doc = "Binary data inversion"]
            pub const fn datainv(&self) -> super::vals::Datainv {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Datainv(val as u8)
            }
            #[doc = "Binary data inversion"]
            pub fn set_datainv(&mut self, val: super::vals::Datainv) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "Most significant bit first"]
            pub const fn msbfirst(&self) -> super::vals::Msbfirst {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Msbfirst(val as u8)
            }
            #[doc = "Most significant bit first"]
            pub fn set_msbfirst(&mut self, val: super::vals::Msbfirst) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
            }
            #[doc = "Auto baud rate enable"]
            pub const fn abren(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Auto baud rate enable"]
            pub fn set_abren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Auto baud rate mode"]
            pub const fn abrmod(&self) -> super::vals::Abrmod {
                let val = (self.0 >> 21usize) & 0x03;
                super::vals::Abrmod(val as u8)
            }
            #[doc = "Auto baud rate mode"]
            pub fn set_abrmod(&mut self, val: super::vals::Abrmod) {
                self.0 = (self.0 & !(0x03 << 21usize)) | (((val.0 as u32) & 0x03) << 21usize);
            }
            #[doc = "Receiver timeout enable"]
            pub const fn rtoen(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver timeout enable"]
            pub fn set_rtoen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "data value"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "data value"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
        #[doc = "Baud rate register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "mantissa of USARTDIV"]
            pub const fn brr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "mantissa of USARTDIV"]
            pub fn set_brr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Guard time and prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gtpr(pub u32);
        impl Gtpr {
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Guard time value"]
            pub const fn gt(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Guard time value"]
            pub fn set_gt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Gtpr {
            fn default() -> Gtpr {
                Gtpr(0)
            }
        }
    }
}
pub mod generic {
    use core::marker::PhantomData;
    #[derive(Copy, Clone)]
    pub struct RW;
    #[derive(Copy, Clone)]
    pub struct R;
    #[derive(Copy, Clone)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        pub fn from_ptr(ptr: *mut u8) -> Self {
            Self {
                ptr,
                phantom: PhantomData,
            }
        }
        pub fn ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        pub unsafe fn read(&self) -> T {
            (self.ptr as *mut T).read_volatile()
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        pub unsafe fn write_value(&self, val: T) {
            (self.ptr as *mut T).write_volatile(val)
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        pub unsafe fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        pub unsafe fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod spi_v3 {
    use crate::generic::*;
    #[doc = "Serial peripheral interface"]
    #[derive(Copy, Clone)]
    pub struct Spi(pub *mut u8);
    unsafe impl Send for Spi {}
    unsafe impl Sync for Spi {}
    impl Spi {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "configuration register 1"]
        pub fn cfg1(self) -> Reg<regs::Cfg1, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "configuration register 2"]
        pub fn cfg2(self) -> Reg<regs::Cfg2, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Interrupt Enable Register"]
        pub fn ier(self) -> Reg<regs::Ier, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Status Register"]
        pub fn sr(self) -> Reg<regs::Sr, R> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Interrupt/Status Flags Clear Register"]
        pub fn ifcr(self) -> Reg<regs::Ifcr, W> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Transmit Data Register"]
        pub fn txdr(self) -> Reg<regs::Txdr, W> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "Receive Data Register"]
        pub fn rxdr(self) -> Reg<regs::Rxdr, R> {
            unsafe { Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "Polynomial Register"]
        pub fn crcpoly(self) -> Reg<regs::Crcpoly, RW> {
            unsafe { Reg::from_ptr(self.0.add(64usize)) }
        }
        #[doc = "Transmitter CRC Register"]
        pub fn txcrc(self) -> Reg<regs::Txcrc, RW> {
            unsafe { Reg::from_ptr(self.0.add(68usize)) }
        }
        #[doc = "Receiver CRC Register"]
        pub fn rxcrc(self) -> Reg<regs::Rxcrc, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "Underrun Data Register"]
        pub fn udrdr(self) -> Reg<regs::Udrdr, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mbr(pub u8);
        impl Mbr {
            #[doc = "f_spi_ker_ck / 2"]
            pub const DIV2: Self = Self(0);
            #[doc = "f_spi_ker_ck / 4"]
            pub const DIV4: Self = Self(0x01);
            #[doc = "f_spi_ker_ck / 8"]
            pub const DIV8: Self = Self(0x02);
            #[doc = "f_spi_ker_ck / 16"]
            pub const DIV16: Self = Self(0x03);
            #[doc = "f_spi_ker_ck / 32"]
            pub const DIV32: Self = Self(0x04);
            #[doc = "f_spi_ker_ck / 64"]
            pub const DIV64: Self = Self(0x05);
            #[doc = "f_spi_ker_ck / 128"]
            pub const DIV128: Self = Self(0x06);
            #[doc = "f_spi_ker_ck / 256"]
            pub const DIV256: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Master(pub u8);
        impl Master {
            #[doc = "Slave configuration"]
            pub const SLAVE: Self = Self(0);
            #[doc = "Master configuration"]
            pub const MASTER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Udrdet(pub u8);
        impl Udrdet {
            #[doc = "Underrun is detected at begin of data frame"]
            pub const STARTOFFRAME: Self = Self(0);
            #[doc = "Underrun is detected at end of last data frame"]
            pub const ENDOFFRAME: Self = Self(0x01);
            #[doc = "Underrun is detected at begin of active SS signal"]
            pub const STARTOFSLAVESELECT: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ssiop(pub u8);
        impl Ssiop {
            #[doc = "Low level is active for SS signal"]
            pub const ACTIVELOW: Self = Self(0);
            #[doc = "High level is active for SS signal"]
            pub const ACTIVEHIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Fthlv(pub u8);
        impl Fthlv {
            #[doc = "1 frame"]
            pub const ONEFRAME: Self = Self(0);
            #[doc = "2 frames"]
            pub const TWOFRAMES: Self = Self(0x01);
            #[doc = "3 frames"]
            pub const THREEFRAMES: Self = Self(0x02);
            #[doc = "4 frames"]
            pub const FOURFRAMES: Self = Self(0x03);
            #[doc = "5 frames"]
            pub const FIVEFRAMES: Self = Self(0x04);
            #[doc = "6 frames"]
            pub const SIXFRAMES: Self = Self(0x05);
            #[doc = "7 frames"]
            pub const SEVENFRAMES: Self = Self(0x06);
            #[doc = "8 frames"]
            pub const EIGHTFRAMES: Self = Self(0x07);
            #[doc = "9 frames"]
            pub const NINEFRAMES: Self = Self(0x08);
            #[doc = "10 frames"]
            pub const TENFRAMES: Self = Self(0x09);
            #[doc = "11 frames"]
            pub const ELEVENFRAMES: Self = Self(0x0a);
            #[doc = "12 frames"]
            pub const TWELVEFRAMES: Self = Self(0x0b);
            #[doc = "13 frames"]
            pub const THIRTEENFRAMES: Self = Self(0x0c);
            #[doc = "14 frames"]
            pub const FOURTEENFRAMES: Self = Self(0x0d);
            #[doc = "15 frames"]
            pub const FIFTEENFRAMES: Self = Self(0x0e);
            #[doc = "16 frames"]
            pub const SIXTEENFRAMES: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxwne(pub u8);
        impl Rxwne {
            #[doc = "Less than 32-bit data frame received"]
            pub const LESSTHAN32: Self = Self(0);
            #[doc = "At least 32-bit data frame received"]
            pub const ATLEAST32: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Comm(pub u8);
        impl Comm {
            #[doc = "Full duplex"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Simplex transmitter only"]
            pub const TRANSMITTER: Self = Self(0x01);
            #[doc = "Simplex receiver only"]
            pub const RECEIVER: Self = Self(0x02);
            #[doc = "Half duplex"]
            pub const HALFDUPLEX: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hddir(pub u8);
        impl Hddir {
            #[doc = "Receiver in half duplex mode"]
            pub const RECEIVER: Self = Self(0);
            #[doc = "Transmitter in half duplex mode"]
            pub const TRANSMITTER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crc(pub u8);
        impl Crc {
            #[doc = "Full size (33/17 bit) CRC polynomial is not used"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Full size (33/17 bit) CRC polynomial is used"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Datfmt(pub u8);
        impl Datfmt {
            #[doc = "The data inside RXDR and TXDR are right aligned"]
            pub const RIGHTALIGNED: Self = Self(0);
            #[doc = "The data inside RXDR and TXDR are left aligned"]
            pub const LEFTALIGNED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sp(pub u8);
        impl Sp {
            #[doc = "Motorola SPI protocol"]
            pub const MOTOROLA: Self = Self(0);
            #[doc = "TI SPI protocol"]
            pub const TI: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Afcntr(pub u8);
        impl Afcntr {
            #[doc = "Peripheral takes no control of GPIOs while disabled"]
            pub const NOTCONTROLLED: Self = Self(0);
            #[doc = "Peripheral controls GPIOs while disabled"]
            pub const CONTROLLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ssom(pub u8);
        impl Ssom {
            #[doc = "SS is asserted until data transfer complete"]
            pub const ASSERTED: Self = Self(0);
            #[doc = "Data frames interleaved with SS not asserted during MIDI"]
            pub const NOTASSERTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Udrcfg(pub u8);
        impl Udrcfg {
            #[doc = "Slave sends a constant underrun pattern"]
            pub const CONSTANT: Self = Self(0);
            #[doc = "Slave repeats last received data frame from master"]
            pub const REPEATRECEIVED: Self = Self(0x01);
            #[doc = "Slave repeats last transmitted data frame"]
            pub const REPEATTRANSMITTED: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Tcrcini(pub u8);
        impl Tcrcini {
            #[doc = "All zeros TX CRC initialization pattern"]
            pub const ALLZEROS: Self = Self(0);
            #[doc = "All ones TX CRC initialization pattern"]
            pub const ALLONES: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRSTEDGE: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECONDEDGE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "CK to 0 when idle"]
            pub const IDLELOW: Self = Self(0);
            #[doc = "CK to 1 when idle"]
            pub const IDLEHIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rcrcini(pub u8);
        impl Rcrcini {
            #[doc = "All zeros RX CRC initialization pattern"]
            pub const ALLZEROS: Self = Self(0);
            #[doc = "All ones RX CRC initialization pattern"]
            pub const ALLONES: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Datlen(pub u8);
        impl Datlen {
            #[doc = "16 bit data length"]
            pub const BITS16: Self = Self(0);
            #[doc = "24 bit data length"]
            pub const BITS24: Self = Self(0x01);
            #[doc = "32 bit data length"]
            pub const BITS32: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxplvl(pub u8);
        impl Rxplvl {
            #[doc = "Zero frames beyond packing ratio available"]
            pub const ZEROFRAMES: Self = Self(0);
            #[doc = "One frame beyond packing ratio available"]
            pub const ONEFRAME: Self = Self(0x01);
            #[doc = "Two frame beyond packing ratio available"]
            pub const TWOFRAMES: Self = Self(0x02);
            #[doc = "Three frame beyond packing ratio available"]
            pub const THREEFRAMES: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lsbfrst(pub u8);
        impl Lsbfrst {
            #[doc = "Data is transmitted/received with the MSB first"]
            pub const MSBFIRST: Self = Self(0);
            #[doc = "Data is transmitted/received with the LSB first"]
            pub const LSBFIRST: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Underrun Data Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Udrdr(pub u32);
        impl Udrdr {
            #[doc = "Data at slave underrun condition"]
            pub const fn udrdr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Data at slave underrun condition"]
            pub fn set_udrdr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Udrdr {
            fn default() -> Udrdr {
                Udrdr(0)
            }
        }
        #[doc = "Transmitter CRC Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txcrc(pub u32);
        impl Txcrc {
            #[doc = "CRC register for transmitter"]
            pub const fn txcrc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "CRC register for transmitter"]
            pub fn set_txcrc(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Txcrc {
            fn default() -> Txcrc {
                Txcrc(0)
            }
        }
        #[doc = "Polynomial Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpoly(pub u32);
        impl Crcpoly {
            #[doc = "CRC polynomial register"]
            pub const fn crcpoly(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "CRC polynomial register"]
            pub fn set_crcpoly(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Crcpoly {
            fn default() -> Crcpoly {
                Crcpoly(0)
            }
        }
        #[doc = "configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg1(pub u32);
        impl Cfg1 {
            #[doc = "Number of bits in at single SPI data frame"]
            pub const fn dsize(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Number of bits in at single SPI data frame"]
            pub fn set_dsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "threshold level"]
            pub const fn fthlv(&self) -> super::vals::Fthlv {
                let val = (self.0 >> 5usize) & 0x0f;
                super::vals::Fthlv(val as u8)
            }
            #[doc = "threshold level"]
            pub fn set_fthlv(&mut self, val: super::vals::Fthlv) {
                self.0 = (self.0 & !(0x0f << 5usize)) | (((val.0 as u32) & 0x0f) << 5usize);
            }
            #[doc = "Behavior of slave transmitter at underrun condition"]
            pub const fn udrcfg(&self) -> super::vals::Udrcfg {
                let val = (self.0 >> 9usize) & 0x03;
                super::vals::Udrcfg(val as u8)
            }
            #[doc = "Behavior of slave transmitter at underrun condition"]
            pub fn set_udrcfg(&mut self, val: super::vals::Udrcfg) {
                self.0 = (self.0 & !(0x03 << 9usize)) | (((val.0 as u32) & 0x03) << 9usize);
            }
            #[doc = "Detection of underrun condition at slave transmitter"]
            pub const fn udrdet(&self) -> super::vals::Udrdet {
                let val = (self.0 >> 11usize) & 0x03;
                super::vals::Udrdet(val as u8)
            }
            #[doc = "Detection of underrun condition at slave transmitter"]
            pub fn set_udrdet(&mut self, val: super::vals::Udrdet) {
                self.0 = (self.0 & !(0x03 << 11usize)) | (((val.0 as u32) & 0x03) << 11usize);
            }
            #[doc = "Rx DMA stream enable"]
            pub const fn rxdmaen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Rx DMA stream enable"]
            pub fn set_rxdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Tx DMA stream enable"]
            pub const fn txdmaen(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Tx DMA stream enable"]
            pub fn set_txdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Length of CRC frame to be transacted and compared"]
            pub const fn crcsize(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x1f;
                val as u8
            }
            #[doc = "Length of CRC frame to be transacted and compared"]
            pub fn set_crcsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
            }
            #[doc = "Hardware CRC computation enable"]
            pub const fn crcen(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware CRC computation enable"]
            pub fn set_crcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Master baud rate"]
            pub const fn mbr(&self) -> super::vals::Mbr {
                let val = (self.0 >> 28usize) & 0x07;
                super::vals::Mbr(val as u8)
            }
            #[doc = "Master baud rate"]
            pub fn set_mbr(&mut self, val: super::vals::Mbr) {
                self.0 = (self.0 & !(0x07 << 28usize)) | (((val.0 as u32) & 0x07) << 28usize);
            }
        }
        impl Default for Cfg1 {
            fn default() -> Cfg1 {
                Cfg1(0)
            }
        }
        #[doc = "configuration register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg2(pub u32);
        impl Cfg2 {
            #[doc = "Master SS Idleness"]
            pub const fn mssi(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Master SS Idleness"]
            pub fn set_mssi(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Master Inter-Data Idleness"]
            pub const fn midi(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "Master Inter-Data Idleness"]
            pub fn set_midi(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
            #[doc = "Swap functionality of MISO and MOSI pins"]
            pub const fn ioswp(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Swap functionality of MISO and MOSI pins"]
            pub fn set_ioswp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "SPI Communication Mode"]
            pub const fn comm(&self) -> super::vals::Comm {
                let val = (self.0 >> 17usize) & 0x03;
                super::vals::Comm(val as u8)
            }
            #[doc = "SPI Communication Mode"]
            pub fn set_comm(&mut self, val: super::vals::Comm) {
                self.0 = (self.0 & !(0x03 << 17usize)) | (((val.0 as u32) & 0x03) << 17usize);
            }
            #[doc = "Serial Protocol"]
            pub const fn sp(&self) -> super::vals::Sp {
                let val = (self.0 >> 19usize) & 0x07;
                super::vals::Sp(val as u8)
            }
            #[doc = "Serial Protocol"]
            pub fn set_sp(&mut self, val: super::vals::Sp) {
                self.0 = (self.0 & !(0x07 << 19usize)) | (((val.0 as u32) & 0x07) << 19usize);
            }
            #[doc = "SPI Master"]
            pub const fn master(&self) -> super::vals::Master {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Master(val as u8)
            }
            #[doc = "SPI Master"]
            pub fn set_master(&mut self, val: super::vals::Master) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val.0 as u32) & 0x01) << 22usize);
            }
            #[doc = "Data frame format"]
            pub const fn lsbfrst(&self) -> super::vals::Lsbfrst {
                let val = (self.0 >> 23usize) & 0x01;
                super::vals::Lsbfrst(val as u8)
            }
            #[doc = "Data frame format"]
            pub fn set_lsbfrst(&mut self, val: super::vals::Lsbfrst) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val.0 as u32) & 0x01) << 23usize);
            }
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 25usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val.0 as u32) & 0x01) << 25usize);
            }
            #[doc = "Software management of SS signal input"]
            pub const fn ssm(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Software management of SS signal input"]
            pub fn set_ssm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "SS input/output polarity"]
            pub const fn ssiop(&self) -> super::vals::Ssiop {
                let val = (self.0 >> 28usize) & 0x01;
                super::vals::Ssiop(val as u8)
            }
            #[doc = "SS input/output polarity"]
            pub fn set_ssiop(&mut self, val: super::vals::Ssiop) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val.0 as u32) & 0x01) << 28usize);
            }
            #[doc = "SS output enable"]
            pub const fn ssoe(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "SS output enable"]
            pub fn set_ssoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "SS output management in master mode"]
            pub const fn ssom(&self) -> super::vals::Ssom {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Ssom(val as u8)
            }
            #[doc = "SS output management in master mode"]
            pub fn set_ssom(&mut self, val: super::vals::Ssom) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val.0 as u32) & 0x01) << 30usize);
            }
            #[doc = "Alternate function GPIOs control"]
            pub const fn afcntr(&self) -> super::vals::Afcntr {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Afcntr(val as u8)
            }
            #[doc = "Alternate function GPIOs control"]
            pub fn set_afcntr(&mut self, val: super::vals::Afcntr) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Cfg2 {
            fn default() -> Cfg2 {
                Cfg2(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Number of data at current transfer"]
            pub const fn tsize(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of data at current transfer"]
            pub fn set_tsize(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "Number of data transfer extension to be reload into TSIZE just when a previous"]
            pub const fn tser(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of data transfer extension to be reload into TSIZE just when a previous"]
            pub fn set_tser(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Receive Data Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxdr(pub u32);
        impl Rxdr {
            #[doc = "Receive data register"]
            pub const fn rxdr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Receive data register"]
            pub fn set_rxdr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Rxdr {
            fn default() -> Rxdr {
                Rxdr(0)
            }
        }
        #[doc = "Interrupt Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ier(pub u32);
        impl Ier {
            #[doc = "RXP Interrupt Enable"]
            pub const fn rxpie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RXP Interrupt Enable"]
            pub fn set_rxpie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TXP interrupt enable"]
            pub const fn txpie(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "TXP interrupt enable"]
            pub fn set_txpie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "DXP interrupt enabled"]
            pub const fn dxpie(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "DXP interrupt enabled"]
            pub fn set_dxpie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "EOT, SUSP and TXC interrupt enable"]
            pub const fn eotie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "EOT, SUSP and TXC interrupt enable"]
            pub fn set_eotie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "TXTFIE interrupt enable"]
            pub const fn txtfie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "TXTFIE interrupt enable"]
            pub fn set_txtfie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "UDR interrupt enable"]
            pub const fn udrie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "UDR interrupt enable"]
            pub fn set_udrie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "OVR interrupt enable"]
            pub const fn ovrie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "OVR interrupt enable"]
            pub fn set_ovrie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "CRC Interrupt enable"]
            pub const fn crceie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "CRC Interrupt enable"]
            pub fn set_crceie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TIFRE interrupt enable"]
            pub const fn tifreie(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TIFRE interrupt enable"]
            pub fn set_tifreie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Mode Fault interrupt enable"]
            pub const fn modfie(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Mode Fault interrupt enable"]
            pub fn set_modfie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Additional number of transactions reload interrupt enable"]
            pub const fn tserfie(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Additional number of transactions reload interrupt enable"]
            pub fn set_tserfie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Ier {
            fn default() -> Ier {
                Ier(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Serial Peripheral Enable"]
            pub const fn spe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Serial Peripheral Enable"]
            pub fn set_spe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Master automatic SUSP in Receive mode"]
            pub const fn masrx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Master automatic SUSP in Receive mode"]
            pub fn set_masrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Master transfer start"]
            pub const fn cstart(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Master transfer start"]
            pub fn set_cstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Master SUSPend request"]
            pub const fn csusp(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Master SUSPend request"]
            pub fn set_csusp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Rx/Tx direction at Half-duplex mode"]
            pub const fn hddir(&self) -> super::vals::Hddir {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Hddir(val as u8)
            }
            #[doc = "Rx/Tx direction at Half-duplex mode"]
            pub fn set_hddir(&mut self, val: super::vals::Hddir) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "Internal SS signal input level"]
            pub const fn ssi(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Internal SS signal input level"]
            pub fn set_ssi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "32-bit CRC polynomial configuration"]
            pub const fn crc33_17(&self) -> super::vals::Crc {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Crc(val as u8)
            }
            #[doc = "32-bit CRC polynomial configuration"]
            pub fn set_crc33_17(&mut self, val: super::vals::Crc) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val.0 as u32) & 0x01) << 13usize);
            }
            #[doc = "CRC calculation initialization pattern control for receiver"]
            pub const fn rcrcini(&self) -> super::vals::Rcrcini {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Rcrcini(val as u8)
            }
            #[doc = "CRC calculation initialization pattern control for receiver"]
            pub fn set_rcrcini(&mut self, val: super::vals::Rcrcini) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "CRC calculation initialization pattern control for transmitter"]
            pub const fn tcrcini(&self) -> super::vals::Tcrcini {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Tcrcini(val as u8)
            }
            #[doc = "CRC calculation initialization pattern control for transmitter"]
            pub fn set_tcrcini(&mut self, val: super::vals::Tcrcini) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "Locking the AF configuration of associated IOs"]
            pub const fn iolock(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Locking the AF configuration of associated IOs"]
            pub fn set_iolock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "Receiver CRC Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrc(pub u32);
        impl Rxcrc {
            #[doc = "CRC register for receiver"]
            pub const fn rxcrc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "CRC register for receiver"]
            pub fn set_rxcrc(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Rxcrc {
            fn default() -> Rxcrc {
                Rxcrc(0)
            }
        }
        #[doc = "Transmit Data Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txdr(pub u32);
        impl Txdr {
            #[doc = "Transmit data register"]
            pub const fn txdr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Transmit data register"]
            pub fn set_txdr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Txdr {
            fn default() -> Txdr {
                Txdr(0)
            }
        }
        #[doc = "Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Rx-Packet available"]
            pub const fn rxp(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Rx-Packet available"]
            pub fn set_rxp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Tx-Packet space available"]
            pub const fn txp(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Tx-Packet space available"]
            pub fn set_txp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Duplex Packet"]
            pub const fn dxp(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Duplex Packet"]
            pub fn set_dxp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "End Of Transfer"]
            pub const fn eot(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "End Of Transfer"]
            pub fn set_eot(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Transmission Transfer Filled"]
            pub const fn txtf(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission Transfer Filled"]
            pub fn set_txtf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Underrun at slave transmission mode"]
            pub const fn udr(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Underrun at slave transmission mode"]
            pub fn set_udr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Overrun"]
            pub const fn ovr(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun"]
            pub fn set_ovr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "CRC Error"]
            pub const fn crce(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "CRC Error"]
            pub fn set_crce(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TI frame format error"]
            pub const fn tifre(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TI frame format error"]
            pub fn set_tifre(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Mode Fault"]
            pub const fn modf(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Mode Fault"]
            pub fn set_modf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Additional number of SPI data to be transacted was reload"]
            pub const fn tserf(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Additional number of SPI data to be transacted was reload"]
            pub fn set_tserf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "SUSPend"]
            pub const fn susp(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "SUSPend"]
            pub fn set_susp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "TxFIFO transmission complete"]
            pub const fn txc(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TxFIFO transmission complete"]
            pub fn set_txc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "RxFIFO Packing LeVeL"]
            pub const fn rxplvl(&self) -> super::vals::Rxplvl {
                let val = (self.0 >> 13usize) & 0x03;
                super::vals::Rxplvl(val as u8)
            }
            #[doc = "RxFIFO Packing LeVeL"]
            pub fn set_rxplvl(&mut self, val: super::vals::Rxplvl) {
                self.0 = (self.0 & !(0x03 << 13usize)) | (((val.0 as u32) & 0x03) << 13usize);
            }
            #[doc = "RxFIFO Word Not Empty"]
            pub const fn rxwne(&self) -> super::vals::Rxwne {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Rxwne(val as u8)
            }
            #[doc = "RxFIFO Word Not Empty"]
            pub fn set_rxwne(&mut self, val: super::vals::Rxwne) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "Number of data frames remaining in current TSIZE session"]
            pub const fn ctsize(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of data frames remaining in current TSIZE session"]
            pub fn set_ctsize(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "Interrupt/Status Flags Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ifcr(pub u32);
        impl Ifcr {
            #[doc = "End Of Transfer flag clear"]
            pub const fn eotc(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "End Of Transfer flag clear"]
            pub fn set_eotc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Transmission Transfer Filled flag clear"]
            pub const fn txtfc(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission Transfer Filled flag clear"]
            pub fn set_txtfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Underrun flag clear"]
            pub const fn udrc(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Underrun flag clear"]
            pub fn set_udrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Overrun flag clear"]
            pub const fn ovrc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun flag clear"]
            pub fn set_ovrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "CRC Error flag clear"]
            pub const fn crcec(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "CRC Error flag clear"]
            pub fn set_crcec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TI frame format error flag clear"]
            pub const fn tifrec(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TI frame format error flag clear"]
            pub fn set_tifrec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Mode Fault flag clear"]
            pub const fn modfc(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Mode Fault flag clear"]
            pub fn set_modfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TSERFC flag clear"]
            pub const fn tserfc(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TSERFC flag clear"]
            pub fn set_tserfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "SUSPend flag clear"]
            pub const fn suspc(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "SUSPend flag clear"]
            pub fn set_suspc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
        }
        impl Default for Ifcr {
            fn default() -> Ifcr {
                Ifcr(0)
            }
        }
    }
}
pub mod syscfg_l0 {
    use crate::generic::*;
    #[doc = "System configuration controller"]
    #[derive(Copy, Clone)]
    pub struct Syscfg(pub *mut u8);
    unsafe impl Send for Syscfg {}
    unsafe impl Sync for Syscfg {}
    impl Syscfg {
        #[doc = "configuration register 1"]
        pub fn cfgr1(self) -> Reg<regs::Cfgr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "CFGR2"]
        pub fn cfgr2(self) -> Reg<regs::Cfgr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "external interrupt configuration register"]
        pub fn exticr(self, n: usize) -> Reg<regs::Exticr, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "CFGR3"]
        pub fn cfgr3(self) -> Reg<regs::Cfgr3, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "external interrupt configuration register 1-4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Exticr(pub u32);
        impl Exticr {
            #[doc = "EXTI configuration bits"]
            pub fn exti(&self, n: usize) -> u8 {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                val as u8
            }
            #[doc = "EXTI configuration bits"]
            pub fn set_exti(&mut self, n: usize, val: u8) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
            }
        }
        impl Default for Exticr {
            fn default() -> Exticr {
                Exticr(0)
            }
        }
        #[doc = "CFGR2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfgr2(pub u32);
        impl Cfgr2 {
            #[doc = "Firewall disable bit"]
            pub const fn fwdis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Firewall disable bit"]
            pub fn set_fwdis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Fm+ drive capability on PB6 enable bit"]
            pub const fn i2c_pb6_fmp(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Fm+ drive capability on PB6 enable bit"]
            pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Fm+ drive capability on PB7 enable bit"]
            pub const fn i2c_pb7_fmp(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Fm+ drive capability on PB7 enable bit"]
            pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Fm+ drive capability on PB8 enable bit"]
            pub const fn i2c_pb8_fmp(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Fm+ drive capability on PB8 enable bit"]
            pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Fm+ drive capability on PB9 enable bit"]
            pub const fn i2c_pb9_fmp(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Fm+ drive capability on PB9 enable bit"]
            pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "I2C1 Fm+ drive capability enable bit"]
            pub const fn i2c1_fmp(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "I2C1 Fm+ drive capability enable bit"]
            pub fn set_i2c1_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "I2C2 Fm+ drive capability enable bit"]
            pub const fn i2c2_fmp(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "I2C2 Fm+ drive capability enable bit"]
            pub fn set_i2c2_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "I2C3 Fm+ drive capability enable bit"]
            pub const fn i2c3_fmp(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "I2C3 Fm+ drive capability enable bit"]
            pub fn set_i2c3_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cfgr2 {
            fn default() -> Cfgr2 {
                Cfgr2(0)
            }
        }
        #[doc = "configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfgr1(pub u32);
        impl Cfgr1 {
            #[doc = "Memory mapping selection bits"]
            pub const fn mem_mode(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "Memory mapping selection bits"]
            pub fn set_mem_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "User bank swapping"]
            pub const fn ufb(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "User bank swapping"]
            pub fn set_ufb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Boot mode selected by the boot pins status bits"]
            pub const fn boot_mode(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x03;
                val as u8
            }
            #[doc = "Boot mode selected by the boot pins status bits"]
            pub fn set_boot_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Cfgr1 {
            fn default() -> Cfgr1 {
                Cfgr1(0)
            }
        }
        #[doc = "CFGR3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfgr3(pub u32);
        impl Cfgr3 {
            #[doc = "VREFINT enable and scaler control for COMP2 enable bit"]
            pub const fn en_vrefint(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "VREFINT enable and scaler control for COMP2 enable bit"]
            pub fn set_en_vrefint(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "VREFINT_ADC connection bit"]
            pub const fn sel_vref_out(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x03;
                val as u8
            }
            #[doc = "VREFINT_ADC connection bit"]
            pub fn set_sel_vref_out(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
            }
            #[doc = "VREFINT reference for ADC enable bit"]
            pub const fn enbuf_vrefint_adc(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "VREFINT reference for ADC enable bit"]
            pub fn set_enbuf_vrefint_adc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Temperature sensor reference for ADC enable bit"]
            pub const fn enbuf_sensor_adc(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Temperature sensor reference for ADC enable bit"]
            pub fn set_enbuf_sensor_adc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "VREFINT reference for COMP2 scaler enable bit"]
            pub const fn enbuf_vrefint_comp2(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "VREFINT reference for COMP2 scaler enable bit"]
            pub fn set_enbuf_vrefint_comp2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "VREFINT reference for HSI48 oscillator enable bit"]
            pub const fn enref_hsi48(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "VREFINT reference for HSI48 oscillator enable bit"]
            pub fn set_enref_hsi48(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "VREFINT ready flag"]
            pub const fn vrefint_rdyf(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "VREFINT ready flag"]
            pub fn set_vrefint_rdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "SYSCFG_CFGR3 lock bit"]
            pub const fn ref_lock(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "SYSCFG_CFGR3 lock bit"]
            pub fn set_ref_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Cfgr3 {
            fn default() -> Cfgr3 {
                Cfgr3(0)
            }
        }
    }
}
pub mod rcc_l0 {
    use crate::generic::*;
    #[doc = "Reset and clock control"]
    #[derive(Copy, Clone)]
    pub struct Rcc(pub *mut u8);
    unsafe impl Send for Rcc {}
    unsafe impl Sync for Rcc {}
    impl Rcc {
        #[doc = "Clock control register"]
        pub fn cr(self) -> Reg<regs::Cr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Internal clock sources calibration register"]
        pub fn icscr(self) -> Reg<regs::Icscr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Clock recovery RC register"]
        pub fn crrcr(self) -> Reg<regs::Crrcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Clock configuration register"]
        pub fn cfgr(self) -> Reg<regs::Cfgr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Clock interrupt enable register"]
        pub fn cier(self) -> Reg<regs::Cier, R> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Clock interrupt flag register"]
        pub fn cifr(self) -> Reg<regs::Cifr, R> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Clock interrupt clear register"]
        pub fn cicr(self) -> Reg<regs::Cicr, R> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "GPIO reset register"]
        pub fn ioprstr(self) -> Reg<regs::Ioprstr, RW> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "AHB peripheral reset register"]
        pub fn ahbrstr(self) -> Reg<regs::Ahbrstr, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "APB2 peripheral reset register"]
        pub fn apb2rstr(self) -> Reg<regs::Apb2rstr, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "APB1 peripheral reset register"]
        pub fn apb1rstr(self) -> Reg<regs::Apb1rstr, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "GPIO clock enable register"]
        pub fn iopenr(self) -> Reg<regs::Iopenr, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "AHB peripheral clock enable register"]
        pub fn ahbenr(self) -> Reg<regs::Ahbenr, RW> {
            unsafe { Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "APB2 peripheral clock enable register"]
        pub fn apb2enr(self) -> Reg<regs::Apb2enr, RW> {
            unsafe { Reg::from_ptr(self.0.add(52usize)) }
        }
        #[doc = "APB1 peripheral clock enable register"]
        pub fn apb1enr(self) -> Reg<regs::Apb1enr, RW> {
            unsafe { Reg::from_ptr(self.0.add(56usize)) }
        }
        #[doc = "GPIO clock enable in sleep mode register"]
        pub fn iopsmen(self) -> Reg<regs::Iopsmen, RW> {
            unsafe { Reg::from_ptr(self.0.add(60usize)) }
        }
        #[doc = "AHB peripheral clock enable in sleep mode register"]
        pub fn ahbsmenr(self) -> Reg<regs::Ahbsmenr, RW> {
            unsafe { Reg::from_ptr(self.0.add(64usize)) }
        }
        #[doc = "APB2 peripheral clock enable in sleep mode register"]
        pub fn apb2smenr(self) -> Reg<regs::Apb2smenr, RW> {
            unsafe { Reg::from_ptr(self.0.add(68usize)) }
        }
        #[doc = "APB1 peripheral clock enable in sleep mode register"]
        pub fn apb1smenr(self) -> Reg<regs::Apb1smenr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "Clock configuration register"]
        pub fn ccipr(self) -> Reg<regs::Ccipr, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
        #[doc = "Control and status register"]
        pub fn csr(self) -> Reg<regs::Csr, RW> {
            unsafe { Reg::from_ptr(self.0.add(80usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Clock control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "16 MHz high-speed internal clock enable"]
            pub const fn hsi16on(&self) -> super::vals::Pllon {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Pllon(val as u8)
            }
            #[doc = "16 MHz high-speed internal clock enable"]
            pub fn set_hsi16on(&mut self, val: super::vals::Pllon) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "High-speed internal clock enable bit for some IP kernels"]
            pub const fn hsi16keron(&self) -> super::vals::Pllon {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Pllon(val as u8)
            }
            #[doc = "High-speed internal clock enable bit for some IP kernels"]
            pub fn set_hsi16keron(&mut self, val: super::vals::Pllon) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Internal high-speed clock ready flag"]
            pub const fn hsi16rdyf(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Internal high-speed clock ready flag"]
            pub fn set_hsi16rdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "HSI16DIVEN"]
            pub const fn hsi16diven(&self) -> super::vals::Hsidiven {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hsidiven(val as u8)
            }
            #[doc = "HSI16DIVEN"]
            pub fn set_hsi16diven(&mut self, val: super::vals::Hsidiven) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "HSI16DIVF"]
            pub const fn hsi16divf(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "HSI16DIVF"]
            pub fn set_hsi16divf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "16 MHz high-speed internal clock output enable"]
            pub const fn hsi16outen(&self) -> super::vals::Hsiouten {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Hsiouten(val as u8)
            }
            #[doc = "16 MHz high-speed internal clock output enable"]
            pub fn set_hsi16outen(&mut self, val: super::vals::Hsiouten) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "MSI clock enable bit"]
            pub const fn msion(&self) -> super::vals::Pllon {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Pllon(val as u8)
            }
            #[doc = "MSI clock enable bit"]
            pub fn set_msion(&mut self, val: super::vals::Pllon) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "MSI clock ready flag"]
            pub const fn msirdy(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "MSI clock ready flag"]
            pub fn set_msirdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "HSE clock enable bit"]
            pub const fn hseon(&self) -> super::vals::Pllon {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Pllon(val as u8)
            }
            #[doc = "HSE clock enable bit"]
            pub fn set_hseon(&mut self, val: super::vals::Pllon) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
            #[doc = "HSE clock ready flag"]
            pub const fn hserdy(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "HSE clock ready flag"]
            pub fn set_hserdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "HSE clock bypass bit"]
            pub const fn hsebyp(&self) -> super::vals::Hsebyp {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Hsebyp(val as u8)
            }
            #[doc = "HSE clock bypass bit"]
            pub fn set_hsebyp(&mut self, val: super::vals::Hsebyp) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "Clock security system on HSE enable bit"]
            pub const fn csshseon(&self) -> super::vals::Pllon {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Pllon(val as u8)
            }
            #[doc = "Clock security system on HSE enable bit"]
            pub fn set_csshseon(&mut self, val: super::vals::Pllon) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
            }
            #[doc = "TC/LCD prescaler"]
            pub const fn rtcpre(&self) -> super::vals::Rtcpre {
                let val = (self.0 >> 20usize) & 0x03;
                super::vals::Rtcpre(val as u8)
            }
            #[doc = "TC/LCD prescaler"]
            pub fn set_rtcpre(&mut self, val: super::vals::Rtcpre) {
                self.0 = (self.0 & !(0x03 << 20usize)) | (((val.0 as u32) & 0x03) << 20usize);
            }
            #[doc = "PLL enable bit"]
            pub const fn pllon(&self) -> super::vals::Pllon {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Pllon(val as u8)
            }
            #[doc = "PLL enable bit"]
            pub fn set_pllon(&mut self, val: super::vals::Pllon) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
            }
            #[doc = "PLL clock ready flag"]
            pub const fn pllrdy(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "PLL clock ready flag"]
            pub fn set_pllrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "Internal clock sources calibration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icscr(pub u32);
        impl Icscr {
            #[doc = "nternal high speed clock calibration"]
            pub const fn hsi16cal(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "nternal high speed clock calibration"]
            pub fn set_hsi16cal(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "High speed internal clock trimming"]
            pub const fn hsi16trim(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "High speed internal clock trimming"]
            pub fn set_hsi16trim(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
            }
            #[doc = "MSI clock ranges"]
            pub const fn msirange(&self) -> super::vals::Msirange {
                let val = (self.0 >> 13usize) & 0x07;
                super::vals::Msirange(val as u8)
            }
            #[doc = "MSI clock ranges"]
            pub fn set_msirange(&mut self, val: super::vals::Msirange) {
                self.0 = (self.0 & !(0x07 << 13usize)) | (((val.0 as u32) & 0x07) << 13usize);
            }
            #[doc = "MSI clock calibration"]
            pub const fn msical(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "MSI clock calibration"]
            pub fn set_msical(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "MSI clock trimming"]
            pub const fn msitrim(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "MSI clock trimming"]
            pub fn set_msitrim(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Icscr {
            fn default() -> Icscr {
                Icscr(0)
            }
        }
        #[doc = "Control and status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Csr(pub u32);
        impl Csr {
            #[doc = "Internal low-speed oscillator enable"]
            pub const fn lsion(&self) -> super::vals::Csslseon {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Csslseon(val as u8)
            }
            #[doc = "Internal low-speed oscillator enable"]
            pub fn set_lsion(&mut self, val: super::vals::Csslseon) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Internal low-speed oscillator ready bit"]
            pub const fn lsirdy(&self) -> super::vals::Lserdy {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Lserdy(val as u8)
            }
            #[doc = "Internal low-speed oscillator ready bit"]
            pub fn set_lsirdy(&mut self, val: super::vals::Lserdy) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "External low-speed oscillator enable bit"]
            pub const fn lseon(&self) -> super::vals::Csslseon {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Csslseon(val as u8)
            }
            #[doc = "External low-speed oscillator enable bit"]
            pub fn set_lseon(&mut self, val: super::vals::Csslseon) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "External low-speed oscillator ready bit"]
            pub const fn lserdy(&self) -> super::vals::Lserdy {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Lserdy(val as u8)
            }
            #[doc = "External low-speed oscillator ready bit"]
            pub fn set_lserdy(&mut self, val: super::vals::Lserdy) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "External low-speed oscillator bypass bit"]
            pub const fn lsebyp(&self) -> super::vals::Lsebyp {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Lsebyp(val as u8)
            }
            #[doc = "External low-speed oscillator bypass bit"]
            pub fn set_lsebyp(&mut self, val: super::vals::Lsebyp) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "LSEDRV"]
            pub const fn lsedrv(&self) -> super::vals::Lsedrv {
                let val = (self.0 >> 11usize) & 0x03;
                super::vals::Lsedrv(val as u8)
            }
            #[doc = "LSEDRV"]
            pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
                self.0 = (self.0 & !(0x03 << 11usize)) | (((val.0 as u32) & 0x03) << 11usize);
            }
            #[doc = "CSSLSEON"]
            pub const fn csslseon(&self) -> super::vals::Csslseon {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Csslseon(val as u8)
            }
            #[doc = "CSSLSEON"]
            pub fn set_csslseon(&mut self, val: super::vals::Csslseon) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val.0 as u32) & 0x01) << 13usize);
            }
            #[doc = "CSS on LSE failure detection flag"]
            pub const fn csslsed(&self) -> super::vals::Csslsed {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Csslsed(val as u8)
            }
            #[doc = "CSS on LSE failure detection flag"]
            pub fn set_csslsed(&mut self, val: super::vals::Csslsed) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "RTC and LCD clock source selection bits"]
            pub const fn rtcsel(&self) -> super::vals::Rtcsel {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Rtcsel(val as u8)
            }
            #[doc = "RTC and LCD clock source selection bits"]
            pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
                self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
            }
            #[doc = "RTC clock enable bit"]
            pub const fn rtcen(&self) -> super::vals::Rtcen {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Rtcen(val as u8)
            }
            #[doc = "RTC clock enable bit"]
            pub fn set_rtcen(&mut self, val: super::vals::Rtcen) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "RTC software reset bit"]
            pub const fn rtcrst(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "RTC software reset bit"]
            pub fn set_rtcrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Remove reset flag"]
            pub const fn rmvf(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Remove reset flag"]
            pub fn set_rmvf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "OBLRSTF"]
            pub const fn oblrstf(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "OBLRSTF"]
            pub fn set_oblrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "PIN reset flag"]
            pub const fn pinrstf(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PIN reset flag"]
            pub fn set_pinrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "POR/PDR reset flag"]
            pub const fn porrstf(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "POR/PDR reset flag"]
            pub fn set_porrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Software reset flag"]
            pub const fn sftrstf(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Software reset flag"]
            pub fn set_sftrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Independent watchdog reset flag"]
            pub const fn iwdgrstf(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Independent watchdog reset flag"]
            pub fn set_iwdgrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Window watchdog reset flag"]
            pub const fn wwdgrstf(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Window watchdog reset flag"]
            pub fn set_wwdgrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Low-power reset flag"]
            pub const fn lpwrrstf(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Low-power reset flag"]
            pub fn set_lpwrrstf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Csr {
            fn default() -> Csr {
                Csr(0)
            }
        }
        #[doc = "Clock configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfgr(pub u32);
        impl Cfgr {
            #[doc = "System clock switch"]
            pub const fn sw(&self) -> super::vals::Sw {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Sw(val as u8)
            }
            #[doc = "System clock switch"]
            pub fn set_sw(&mut self, val: super::vals::Sw) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
            }
            #[doc = "System clock switch status"]
            pub const fn sws(&self) -> super::vals::Sws {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Sws(val as u8)
            }
            #[doc = "System clock switch status"]
            pub fn set_sws(&mut self, val: super::vals::Sws) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.0 as u32) & 0x03) << 2usize);
            }
            #[doc = "AHB prescaler"]
            pub const fn hpre(&self) -> super::vals::Hpre {
                let val = (self.0 >> 4usize) & 0x0f;
                super::vals::Hpre(val as u8)
            }
            #[doc = "AHB prescaler"]
            pub fn set_hpre(&mut self, val: super::vals::Hpre) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val.0 as u32) & 0x0f) << 4usize);
            }
            #[doc = "APB low-speed prescaler (APB1)"]
            pub fn ppre(&self, n: usize) -> super::vals::Ppre {
                assert!(n < 2usize);
                let offs = 8usize + n * 3usize;
                let val = (self.0 >> offs) & 0x07;
                super::vals::Ppre(val as u8)
            }
            #[doc = "APB low-speed prescaler (APB1)"]
            pub fn set_ppre(&mut self, n: usize, val: super::vals::Ppre) {
                assert!(n < 2usize);
                let offs = 8usize + n * 3usize;
                self.0 = (self.0 & !(0x07 << offs)) | (((val.0 as u32) & 0x07) << offs);
            }
            #[doc = "Wake-up from stop clock selection"]
            pub const fn stopwuck(&self) -> super::vals::Stopwuck {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Stopwuck(val as u8)
            }
            #[doc = "Wake-up from stop clock selection"]
            pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "PLL entry clock source"]
            pub const fn pllsrc(&self) -> super::vals::Pllsrc {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Pllsrc(val as u8)
            }
            #[doc = "PLL entry clock source"]
            pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
            #[doc = "PLL multiplication factor"]
            pub const fn pllmul(&self) -> super::vals::Pllmul {
                let val = (self.0 >> 18usize) & 0x0f;
                super::vals::Pllmul(val as u8)
            }
            #[doc = "PLL multiplication factor"]
            pub fn set_pllmul(&mut self, val: super::vals::Pllmul) {
                self.0 = (self.0 & !(0x0f << 18usize)) | (((val.0 as u32) & 0x0f) << 18usize);
            }
            #[doc = "PLL output division"]
            pub const fn plldiv(&self) -> super::vals::Plldiv {
                let val = (self.0 >> 22usize) & 0x03;
                super::vals::Plldiv(val as u8)
            }
            #[doc = "PLL output division"]
            pub fn set_plldiv(&mut self, val: super::vals::Plldiv) {
                self.0 = (self.0 & !(0x03 << 22usize)) | (((val.0 as u32) & 0x03) << 22usize);
            }
            #[doc = "Microcontroller clock output selection"]
            pub const fn mcosel(&self) -> super::vals::Mcosel {
                let val = (self.0 >> 24usize) & 0x0f;
                super::vals::Mcosel(val as u8)
            }
            #[doc = "Microcontroller clock output selection"]
            pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
                self.0 = (self.0 & !(0x0f << 24usize)) | (((val.0 as u32) & 0x0f) << 24usize);
            }
            #[doc = "Microcontroller clock output prescaler"]
            pub const fn mcopre(&self) -> super::vals::Mcopre {
                let val = (self.0 >> 28usize) & 0x07;
                super::vals::Mcopre(val as u8)
            }
            #[doc = "Microcontroller clock output prescaler"]
            pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
                self.0 = (self.0 & !(0x07 << 28usize)) | (((val.0 as u32) & 0x07) << 28usize);
            }
        }
        impl Default for Cfgr {
            fn default() -> Cfgr {
                Cfgr(0)
            }
        }
        #[doc = "Clock recovery RC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crrcr(pub u32);
        impl Crrcr {
            #[doc = "48MHz HSI clock enable bit"]
            pub const fn hsi48on(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "48MHz HSI clock enable bit"]
            pub fn set_hsi48on(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "48MHz HSI clock ready flag"]
            pub const fn hsi48rdy(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "48MHz HSI clock ready flag"]
            pub fn set_hsi48rdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "48 MHz HSI clock divided by 6 output enable"]
            pub const fn hsi48div6en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "48 MHz HSI clock divided by 6 output enable"]
            pub fn set_hsi48div6en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "48 MHz HSI clock calibration"]
            pub const fn hsi48cal(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "48 MHz HSI clock calibration"]
            pub fn set_hsi48cal(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Crrcr {
            fn default() -> Crrcr {
                Crrcr(0)
            }
        }
        #[doc = "GPIO clock enable in sleep mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Iopsmen(pub u32);
        impl Iopsmen {
            #[doc = "IOPASMEN"]
            pub const fn iopasmen(&self) -> super::vals::Iophsmen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Iophsmen(val as u8)
            }
            #[doc = "IOPASMEN"]
            pub fn set_iopasmen(&mut self, val: super::vals::Iophsmen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "IOPBSMEN"]
            pub const fn iopbsmen(&self) -> super::vals::Iophsmen {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Iophsmen(val as u8)
            }
            #[doc = "IOPBSMEN"]
            pub fn set_iopbsmen(&mut self, val: super::vals::Iophsmen) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "IOPCSMEN"]
            pub const fn iopcsmen(&self) -> super::vals::Iophsmen {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Iophsmen(val as u8)
            }
            #[doc = "IOPCSMEN"]
            pub fn set_iopcsmen(&mut self, val: super::vals::Iophsmen) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "IOPDSMEN"]
            pub const fn iopdsmen(&self) -> super::vals::Iophsmen {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Iophsmen(val as u8)
            }
            #[doc = "IOPDSMEN"]
            pub fn set_iopdsmen(&mut self, val: super::vals::Iophsmen) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Port E clock enable during Sleep mode bit"]
            pub const fn iopesmen(&self) -> super::vals::Iophsmen {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Iophsmen(val as u8)
            }
            #[doc = "Port E clock enable during Sleep mode bit"]
            pub fn set_iopesmen(&mut self, val: super::vals::Iophsmen) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "IOPHSMEN"]
            pub const fn iophsmen(&self) -> super::vals::Iophsmen {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Iophsmen(val as u8)
            }
            #[doc = "IOPHSMEN"]
            pub fn set_iophsmen(&mut self, val: super::vals::Iophsmen) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Iopsmen {
            fn default() -> Iopsmen {
                Iopsmen(0)
            }
        }
        #[doc = "APB1 peripheral clock enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Apb1enr(pub u32);
        impl Apb1enr {
            #[doc = "Timer2 clock enable bit"]
            pub const fn tim2en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Timer2 clock enable bit"]
            pub fn set_tim2en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Timer3 clock enable bit"]
            pub const fn tim3en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Timer3 clock enable bit"]
            pub fn set_tim3en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Timer 6 clock enable bit"]
            pub const fn tim6en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Timer 6 clock enable bit"]
            pub fn set_tim6en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Timer 7 clock enable bit"]
            pub const fn tim7en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Timer 7 clock enable bit"]
            pub fn set_tim7en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "Window watchdog clock enable bit"]
            pub const fn wwdgen(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Window watchdog clock enable bit"]
            pub fn set_wwdgen(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "SPI2 clock enable bit"]
            pub const fn spi2en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "SPI2 clock enable bit"]
            pub fn set_spi2en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "UART2 clock enable bit"]
            pub const fn usart2en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "UART2 clock enable bit"]
            pub fn set_usart2en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
            }
            #[doc = "LPUART1 clock enable bit"]
            pub const fn lpuart1en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "LPUART1 clock enable bit"]
            pub fn set_lpuart1en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "USART4 clock enable bit"]
            pub const fn usart4en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "USART4 clock enable bit"]
            pub fn set_usart4en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
            }
            #[doc = "USART5 clock enable bit"]
            pub const fn usart5en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "USART5 clock enable bit"]
            pub fn set_usart5en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val.0 as u32) & 0x01) << 20usize);
            }
            #[doc = "I2C1 clock enable bit"]
            pub const fn i2c1en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 21usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "I2C1 clock enable bit"]
            pub fn set_i2c1en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val.0 as u32) & 0x01) << 21usize);
            }
            #[doc = "I2C2 clock enable bit"]
            pub const fn i2c2en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "I2C2 clock enable bit"]
            pub fn set_i2c2en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val.0 as u32) & 0x01) << 22usize);
            }
            #[doc = "USB clock enable bit"]
            pub const fn usben(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 23usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "USB clock enable bit"]
            pub fn set_usben(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val.0 as u32) & 0x01) << 23usize);
            }
            #[doc = "Clock recovery system clock enable bit"]
            pub const fn crsen(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 27usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Clock recovery system clock enable bit"]
            pub fn set_crsen(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val.0 as u32) & 0x01) << 27usize);
            }
            #[doc = "Power interface clock enable bit"]
            pub const fn pwren(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 28usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Power interface clock enable bit"]
            pub fn set_pwren(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val.0 as u32) & 0x01) << 28usize);
            }
            #[doc = "DAC interface clock enable bit"]
            pub const fn dacen(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 29usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "DAC interface clock enable bit"]
            pub fn set_dacen(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val.0 as u32) & 0x01) << 29usize);
            }
            #[doc = "I2C3 clock enable bit"]
            pub const fn i2c3en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "I2C3 clock enable bit"]
            pub fn set_i2c3en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val.0 as u32) & 0x01) << 30usize);
            }
            #[doc = "Low power timer clock enable bit"]
            pub const fn lptim1en(&self) -> super::vals::Lptimen {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Lptimen(val as u8)
            }
            #[doc = "Low power timer clock enable bit"]
            pub fn set_lptim1en(&mut self, val: super::vals::Lptimen) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Apb1enr {
            fn default() -> Apb1enr {
                Apb1enr(0)
            }
        }
        #[doc = "AHB peripheral clock enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ahbenr(pub u32);
        impl Ahbenr {
            #[doc = "DMA clock enable bit"]
            pub const fn dmaen(&self) -> super::vals::Crypen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Crypen(val as u8)
            }
            #[doc = "DMA clock enable bit"]
            pub fn set_dmaen(&mut self, val: super::vals::Crypen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "NVM interface clock enable bit"]
            pub const fn mifen(&self) -> super::vals::Crypen {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Crypen(val as u8)
            }
            #[doc = "NVM interface clock enable bit"]
            pub fn set_mifen(&mut self, val: super::vals::Crypen) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "CRC clock enable bit"]
            pub const fn crcen(&self) -> super::vals::Crypen {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Crypen(val as u8)
            }
            #[doc = "CRC clock enable bit"]
            pub fn set_crcen(&mut self, val: super::vals::Crypen) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Touch Sensing clock enable bit"]
            pub const fn touchen(&self) -> super::vals::Crypen {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Crypen(val as u8)
            }
            #[doc = "Touch Sensing clock enable bit"]
            pub fn set_touchen(&mut self, val: super::vals::Crypen) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
            #[doc = "Random Number Generator clock enable bit"]
            pub const fn rngen(&self) -> super::vals::Crypen {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Crypen(val as u8)
            }
            #[doc = "Random Number Generator clock enable bit"]
            pub fn set_rngen(&mut self, val: super::vals::Crypen) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val.0 as u32) & 0x01) << 20usize);
            }
            #[doc = "Crypto clock enable bit"]
            pub const fn crypen(&self) -> super::vals::Crypen {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Crypen(val as u8)
            }
            #[doc = "Crypto clock enable bit"]
            pub fn set_crypen(&mut self, val: super::vals::Crypen) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Ahbenr {
            fn default() -> Ahbenr {
                Ahbenr(0)
            }
        }
        #[doc = "AHB peripheral clock enable in sleep mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ahbsmenr(pub u32);
        impl Ahbsmenr {
            #[doc = "DMA clock enable during sleep mode bit"]
            pub const fn dmasmen(&self) -> super::vals::Dmasmen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dmasmen(val as u8)
            }
            #[doc = "DMA clock enable during sleep mode bit"]
            pub fn set_dmasmen(&mut self, val: super::vals::Dmasmen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "NVM interface clock enable during sleep mode bit"]
            pub const fn mifsmen(&self) -> super::vals::Mifsmen {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Mifsmen(val as u8)
            }
            #[doc = "NVM interface clock enable during sleep mode bit"]
            pub fn set_mifsmen(&mut self, val: super::vals::Mifsmen) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "SRAM interface clock enable during sleep mode bit"]
            pub const fn sramsmen(&self) -> super::vals::Sramsmen {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Sramsmen(val as u8)
            }
            #[doc = "SRAM interface clock enable during sleep mode bit"]
            pub fn set_sramsmen(&mut self, val: super::vals::Sramsmen) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "CRC clock enable during sleep mode bit"]
            pub const fn crcsmen(&self) -> super::vals::Crcsmen {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Crcsmen(val as u8)
            }
            #[doc = "CRC clock enable during sleep mode bit"]
            pub fn set_crcsmen(&mut self, val: super::vals::Crcsmen) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Touch Sensing clock enable during sleep mode bit"]
            pub const fn touchsmen(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Touch Sensing clock enable during sleep mode bit"]
            pub fn set_touchsmen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Random Number Generator clock enable during sleep mode bit"]
            pub const fn rngsmen(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Random Number Generator clock enable during sleep mode bit"]
            pub fn set_rngsmen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Crypto clock enable during sleep mode bit"]
            pub const fn crypsmen(&self) -> super::vals::Crypsmen {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Crypsmen(val as u8)
            }
            #[doc = "Crypto clock enable during sleep mode bit"]
            pub fn set_crypsmen(&mut self, val: super::vals::Crypsmen) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Ahbsmenr {
            fn default() -> Ahbsmenr {
                Ahbsmenr(0)
            }
        }
        #[doc = "Clock interrupt flag register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cifr(pub u32);
        impl Cifr {
            #[doc = "LSI ready interrupt flag"]
            pub const fn lsirdyf(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "LSI ready interrupt flag"]
            pub fn set_lsirdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "LSE ready interrupt flag"]
            pub const fn lserdyf(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "LSE ready interrupt flag"]
            pub fn set_lserdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "HSI16 ready interrupt flag"]
            pub const fn hsi16rdyf(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "HSI16 ready interrupt flag"]
            pub fn set_hsi16rdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "HSE ready interrupt flag"]
            pub const fn hserdyf(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "HSE ready interrupt flag"]
            pub fn set_hserdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PLL ready interrupt flag"]
            pub const fn pllrdyf(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PLL ready interrupt flag"]
            pub fn set_pllrdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "MSI ready interrupt flag"]
            pub const fn msirdyf(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "MSI ready interrupt flag"]
            pub fn set_msirdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "HSI48 ready interrupt flag"]
            pub const fn hsi48rdyf(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "HSI48 ready interrupt flag"]
            pub fn set_hsi48rdyf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "LSE Clock Security System Interrupt flag"]
            pub const fn csslsef(&self) -> super::vals::Csslsef {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Csslsef(val as u8)
            }
            #[doc = "LSE Clock Security System Interrupt flag"]
            pub fn set_csslsef(&mut self, val: super::vals::Csslsef) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Clock Security System Interrupt flag"]
            pub const fn csshsef(&self) -> super::vals::Csshsef {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Csshsef(val as u8)
            }
            #[doc = "Clock Security System Interrupt flag"]
            pub fn set_csshsef(&mut self, val: super::vals::Csshsef) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Cifr {
            fn default() -> Cifr {
                Cifr(0)
            }
        }
        #[doc = "APB2 peripheral clock enable in sleep mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Apb2smenr(pub u32);
        impl Apb2smenr {
            #[doc = "System configuration controller clock enable during sleep mode bit"]
            pub const fn syscfgsmen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "System configuration controller clock enable during sleep mode bit"]
            pub fn set_syscfgsmen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "TIM21 timer clock enable during sleep mode bit"]
            pub const fn tim21smen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "TIM21 timer clock enable during sleep mode bit"]
            pub fn set_tim21smen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "TIM22 timer clock enable during sleep mode bit"]
            pub const fn tim22smen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "TIM22 timer clock enable during sleep mode bit"]
            pub fn set_tim22smen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "ADC clock enable during sleep mode bit"]
            pub const fn adcsmen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "ADC clock enable during sleep mode bit"]
            pub fn set_adcsmen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "SPI1 clock enable during sleep mode bit"]
            pub const fn spi1smen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "SPI1 clock enable during sleep mode bit"]
            pub fn set_spi1smen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "USART1 clock enable during sleep mode bit"]
            pub const fn usart1smen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "USART1 clock enable during sleep mode bit"]
            pub fn set_usart1smen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "DBG clock enable during sleep mode bit"]
            pub const fn dbgsmen(&self) -> super::vals::Dbgsmen {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Dbgsmen(val as u8)
            }
            #[doc = "DBG clock enable during sleep mode bit"]
            pub fn set_dbgsmen(&mut self, val: super::vals::Dbgsmen) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val.0 as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Apb2smenr {
            fn default() -> Apb2smenr {
                Apb2smenr(0)
            }
        }
        #[doc = "APB2 peripheral reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Apb2rstr(pub u32);
        impl Apb2rstr {
            #[doc = "System configuration controller reset"]
            pub const fn syscfgrst(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "System configuration controller reset"]
            pub fn set_syscfgrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TIM21 timer reset"]
            pub const fn tim21rst(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "TIM21 timer reset"]
            pub fn set_tim21rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TIM22 timer reset"]
            pub const fn tim22rst(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TIM22 timer reset"]
            pub fn set_tim22rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "ADC interface reset"]
            pub const fn adcrst(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "ADC interface reset"]
            pub fn set_adcrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "SPI 1 reset"]
            pub const fn spi1rst(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "SPI 1 reset"]
            pub fn set_spi1rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "USART1 reset"]
            pub const fn usart1rst(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "USART1 reset"]
            pub fn set_usart1rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "DBG reset"]
            pub const fn dbgrst(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "DBG reset"]
            pub fn set_dbgrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Apb2rstr {
            fn default() -> Apb2rstr {
                Apb2rstr(0)
            }
        }
        #[doc = "Clock interrupt clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cicr(pub u32);
        impl Cicr {
            #[doc = "LSI ready Interrupt clear"]
            pub const fn lsirdyc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "LSI ready Interrupt clear"]
            pub fn set_lsirdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "LSE ready Interrupt clear"]
            pub const fn lserdyc(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "LSE ready Interrupt clear"]
            pub fn set_lserdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "HSI16 ready Interrupt clear"]
            pub const fn hsi16rdyc(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "HSI16 ready Interrupt clear"]
            pub fn set_hsi16rdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "HSE ready Interrupt clear"]
            pub const fn hserdyc(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "HSE ready Interrupt clear"]
            pub fn set_hserdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PLL ready Interrupt clear"]
            pub const fn pllrdyc(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PLL ready Interrupt clear"]
            pub fn set_pllrdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "MSI ready Interrupt clear"]
            pub const fn msirdyc(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "MSI ready Interrupt clear"]
            pub fn set_msirdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "HSI48 ready Interrupt clear"]
            pub const fn hsi48rdyc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "HSI48 ready Interrupt clear"]
            pub fn set_hsi48rdyc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "LSE Clock Security System Interrupt clear"]
            pub const fn csslsec(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "LSE Clock Security System Interrupt clear"]
            pub fn set_csslsec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Clock Security System Interrupt clear"]
            pub const fn csshsec(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Clock Security System Interrupt clear"]
            pub fn set_csshsec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Cicr {
            fn default() -> Cicr {
                Cicr(0)
            }
        }
        #[doc = "APB2 peripheral clock enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Apb2enr(pub u32);
        impl Apb2enr {
            #[doc = "System configuration controller clock enable bit"]
            pub const fn syscfgen(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "System configuration controller clock enable bit"]
            pub fn set_syscfgen(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "TIM21 timer clock enable bit"]
            pub const fn tim21en(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "TIM21 timer clock enable bit"]
            pub fn set_tim21en(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "TIM22 timer clock enable bit"]
            pub const fn tim22en(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "TIM22 timer clock enable bit"]
            pub fn set_tim22en(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "MiFaRe Firewall clock enable bit"]
            pub const fn mifien(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "MiFaRe Firewall clock enable bit"]
            pub fn set_mifien(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "ADC clock enable bit"]
            pub const fn adcen(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "ADC clock enable bit"]
            pub fn set_adcen(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "SPI1 clock enable bit"]
            pub const fn spi1en(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "SPI1 clock enable bit"]
            pub fn set_spi1en(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "USART1 clock enable bit"]
            pub const fn usart1en(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "USART1 clock enable bit"]
            pub fn set_usart1en(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "DBG clock enable bit"]
            pub const fn dbgen(&self) -> super::vals::Dbgen {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Dbgen(val as u8)
            }
            #[doc = "DBG clock enable bit"]
            pub fn set_dbgen(&mut self, val: super::vals::Dbgen) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val.0 as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Apb2enr {
            fn default() -> Apb2enr {
                Apb2enr(0)
            }
        }
        #[doc = "APB1 peripheral clock enable in sleep mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Apb1smenr(pub u32);
        impl Apb1smenr {
            #[doc = "Timer2 clock enable during sleep mode bit"]
            pub const fn tim2smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Timer2 clock enable during sleep mode bit"]
            pub fn set_tim2smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Timer3 clock enable during Sleep mode bit"]
            pub const fn tim3smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Timer3 clock enable during Sleep mode bit"]
            pub fn set_tim3smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Timer 6 clock enable during sleep mode bit"]
            pub const fn tim6smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Timer 6 clock enable during sleep mode bit"]
            pub fn set_tim6smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Timer 7 clock enable during Sleep mode bit"]
            pub const fn tim7smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Timer 7 clock enable during Sleep mode bit"]
            pub fn set_tim7smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "Window watchdog clock enable during sleep mode bit"]
            pub const fn wwdgsmen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Window watchdog clock enable during sleep mode bit"]
            pub fn set_wwdgsmen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "SPI2 clock enable during sleep mode bit"]
            pub const fn spi2smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "SPI2 clock enable during sleep mode bit"]
            pub fn set_spi2smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "UART2 clock enable during sleep mode bit"]
            pub const fn usart2smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "UART2 clock enable during sleep mode bit"]
            pub fn set_usart2smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
            }
            #[doc = "LPUART1 clock enable during sleep mode bit"]
            pub const fn lpuart1smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "LPUART1 clock enable during sleep mode bit"]
            pub fn set_lpuart1smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "USART4 clock enable during Sleep mode bit"]
            pub const fn usart4smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "USART4 clock enable during Sleep mode bit"]
            pub fn set_usart4smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
            }
            #[doc = "USART5 clock enable during Sleep mode bit"]
            pub const fn usart5smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "USART5 clock enable during Sleep mode bit"]
            pub fn set_usart5smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val.0 as u32) & 0x01) << 20usize);
            }
            #[doc = "I2C1 clock enable during sleep mode bit"]
            pub const fn i2c1smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 21usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "I2C1 clock enable during sleep mode bit"]
            pub fn set_i2c1smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val.0 as u32) & 0x01) << 21usize);
            }
            #[doc = "I2C2 clock enable during sleep mode bit"]
            pub const fn i2c2smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "I2C2 clock enable during sleep mode bit"]
            pub fn set_i2c2smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val.0 as u32) & 0x01) << 22usize);
            }
            #[doc = "USB clock enable during sleep mode bit"]
            pub const fn usbsmen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 23usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "USB clock enable during sleep mode bit"]
            pub fn set_usbsmen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val.0 as u32) & 0x01) << 23usize);
            }
            #[doc = "Clock recovery system clock enable during sleep mode bit"]
            pub const fn crssmen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 27usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Clock recovery system clock enable during sleep mode bit"]
            pub fn set_crssmen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val.0 as u32) & 0x01) << 27usize);
            }
            #[doc = "Power interface clock enable during sleep mode bit"]
            pub const fn pwrsmen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 28usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Power interface clock enable during sleep mode bit"]
            pub fn set_pwrsmen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val.0 as u32) & 0x01) << 28usize);
            }
            #[doc = "DAC interface clock enable during sleep mode bit"]
            pub const fn dacsmen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 29usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "DAC interface clock enable during sleep mode bit"]
            pub fn set_dacsmen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val.0 as u32) & 0x01) << 29usize);
            }
            #[doc = "2C3 clock enable during Sleep mode bit"]
            pub const fn i2c3smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "2C3 clock enable during Sleep mode bit"]
            pub fn set_i2c3smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val.0 as u32) & 0x01) << 30usize);
            }
            #[doc = "Low power timer clock enable during sleep mode bit"]
            pub const fn lptim1smen(&self) -> super::vals::Lptimsmen {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Lptimsmen(val as u8)
            }
            #[doc = "Low power timer clock enable during sleep mode bit"]
            pub fn set_lptim1smen(&mut self, val: super::vals::Lptimsmen) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Apb1smenr {
            fn default() -> Apb1smenr {
                Apb1smenr(0)
            }
        }
        #[doc = "GPIO clock enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Iopenr(pub u32);
        impl Iopenr {
            #[doc = "IO port A clock enable bit"]
            pub const fn iopaen(&self) -> super::vals::Iophen {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Iophen(val as u8)
            }
            #[doc = "IO port A clock enable bit"]
            pub fn set_iopaen(&mut self, val: super::vals::Iophen) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "IO port B clock enable bit"]
            pub const fn iopben(&self) -> super::vals::Iophen {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Iophen(val as u8)
            }
            #[doc = "IO port B clock enable bit"]
            pub fn set_iopben(&mut self, val: super::vals::Iophen) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "IO port A clock enable bit"]
            pub const fn iopcen(&self) -> super::vals::Iophen {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Iophen(val as u8)
            }
            #[doc = "IO port A clock enable bit"]
            pub fn set_iopcen(&mut self, val: super::vals::Iophen) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "I/O port D clock enable bit"]
            pub const fn iopden(&self) -> super::vals::Iophen {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Iophen(val as u8)
            }
            #[doc = "I/O port D clock enable bit"]
            pub fn set_iopden(&mut self, val: super::vals::Iophen) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "I/O port E clock enable bit"]
            pub const fn iopeen(&self) -> super::vals::Iophen {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Iophen(val as u8)
            }
            #[doc = "I/O port E clock enable bit"]
            pub fn set_iopeen(&mut self, val: super::vals::Iophen) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "I/O port H clock enable bit"]
            pub const fn iophen(&self) -> super::vals::Iophen {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Iophen(val as u8)
            }
            #[doc = "I/O port H clock enable bit"]
            pub fn set_iophen(&mut self, val: super::vals::Iophen) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Iopenr {
            fn default() -> Iopenr {
                Iopenr(0)
            }
        }
        #[doc = "APB1 peripheral reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Apb1rstr(pub u32);
        impl Apb1rstr {
            #[doc = "Timer2 reset"]
            pub const fn tim2rst(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Timer2 reset"]
            pub fn set_tim2rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Timer3 reset"]
            pub const fn tim3rst(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Timer3 reset"]
            pub fn set_tim3rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Timer 6 reset"]
            pub const fn tim6rst(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Timer 6 reset"]
            pub fn set_tim6rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Timer 7 reset"]
            pub const fn tim7rst(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Timer 7 reset"]
            pub fn set_tim7rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Window watchdog reset"]
            pub const fn wwdrst(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Window watchdog reset"]
            pub fn set_wwdrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "SPI2 reset"]
            pub const fn spi2rst(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "SPI2 reset"]
            pub fn set_spi2rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "UART2 reset"]
            pub const fn lpuart12rst(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "UART2 reset"]
            pub fn set_lpuart12rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "LPUART1 reset"]
            pub const fn lpuart1rst(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "LPUART1 reset"]
            pub fn set_lpuart1rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "USART4 reset"]
            pub const fn usart4rst(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "USART4 reset"]
            pub fn set_usart4rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "USART5 reset"]
            pub const fn usart5rst(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "USART5 reset"]
            pub fn set_usart5rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "I2C1 reset"]
            pub const fn i2c1rst(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "I2C1 reset"]
            pub fn set_i2c1rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "I2C2 reset"]
            pub const fn i2c2rst(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "I2C2 reset"]
            pub fn set_i2c2rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "USB reset"]
            pub const fn usbrst(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "USB reset"]
            pub fn set_usbrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Clock recovery system reset"]
            pub const fn crsrst(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Clock recovery system reset"]
            pub fn set_crsrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Power interface reset"]
            pub const fn pwrrst(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Power interface reset"]
            pub fn set_pwrrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "DAC interface reset"]
            pub const fn dacrst(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "DAC interface reset"]
            pub fn set_dacrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "I2C3 reset"]
            pub const fn i2c3rst(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "I2C3 reset"]
            pub fn set_i2c3rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Low power timer reset"]
            pub const fn lptim1rst(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Low power timer reset"]
            pub fn set_lptim1rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Apb1rstr {
            fn default() -> Apb1rstr {
                Apb1rstr(0)
            }
        }
        #[doc = "GPIO reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ioprstr(pub u32);
        impl Ioprstr {
            #[doc = "I/O port A reset"]
            pub const fn ioparst(&self) -> super::vals::Iophrst {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Iophrst(val as u8)
            }
            #[doc = "I/O port A reset"]
            pub fn set_ioparst(&mut self, val: super::vals::Iophrst) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "I/O port B reset"]
            pub const fn iopbrst(&self) -> super::vals::Iophrst {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Iophrst(val as u8)
            }
            #[doc = "I/O port B reset"]
            pub fn set_iopbrst(&mut self, val: super::vals::Iophrst) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "I/O port A reset"]
            pub const fn iopcrst(&self) -> super::vals::Iophrst {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Iophrst(val as u8)
            }
            #[doc = "I/O port A reset"]
            pub fn set_iopcrst(&mut self, val: super::vals::Iophrst) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "I/O port D reset"]
            pub const fn iopdrst(&self) -> super::vals::Iophrst {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Iophrst(val as u8)
            }
            #[doc = "I/O port D reset"]
            pub fn set_iopdrst(&mut self, val: super::vals::Iophrst) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "I/O port E reset"]
            pub const fn ioperst(&self) -> super::vals::Iophrst {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Iophrst(val as u8)
            }
            #[doc = "I/O port E reset"]
            pub fn set_ioperst(&mut self, val: super::vals::Iophrst) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "I/O port H reset"]
            pub const fn iophrst(&self) -> super::vals::Iophrst {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Iophrst(val as u8)
            }
            #[doc = "I/O port H reset"]
            pub fn set_iophrst(&mut self, val: super::vals::Iophrst) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Ioprstr {
            fn default() -> Ioprstr {
                Ioprstr(0)
            }
        }
        #[doc = "Clock interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cier(pub u32);
        impl Cier {
            #[doc = "LSI ready interrupt flag"]
            pub const fn lsirdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "LSI ready interrupt flag"]
            pub fn set_lsirdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "LSE ready interrupt flag"]
            pub const fn lserdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "LSE ready interrupt flag"]
            pub fn set_lserdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "HSI16 ready interrupt flag"]
            pub const fn hsi16rdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "HSI16 ready interrupt flag"]
            pub fn set_hsi16rdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "HSE ready interrupt flag"]
            pub const fn hserdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "HSE ready interrupt flag"]
            pub fn set_hserdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "PLL ready interrupt flag"]
            pub const fn pllrdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "PLL ready interrupt flag"]
            pub fn set_pllrdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "MSI ready interrupt flag"]
            pub const fn msirdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "MSI ready interrupt flag"]
            pub fn set_msirdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "HSI48 ready interrupt flag"]
            pub const fn hsi48rdyie(&self) -> super::vals::Hsirdyie {
                let val = (self.0 >> 6usize) & 0x01;
                super::vals::Hsirdyie(val as u8)
            }
            #[doc = "HSI48 ready interrupt flag"]
            pub fn set_hsi48rdyie(&mut self, val: super::vals::Hsirdyie) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
            }
            #[doc = "LSE CSS interrupt flag"]
            pub const fn csslse(&self) -> super::vals::Csslse {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Csslse(val as u8)
            }
            #[doc = "LSE CSS interrupt flag"]
            pub fn set_csslse(&mut self, val: super::vals::Csslse) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cier {
            fn default() -> Cier {
                Cier(0)
            }
        }
        #[doc = "Clock configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ccipr(pub u32);
        impl Ccipr {
            #[doc = "USART1 clock source selection bits"]
            pub const fn usart1sel(&self) -> super::vals::Lpuartsel {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lpuartsel(val as u8)
            }
            #[doc = "USART1 clock source selection bits"]
            pub fn set_usart1sel(&mut self, val: super::vals::Lpuartsel) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
            }
            #[doc = "USART2 clock source selection bits"]
            pub const fn usart2sel(&self) -> super::vals::Lpuartsel {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Lpuartsel(val as u8)
            }
            #[doc = "USART2 clock source selection bits"]
            pub fn set_usart2sel(&mut self, val: super::vals::Lpuartsel) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.0 as u32) & 0x03) << 2usize);
            }
            #[doc = "LPUART1 clock source selection bits"]
            pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
                let val = (self.0 >> 10usize) & 0x03;
                super::vals::Lpuartsel(val as u8)
            }
            #[doc = "LPUART1 clock source selection bits"]
            pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
                self.0 = (self.0 & !(0x03 << 10usize)) | (((val.0 as u32) & 0x03) << 10usize);
            }
            #[doc = "I2C1 clock source selection bits"]
            pub const fn i2c1sel(&self) -> super::vals::Icsel {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Icsel(val as u8)
            }
            #[doc = "I2C1 clock source selection bits"]
            pub fn set_i2c1sel(&mut self, val: super::vals::Icsel) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "I2C3 clock source selection bits"]
            pub const fn i2c3sel(&self) -> super::vals::Icsel {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Icsel(val as u8)
            }
            #[doc = "I2C3 clock source selection bits"]
            pub fn set_i2c3sel(&mut self, val: super::vals::Icsel) {
                self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
            }
            #[doc = "Low Power Timer clock source selection bits"]
            pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
                let val = (self.0 >> 18usize) & 0x03;
                super::vals::Lptimsel(val as u8)
            }
            #[doc = "Low Power Timer clock source selection bits"]
            pub fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
                self.0 = (self.0 & !(0x03 << 18usize)) | (((val.0 as u32) & 0x03) << 18usize);
            }
            #[doc = "48 MHz HSI48 clock source selection bit"]
            pub const fn hsi48msel(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "48 MHz HSI48 clock source selection bit"]
            pub fn set_hsi48msel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
        }
        impl Default for Ccipr {
            fn default() -> Ccipr {
                Ccipr(0)
            }
        }
        #[doc = "AHB peripheral reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ahbrstr(pub u32);
        impl Ahbrstr {
            #[doc = "DMA reset"]
            pub const fn dmarst(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DMA reset"]
            pub fn set_dmarst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Memory interface reset"]
            pub const fn mifrst(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Memory interface reset"]
            pub fn set_mifrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Test integration module reset"]
            pub const fn crcrst(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Test integration module reset"]
            pub fn set_crcrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Touch Sensing reset"]
            pub const fn touchrst(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Touch Sensing reset"]
            pub fn set_touchrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Random Number Generator module reset"]
            pub const fn rngrst(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Random Number Generator module reset"]
            pub fn set_rngrst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Crypto module reset"]
            pub const fn cryprst(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Crypto module reset"]
            pub fn set_cryprst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Ahbrstr {
            fn default() -> Ahbrstr {
                Ahbrstr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Msirange(pub u8);
        impl Msirange {
            #[doc = "range 0 around 65.536 kHz"]
            pub const RANGE0: Self = Self(0);
            #[doc = "range 1 around 131.072 kHz"]
            pub const RANGE1: Self = Self(0x01);
            #[doc = "range 2 around 262.144 kHz"]
            pub const RANGE2: Self = Self(0x02);
            #[doc = "range 3 around 524.288 kHz"]
            pub const RANGE3: Self = Self(0x03);
            #[doc = "range 4 around 1.048 MHz"]
            pub const RANGE4: Self = Self(0x04);
            #[doc = "range 5 around 2.097 MHz (reset value)"]
            pub const RANGE5: Self = Self(0x05);
            #[doc = "range 6 around 4.194 MHz"]
            pub const RANGE6: Self = Self(0x06);
            #[doc = "not allowed"]
            pub const RANGE7: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hserdyr(pub u8);
        impl Hserdyr {
            #[doc = "Oscillator is not stable"]
            pub const NOTREADY: Self = Self(0);
            #[doc = "Oscillator is stable"]
            pub const READY: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crcsmen(pub u8);
        impl Crcsmen {
            #[doc = "Test integration module clock disabled in Sleep mode"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Test integration module clock enabled in Sleep mode (if enabled by CRCEN)"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cryprstw(pub u8);
        impl Cryprstw {
            #[doc = "Reset the module"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dbgrstw(pub u8);
        impl Dbgrstw {
            #[doc = "Reset the module"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsirdyie(pub u8);
        impl Hsirdyie {
            #[doc = "Ready interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Ready interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsidivfr(pub u8);
        impl Hsidivfr {
            #[doc = "16 MHz HSI clock not divided"]
            pub const NOTDIVIDED: Self = Self(0);
            #[doc = "16 MHz HSI clock divided by 4"]
            pub const DIV4: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Stopwuck(pub u8);
        impl Stopwuck {
            #[doc = "Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock"]
            pub const MSI: Self = Self(0);
            #[doc = "Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)"]
            pub const HSI16: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsi48rdyfr(pub u8);
        impl Hsi48rdyfr {
            #[doc = "No clock ready interrupt"]
            pub const NOTINTERRUPTED: Self = Self(0);
            #[doc = "Clock ready interrupt"]
            pub const INTERRUPTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rtcpre(pub u8);
        impl Rtcpre {
            #[doc = "HSE divided by 2"]
            pub const DIV2: Self = Self(0);
            #[doc = "HSE divided by 4"]
            pub const DIV4: Self = Self(0x01);
            #[doc = "HSE divided by 8"]
            pub const DIV8: Self = Self(0x02);
            #[doc = "HSE divided by 16"]
            pub const DIV16: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Csshsecw(pub u8);
        impl Csshsecw {
            #[doc = "Clear interrupt flag"]
            pub const CLEAR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Csslsef(pub u8);
        impl Csslsef {
            #[doc = "No failure detected on LSE clock failure"]
            pub const NOFAILURE: Self = Self(0);
            #[doc = "Failure detected on LSE clock failure"]
            pub const FAILURE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Csslsed(pub u8);
        impl Csslsed {
            #[doc = "No failure detected on LSE (32 kHz oscillator)"]
            pub const NOFAILURE: Self = Self(0);
            #[doc = "Failure detected on LSE (32 kHz oscillator)"]
            pub const FAILURE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lptimen(pub u8);
        impl Lptimen {
            #[doc = "Clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rtcsel(pub u8);
        impl Rtcsel {
            #[doc = "No clock"]
            pub const NOCLOCK: Self = Self(0);
            #[doc = "LSE oscillator clock used as RTC clock"]
            pub const LSE: Self = Self(0x01);
            #[doc = "LSI oscillator clock used as RTC clock"]
            pub const LSI: Self = Self(0x02);
            #[doc = "HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE[1:0]
bits in the RCC clock control register (RCC_CR)) used as the RTC clock"]
            pub const HSE: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lptimsel(pub u8);
        impl Lptimsel {
            #[doc = "APB clock selected as Timer clock"]
            pub const APB: Self = Self(0);
            #[doc = "LSI clock selected as Timer clock"]
            pub const LSI: Self = Self(0x01);
            #[doc = "HSI16 clock selected as Timer clock"]
            pub const HSI16: Self = Self(0x02);
            #[doc = "LSE clock selected as Timer clock"]
            pub const LSE: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Csslseon(pub u8);
        impl Csslseon {
            #[doc = "Oscillator OFF"]
            pub const OFF: Self = Self(0);
            #[doc = "Oscillator ON"]
            pub const ON: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dmasmen(pub u8);
        impl Dmasmen {
            #[doc = "DMA clock disabled in Sleep mode"]
            pub const DISABLED: Self = Self(0);
            #[doc = "DMA clock enabled in Sleep mode"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lpwrrstfr(pub u8);
        impl Lpwrrstfr {
            #[doc = "No reset has occured"]
            pub const NORESET: Self = Self(0);
            #[doc = "A reset has occured"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mcosel(pub u8);
        impl Mcosel {
            #[doc = "No clock"]
            pub const NOCLOCK: Self = Self(0);
            #[doc = "SYSCLK clock selected"]
            pub const SYSCLK: Self = Self(0x01);
            #[doc = "HSI oscillator clock selected"]
            pub const HSI16: Self = Self(0x02);
            #[doc = "MSI oscillator clock selected"]
            pub const MSI: Self = Self(0x03);
            #[doc = "HSE oscillator clock selected"]
            pub const HSE: Self = Self(0x04);
            #[doc = "PLL clock selected"]
            pub const PLL: Self = Self(0x05);
            #[doc = "LSI oscillator clock selected"]
            pub const LSI: Self = Self(0x06);
            #[doc = "LSE oscillator clock selected"]
            pub const LSE: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rmvfw(pub u8);
        impl Rmvfw {
            #[doc = "Clears the reset flag"]
            pub const CLEAR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Iophrst(pub u8);
        impl Iophrst {
            #[doc = "Reset I/O port"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lsebyp(pub u8);
        impl Lsebyp {
            #[doc = "LSE oscillator not bypassed"]
            pub const NOTBYPASSED: Self = Self(0);
            #[doc = "LSE oscillator bypassed"]
            pub const BYPASSED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sramsmen(pub u8);
        impl Sramsmen {
            #[doc = "NVM interface clock disabled in Sleep mode"]
            pub const DISABLED: Self = Self(0);
            #[doc = "NVM interface clock enabled in Sleep mode"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Icsel(pub u8);
        impl Icsel {
            #[doc = "APB clock selected as peripheral clock"]
            pub const APB: Self = Self(0);
            #[doc = "System clock selected as peripheral clock"]
            pub const SYSTEM: Self = Self(0x01);
            #[doc = "HSI16 clock selected as peripheral clock"]
            pub const HSI16: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crypen(pub u8);
        impl Crypen {
            #[doc = "Clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pllrdyr(pub u8);
        impl Pllrdyr {
            #[doc = "PLL unlocked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "PLL locked"]
            pub const LOCKED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lptimrstw(pub u8);
        impl Lptimrstw {
            #[doc = "Reset the module"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lsedrv(pub u8);
        impl Lsedrv {
            #[doc = "Lowest drive"]
            pub const LOW: Self = Self(0);
            #[doc = "Medium low drive"]
            pub const MEDIUMLOW: Self = Self(0x01);
            #[doc = "Medium high drive"]
            pub const MEDIUMHIGH: Self = Self(0x02);
            #[doc = "Highest drive"]
            pub const HIGH: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lpuartsel(pub u8);
        impl Lpuartsel {
            #[doc = "APB clock selected as peripheral clock"]
            pub const APB: Self = Self(0);
            #[doc = "System clock selected as peripheral clock"]
            pub const SYSTEM: Self = Self(0x01);
            #[doc = "HSI16 clock selected as peripheral clock"]
            pub const HSI16: Self = Self(0x02);
            #[doc = "LSE clock selected as peripheral clock"]
            pub const LSE: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mifsmen(pub u8);
        impl Mifsmen {
            #[doc = "NVM interface clock disabled in Sleep mode"]
            pub const DISABLED: Self = Self(0);
            #[doc = "NVM interface clock enabled in Sleep mode"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dbgsmen(pub u8);
        impl Dbgsmen {
            #[doc = "Clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hpre(pub u8);
        impl Hpre {
            #[doc = "system clock not divided"]
            pub const DIV1: Self = Self(0);
            #[doc = "system clock divided by 2"]
            pub const DIV2: Self = Self(0x08);
            #[doc = "system clock divided by 4"]
            pub const DIV4: Self = Self(0x09);
            #[doc = "system clock divided by 8"]
            pub const DIV8: Self = Self(0x0a);
            #[doc = "system clock divided by 16"]
            pub const DIV16: Self = Self(0x0b);
            #[doc = "system clock divided by 64"]
            pub const DIV64: Self = Self(0x0c);
            #[doc = "system clock divided by 128"]
            pub const DIV128: Self = Self(0x0d);
            #[doc = "system clock divided by 256"]
            pub const DIV256: Self = Self(0x0e);
            #[doc = "system clock divided by 512"]
            pub const DIV512: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Csshsef(pub u8);
        impl Csshsef {
            #[doc = "No clock security interrupt caused by HSE clock failure"]
            pub const NOCLOCK: Self = Self(0);
            #[doc = "Clock security interrupt caused by HSE clock failure"]
            pub const CLOCK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sws(pub u8);
        impl Sws {
            #[doc = "MSI oscillator used as system clock"]
            pub const MSI: Self = Self(0);
            #[doc = "HSI oscillator used as system clock"]
            pub const HSI16: Self = Self(0x01);
            #[doc = "HSE oscillator used as system clock"]
            pub const HSE: Self = Self(0x02);
            #[doc = "PLL used as system clock"]
            pub const PLL: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pllon(pub u8);
        impl Pllon {
            #[doc = "Clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lptimsmen(pub u8);
        impl Lptimsmen {
            #[doc = "Clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dbgen(pub u8);
        impl Dbgen {
            #[doc = "Clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsiouten(pub u8);
        impl Hsiouten {
            #[doc = "HSI output clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "HSI output clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mcopre(pub u8);
        impl Mcopre {
            #[doc = "No division"]
            pub const DIV1: Self = Self(0);
            #[doc = "Division by 2"]
            pub const DIV2: Self = Self(0x01);
            #[doc = "Division by 4"]
            pub const DIV4: Self = Self(0x02);
            #[doc = "Division by 8"]
            pub const DIV8: Self = Self(0x03);
            #[doc = "Division by 16"]
            pub const DIV16: Self = Self(0x04);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Csslse(pub u8);
        impl Csslse {
            #[doc = "LSE CSS interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "LSE CSS interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsi16rdyfr(pub u8);
        impl Hsi16rdyfr {
            #[doc = "HSI 16 MHz oscillator not ready"]
            pub const NOTREADY: Self = Self(0);
            #[doc = "HSI 16 MHz oscillator ready"]
            pub const READY: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rtcrstw(pub u8);
        impl Rtcrstw {
            #[doc = "Resets the RTC peripheral"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pllmul(pub u8);
        impl Pllmul {
            #[doc = "PLL clock entry x 3"]
            pub const MUL3: Self = Self(0);
            #[doc = "PLL clock entry x 4"]
            pub const MUL4: Self = Self(0x01);
            #[doc = "PLL clock entry x 6"]
            pub const MUL6: Self = Self(0x02);
            #[doc = "PLL clock entry x 8"]
            pub const MUL8: Self = Self(0x03);
            #[doc = "PLL clock entry x 12"]
            pub const MUL12: Self = Self(0x04);
            #[doc = "PLL clock entry x 16"]
            pub const MUL16: Self = Self(0x05);
            #[doc = "PLL clock entry x 24"]
            pub const MUL24: Self = Self(0x06);
            #[doc = "PLL clock entry x 32"]
            pub const MUL32: Self = Self(0x07);
            #[doc = "PLL clock entry x 48"]
            pub const MUL48: Self = Self(0x08);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsidiven(pub u8);
        impl Hsidiven {
            #[doc = "no 16 MHz HSI division requested"]
            pub const NOTDIVIDED: Self = Self(0);
            #[doc = "16 MHz HSI division by 4 requested"]
            pub const DIV4: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Iophsmen(pub u8);
        impl Iophsmen {
            #[doc = "Port x clock is disabled in Sleep mode"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pllsrc(pub u8);
        impl Pllsrc {
            #[doc = "HSI selected as PLL input clock"]
            pub const HSI16: Self = Self(0);
            #[doc = "HSE selected as PLL input clock"]
            pub const HSE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hsebyp(pub u8);
        impl Hsebyp {
            #[doc = "HSE oscillator not bypassed"]
            pub const NOTBYPASSED: Self = Self(0);
            #[doc = "HSE oscillator bypassed"]
            pub const BYPASSED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Plldiv(pub u8);
        impl Plldiv {
            #[doc = "PLLVCO / 2"]
            pub const DIV2: Self = Self(0x01);
            #[doc = "PLLVCO / 3"]
            pub const DIV3: Self = Self(0x02);
            #[doc = "PLLVCO / 4"]
            pub const DIV4: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sw(pub u8);
        impl Sw {
            #[doc = "MSI oscillator used as system clock"]
            pub const MSI: Self = Self(0);
            #[doc = "HSI oscillator used as system clock"]
            pub const HSI16: Self = Self(0x01);
            #[doc = "HSE oscillator used as system clock"]
            pub const HSE: Self = Self(0x02);
            #[doc = "PLL used as system clock"]
            pub const PLL: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crypsmen(pub u8);
        impl Crypsmen {
            #[doc = "Crypto clock disabled in Sleep mode"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Crypto clock enabled in Sleep mode"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rtcen(pub u8);
        impl Rtcen {
            #[doc = "RTC clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "RTC clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lserdy(pub u8);
        impl Lserdy {
            #[doc = "Oscillator not ready"]
            pub const NOTREADY: Self = Self(0);
            #[doc = "Oscillator ready"]
            pub const READY: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Iophen(pub u8);
        impl Iophen {
            #[doc = "Port clock disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Port clock enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ppre(pub u8);
        impl Ppre {
            #[doc = "HCLK not divided"]
            pub const DIV1: Self = Self(0);
            #[doc = "HCLK divided by 2"]
            pub const DIV2: Self = Self(0x04);
            #[doc = "HCLK divided by 4"]
            pub const DIV4: Self = Self(0x05);
            #[doc = "HCLK divided by 8"]
            pub const DIV8: Self = Self(0x06);
            #[doc = "HCLK divided by 16"]
            pub const DIV16: Self = Self(0x07);
        }
    }
}
pub mod dma_v2 {
    use crate::generic::*;
    #[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    #[derive(Copy, Clone)]
    pub struct St(pub *mut u8);
    unsafe impl Send for St {}
    unsafe impl Sync for St {}
    impl St {
        #[doc = "stream x configuration register"]
        pub fn cr(self) -> Reg<regs::Cr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "stream x number of data register"]
        pub fn ndtr(self) -> Reg<regs::Ndtr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "stream x peripheral address register"]
        pub fn par(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "stream x memory 0 address register"]
        pub fn m0ar(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "stream x memory 1 address register"]
        pub fn m1ar(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "stream x FIFO control register"]
        pub fn fcr(self) -> Reg<regs::Fcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    #[doc = "DMA controller"]
    #[derive(Copy, Clone)]
    pub struct Dma(pub *mut u8);
    unsafe impl Send for Dma {}
    unsafe impl Sync for Dma {}
    impl Dma {
        #[doc = "low interrupt status register"]
        pub fn isr(self, n: usize) -> Reg<regs::Ixr, R> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
        }
        #[doc = "low interrupt flag clear register"]
        pub fn ifcr(self, n: usize) -> Reg<regs::Ixr, W> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
        pub fn st(self, n: usize) -> St {
            assert!(n < 8usize);
            unsafe { St(self.0.add(16usize + n * 24usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dmdis(pub u8);
        impl Dmdis {
            #[doc = "Direct mode is enabled"]
            pub const ENABLED: Self = Self(0);
            #[doc = "Direct mode is disabled"]
            pub const DISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dbm(pub u8);
        impl Dbm {
            #[doc = "No buffer switching at the end of transfer"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Memory target switched at the end of the DMA transfer"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ct(pub u8);
        impl Ct {
            #[doc = "The current target memory is Memory 0"]
            pub const MEMORY0: Self = Self(0);
            #[doc = "The current target memory is Memory 1"]
            pub const MEMORY1: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dir(pub u8);
        impl Dir {
            #[doc = "Peripheral-to-memory"]
            pub const PERIPHERALTOMEMORY: Self = Self(0);
            #[doc = "Memory-to-peripheral"]
            pub const MEMORYTOPERIPHERAL: Self = Self(0x01);
            #[doc = "Memory-to-memory"]
            pub const MEMORYTOMEMORY: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Inc(pub u8);
        impl Inc {
            #[doc = "Address pointer is fixed"]
            pub const FIXED: Self = Self(0);
            #[doc = "Address pointer is incremented after each data transfer"]
            pub const INCREMENTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pincos(pub u8);
        impl Pincos {
            #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
            pub const PSIZE: Self = Self(0);
            #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
            pub const FIXED4: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pl(pub u8);
        impl Pl {
            #[doc = "Low"]
            pub const LOW: Self = Self(0);
            #[doc = "Medium"]
            pub const MEDIUM: Self = Self(0x01);
            #[doc = "High"]
            pub const HIGH: Self = Self(0x02);
            #[doc = "Very high"]
            pub const VERYHIGH: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Fth(pub u8);
        impl Fth {
            #[doc = "1/4 full FIFO"]
            pub const QUARTER: Self = Self(0);
            #[doc = "1/2 full FIFO"]
            pub const HALF: Self = Self(0x01);
            #[doc = "3/4 full FIFO"]
            pub const THREEQUARTERS: Self = Self(0x02);
            #[doc = "Full FIFO"]
            pub const FULL: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pfctrl(pub u8);
        impl Pfctrl {
            #[doc = "The DMA is the flow controller"]
            pub const DMA: Self = Self(0);
            #[doc = "The peripheral is the flow controller"]
            pub const PERIPHERAL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Circ(pub u8);
        impl Circ {
            #[doc = "Circular mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Circular mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Fs(pub u8);
        impl Fs {
            #[doc = "0 < fifo_level < 1/4"]
            pub const QUARTER1: Self = Self(0);
            #[doc = "1/4 <= fifo_level < 1/2"]
            pub const QUARTER2: Self = Self(0x01);
            #[doc = "1/2 <= fifo_level < 3/4"]
            pub const QUARTER3: Self = Self(0x02);
            #[doc = "3/4 <= fifo_level < full"]
            pub const QUARTER4: Self = Self(0x03);
            #[doc = "FIFO is empty"]
            pub const EMPTY: Self = Self(0x04);
            #[doc = "FIFO is full"]
            pub const FULL: Self = Self(0x05);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Size(pub u8);
        impl Size {
            #[doc = "Byte (8-bit)"]
            pub const BITS8: Self = Self(0);
            #[doc = "Half-word (16-bit)"]
            pub const BITS16: Self = Self(0x01);
            #[doc = "Word (32-bit)"]
            pub const BITS32: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Burst(pub u8);
        impl Burst {
            #[doc = "Single transfer"]
            pub const SINGLE: Self = Self(0);
            #[doc = "Incremental burst of 4 beats"]
            pub const INCR4: Self = Self(0x01);
            #[doc = "Incremental burst of 8 beats"]
            pub const INCR8: Self = Self(0x02);
            #[doc = "Incremental burst of 16 beats"]
            pub const INCR16: Self = Self(0x03);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "stream x FIFO control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fcr(pub u32);
        impl Fcr {
            #[doc = "FIFO threshold selection"]
            pub const fn fth(&self) -> super::vals::Fth {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Fth(val as u8)
            }
            #[doc = "FIFO threshold selection"]
            pub fn set_fth(&mut self, val: super::vals::Fth) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
            }
            #[doc = "Direct mode disable"]
            pub const fn dmdis(&self) -> super::vals::Dmdis {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Dmdis(val as u8)
            }
            #[doc = "Direct mode disable"]
            pub fn set_dmdis(&mut self, val: super::vals::Dmdis) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "FIFO status"]
            pub const fn fs(&self) -> super::vals::Fs {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Fs(val as u8)
            }
            #[doc = "FIFO status"]
            pub fn set_fs(&mut self, val: super::vals::Fs) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.0 as u32) & 0x07) << 3usize);
            }
            #[doc = "FIFO error interrupt enable"]
            pub const fn feie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "FIFO error interrupt enable"]
            pub fn set_feie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Fcr {
            fn default() -> Fcr {
                Fcr(0)
            }
        }
        #[doc = "interrupt register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ixr(pub u32);
        impl Ixr {
            #[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
            pub fn feif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
            pub fn set_feif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
            pub fn dmeif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 2usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
            pub fn set_dmeif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 2usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x transfer error interrupt flag (x=3..0)"]
            pub fn teif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 3usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x transfer error interrupt flag (x=3..0)"]
            pub fn set_teif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 3usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x half transfer interrupt flag (x=3..0)"]
            pub fn htif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 4usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x half transfer interrupt flag (x=3..0)"]
            pub fn set_htif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 4usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
            pub fn tcif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 5usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
            pub fn set_tcif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 5usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Ixr {
            fn default() -> Ixr {
                Ixr(0)
            }
        }
        #[doc = "stream x configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Stream enable / flag stream ready when read low"]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Stream enable / flag stream ready when read low"]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Direct mode error interrupt enable"]
            pub const fn dmeie(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Direct mode error interrupt enable"]
            pub fn set_dmeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Transfer error interrupt enable"]
            pub const fn teie(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer error interrupt enable"]
            pub fn set_teie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Half transfer interrupt enable"]
            pub const fn htie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Half transfer interrupt enable"]
            pub fn set_htie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Transfer complete interrupt enable"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer complete interrupt enable"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Peripheral flow controller"]
            pub const fn pfctrl(&self) -> super::vals::Pfctrl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Pfctrl(val as u8)
            }
            #[doc = "Peripheral flow controller"]
            pub fn set_pfctrl(&mut self, val: super::vals::Pfctrl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "Data transfer direction"]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 6usize) & 0x03;
                super::vals::Dir(val as u8)
            }
            #[doc = "Data transfer direction"]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val.0 as u32) & 0x03) << 6usize);
            }
            #[doc = "Circular mode"]
            pub const fn circ(&self) -> super::vals::Circ {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Circ(val as u8)
            }
            #[doc = "Circular mode"]
            pub fn set_circ(&mut self, val: super::vals::Circ) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "Peripheral increment mode"]
            pub const fn pinc(&self) -> super::vals::Inc {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Inc(val as u8)
            }
            #[doc = "Peripheral increment mode"]
            pub fn set_pinc(&mut self, val: super::vals::Inc) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Memory increment mode"]
            pub const fn minc(&self) -> super::vals::Inc {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Inc(val as u8)
            }
            #[doc = "Memory increment mode"]
            pub fn set_minc(&mut self, val: super::vals::Inc) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Peripheral data size"]
            pub const fn psize(&self) -> super::vals::Size {
                let val = (self.0 >> 11usize) & 0x03;
                super::vals::Size(val as u8)
            }
            #[doc = "Peripheral data size"]
            pub fn set_psize(&mut self, val: super::vals::Size) {
                self.0 = (self.0 & !(0x03 << 11usize)) | (((val.0 as u32) & 0x03) << 11usize);
            }
            #[doc = "Memory data size"]
            pub const fn msize(&self) -> super::vals::Size {
                let val = (self.0 >> 13usize) & 0x03;
                super::vals::Size(val as u8)
            }
            #[doc = "Memory data size"]
            pub fn set_msize(&mut self, val: super::vals::Size) {
                self.0 = (self.0 & !(0x03 << 13usize)) | (((val.0 as u32) & 0x03) << 13usize);
            }
            #[doc = "Peripheral increment offset size"]
            pub const fn pincos(&self) -> super::vals::Pincos {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Pincos(val as u8)
            }
            #[doc = "Peripheral increment offset size"]
            pub fn set_pincos(&mut self, val: super::vals::Pincos) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "Priority level"]
            pub const fn pl(&self) -> super::vals::Pl {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Pl(val as u8)
            }
            #[doc = "Priority level"]
            pub fn set_pl(&mut self, val: super::vals::Pl) {
                self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
            }
            #[doc = "Double buffer mode"]
            pub const fn dbm(&self) -> super::vals::Dbm {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Dbm(val as u8)
            }
            #[doc = "Double buffer mode"]
            pub fn set_dbm(&mut self, val: super::vals::Dbm) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "Current target (only in double buffer mode)"]
            pub const fn ct(&self) -> super::vals::Ct {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Ct(val as u8)
            }
            #[doc = "Current target (only in double buffer mode)"]
            pub fn set_ct(&mut self, val: super::vals::Ct) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
            }
            #[doc = "Peripheral burst transfer configuration"]
            pub const fn pburst(&self) -> super::vals::Burst {
                let val = (self.0 >> 21usize) & 0x03;
                super::vals::Burst(val as u8)
            }
            #[doc = "Peripheral burst transfer configuration"]
            pub fn set_pburst(&mut self, val: super::vals::Burst) {
                self.0 = (self.0 & !(0x03 << 21usize)) | (((val.0 as u32) & 0x03) << 21usize);
            }
            #[doc = "Memory burst transfer configuration"]
            pub const fn mburst(&self) -> super::vals::Burst {
                let val = (self.0 >> 23usize) & 0x03;
                super::vals::Burst(val as u8)
            }
            #[doc = "Memory burst transfer configuration"]
            pub fn set_mburst(&mut self, val: super::vals::Burst) {
                self.0 = (self.0 & !(0x03 << 23usize)) | (((val.0 as u32) & 0x03) << 23usize);
            }
            #[doc = "Channel selection"]
            pub const fn chsel(&self) -> u8 {
                let val = (self.0 >> 25usize) & 0x0f;
                val as u8
            }
            #[doc = "Channel selection"]
            pub fn set_chsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "stream x number of data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ndtr(pub u32);
        impl Ndtr {
            #[doc = "Number of data items to transfer"]
            pub const fn ndt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of data items to transfer"]
            pub fn set_ndt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Ndtr {
            fn default() -> Ndtr {
                Ndtr(0)
            }
        }
    }
}
pub mod sdmmc_v2 {
    use crate::generic::*;
    #[doc = "SDMMC"]
    #[derive(Copy, Clone)]
    pub struct Sdmmc(pub *mut u8);
    unsafe impl Send for Sdmmc {}
    unsafe impl Sync for Sdmmc {}
    impl Sdmmc {
        #[doc = "SDMMC power control register"]
        pub fn power(self) -> Reg<regs::Power, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
        pub fn clkcr(self) -> Reg<regs::Clkcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
        pub fn argr(self) -> Reg<regs::Argr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
        pub fn cmdr(self) -> Reg<regs::Cmdr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "SDMMC command response register"]
        pub fn respcmdr(self) -> Reg<regs::Respcmdr, R> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
        pub fn respr(self, n: usize) -> Reg<regs::Resp1r, R> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(20usize + n * 4usize)) }
        }
        #[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
        pub fn dtimer(self) -> Reg<regs::Dtimer, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
        pub fn dlenr(self) -> Reg<regs::Dlenr, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
        pub fn dctrl(self) -> Reg<regs::Dctrl, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
        pub fn dcntr(self) -> Reg<regs::Dcntr, R> {
            unsafe { Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits [29,21,11:0]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits [20:12]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
        pub fn star(self) -> Reg<regs::Star, R> {
            unsafe { Reg::from_ptr(self.0.add(52usize)) }
        }
        #[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
        pub fn icr(self) -> Reg<regs::Icr, RW> {
            unsafe { Reg::from_ptr(self.0.add(56usize)) }
        }
        #[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
        pub fn maskr(self) -> Reg<regs::Maskr, RW> {
            unsafe { Reg::from_ptr(self.0.add(60usize)) }
        }
        #[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
        pub fn acktimer(self) -> Reg<regs::Acktimer, RW> {
            unsafe { Reg::from_ptr(self.0.add(64usize)) }
        }
        #[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
        pub fn idmactrlr(self) -> Reg<regs::Idmactrlr, RW> {
            unsafe { Reg::from_ptr(self.0.add(80usize)) }
        }
        #[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
        pub fn idmabsizer(self) -> Reg<regs::Idmabsizer, RW> {
            unsafe { Reg::from_ptr(self.0.add(84usize)) }
        }
        #[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
        pub fn idmabase0r(self) -> Reg<regs::Idmabase0r, RW> {
            unsafe { Reg::from_ptr(self.0.add(88usize)) }
        }
        #[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
        pub fn idmabase1r(self) -> Reg<regs::Idmabase1r, RW> {
            unsafe { Reg::from_ptr(self.0.add(92usize)) }
        }
        #[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
        pub fn fifor(self) -> Reg<regs::Fifor, RW> {
            unsafe { Reg::from_ptr(self.0.add(128usize)) }
        }
        #[doc = "SDMMC IP version register"]
        pub fn ver(self) -> Reg<regs::Ver, R> {
            unsafe { Reg::from_ptr(self.0.add(1012usize)) }
        }
        #[doc = "SDMMC IP identification register"]
        pub fn id(self) -> Reg<regs::Id, R> {
            unsafe { Reg::from_ptr(self.0.add(1016usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idmactrlr(pub u32);
        impl Idmactrlr {
            #[doc = "IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn idmaen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_idmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn idmabmode(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_idmabmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
            pub const fn idmabact(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Double buffer mode active buffer indication This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). When IDMA is enabled this bit is toggled by hardware."]
            pub fn set_idmabact(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Idmactrlr {
            fn default() -> Idmactrlr {
                Idmactrlr(0)
            }
        }
        #[doc = "SDMMC command response register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Respcmdr(pub u32);
        impl Respcmdr {
            #[doc = "Response command index"]
            pub const fn respcmd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "Response command index"]
            pub fn set_respcmd(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
        }
        impl Default for Respcmdr {
            fn default() -> Respcmdr {
                Respcmdr(0)
            }
        }
        #[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fifor(pub u32);
        impl Fifor {
            #[doc = "Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
            pub const fn fifodata(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
            pub fn set_fifodata(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Fifor {
            fn default() -> Fifor {
                Fifor(0)
            }
        }
        #[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icr(pub u32);
        impl Icr {
            #[doc = "CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
            pub const fn ccrcfailc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
            pub fn set_ccrcfailc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
            pub const fn dcrcfailc(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
            pub fn set_dcrcfailc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
            pub const fn ctimeoutc(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
            pub fn set_ctimeoutc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
            pub const fn dtimeoutc(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
            pub fn set_dtimeoutc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
            pub const fn txunderrc(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
            pub fn set_txunderrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
            pub const fn rxoverrc(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
            pub fn set_rxoverrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "CMDREND flag clear bit Set by software to clear the CMDREND flag."]
            pub const fn cmdrendc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "CMDREND flag clear bit Set by software to clear the CMDREND flag."]
            pub fn set_cmdrendc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
            pub const fn cmdsentc(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
            pub fn set_cmdsentc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "DATAEND flag clear bit Set by software to clear the DATAEND flag."]
            pub const fn dataendc(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DATAEND flag clear bit Set by software to clear the DATAEND flag."]
            pub fn set_dataendc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "DHOLD flag clear bit Set by software to clear the DHOLD flag."]
            pub const fn dholdc(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "DHOLD flag clear bit Set by software to clear the DHOLD flag."]
            pub fn set_dholdc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
            pub const fn dbckendc(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
            pub fn set_dbckendc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "DABORT flag clear bit Set by software to clear the DABORT flag."]
            pub const fn dabortc(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "DABORT flag clear bit Set by software to clear the DABORT flag."]
            pub fn set_dabortc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
            pub const fn busyd0endc(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
            pub fn set_busyd0endc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
            pub const fn sdioitc(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
            pub fn set_sdioitc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
            pub const fn ackfailc(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
            pub fn set_ackfailc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
            pub const fn acktimeoutc(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
            pub fn set_acktimeoutc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "VSWEND flag clear bit Set by software to clear the VSWEND flag."]
            pub const fn vswendc(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "VSWEND flag clear bit Set by software to clear the VSWEND flag."]
            pub fn set_vswendc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
            pub const fn ckstopc(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
            pub fn set_ckstopc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
            pub const fn idmatec(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
            pub fn set_idmatec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
            pub const fn idmabtcc(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
            pub fn set_idmabtcc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
        }
        impl Default for Icr {
            fn default() -> Icr {
                Icr(0)
            }
        }
        #[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idmabsizer(pub u32);
        impl Idmabsizer {
            #[doc = "Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn idmabndt(&self) -> u8 {
                let val = (self.0 >> 5usize) & 0xff;
                val as u8
            }
            #[doc = "Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_idmabndt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
            }
        }
        impl Default for Idmabsizer {
            fn default() -> Idmabsizer {
                Idmabsizer(0)
            }
        }
        #[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Clkcr(pub u32);
        impl Clkcr {
            #[doc = "Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / [2 * CLKDIV]. 0xx: etc.. xxx: etc.."]
            pub const fn clkdiv(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / [2 * CLKDIV]. 0xx: etc.. xxx: etc.."]
            pub fn set_clkdiv(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
            #[doc = "Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
            pub const fn pwrsav(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
            pub fn set_pwrsav(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
            pub const fn widbus(&self) -> u8 {
                let val = (self.0 >> 14usize) & 0x03;
                val as u8
            }
            #[doc = "Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
            pub fn set_widbus(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
            }
            #[doc = "SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &gt;1 (CLKDIV &gt; 0) &amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) & DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
            pub const fn negedge(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &gt;1 (CLKDIV &gt; 0) &amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) & DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
            pub fn set_negedge(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
            pub const fn hwfc_en(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
            pub fn set_hwfc_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &gt;1. (CLKDIV &gt; 0)"]
            pub const fn ddr(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &gt;1. (CLKDIV &gt; 0)"]
            pub fn set_ddr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
            pub const fn busspeed(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
            pub fn set_busspeed(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
            pub const fn selclkrx(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x03;
                val as u8
            }
            #[doc = "Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
            pub fn set_selclkrx(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
            }
        }
        impl Default for Clkcr {
            fn default() -> Clkcr {
                Clkcr(0)
            }
        }
        #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resp2r(pub u32);
        impl Resp2r {
            #[doc = "see Table404."]
            pub const fn cardstatus2(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "see Table404."]
            pub fn set_cardstatus2(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Resp2r {
            fn default() -> Resp2r {
                Resp2r(0)
            }
        }
        #[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits [29,21,11:0]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits [20:12]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Star(pub u32);
        impl Star {
            #[doc = "Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn ccrcfail(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_ccrcfail(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn dcrcfail(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_dcrcfail(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods."]
            pub const fn ctimeout(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods."]
            pub fn set_ctimeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn dtimeout(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_dtimeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn txunderr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_txunderr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn rxoverr(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_rxoverr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn cmdrend(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_cmdrend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn cmdsent(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_cmdsent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn dataend(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_dataend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Data transfer Hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn dhold(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Data transfer Hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_dhold(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn dbckend(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_dbckend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn dabort(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_dabort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
            pub const fn dpsmact(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
            pub fn set_dpsmact(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
            pub const fn cpsmact(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt."]
            pub fn set_cpsmact(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full."]
            pub const fn txfifohe(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full."]
            pub fn set_txfifohe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty."]
            pub const fn rxfifohf(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty."]
            pub fn set_rxfifohf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty."]
            pub const fn txfifof(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty."]
            pub fn set_txfifof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Receive FIFO full This bit is cleared when one FIFO location becomes empty."]
            pub const fn rxfifof(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO full This bit is cleared when one FIFO location becomes empty."]
            pub fn set_rxfifof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Transmit FIFO empty This bit is cleared when one FIFO location becomes full."]
            pub const fn txfifoe(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO empty This bit is cleared when one FIFO location becomes full."]
            pub fn set_txfifoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full."]
            pub const fn rxfifoe(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full."]
            pub fn set_rxfifoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt."]
            pub const fn busyd0(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt."]
            pub fn set_busyd0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn busyd0end(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_busyd0end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn sdioit(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_sdioit(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Boot acknowledgment received (boot acknowledgment check fail). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn ackfail(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Boot acknowledgment received (boot acknowledgment check fail). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_ackfail(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Boot acknowledgment timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn acktimeout(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Boot acknowledgment timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_acktimeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Voltage switch critical timing section completion. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn vswend(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage switch critical timing section completion. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_vswend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "SDMMC_CK stopped in Voltage switch procedure. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn ckstop(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "SDMMC_CK stopped in Voltage switch procedure. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_ckstop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "IDMA transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn idmate(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "IDMA transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_idmate(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "IDMA buffer transfer complete. interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub const fn idmabtc(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "IDMA buffer transfer complete. interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR."]
            pub fn set_idmabtc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
        }
        impl Default for Star {
            fn default() -> Star {
                Star(0)
            }
        }
        #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resp3r(pub u32);
        impl Resp3r {
            #[doc = "see Table404."]
            pub const fn cardstatus3(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "see Table404."]
            pub fn set_cardstatus3(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Resp3r {
            fn default() -> Resp3r {
                Resp3r(0)
            }
        }
        #[doc = "SDMMC power control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
            pub const fn pwrctrl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
            pub fn set_pwrctrl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
            pub const fn vswitch(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
            pub fn set_vswitch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
            pub const fn vswitchen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
            pub fn set_vswitchen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
            pub const fn dirpol(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
            pub fn set_dirpol(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Power {
            fn default() -> Power {
                Power(0)
            }
        }
        #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resp1r(pub u32);
        impl Resp1r {
            #[doc = "see Table 432"]
            pub const fn cardstatus1(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "see Table 432"]
            pub fn set_cardstatus1(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Resp1r {
            fn default() -> Resp1r {
                Resp1r(0)
            }
        }
        #[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resp4r(pub u32);
        impl Resp4r {
            #[doc = "see Table404."]
            pub const fn cardstatus4(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "see Table404."]
            pub fn set_cardstatus4(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Resp4r {
            fn default() -> Resp4r {
                Resp4r(0)
            }
        }
        #[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Acktimer(pub u32);
        impl Acktimer {
            #[doc = "Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
            pub const fn acktime(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x01ff_ffff;
                val as u32
            }
            #[doc = "Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
            pub fn set_acktime(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
            }
        }
        impl Default for Acktimer {
            fn default() -> Acktimer {
                Acktimer(0)
            }
        }
        #[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Argr(pub u32);
        impl Argr {
            #[doc = "Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
            pub const fn cmdarg(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
            pub fn set_cmdarg(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Argr {
            fn default() -> Argr {
                Argr(0)
            }
        }
        #[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dlenr(pub u32);
        impl Dlenr {
            #[doc = "Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
            pub const fn datalength(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x01ff_ffff;
                val as u32
            }
            #[doc = "Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
            pub fn set_datalength(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
            }
        }
        impl Default for Dlenr {
            fn default() -> Dlenr {
                Dlenr(0)
            }
        }
        #[doc = "SDMMC IP version register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ver(pub u32);
        impl Ver {
            #[doc = "IP minor revision number."]
            pub const fn minrev(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "IP minor revision number."]
            pub fn set_minrev(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "IP major revision number."]
            pub const fn majrev(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "IP major revision number."]
            pub fn set_majrev(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Ver {
            fn default() -> Ver {
                Ver(0)
            }
        }
        #[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maskr(pub u32);
        impl Maskr {
            #[doc = "Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
            pub const fn ccrcfailie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
            pub fn set_ccrcfailie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
            pub const fn dcrcfailie(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
            pub fn set_dcrcfailie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
            pub const fn ctimeoutie(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
            pub fn set_ctimeoutie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
            pub const fn dtimeoutie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
            pub fn set_dtimeoutie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
            pub const fn txunderrie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
            pub fn set_txunderrie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
            pub const fn rxoverrie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
            pub fn set_rxoverrie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
            pub const fn cmdrendie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
            pub fn set_cmdrendie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
            pub const fn cmdsentie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
            pub fn set_cmdsentie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
            pub const fn dataendie(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
            pub fn set_dataendie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
            pub const fn dholdie(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
            pub fn set_dholdie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
            pub const fn dbckendie(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
            pub fn set_dbckendie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
            pub const fn dabortie(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
            pub fn set_dabortie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
            pub const fn txfifoheie(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
            pub fn set_txfifoheie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
            pub const fn rxfifohfie(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
            pub fn set_rxfifohfie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
            pub const fn rxfifofie(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
            pub fn set_rxfifofie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
            pub const fn txfifoeie(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
            pub fn set_txfifoeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
            pub const fn busyd0endie(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
            pub fn set_busyd0endie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
            pub const fn sdioitie(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
            pub fn set_sdioitie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
            pub const fn ackfailie(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
            pub fn set_ackfailie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
            pub const fn acktimeoutie(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
            pub fn set_acktimeoutie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
            pub const fn vswendie(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
            pub fn set_vswendie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
            pub const fn ckstopie(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
            pub fn set_ckstopie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
            pub const fn idmabtcie(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
            pub fn set_idmabtcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
        }
        impl Default for Maskr {
            fn default() -> Maskr {
                Maskr(0)
            }
        }
        #[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cmdr(pub u32);
        impl Cmdr {
            #[doc = "Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
            pub const fn cmdindex(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
            pub fn set_cmdindex(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
            #[doc = "The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
            pub const fn cmdtrans(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
            pub fn set_cmdtrans(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
            pub const fn cmdstop(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
            pub fn set_cmdstop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
            pub const fn waitresp(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x03;
                val as u8
            }
            #[doc = "Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
            pub fn set_waitresp(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
            }
            #[doc = "CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
            pub const fn waitint(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
            pub fn set_waitint(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
            pub const fn waitpend(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
            pub fn set_waitpend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
            pub const fn cpsmen(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
            pub fn set_cpsmen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
            pub const fn dthold(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
            pub fn set_dthold(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
            pub const fn bootmode(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
            pub fn set_bootmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable boot mode procedure."]
            pub const fn booten(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable boot mode procedure."]
            pub fn set_booten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
            pub const fn cmdsuspend(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
            pub fn set_cmdsuspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Cmdr {
            fn default() -> Cmdr {
                Cmdr(0)
            }
        }
        #[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dctrl(pub u32);
        impl Dctrl {
            #[doc = "Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
            pub const fn dten(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
            pub fn set_dten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn dtdir(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_dtdir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn dtmode(&self) -> u8 {
                let val = (self.0 >> 2usize) & 0x03;
                val as u8
            }
            #[doc = "Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_dtmode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
            }
            #[doc = "Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
            pub const fn dblocksize(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
            pub fn set_dblocksize(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
            #[doc = "Read wait start. If this bit is set, read wait operation starts."]
            pub const fn rwstart(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Read wait start. If this bit is set, read wait operation starts."]
            pub fn set_rwstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
            pub const fn rwstop(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
            pub fn set_rwstop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn rwmod(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_rwmod(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
            pub const fn sdioen(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
            pub fn set_sdioen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub const fn bootacken(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
            pub fn set_bootacken(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
            pub const fn fiforst(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
            pub fn set_fiforst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Dctrl {
            fn default() -> Dctrl {
                Dctrl(0)
            }
        }
        #[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idmabase1r(pub u32);
        impl Idmabase1r {
            #[doc = "Buffer 1 memory base address, shall be word aligned (bit [1:0]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
            pub const fn idmabase1(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Buffer 1 memory base address, shall be word aligned (bit [1:0]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
            pub fn set_idmabase1(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Idmabase1r {
            fn default() -> Idmabase1r {
                Idmabase1r(0)
            }
        }
        #[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcntr(pub u32);
        impl Dcntr {
            #[doc = "Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
            pub const fn datacount(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x01ff_ffff;
                val as u32
            }
            #[doc = "Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
            pub fn set_datacount(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
            }
        }
        impl Default for Dcntr {
            fn default() -> Dcntr {
                Dcntr(0)
            }
        }
        #[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dtimer(pub u32);
        impl Dtimer {
            #[doc = "Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
            pub const fn datatime(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
            pub fn set_datatime(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Dtimer {
            fn default() -> Dtimer {
                Dtimer(0)
            }
        }
        #[doc = "SDMMC IP identification register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Id(pub u32);
        impl Id {
            #[doc = "SDMMC IP identification."]
            pub const fn ip_id(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "SDMMC IP identification."]
            pub fn set_ip_id(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Id {
            fn default() -> Id {
                Id(0)
            }
        }
        #[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idmabase0r(pub u32);
        impl Idmabase0r {
            #[doc = "Buffer 0 memory base address bits [31:2], shall be word aligned (bit [1:0]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
            pub const fn idmabase0(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Buffer 0 memory base address bits [31:2], shall be word aligned (bit [1:0]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
            pub fn set_idmabase0(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Idmabase0r {
            fn default() -> Idmabase0r {
                Idmabase0r(0)
            }
        }
    }
}
pub mod gpio_v1 {
    use crate::generic::*;
    #[doc = "General purpose I/O"]
    #[derive(Copy, Clone)]
    pub struct Gpio(pub *mut u8);
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[doc = "Port configuration register low (GPIOn_CRL)"]
        pub fn cr(self, n: usize) -> Reg<regs::Cr, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
        }
        #[doc = "Port input data register (GPIOn_IDR)"]
        pub fn idr(self) -> Reg<regs::Idr, R> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Port output data register (GPIOn_ODR)"]
        pub fn odr(self) -> Reg<regs::Odr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
        pub fn bsrr(self) -> Reg<regs::Bsrr, W> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Port bit reset register (GPIOn_BRR)"]
        pub fn brr(self) -> Reg<regs::Brr, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Port configuration lock register"]
        pub fn lckr(self) -> Reg<regs::Lckr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cnf(pub u8);
        impl Cnf {
            #[doc = "Analog mode / Push-Pull mode"]
            pub const PUSHPULL: Self = Self(0);
            #[doc = "Floating input (reset state) / Open Drain-Mode"]
            pub const OPENDRAIN: Self = Self(0x01);
            #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
            pub const ALTPUSHPULL: Self = Self(0x02);
            #[doc = "Alternate Function Open-Drain Mode"]
            pub const ALTOPENDRAIN: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Brw(pub u8);
        impl Brw {
            #[doc = "No action on the corresponding ODx bit"]
            pub const NOACTION: Self = Self(0);
            #[doc = "Reset the ODx bit"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bsw(pub u8);
        impl Bsw {
            #[doc = "No action on the corresponding ODx bit"]
            pub const NOACTION: Self = Self(0);
            #[doc = "Sets the corresponding ODRx bit"]
            pub const SET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lckk(pub u8);
        impl Lckk {
            #[doc = "Port configuration lock key not active"]
            pub const NOTACTIVE: Self = Self(0);
            #[doc = "Port configuration lock key active"]
            pub const ACTIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mode(pub u8);
        impl Mode {
            #[doc = "Input mode (reset state)"]
            pub const INPUT: Self = Self(0);
            #[doc = "Output mode 10 MHz"]
            pub const OUTPUT: Self = Self(0x01);
            #[doc = "Output mode 2 MHz"]
            pub const OUTPUT2: Self = Self(0x02);
            #[doc = "Output mode 50 MHz"]
            pub const OUTPUT50: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lck(pub u8);
        impl Lck {
            #[doc = "Port configuration not locked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "Port configuration locked"]
            pub const LOCKED: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Port A Lock bit"]
            pub fn lck(&self, n: usize) -> super::vals::Lck {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Lck(val as u8)
            }
            #[doc = "Port A Lock bit"]
            pub fn set_lck(&mut self, n: usize, val: super::vals::Lck) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Lock key"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Lock key"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
        #[doc = "Port input data register (GPIOn_IDR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idr(pub u32);
        impl Idr {
            #[doc = "Port input data"]
            pub fn idr(&self, n: usize) -> super::vals::Idr {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Idr(val as u8)
            }
            #[doc = "Port input data"]
            pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Idr {
            fn default() -> Idr {
                Idr(0)
            }
        }
        #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
            #[doc = "Set bit"]
            pub fn bs(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set bit"]
            pub fn set_bs(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Reset bit"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Reset bit"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Bsrr {
            fn default() -> Bsrr {
                Bsrr(0)
            }
        }
        #[doc = "Port configuration register (GPIOn_CRx)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Port n mode bits"]
            pub fn mode(&self, n: usize) -> super::vals::Mode {
                assert!(n < 8usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Mode(val as u8)
            }
            #[doc = "Port n mode bits"]
            pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
                assert!(n < 8usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
            #[doc = "Port n configuration bits"]
            pub fn cnf(&self, n: usize) -> super::vals::Cnf {
                assert!(n < 8usize);
                let offs = 2usize + n * 4usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Cnf(val as u8)
            }
            #[doc = "Port n configuration bits"]
            pub fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
                assert!(n < 8usize);
                let offs = 2usize + n * 4usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "Port bit reset register (GPIOn_BRR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "Reset bit"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Reset bit"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Port output data register (GPIOn_ODR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Odr(pub u32);
        impl Odr {
            #[doc = "Port output data"]
            pub fn odr(&self, n: usize) -> super::vals::Odr {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Odr(val as u8)
            }
            #[doc = "Port output data"]
            pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Odr {
            fn default() -> Odr {
                Odr(0)
            }
        }
    }
}
pub mod syscfg_l4 {
    use crate::generic::*;
    #[doc = "System configuration controller"]
    #[derive(Copy, Clone)]
    pub struct Syscfg(pub *mut u8);
    unsafe impl Send for Syscfg {}
    unsafe impl Sync for Syscfg {}
    impl Syscfg {
        #[doc = "memory remap register"]
        pub fn memrmp(self) -> Reg<regs::Memrmp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "configuration register 1"]
        pub fn cfgr1(self) -> Reg<regs::Cfgr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "external interrupt configuration register 1"]
        pub fn exticr(self, n: usize) -> Reg<regs::Exticr, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "SCSR"]
        pub fn scsr(self) -> Reg<regs::Scsr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "CFGR2"]
        pub fn cfgr2(self) -> Reg<regs::Cfgr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "SWPR"]
        pub fn swpr(self) -> Reg<regs::Swpr, W> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "SKR"]
        pub fn skr(self) -> Reg<regs::Skr, W> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "CFGR2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfgr2(pub u32);
        impl Cfgr2 {
            #[doc = "Cortex LOCKUP (Hardfault) output enable bit"]
            pub const fn cll(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Cortex LOCKUP (Hardfault) output enable bit"]
            pub fn set_cll(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SRAM2 parity lock bit"]
            pub const fn spl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SRAM2 parity lock bit"]
            pub fn set_spl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "PVD lock enable bit"]
            pub const fn pvdl(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "PVD lock enable bit"]
            pub fn set_pvdl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "ECC Lock"]
            pub const fn eccl(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ECC Lock"]
            pub fn set_eccl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "SRAM2 parity error flag"]
            pub const fn spf(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "SRAM2 parity error flag"]
            pub fn set_spf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Cfgr2 {
            fn default() -> Cfgr2 {
                Cfgr2(0)
            }
        }
        #[doc = "SKR"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Skr(pub u32);
        impl Skr {
            #[doc = "SRAM2 write protection key for software erase"]
            pub const fn key(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "SRAM2 write protection key for software erase"]
            pub fn set_key(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Skr {
            fn default() -> Skr {
                Skr(0)
            }
        }
        #[doc = "configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfgr1(pub u32);
        impl Cfgr1 {
            #[doc = "Firewall disable"]
            pub const fn fwdis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Firewall disable"]
            pub fn set_fwdis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "I/O analog switch voltage booster enable"]
            pub const fn boosten(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "I/O analog switch voltage booster enable"]
            pub fn set_boosten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
            pub const fn i2c_pb6_fmp(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
            pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
            pub const fn i2c_pb7_fmp(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
            pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
            pub const fn i2c_pb8_fmp(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
            pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
            pub const fn i2c_pb9_fmp(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
            pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "I2C1 Fast-mode Plus driving capability activation"]
            pub const fn i2c1_fmp(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "I2C1 Fast-mode Plus driving capability activation"]
            pub fn set_i2c1_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "I2C2 Fast-mode Plus driving capability activation"]
            pub const fn i2c2_fmp(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "I2C2 Fast-mode Plus driving capability activation"]
            pub fn set_i2c2_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "I2C3 Fast-mode Plus driving capability activation"]
            pub const fn i2c3_fmp(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "I2C3 Fast-mode Plus driving capability activation"]
            pub fn set_i2c3_fmp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Floating Point Unit interrupts enable bits"]
            pub const fn fpu_ie(&self) -> u8 {
                let val = (self.0 >> 26usize) & 0x3f;
                val as u8
            }
            #[doc = "Floating Point Unit interrupts enable bits"]
            pub fn set_fpu_ie(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
            }
        }
        impl Default for Cfgr1 {
            fn default() -> Cfgr1 {
                Cfgr1(0)
            }
        }
        #[doc = "SWPR"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Swpr(pub u32);
        impl Swpr {
            #[doc = "SRAWM2 write protection."]
            pub fn pwp(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "SRAWM2 write protection."]
            pub fn set_pwp(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Swpr {
            fn default() -> Swpr {
                Swpr(0)
            }
        }
        #[doc = "SCSR"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Scsr(pub u32);
        impl Scsr {
            #[doc = "SRAM2 Erase"]
            pub const fn sram2er(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "SRAM2 Erase"]
            pub fn set_sram2er(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SRAM2 busy by erase operation"]
            pub const fn sram2bsy(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SRAM2 busy by erase operation"]
            pub fn set_sram2bsy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Scsr {
            fn default() -> Scsr {
                Scsr(0)
            }
        }
        #[doc = "external interrupt configuration register 4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Exticr(pub u32);
        impl Exticr {
            #[doc = "EXTI12 configuration bits"]
            pub fn exti(&self, n: usize) -> u8 {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                val as u8
            }
            #[doc = "EXTI12 configuration bits"]
            pub fn set_exti(&mut self, n: usize, val: u8) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
            }
        }
        impl Default for Exticr {
            fn default() -> Exticr {
                Exticr(0)
            }
        }
        #[doc = "memory remap register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Memrmp(pub u32);
        impl Memrmp {
            #[doc = "Memory mapping selection"]
            pub const fn mem_mode(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Memory mapping selection"]
            pub fn set_mem_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
            #[doc = "QUADSPI memory mapping swap"]
            pub const fn qfs(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "QUADSPI memory mapping swap"]
            pub fn set_qfs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Flash Bank mode selection"]
            pub const fn fb_mode(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Flash Bank mode selection"]
            pub fn set_fb_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Memrmp {
            fn default() -> Memrmp {
                Memrmp(0)
            }
        }
    }
}
