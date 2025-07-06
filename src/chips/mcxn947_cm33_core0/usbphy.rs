#[doc = "USBPHY"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbphy {
    ptr: *mut u8,
}
unsafe impl Send for Usbphy {}
unsafe impl Sync for Usbphy {}
impl Usbphy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn pwd(self) -> crate::common::Reg<regs::Pwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn pwd_set(self) -> crate::common::Reg<regs::PwdSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn pwd_clr(self) -> crate::common::Reg<regs::PwdClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn pwd_tog(self) -> crate::common::Reg<regs::PwdTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "TX Control"]
    #[inline(always)]
    pub const fn tx(self) -> crate::common::Reg<regs::Tx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "TX Control"]
    #[inline(always)]
    pub const fn tx_set(self) -> crate::common::Reg<regs::TxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TX Control"]
    #[inline(always)]
    pub const fn tx_clr(self) -> crate::common::Reg<regs::TxClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TX Control"]
    #[inline(always)]
    pub const fn tx_tog(self) -> crate::common::Reg<regs::TxTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RX Control"]
    #[inline(always)]
    pub const fn rx(self) -> crate::common::Reg<regs::Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RX Control"]
    #[inline(always)]
    pub const fn rx_set(self) -> crate::common::Reg<regs::RxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RX Control"]
    #[inline(always)]
    pub const fn rx_clr(self) -> crate::common::Reg<regs::RxClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RX Control"]
    #[inline(always)]
    pub const fn rx_tog(self) -> crate::common::Reg<regs::RxTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "General Purpose Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "General Purpose Control"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "General Purpose Control"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "General Purpose Control"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Debug 0"]
    #[inline(always)]
    pub const fn debug0(self) -> crate::common::Reg<regs::Debug0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Debug 0"]
    #[inline(always)]
    pub const fn debug0_set(self) -> crate::common::Reg<regs::Debug0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Debug 0"]
    #[inline(always)]
    pub const fn debug0_clr(self) -> crate::common::Reg<regs::Debug0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Debug 0"]
    #[inline(always)]
    pub const fn debug0_tog(self) -> crate::common::Reg<regs::Debug0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Version"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "IP Block"]
    #[inline(always)]
    pub const fn ip(self) -> crate::common::Reg<regs::Ip, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "IP Block"]
    #[inline(always)]
    pub const fn ip_set(self) -> crate::common::Reg<regs::IpSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "IP Block"]
    #[inline(always)]
    pub const fn ip_clr(self) -> crate::common::Reg<regs::IpClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "IP Block"]
    #[inline(always)]
    pub const fn ip_tog(self) -> crate::common::Reg<regs::IpTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "PLL SIC"]
    #[inline(always)]
    pub const fn pll_sic(self) -> crate::common::Reg<regs::PllSic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "PLL SIC"]
    #[inline(always)]
    pub const fn pll_sic_set(self) -> crate::common::Reg<regs::PllSicSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "PLL SIC"]
    #[inline(always)]
    pub const fn pll_sic_clr(self) -> crate::common::Reg<regs::PllSicClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "PLL SIC"]
    #[inline(always)]
    pub const fn pll_sic_tog(self) -> crate::common::Reg<regs::PllSicTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "VBUS Detect"]
    #[inline(always)]
    pub const fn usb1_vbus_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "VBUS Detect"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "VBUS Detect"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "VBUS Detect"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "VBUS Detect Status"]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "VBUS Detect Status"]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat_set(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetStatSet, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "VBUS Detect Status"]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetStatClr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "VBUS Detect Status"]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetStatTog, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Charger Detect"]
    #[inline(always)]
    pub const fn usb1_chrg_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Charger Detect"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "Charger Detect"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "Charger Detect"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "Charger Detect Status"]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Charger Detect Status"]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat_set(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetStatSet, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "Charger Detect Status"]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetStatClr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Charger Detect Status"]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetStatTog, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "Analog Control"]
    #[inline(always)]
    pub const fn anactrl(self) -> crate::common::Reg<regs::Anactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Analog Control"]
    #[inline(always)]
    pub const fn anactrl_set(self) -> crate::common::Reg<regs::AnactrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Analog Control"]
    #[inline(always)]
    pub const fn anactrl_clr(self) -> crate::common::Reg<regs::AnactrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Analog Control"]
    #[inline(always)]
    pub const fn anactrl_tog(self) -> crate::common::Reg<regs::AnactrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Trim"]
    #[inline(always)]
    pub const fn trim_override_en(
        self,
    ) -> crate::common::Reg<regs::TrimOverrideEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Trim"]
    #[inline(always)]
    pub const fn trim_override_en_set(
        self,
    ) -> crate::common::Reg<regs::TrimOverrideEnSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Trim"]
    #[inline(always)]
    pub const fn trim_override_en_clr(
        self,
    ) -> crate::common::Reg<regs::TrimOverrideEnClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Trim"]
    #[inline(always)]
    pub const fn trim_override_en_tog(
        self,
    ) -> crate::common::Reg<regs::TrimOverrideEnTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "PFD A"]
    #[inline(always)]
    pub const fn pfda(self) -> crate::common::Reg<regs::Pfda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "PFD A"]
    #[inline(always)]
    pub const fn pfda_set(self) -> crate::common::Reg<regs::PfdaSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "PFD A"]
    #[inline(always)]
    pub const fn pfda_clr(self) -> crate::common::Reg<regs::PfdaClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "PFD A"]
    #[inline(always)]
    pub const fn pfda_tog(self) -> crate::common::Reg<regs::PfdaTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
