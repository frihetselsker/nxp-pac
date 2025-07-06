#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Adc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0 {
    #[inline(always)]
    fn from(val: u8) -> Adc0 {
        Adc0::from_bits(val)
    }
}
impl From<Adc0> for u8 {
    #[inline(always)]
    fn from(val: Adc0) -> u8 {
        Adc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Adc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Adc0Rst {
        Adc0Rst::from_bits(val)
    }
}
impl From<Adc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Adc0Rst) -> u8 {
        Adc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Adc0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivHalt {
        Adc0clkdivHalt::from_bits(val)
    }
}
impl From<Adc0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivHalt) -> u8 {
        Adc0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Adc0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivReset {
        Adc0clkdivReset::from_bits(val)
    }
}
impl From<Adc0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivReset) -> u8 {
        Adc0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Adc0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivUnstab {
        Adc0clkdivUnstab::from_bits(val)
    }
}
impl From<Adc0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivUnstab) -> u8 {
        Adc0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 12 MHz clock"]
    ENUM_0X3 = 0x03,
    #[doc = "Clk_in"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Adc0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkselSel {
        Adc0clkselSel::from_bits(val)
    }
}
impl From<Adc0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkselSel) -> u8 {
        Adc0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Adc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1 {
    #[inline(always)]
    fn from(val: u8) -> Adc1 {
        Adc1::from_bits(val)
    }
}
impl From<Adc1> for u8 {
    #[inline(always)]
    fn from(val: Adc1) -> u8 {
        Adc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Adc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Adc1Rst {
        Adc1Rst::from_bits(val)
    }
}
impl From<Adc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Adc1Rst) -> u8 {
        Adc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Adc1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivHalt {
        Adc1clkdivHalt::from_bits(val)
    }
}
impl From<Adc1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivHalt) -> u8 {
        Adc1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Adc1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivReset {
        Adc1clkdivReset::from_bits(val)
    }
}
impl From<Adc1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivReset) -> u8 {
        Adc1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Adc1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivUnstab {
        Adc1clkdivUnstab::from_bits(val)
    }
}
impl From<Adc1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivUnstab) -> u8 {
        Adc1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 12 MHz clock"]
    ENUM_0X3 = 0x03,
    #[doc = "Clk_in clock"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Adc1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkselSel {
        Adc1clkselSel::from_bits(val)
    }
}
impl From<Adc1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkselSel) -> u8 {
        Adc1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0Dma0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0Dma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0Dma0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0Dma0 {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0Dma0 {
        Ahbclkctrl0Dma0::from_bits(val)
    }
}
impl From<Ahbclkctrl0Dma0> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0Dma0) -> u8 {
        Ahbclkctrl0Dma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RambCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RambCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RambCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RambCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RambCtrl {
        Ahbclkctrl0RambCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RambCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RambCtrl) -> u8 {
        Ahbclkctrl0RambCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RamcCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RamcCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RamcCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RamcCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RamcCtrl {
        Ahbclkctrl0RamcCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RamcCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RamcCtrl) -> u8 {
        Ahbclkctrl0RamcCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RamdCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RamdCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RamdCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RamdCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RamdCtrl {
        Ahbclkctrl0RamdCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RamdCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RamdCtrl) -> u8 {
        Ahbclkctrl0RamdCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RameCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RameCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RameCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RameCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RameCtrl {
        Ahbclkctrl0RameCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RameCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RameCtrl) -> u8 {
        Ahbclkctrl0RameCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RamfCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RamfCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RamfCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RamfCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RamfCtrl {
        Ahbclkctrl0RamfCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RamfCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RamfCtrl) -> u8 {
        Ahbclkctrl0RamfCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RamgCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RamgCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RamgCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RamgCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RamgCtrl {
        Ahbclkctrl0RamgCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RamgCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RamgCtrl) -> u8 {
        Ahbclkctrl0RamgCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0RamhCtrl {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl0RamhCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0RamhCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0RamhCtrl {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0RamhCtrl {
        Ahbclkctrl0RamhCtrl::from_bits(val)
    }
}
impl From<Ahbclkctrl0RamhCtrl> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0RamhCtrl) -> u8 {
        Ahbclkctrl0RamhCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl2Dma1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ahbclkctrl2Dma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl2Dma1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl2Dma1 {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl2Dma1 {
        Ahbclkctrl2Dma1::from_bits(val)
    }
}
impl From<Ahbclkctrl2Dma1> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl2Dma1) -> u8 {
        Ahbclkctrl2Dma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma1 {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma1 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma1 {
        AhbmatprioDma1::from_bits(val)
    }
}
impl From<AhbmatprioDma1> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma1) -> u8 {
        AhbmatprioDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Aoi0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi0Rst {
    #[inline(always)]
    fn from(val: u8) -> Aoi0Rst {
        Aoi0Rst::from_bits(val)
    }
}
impl From<Aoi0Rst> for u8 {
    #[inline(always)]
    fn from(val: Aoi0Rst) -> u8 {
        Aoi0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AssetProtection {
    #[doc = "ELS asset is protected"]
    VALUE0 = 0x0,
    #[doc = "ELS asset is not protected"]
    VALUE1 = 0x01,
    #[doc = "ELS asset is protected"]
    VALUE2 = 0x02,
    #[doc = "ELS asset is protected"]
    VALUE3 = 0x03,
}
impl AssetProtection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AssetProtection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AssetProtection {
    #[inline(always)]
    fn from(val: u8) -> AssetProtection {
        AssetProtection::from_bits(val)
    }
}
impl From<AssetProtection> for u8 {
    #[inline(always)]
    fn from(val: AssetProtection) -> u8 {
        AssetProtection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRambCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRambCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRambCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRambCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRambCtrl {
        AutoclkgateoverrideRambCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRambCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRambCtrl) -> u8 {
        AutoclkgateoverrideRambCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRamcCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRamcCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRamcCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRamcCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRamcCtrl {
        AutoclkgateoverrideRamcCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRamcCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRamcCtrl) -> u8 {
        AutoclkgateoverrideRamcCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRamdCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRamdCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRamdCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRamdCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRamdCtrl {
        AutoclkgateoverrideRamdCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRamdCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRamdCtrl) -> u8 {
        AutoclkgateoverrideRamdCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRameCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRameCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRameCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRameCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRameCtrl {
        AutoclkgateoverrideRameCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRameCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRameCtrl) -> u8 {
        AutoclkgateoverrideRameCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRamfCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRamfCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRamfCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRamfCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRamfCtrl {
        AutoclkgateoverrideRamfCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRamfCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRamfCtrl) -> u8 {
        AutoclkgateoverrideRamfCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRamgCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRamgCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRamgCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRamgCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRamgCtrl {
        AutoclkgateoverrideRamgCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRamgCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRamgCtrl) -> u8 {
        AutoclkgateoverrideRamgCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRamhCtrl {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRamhCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRamhCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRamhCtrl {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRamhCtrl {
        AutoclkgateoverrideRamhCtrl::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRamhCtrl> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRamhCtrl) -> u8 {
        AutoclkgateoverrideRamhCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootImage {
    #[doc = "Internal flash image 0"]
    ENUM0 = 0x0,
    #[doc = "Internal flash image 1"]
    ENUM1 = 0x01,
    #[doc = "FlexSPI flash image 0"]
    ENUM2 = 0x02,
    #[doc = "FlexSPI flash image 1"]
    ENUM3 = 0x03,
    #[doc = "Recovery SPI flash image"]
    ENUM4 = 0x04,
    #[doc = "Serial boot image (write-memory and execute ISP command used)"]
    ENUM5 = 0x05,
    #[doc = "Receive SB3 containing SB_JUMP command is used."]
    ENUM6 = 0x06,
    #[doc = "Customer SBL/recovery image (Bank1 IFR0)."]
    ENUM7 = 0x07,
    #[doc = "NXP MAD recovery image (Bank1 IFR0)."]
    ENUM8 = 0x08,
    #[doc = "NXP ROM extension (NMPA - Bank0 IFR0)."]
    ENUM9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl BootImage {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootImage {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootImage {
    #[inline(always)]
    fn from(val: u8) -> BootImage {
        BootImage::from_bits(val)
    }
}
impl From<BootImage> for u8 {
    #[inline(always)]
    fn from(val: BootImage) -> u8 {
        BootImage::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkinEna {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl ClkinEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkinEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkinEna {
    #[inline(always)]
    fn from(val: u8) -> ClkinEna {
        ClkinEna::from_bits(val)
    }
}
impl From<ClkinEna> for u8 {
    #[inline(always)]
    fn from(val: ClkinEna) -> u8 {
        ClkinEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkinEnaFmUsbhLpt {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl ClkinEnaFmUsbhLpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkinEnaFmUsbhLpt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkinEnaFmUsbhLpt {
    #[inline(always)]
    fn from(val: u8) -> ClkinEnaFmUsbhLpt {
        ClkinEnaFmUsbhLpt::from_bits(val)
    }
}
impl From<ClkinEnaFmUsbhLpt> for u8 {
    #[inline(always)]
    fn from(val: ClkinEnaFmUsbhLpt) -> u8 {
        ClkinEnaFmUsbhLpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl ClkoutdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivHalt {
        ClkoutdivHalt::from_bits(val)
    }
}
impl From<ClkoutdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivHalt) -> u8 {
        ClkoutdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl ClkoutdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReset {
        ClkoutdivReset::from_bits(val)
    }
}
impl From<ClkoutdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReset) -> u8 {
        ClkoutdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl ClkoutdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivUnstab {
        ClkoutdivUnstab::from_bits(val)
    }
}
impl From<ClkoutdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivUnstab) -> u8 {
        ClkoutdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutselSel {
    #[doc = "Main clock (main_clk)"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock (pll0_clk)"]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock (clk_in)"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock (fro_hf)"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 12 MHz clock (fro_12m)"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock (pll1_clk)"]
    ENUM_0X5 = 0x05,
    #[doc = "LP Oscillator clock (lp_osc)"]
    ENUM_0X6 = 0x06,
    #[doc = "USB PLL clock (usb_pll_clk)"]
    ENUM_0X7 = 0x07,
    #[doc = "No clock"]
    ENUM_0X8 = 0x08,
    #[doc = "No clock"]
    ENUM_0X9 = 0x09,
    #[doc = "No clock"]
    ENUM_0X_A = 0x0a,
    #[doc = "No clock"]
    ENUM_0X_B = 0x0b,
    #[doc = "No clock"]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock"]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock"]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock"]
    ENUM_0X_F = 0x0f,
}
impl ClkoutselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutselSel {
    #[inline(always)]
    fn from(val: u8) -> ClkoutselSel {
        ClkoutselSel::from_bits(val)
    }
}
impl From<ClkoutselSel> for u8 {
    #[inline(always)]
    fn from(val: ClkoutselSel) -> u8 {
        ClkoutselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrFlashCache {
    #[doc = "No clear flash cache"]
    ENABLE = 0x0,
    #[doc = "Clears flash cache"]
    DISABLE = 0x01,
}
impl ClrFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrFlashCache {
    #[inline(always)]
    fn from(val: u8) -> ClrFlashCache {
        ClrFlashCache::from_bits(val)
    }
}
impl From<ClrFlashCache> for u8 {
    #[inline(always)]
    fn from(val: ClrFlashCache) -> u8 {
        ClrFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache"]
    ENABLE = 0x0,
    #[doc = "Clears the cache"]
    DISABLE = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Cmp0fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkdivHalt {
        Cmp0fclkdivHalt::from_bits(val)
    }
}
impl From<Cmp0fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkdivHalt) -> u8 {
        Cmp0fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Cmp0fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkdivReset {
        Cmp0fclkdivReset::from_bits(val)
    }
}
impl From<Cmp0fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkdivReset) -> u8 {
        Cmp0fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Cmp0fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkdivUnstab {
        Cmp0fclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp0fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkdivUnstab) -> u8 {
        Cmp0fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkselSel {
        Cmp0fclkselSel::from_bits(val)
    }
}
impl From<Cmp0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkselSel) -> u8 {
        Cmp0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Cmp0rrclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkdivHalt {
        Cmp0rrclkdivHalt::from_bits(val)
    }
}
impl From<Cmp0rrclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkdivHalt) -> u8 {
        Cmp0rrclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Cmp0rrclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkdivReset {
        Cmp0rrclkdivReset::from_bits(val)
    }
}
impl From<Cmp0rrclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkdivReset) -> u8 {
        Cmp0rrclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Cmp0rrclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkdivUnstab {
        Cmp0rrclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp0rrclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkdivUnstab) -> u8 {
        Cmp0rrclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp0rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkselSel {
        Cmp0rrclkselSel::from_bits(val)
    }
}
impl From<Cmp0rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkselSel) -> u8 {
        Cmp0rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Cmp1fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkdivHalt {
        Cmp1fclkdivHalt::from_bits(val)
    }
}
impl From<Cmp1fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkdivHalt) -> u8 {
        Cmp1fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Cmp1fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkdivReset {
        Cmp1fclkdivReset::from_bits(val)
    }
}
impl From<Cmp1fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkdivReset) -> u8 {
        Cmp1fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Cmp1fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkdivUnstab {
        Cmp1fclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp1fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkdivUnstab) -> u8 {
        Cmp1fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkselSel {
        Cmp1fclkselSel::from_bits(val)
    }
}
impl From<Cmp1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkselSel) -> u8 {
        Cmp1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Cmp1rrclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkdivHalt {
        Cmp1rrclkdivHalt::from_bits(val)
    }
}
impl From<Cmp1rrclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkdivHalt) -> u8 {
        Cmp1rrclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Cmp1rrclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkdivReset {
        Cmp1rrclkdivReset::from_bits(val)
    }
}
impl From<Cmp1rrclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkdivReset) -> u8 {
        Cmp1rrclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Cmp1rrclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkdivUnstab {
        Cmp1rrclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp1rrclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkdivUnstab) -> u8 {
        Cmp1rrclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp1rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkselSel {
        Cmp1rrclkselSel::from_bits(val)
    }
}
impl From<Cmp1rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkselSel) -> u8 {
        Cmp1rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Cmp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2 {
    #[inline(always)]
    fn from(val: u8) -> Cmp2 {
        Cmp2::from_bits(val)
    }
}
impl From<Cmp2> for u8 {
    #[inline(always)]
    fn from(val: Cmp2) -> u8 {
        Cmp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Cmp2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2Rst {
    #[inline(always)]
    fn from(val: u8) -> Cmp2Rst {
        Cmp2Rst::from_bits(val)
    }
}
impl From<Cmp2Rst> for u8 {
    #[inline(always)]
    fn from(val: Cmp2Rst) -> u8 {
        Cmp2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2fclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Cmp2fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp2fclkdivHalt {
        Cmp2fclkdivHalt::from_bits(val)
    }
}
impl From<Cmp2fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp2fclkdivHalt) -> u8 {
        Cmp2fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2fclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Cmp2fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp2fclkdivReset {
        Cmp2fclkdivReset::from_bits(val)
    }
}
impl From<Cmp2fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp2fclkdivReset) -> u8 {
        Cmp2fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2fclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Cmp2fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp2fclkdivUnstab {
        Cmp2fclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp2fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp2fclkdivUnstab) -> u8 {
        Cmp2fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp2fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp2fclkselSel {
        Cmp2fclkselSel::from_bits(val)
    }
}
impl From<Cmp2fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp2fclkselSel) -> u8 {
        Cmp2fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2rrclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Cmp2rrclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2rrclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2rrclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp2rrclkdivHalt {
        Cmp2rrclkdivHalt::from_bits(val)
    }
}
impl From<Cmp2rrclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp2rrclkdivHalt) -> u8 {
        Cmp2rrclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2rrclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Cmp2rrclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2rrclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2rrclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp2rrclkdivReset {
        Cmp2rrclkdivReset::from_bits(val)
    }
}
impl From<Cmp2rrclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp2rrclkdivReset) -> u8 {
        Cmp2rrclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2rrclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Cmp2rrclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2rrclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2rrclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp2rrclkdivUnstab {
        Cmp2rrclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp2rrclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp2rrclkdivUnstab) -> u8 {
        Cmp2rrclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2rrclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock0"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp2rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp2rrclkselSel {
        Cmp2rrclkselSel::from_bits(val)
    }
}
impl From<Cmp2rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp2rrclkselSel) -> u8 {
        Cmp2rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cnten {
    #[doc = "ETB counter is disabled"]
    DISABLE = 0x0,
    #[doc = "ETB counter is enabled"]
    ENABLE = 0x01,
}
impl Cnten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cnten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cnten {
    #[inline(always)]
    fn from(val: u8) -> Cnten {
        Cnten::from_bits(val)
    }
}
impl From<Cnten> for u8 {
    #[inline(always)]
    fn from(val: Cnten) -> u8 {
        Cnten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coolflux {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Coolflux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coolflux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coolflux {
    #[inline(always)]
    fn from(val: u8) -> Coolflux {
        Coolflux::from_bits(val)
    }
}
impl From<Coolflux> for u8 {
    #[inline(always)]
    fn from(val: Coolflux) -> u8 {
        Coolflux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoolfluxApb {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock (CoolFlux needs to be properly programmed before the clock enabled.)"]
    ENABLE = 0x01,
}
impl CoolfluxApb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoolfluxApb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoolfluxApb {
    #[inline(always)]
    fn from(val: u8) -> CoolfluxApb {
        CoolfluxApb::from_bits(val)
    }
}
impl From<CoolfluxApb> for u8 {
    #[inline(always)]
    fn from(val: CoolfluxApb) -> u8 {
        CoolfluxApb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoolfluxApbRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl CoolfluxApbRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoolfluxApbRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoolfluxApbRst {
    #[inline(always)]
    fn from(val: u8) -> CoolfluxApbRst {
        CoolfluxApbRst::from_bits(val)
    }
}
impl From<CoolfluxApbRst> for u8 {
    #[inline(always)]
    fn from(val: CoolfluxApbRst) -> u8 {
        CoolfluxApbRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoolfluxRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl CoolfluxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoolfluxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoolfluxRst {
    #[inline(always)]
    fn from(val: u8) -> CoolfluxRst {
        CoolfluxRst::from_bits(val)
    }
}
impl From<CoolfluxRst> for u8 {
    #[inline(always)]
    fn from(val: CoolfluxRst) -> u8 {
        CoolfluxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup"]
    AWAKE = 0x0,
    #[doc = "CPU is in lockup"]
    SLEEPING = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalNoref {
    #[doc = "Reference clock is provided"]
    YES_REF = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF = 0x01,
}
impl Cpu0nstckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalNoref {
        Cpu0nstckcalNoref::from_bits(val)
    }
}
impl From<Cpu0nstckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalNoref) -> u8 {
        Cpu0nstckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalSkew {
    #[doc = "TENMS value is exact"]
    EXACT = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT = 0x01,
}
impl Cpu0nstckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalSkew {
        Cpu0nstckcalSkew::from_bits(val)
    }
}
impl From<Cpu0nstckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalSkew) -> u8 {
        Cpu0nstckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping"]
    AWAKE = 0x0,
    #[doc = "CPU is sleeping"]
    SLEEPING = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalNoref {
    #[doc = "Reference clock is provided"]
    YES_REF = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF = 0x01,
}
impl Cpu0stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalNoref {
        Cpu0stckcalNoref::from_bits(val)
    }
}
impl From<Cpu0stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalNoref) -> u8 {
        Cpu0stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalSkew {
    #[doc = "TENMS value is exact"]
    EXACT = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT = 0x01,
}
impl Cpu0stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalSkew {
        Cpu0stckcalSkew::from_bits(val)
    }
}
impl From<Cpu0stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalSkew) -> u8 {
        Cpu0stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1clken {
    #[doc = "The CPU1 clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "The CPU1 clock is enabled"]
    ENABLE = 0x01,
}
impl Cpu1clken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1clken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1clken {
    #[inline(always)]
    fn from(val: u8) -> Cpu1clken {
        Cpu1clken::from_bits(val)
    }
}
impl From<Cpu1clken> for u8 {
    #[inline(always)]
    fn from(val: Cpu1clken) -> u8 {
        Cpu1clken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1lockup {
    #[doc = "CPU is not in lockup"]
    AWAKE = 0x0,
    #[doc = "CPU is in lockup"]
    SLEEPING = 0x01,
}
impl Cpu1lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu1lockup {
        Cpu1lockup::from_bits(val)
    }
}
impl From<Cpu1lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu1lockup) -> u8 {
        Cpu1lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1rsten {
    #[doc = "The CPU1 is not reset."]
    RELEASED = 0x0,
    #[doc = "The CPU1 is reset."]
    ASSERTED = 0x01,
}
impl Cpu1rsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1rsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1rsten {
    #[inline(always)]
    fn from(val: u8) -> Cpu1rsten {
        Cpu1rsten::from_bits(val)
    }
}
impl From<Cpu1rsten> for u8 {
    #[inline(always)]
    fn from(val: Cpu1rsten) -> u8 {
        Cpu1rsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1sleeping {
    #[doc = "CPU is not sleeping"]
    AWAKE = 0x0,
    #[doc = "CPU is sleeping"]
    SLEEPING = 0x01,
}
impl Cpu1sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu1sleeping {
        Cpu1sleeping::from_bits(val)
    }
}
impl From<Cpu1sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu1sleeping) -> u8 {
        Cpu1sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1stckcalNoref {
    #[doc = "Reference clock is provided"]
    YES_REF_1 = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF_1 = 0x01,
}
impl Cpu1stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu1stckcalNoref {
        Cpu1stckcalNoref::from_bits(val)
    }
}
impl From<Cpu1stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu1stckcalNoref) -> u8 {
        Cpu1stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1stckcalSkew {
    #[doc = "TENMS value is exact"]
    EXACT_1 = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT_1 = 0x01,
}
impl Cpu1stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu1stckcalSkew {
        Cpu1stckcalSkew::from_bits(val)
    }
}
impl From<Cpu1stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu1stckcalSkew) -> u8 {
        Cpu1stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Crc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc {
    #[inline(always)]
    fn from(val: u8) -> Crc {
        Crc::from_bits(val)
    }
}
impl From<Crc> for u8 {
    #[inline(always)]
    fn from(val: Crc) -> u8 {
        Crc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl CrcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRst {
    #[inline(always)]
    fn from(val: u8) -> CrcRst {
        CrcRst::from_bits(val)
    }
}
impl From<CrcRst> for u8 {
    #[inline(always)]
    fn from(val: CrcRst) -> u8 {
        CrcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0ClkEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Ctimer0ClkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0ClkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0ClkEn {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0ClkEn {
        Ctimer0ClkEn::from_bits(val)
    }
}
impl From<Ctimer0ClkEn> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0ClkEn) -> u8 {
        Ctimer0ClkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1ClkEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Ctimer1ClkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1ClkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1ClkEn {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1ClkEn {
        Ctimer1ClkEn::from_bits(val)
    }
}
impl From<Ctimer1ClkEn> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1ClkEn) -> u8 {
        Ctimer1ClkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2ClkEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Ctimer2ClkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2ClkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2ClkEn {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2ClkEn {
        Ctimer2ClkEn::from_bits(val)
    }
}
impl From<Ctimer2ClkEn> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2ClkEn) -> u8 {
        Ctimer2ClkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3ClkEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Ctimer3ClkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3ClkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3ClkEn {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3ClkEn {
        Ctimer3ClkEn::from_bits(val)
    }
}
impl From<Ctimer3ClkEn> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3ClkEn) -> u8 {
        Ctimer3ClkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4ClkEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Ctimer4ClkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4ClkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4ClkEn {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4ClkEn {
        Ctimer4ClkEn::from_bits(val)
    }
}
impl From<Ctimer4ClkEn> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4ClkEn) -> u8 {
        Ctimer4ClkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivHalt {
    #[doc = "Divider clock is running"]
    ENABLE = 0x0,
    #[doc = "Divider clock has stopped"]
    DISABLE = 0x01,
}
impl CtimerclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivHalt {
        CtimerclkdivHalt::from_bits(val)
    }
}
impl From<CtimerclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivHalt) -> u8 {
        CtimerclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivReset {
    #[doc = "Divider is not reset"]
    DISABLE = 0x0,
    #[doc = "Divider is reset"]
    ENABLE = 0x01,
}
impl CtimerclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivReset {
        CtimerclkdivReset::from_bits(val)
    }
}
impl From<CtimerclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivReset) -> u8 {
        CtimerclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivUnstab {
    #[doc = "Stable divider clock"]
    ENABLE = 0x0,
    #[doc = "Unstable clock frequency"]
    DISABLE = 0x01,
}
impl CtimerclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivUnstab {
        CtimerclkdivUnstab::from_bits(val)
    }
}
impl From<CtimerclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivUnstab) -> u8 {
        CtimerclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkselSel {
    #[doc = "FRO_1M clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 12MHz clock"]
    ENUM_0X4 = 0x04,
    #[doc = "SAI0 MCLK IN clock"]
    ENUM_0X5 = 0x05,
    #[doc = "LP Oscillator clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
    #[doc = "SAI1 MCLK IN clock"]
    ENUM_0X8 = 0x08,
    #[doc = "SAI0 TX_BCLK clock"]
    ENUM_0X9 = 0x09,
    #[doc = "SAI0 RX_BCLK clock"]
    ENUM_0X_A = 0x0a,
    #[doc = "SAI1 TX_BCLK clock"]
    ENUM_0X_B = 0x0b,
    #[doc = "SAI1 RX_BCLK clock"]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock"]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock"]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock"]
    ENUM_0X_F = 0x0f,
}
impl CtimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkselSel {
        CtimerclkselSel::from_bits(val)
    }
}
impl From<CtimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkselSel) -> u8 {
        CtimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Dac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0 {
    #[inline(always)]
    fn from(val: u8) -> Dac0 {
        Dac0::from_bits(val)
    }
}
impl From<Dac0> for u8 {
    #[inline(always)]
    fn from(val: Dac0) -> u8 {
        Dac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Dac0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dac0Rst {
        Dac0Rst::from_bits(val)
    }
}
impl From<Dac0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dac0Rst) -> u8 {
        Dac0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Dac0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Dac0clkdivHalt {
        Dac0clkdivHalt::from_bits(val)
    }
}
impl From<Dac0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Dac0clkdivHalt) -> u8 {
        Dac0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Dac0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Dac0clkdivReset {
        Dac0clkdivReset::from_bits(val)
    }
}
impl From<Dac0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Dac0clkdivReset) -> u8 {
        Dac0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Dac0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Dac0clkdivUnstab {
        Dac0clkdivUnstab::from_bits(val)
    }
}
impl From<Dac0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Dac0clkdivUnstab) -> u8 {
        Dac0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "Clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12M"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Dac0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dac0clkselSel {
        Dac0clkselSel::from_bits(val)
    }
}
impl From<Dac0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dac0clkselSel) -> u8 {
        Dac0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Dac1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1 {
    #[inline(always)]
    fn from(val: u8) -> Dac1 {
        Dac1::from_bits(val)
    }
}
impl From<Dac1> for u8 {
    #[inline(always)]
    fn from(val: Dac1) -> u8 {
        Dac1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Dac1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dac1Rst {
        Dac1Rst::from_bits(val)
    }
}
impl From<Dac1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dac1Rst) -> u8 {
        Dac1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Dac1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Dac1clkdivHalt {
        Dac1clkdivHalt::from_bits(val)
    }
}
impl From<Dac1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Dac1clkdivHalt) -> u8 {
        Dac1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Dac1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Dac1clkdivReset {
        Dac1clkdivReset::from_bits(val)
    }
}
impl From<Dac1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Dac1clkdivReset) -> u8 {
        Dac1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Dac1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Dac1clkdivUnstab {
        Dac1clkdivUnstab::from_bits(val)
    }
}
impl From<Dac1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Dac1clkdivUnstab) -> u8 {
        Dac1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "Clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12M"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Dac1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dac1clkselSel {
        Dac1clkselSel::from_bits(val)
    }
}
impl From<Dac1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dac1clkselSel) -> u8 {
        Dac1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Dac2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2 {
    #[inline(always)]
    fn from(val: u8) -> Dac2 {
        Dac2::from_bits(val)
    }
}
impl From<Dac2> for u8 {
    #[inline(always)]
    fn from(val: Dac2) -> u8 {
        Dac2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Dac2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2Rst {
    #[inline(always)]
    fn from(val: u8) -> Dac2Rst {
        Dac2Rst::from_bits(val)
    }
}
impl From<Dac2Rst> for u8 {
    #[inline(always)]
    fn from(val: Dac2Rst) -> u8 {
        Dac2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Dac2clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Dac2clkdivHalt {
        Dac2clkdivHalt::from_bits(val)
    }
}
impl From<Dac2clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Dac2clkdivHalt) -> u8 {
        Dac2clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Dac2clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Dac2clkdivReset {
        Dac2clkdivReset::from_bits(val)
    }
}
impl From<Dac2clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Dac2clkdivReset) -> u8 {
        Dac2clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Dac2clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Dac2clkdivUnstab {
        Dac2clkdivUnstab::from_bits(val)
    }
}
impl From<Dac2clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Dac2clkdivUnstab) -> u8 {
        Dac2clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "Clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12M"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Dac2clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dac2clkselSel {
        Dac2clkselSel::from_bits(val)
    }
}
impl From<Dac2clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dac2clkselSel) -> u8 {
        Dac2clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgHaltReq {
    #[doc = "The debug halt request signal is not asserted"]
    DISABLE = 0x0,
    #[doc = "The debug halt request signal is asserted when the ETB count expires"]
    ENABLE = 0x01,
}
impl DbgHaltReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgHaltReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgHaltReq {
    #[inline(always)]
    fn from(val: u8) -> DbgHaltReq {
        DbgHaltReq::from_bits(val)
    }
}
impl From<DbgHaltReq> for u8 {
    #[inline(always)]
    fn from(val: DbgHaltReq) -> u8 {
        DbgHaltReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spiden {
        DebugFeaturesCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spiden) -> u8 {
        DebugFeaturesCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spniden {
        DebugFeaturesCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spniden) -> u8 {
        DebugFeaturesCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Dbgen {
        DebugFeaturesCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Dbgen) -> u8 {
        DebugFeaturesCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Niden {
        DebugFeaturesCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Niden) -> u8 {
        DebugFeaturesCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spiden {
        DebugFeaturesDpCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spiden) -> u8 {
        DebugFeaturesDpCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spniden {
        DebugFeaturesDpCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spniden) -> u8 {
        DebugFeaturesDpCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        DebugFeaturesDpCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Dbgen) -> u8 {
        DebugFeaturesDpCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Niden {
        DebugFeaturesDpCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Niden) -> u8 {
        DebugFeaturesDpCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation"]
    ENABLE = 0x0,
    #[doc = "Disables data speculation"]
    DISABLE = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashCache {
    #[doc = "Enables flash cache"]
    ENABLE = 0x0,
    #[doc = "Disables flash cache"]
    DISABLE = 0x01,
}
impl DisFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashCache {
    #[inline(always)]
    fn from(val: u8) -> DisFlashCache {
        DisFlashCache::from_bits(val)
    }
}
impl From<DisFlashCache> for u8 {
    #[inline(always)]
    fn from(val: DisFlashCache) -> u8 {
        DisFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashData {
    #[doc = "Enables flash data cache when DIS_FLASH_CACHE=0"]
    ENABLE = 0x0,
    #[doc = "Disables flash data cache"]
    DISABLE = 0x01,
}
impl DisFlashData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashData {
    #[inline(always)]
    fn from(val: u8) -> DisFlashData {
        DisFlashData::from_bits(val)
    }
}
impl From<DisFlashData> for u8 {
    #[inline(always)]
    fn from(val: DisFlashData) -> u8 {
        DisFlashData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashInst {
    #[doc = "Enables flash instruction cache when DIS_FLASH_CACHE=0"]
    ENABLE = 0x0,
    #[doc = "Disables flash instruction cache"]
    DISABLE = 0x01,
}
impl DisFlashInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashInst {
    #[inline(always)]
    fn from(val: u8) -> DisFlashInst {
        DisFlashInst::from_bits(val)
    }
}
impl From<DisFlashInst> for u8 {
    #[inline(always)]
    fn from(val: DisFlashInst) -> u8 {
        DisFlashInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation"]
    ENABLE = 0x0,
    #[doc = "Disables flash speculation"]
    DISABLE = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
    DISABLE = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcacWtbf {
    #[doc = "Enables write through buffer"]
    DISABLE = 0x0,
    #[doc = "Disables write through buffer"]
    ENABLE = 0x01,
}
impl DisLpcacWtbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcacWtbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcacWtbf {
    #[inline(always)]
    fn from(val: u8) -> DisLpcacWtbf {
        DisLpcacWtbf::from_bits(val)
    }
}
impl From<DisLpcacWtbf> for u8 {
    #[inline(always)]
    fn from(val: DisLpcacWtbf) -> u8 {
        DisLpcacWtbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data"]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data"]
    DISABLE = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction"]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction"]
    DISABLE = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Dma0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma0Rst {
        Dma0Rst::from_bits(val)
    }
}
impl From<Dma0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma0Rst) -> u8 {
        Dma0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Dma1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma1Rst {
        Dma1Rst::from_bits(val)
    }
}
impl From<Dma1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma1Rst) -> u8 {
        Dma1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspDbgden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DspDbgden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspDbgden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspDbgden {
    #[inline(always)]
    fn from(val: u8) -> DspDbgden {
        DspDbgden::from_bits(val)
    }
}
impl From<DspDbgden> for u8 {
    #[inline(always)]
    fn from(val: DspDbgden) -> u8 {
        DspDbgden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspDbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DspDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspDbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspDbgen {
    #[inline(always)]
    fn from(val: u8) -> DspDbgen {
        DspDbgen::from_bits(val)
    }
}
impl From<DspDbgen> for u8 {
    #[inline(always)]
    fn from(val: DspDbgen) -> u8 {
        DspDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EfuseAttackDetect {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl EfuseAttackDetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EfuseAttackDetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EfuseAttackDetect {
    #[inline(always)]
    fn from(val: u8) -> EfuseAttackDetect {
        EfuseAttackDetect::from_bits(val)
    }
}
impl From<EfuseAttackDetect> for u8 {
    #[inline(always)]
    fn from(val: EfuseAttackDetect) -> u8 {
        EfuseAttackDetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eim {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Eim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eim {
    #[inline(always)]
    fn from(val: u8) -> Eim {
        Eim::from_bits(val)
    }
}
impl From<Eim> for u8 {
    #[inline(always)]
    fn from(val: Eim) -> u8 {
        Eim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EimRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl EimRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EimRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EimRst {
    #[inline(always)]
    fn from(val: u8) -> EimRst {
        EimRst::from_bits(val)
    }
}
impl From<EimRst> for u8 {
    #[inline(always)]
    fn from(val: EimRst) -> u8 {
        EimRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Els {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Els {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Els {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Els {
    #[inline(always)]
    fn from(val: u8) -> Els {
        Els::from_bits(val)
    }
}
impl From<Els> for u8 {
    #[inline(always)]
    fn from(val: Els) -> u8 {
        Els::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Emvsim0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkdivHalt {
        Emvsim0clkdivHalt::from_bits(val)
    }
}
impl From<Emvsim0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkdivHalt) -> u8 {
        Emvsim0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Emvsim0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkdivReset {
        Emvsim0clkdivReset::from_bits(val)
    }
}
impl From<Emvsim0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkdivReset) -> u8 {
        Emvsim0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Emvsim0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkdivUnstab {
        Emvsim0clkdivUnstab::from_bits(val)
    }
}
impl From<Emvsim0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkdivUnstab) -> u8 {
        Emvsim0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock0"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Emvsim0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkselSel {
        Emvsim0clkselSel::from_bits(val)
    }
}
impl From<Emvsim0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkselSel) -> u8 {
        Emvsim0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Emvsim1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkdivHalt {
        Emvsim1clkdivHalt::from_bits(val)
    }
}
impl From<Emvsim1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkdivHalt) -> u8 {
        Emvsim1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Emvsim1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkdivReset {
        Emvsim1clkdivReset::from_bits(val)
    }
}
impl From<Emvsim1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkdivReset) -> u8 {
        Emvsim1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Emvsim1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkdivUnstab {
        Emvsim1clkdivUnstab::from_bits(val)
    }
}
impl From<Emvsim1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkdivUnstab) -> u8 {
        Emvsim1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock0"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Emvsim1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkselSel {
        Emvsim1clkselSel::from_bits(val)
    }
}
impl From<Emvsim1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkselSel) -> u8 {
        Emvsim1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Enet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet {
    #[inline(always)]
    fn from(val: u8) -> Enet {
        Enet::from_bits(val)
    }
}
impl From<Enet> for u8 {
    #[inline(always)]
    fn from(val: Enet) -> u8 {
        Enet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl EnetRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetRst {
    #[inline(always)]
    fn from(val: u8) -> EnetRst {
        EnetRst::from_bits(val)
    }
}
impl From<EnetRst> for u8 {
    #[inline(always)]
    fn from(val: EnetRst) -> u8 {
        EnetRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl EnetptprefclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkdivHalt {
        EnetptprefclkdivHalt::from_bits(val)
    }
}
impl From<EnetptprefclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkdivHalt) -> u8 {
        EnetptprefclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl EnetptprefclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkdivReset {
        EnetptprefclkdivReset::from_bits(val)
    }
}
impl From<EnetptprefclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkdivReset) -> u8 {
        EnetptprefclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl EnetptprefclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkdivUnstab {
        EnetptprefclkdivUnstab::from_bits(val)
    }
}
impl From<EnetptprefclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkdivUnstab) -> u8 {
        EnetptprefclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "enet0_tx_clk clock"]
    ENUM4 = 0x04,
    #[doc = "pll1_clk1 clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl EnetptprefclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkselSel {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkselSel {
        EnetptprefclkselSel::from_bits(val)
    }
}
impl From<EnetptprefclkselSel> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkselSel) -> u8 {
        EnetptprefclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl EnetrmiiclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkdivHalt {
        EnetrmiiclkdivHalt::from_bits(val)
    }
}
impl From<EnetrmiiclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkdivHalt) -> u8 {
        EnetrmiiclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl EnetrmiiclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkdivReset {
        EnetrmiiclkdivReset::from_bits(val)
    }
}
impl From<EnetrmiiclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkdivReset) -> u8 {
        EnetrmiiclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl EnetrmiiclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkdivUnstab {
        EnetrmiiclkdivUnstab::from_bits(val)
    }
}
impl From<EnetrmiiclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkdivUnstab) -> u8 {
        EnetrmiiclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl EnetrmiiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkselSel {
        EnetrmiiclkselSel::from_bits(val)
    }
}
impl From<EnetrmiiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkselSel) -> u8 {
        EnetrmiiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erm {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Erm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erm {
    #[inline(always)]
    fn from(val: u8) -> Erm {
        Erm::from_bits(val)
    }
}
impl From<Erm> for u8 {
    #[inline(always)]
    fn from(val: Erm) -> u8 {
        Erm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventClrFlag {
    #[doc = "Event counter not cleared"]
    DISABLE = 0x0,
    #[doc = "Event counter cleared"]
    ENABLE = 0x01,
}
impl EventClrFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventClrFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventClrFlag {
    #[inline(always)]
    fn from(val: u8) -> EventClrFlag {
        EventClrFlag::from_bits(val)
    }
}
impl From<EventClrFlag> for u8 {
    #[inline(always)]
    fn from(val: EventClrFlag) -> u8 {
        EventClrFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evsim0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Evsim0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evsim0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evsim0 {
    #[inline(always)]
    fn from(val: u8) -> Evsim0 {
        Evsim0::from_bits(val)
    }
}
impl From<Evsim0> for u8 {
    #[inline(always)]
    fn from(val: Evsim0) -> u8 {
        Evsim0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evsim0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Evsim0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evsim0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evsim0Rst {
    #[inline(always)]
    fn from(val: u8) -> Evsim0Rst {
        Evsim0Rst::from_bits(val)
    }
}
impl From<Evsim0Rst> for u8 {
    #[inline(always)]
    fn from(val: Evsim0Rst) -> u8 {
        Evsim0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evsim1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Evsim1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evsim1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evsim1 {
    #[inline(always)]
    fn from(val: u8) -> Evsim1 {
        Evsim1::from_bits(val)
    }
}
impl From<Evsim1> for u8 {
    #[inline(always)]
    fn from(val: Evsim1) -> u8 {
        Evsim1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evsim1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Evsim1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evsim1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evsim1Rst {
    #[inline(always)]
    fn from(val: u8) -> Evsim1Rst {
        Evsim1Rst::from_bits(val)
    }
}
impl From<Evsim1Rst> for u8 {
    #[inline(always)]
    fn from(val: Evsim1Rst) -> u8 {
        Evsim1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evtg {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Evtg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evtg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evtg {
    #[inline(always)]
    fn from(val: u8) -> Evtg {
        Evtg::from_bits(val)
    }
}
impl From<Evtg> for u8 {
    #[inline(always)]
    fn from(val: Evtg) -> u8 {
        Evtg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ewm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm {
    #[inline(always)]
    fn from(val: u8) -> Ewm {
        Ewm::from_bits(val)
    }
}
impl From<Ewm> for u8 {
    #[inline(always)]
    fn from(val: Ewm) -> u8 {
        Ewm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0clkselSel {
    #[doc = "clk_16k\\[2\\]"]
    ENUM0 = 0x0,
    #[doc = "xtal32k\\[2\\]"]
    ENUM1 = 0x01,
}
impl Ewm0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0clkselSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Ewm0clkselSel {
        Ewm0clkselSel::from_bits(val)
    }
}
impl From<Ewm0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Ewm0clkselSel) -> u8 {
        Ewm0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EwmRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl EwmRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EwmRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EwmRst {
    #[inline(always)]
    fn from(val: u8) -> EwmRst {
        EwmRst::from_bits(val)
    }
}
impl From<EwmRst> for u8 {
    #[inline(always)]
    fn from(val: EwmRst) -> u8 {
        EwmRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0 {
    #[inline(always)]
    fn from(val: u8) -> Fc0 {
        Fc0::from_bits(val)
    }
}
impl From<Fc0> for u8 {
    #[inline(always)]
    fn from(val: Fc0) -> u8 {
        Fc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc0Rst {
        Fc0Rst::from_bits(val)
    }
}
impl From<Fc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc0Rst) -> u8 {
        Fc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1 {
    #[inline(always)]
    fn from(val: u8) -> Fc1 {
        Fc1::from_bits(val)
    }
}
impl From<Fc1> for u8 {
    #[inline(always)]
    fn from(val: Fc1) -> u8 {
        Fc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc1Rst {
        Fc1Rst::from_bits(val)
    }
}
impl From<Fc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc1Rst) -> u8 {
        Fc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2 {
    #[inline(always)]
    fn from(val: u8) -> Fc2 {
        Fc2::from_bits(val)
    }
}
impl From<Fc2> for u8 {
    #[inline(always)]
    fn from(val: Fc2) -> u8 {
        Fc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc2Rst {
        Fc2Rst::from_bits(val)
    }
}
impl From<Fc2Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc2Rst) -> u8 {
        Fc2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3 {
    #[inline(always)]
    fn from(val: u8) -> Fc3 {
        Fc3::from_bits(val)
    }
}
impl From<Fc3> for u8 {
    #[inline(always)]
    fn from(val: Fc3) -> u8 {
        Fc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc3Rst {
        Fc3Rst::from_bits(val)
    }
}
impl From<Fc3Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc3Rst) -> u8 {
        Fc3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4 {
    #[inline(always)]
    fn from(val: u8) -> Fc4 {
        Fc4::from_bits(val)
    }
}
impl From<Fc4> for u8 {
    #[inline(always)]
    fn from(val: Fc4) -> u8 {
        Fc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc4Rst {
        Fc4Rst::from_bits(val)
    }
}
impl From<Fc4Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc4Rst) -> u8 {
        Fc4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5 {
    #[inline(always)]
    fn from(val: u8) -> Fc5 {
        Fc5::from_bits(val)
    }
}
impl From<Fc5> for u8 {
    #[inline(always)]
    fn from(val: Fc5) -> u8 {
        Fc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc5Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc5Rst {
        Fc5Rst::from_bits(val)
    }
}
impl From<Fc5Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc5Rst) -> u8 {
        Fc5Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6 {
    #[inline(always)]
    fn from(val: u8) -> Fc6 {
        Fc6::from_bits(val)
    }
}
impl From<Fc6> for u8 {
    #[inline(always)]
    fn from(val: Fc6) -> u8 {
        Fc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc6Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc6Rst {
        Fc6Rst::from_bits(val)
    }
}
impl From<Fc6Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc6Rst) -> u8 {
        Fc6Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7 {
    #[inline(always)]
    fn from(val: u8) -> Fc7 {
        Fc7::from_bits(val)
    }
}
impl From<Fc7> for u8 {
    #[inline(always)]
    fn from(val: Fc7) -> u8 {
        Fc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc7Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc7Rst {
        Fc7Rst::from_bits(val)
    }
}
impl From<Fc7Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc7Rst) -> u8 {
        Fc7Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc8 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc8 {
    #[inline(always)]
    fn from(val: u8) -> Fc8 {
        Fc8::from_bits(val)
    }
}
impl From<Fc8> for u8 {
    #[inline(always)]
    fn from(val: Fc8) -> u8 {
        Fc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc8Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc8Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc8Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc8Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc8Rst {
        Fc8Rst::from_bits(val)
    }
}
impl From<Fc8Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc8Rst) -> u8 {
        Fc8Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc9 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fc9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc9 {
    #[inline(always)]
    fn from(val: u8) -> Fc9 {
        Fc9::from_bits(val)
    }
}
impl From<Fc9> for u8 {
    #[inline(always)]
    fn from(val: Fc9) -> u8 {
        Fc9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc9Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Fc9Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc9Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc9Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc9Rst {
        Fc9Rst::from_bits(val)
    }
}
impl From<Fc9Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc9Rst) -> u8 {
        Fc9Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL divided clock"]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 12 MHz clock"]
    ENUM_0X2 = 0x02,
    #[doc = "fro_hf_div clock"]
    ENUM_0X3 = 0x03,
    #[doc = "clk_1m clock"]
    ENUM_0X4 = 0x04,
    #[doc = "USB PLL clock"]
    ENUM_0X5 = 0x05,
    #[doc = "LP Oscillator clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl FcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcclkselSel {
        FcclkselSel::from_bits(val)
    }
}
impl From<FcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcclkselSel) -> u8 {
        FcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagAnaGlitchDetected {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagAnaGlitchDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagAnaGlitchDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagAnaGlitchDetected {
    #[inline(always)]
    fn from(val: u8) -> FlagAnaGlitchDetected {
        FlagAnaGlitchDetected::from_bits(val)
    }
}
impl From<FlagAnaGlitchDetected> for u8 {
    #[inline(always)]
    fn from(val: FlagAnaGlitchDetected) -> u8 {
        FlagAnaGlitchDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagApEnableCpu0 {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagApEnableCpu0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagApEnableCpu0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagApEnableCpu0 {
    #[inline(always)]
    fn from(val: u8) -> FlagApEnableCpu0 {
        FlagApEnableCpu0::from_bits(val)
    }
}
impl From<FlagApEnableCpu0> for u8 {
    #[inline(always)]
    fn from(val: FlagApEnableCpu0) -> u8 {
        FlagApEnableCpu0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagApEnableCpu1 {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagApEnableCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagApEnableCpu1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagApEnableCpu1 {
    #[inline(always)]
    fn from(val: u8) -> FlagApEnableCpu1 {
        FlagApEnableCpu1::from_bits(val)
    }
}
impl From<FlagApEnableCpu1> for u8 {
    #[inline(always)]
    fn from(val: FlagApEnableCpu1) -> u8 {
        FlagApEnableCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagApEnableDsp {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagApEnableDsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagApEnableDsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagApEnableDsp {
    #[inline(always)]
    fn from(val: u8) -> FlagApEnableDsp {
        FlagApEnableDsp::from_bits(val)
    }
}
impl From<FlagApEnableDsp> for u8 {
    #[inline(always)]
    fn from(val: FlagApEnableDsp) -> u8 {
        FlagApEnableDsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagClktamperDetIrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagClktamperDetIrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagClktamperDetIrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagClktamperDetIrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagClktamperDetIrqOccured {
        FlagClktamperDetIrqOccured::from_bits(val)
    }
}
impl From<FlagClktamperDetIrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagClktamperDetIrqOccured) -> u8 {
        FlagClktamperDetIrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagCpu0NsCAccOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagCpu0NsCAccOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagCpu0NsCAccOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagCpu0NsCAccOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagCpu0NsCAccOccured {
        FlagCpu0NsCAccOccured::from_bits(val)
    }
}
impl From<FlagCpu0NsCAccOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagCpu0NsCAccOccured) -> u8 {
        FlagCpu0NsCAccOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagCpu0NsDAccOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagCpu0NsDAccOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagCpu0NsDAccOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagCpu0NsDAccOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagCpu0NsDAccOccured {
        FlagCpu0NsDAccOccured::from_bits(val)
    }
}
impl From<FlagCpu0NsDAccOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagCpu0NsDAccOccured) -> u8 {
        FlagCpu0NsDAccOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagCwdt0IrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagCwdt0IrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagCwdt0IrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagCwdt0IrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagCwdt0IrqOccured {
        FlagCwdt0IrqOccured::from_bits(val)
    }
}
impl From<FlagCwdt0IrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagCwdt0IrqOccured) -> u8 {
        FlagCwdt0IrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagCwdt0ResetOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagCwdt0ResetOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagCwdt0ResetOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagCwdt0ResetOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagCwdt0ResetOccured {
        FlagCwdt0ResetOccured::from_bits(val)
    }
}
impl From<FlagCwdt0ResetOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagCwdt0ResetOccured) -> u8 {
        FlagCwdt0ResetOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagCwdt1IrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagCwdt1IrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagCwdt1IrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagCwdt1IrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagCwdt1IrqOccured {
        FlagCwdt1IrqOccured::from_bits(val)
    }
}
impl From<FlagCwdt1IrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagCwdt1IrqOccured) -> u8 {
        FlagCwdt1IrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagCwdt1ResetOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagCwdt1ResetOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagCwdt1ResetOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagCwdt1ResetOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagCwdt1ResetOccured {
        FlagCwdt1ResetOccured::from_bits(val)
    }
}
impl From<FlagCwdt1ResetOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagCwdt1ResetOccured) -> u8 {
        FlagCwdt1ResetOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagElsGlitchDetected {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagElsGlitchDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagElsGlitchDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagElsGlitchDetected {
    #[inline(always)]
    fn from(val: u8) -> FlagElsGlitchDetected {
        FlagElsGlitchDetected::from_bits(val)
    }
}
impl From<FlagElsGlitchDetected> for u8 {
    #[inline(always)]
    fn from(val: FlagElsGlitchDetected) -> u8 {
        FlagElsGlitchDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagFlashEccInvalid {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagFlashEccInvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagFlashEccInvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagFlashEccInvalid {
    #[inline(always)]
    fn from(val: u8) -> FlagFlashEccInvalid {
        FlagFlashEccInvalid::from_bits(val)
    }
}
impl From<FlagFlashEccInvalid> for u8 {
    #[inline(always)]
    fn from(val: FlagFlashEccInvalid) -> u8 {
        FlagFlashEccInvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagHvdCoreOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagHvdCoreOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagHvdCoreOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagHvdCoreOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagHvdCoreOccured {
        FlagHvdCoreOccured::from_bits(val)
    }
}
impl From<FlagHvdCoreOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagHvdCoreOccured) -> u8 {
        FlagHvdCoreOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagHvdVddioOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagHvdVddioOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagHvdVddioOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagHvdVddioOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagHvdVddioOccured {
        FlagHvdVddioOccured::from_bits(val)
    }
}
impl From<FlagHvdVddioOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagHvdVddioOccured) -> u8 {
        FlagHvdVddioOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagHvdVsysOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagHvdVsysOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagHvdVsysOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagHvdVsysOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagHvdVsysOccured {
        FlagHvdVsysOccured::from_bits(val)
    }
}
impl From<FlagHvdVsysOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagHvdVsysOccured) -> u8 {
        FlagHvdVsysOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagLhttamperDetIrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagLhttamperDetIrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagLhttamperDetIrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagLhttamperDetIrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagLhttamperDetIrqOccured {
        FlagLhttamperDetIrqOccured::from_bits(val)
    }
}
impl From<FlagLhttamperDetIrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagLhttamperDetIrqOccured) -> u8 {
        FlagLhttamperDetIrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagLvdCoreOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagLvdCoreOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagLvdCoreOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagLvdCoreOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagLvdCoreOccured {
        FlagLvdCoreOccured::from_bits(val)
    }
}
impl From<FlagLvdCoreOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagLvdCoreOccured) -> u8 {
        FlagLvdCoreOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagLvdVddioOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagLvdVddioOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagLvdVddioOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagLvdVddioOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagLvdVddioOccured {
        FlagLvdVddioOccured::from_bits(val)
    }
}
impl From<FlagLvdVddioOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagLvdVddioOccured) -> u8 {
        FlagLvdVddioOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagLvdVsysOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagLvdVsysOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagLvdVsysOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagLvdVsysOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagLvdVsysOccured {
        FlagLvdVsysOccured::from_bits(val)
    }
}
impl From<FlagLvdVsysOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagLvdVsysOccured) -> u8 {
        FlagLvdVsysOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagQkError {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagQkError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagQkError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagQkError {
    #[inline(always)]
    fn from(val: u8) -> FlagQkError {
        FlagQkError::from_bits(val)
    }
}
impl From<FlagQkError> for u8 {
    #[inline(always)]
    fn from(val: FlagQkError) -> u8 {
        FlagQkError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagSecViolIrqOcurred {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagSecViolIrqOcurred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagSecViolIrqOcurred {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagSecViolIrqOcurred {
    #[inline(always)]
    fn from(val: u8) -> FlagSecViolIrqOcurred {
        FlagSecViolIrqOcurred::from_bits(val)
    }
}
impl From<FlagSecViolIrqOcurred> for u8 {
    #[inline(always)]
    fn from(val: FlagSecViolIrqOcurred) -> u8 {
        FlagSecViolIrqOcurred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagTamperEventDetected {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagTamperEventDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagTamperEventDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagTamperEventDetected {
    #[inline(always)]
    fn from(val: u8) -> FlagTamperEventDetected {
        FlagTamperEventDetected::from_bits(val)
    }
}
impl From<FlagTamperEventDetected> for u8 {
    #[inline(always)]
    fn from(val: FlagTamperEventDetected) -> u8 {
        FlagTamperEventDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagTemptamperDetIrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagTemptamperDetIrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagTemptamperDetIrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagTemptamperDetIrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagTemptamperDetIrqOccured {
        FlagTemptamperDetIrqOccured::from_bits(val)
    }
}
impl From<FlagTemptamperDetIrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagTemptamperDetIrqOccured) -> u8 {
        FlagTemptamperDetIrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagVoltamperDetIrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagVoltamperDetIrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagVoltamperDetIrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagVoltamperDetIrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagVoltamperDetIrqOccured {
        FlagVoltamperDetIrqOccured::from_bits(val)
    }
}
impl From<FlagVoltamperDetIrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagVoltamperDetIrqOccured) -> u8 {
        FlagVoltamperDetIrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagWdt0IrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagWdt0IrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagWdt0IrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagWdt0IrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagWdt0IrqOccured {
        FlagWdt0IrqOccured::from_bits(val)
    }
}
impl From<FlagWdt0IrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagWdt0IrqOccured) -> u8 {
        FlagWdt0IrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagWdt0ResetOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagWdt0ResetOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagWdt0ResetOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagWdt0ResetOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagWdt0ResetOccured {
        FlagWdt0ResetOccured::from_bits(val)
    }
}
impl From<FlagWdt0ResetOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagWdt0ResetOccured) -> u8 {
        FlagWdt0ResetOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagWdt1IrqOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagWdt1IrqOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagWdt1IrqOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagWdt1IrqOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagWdt1IrqOccured {
        FlagWdt1IrqOccured::from_bits(val)
    }
}
impl From<FlagWdt1IrqOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagWdt1IrqOccured) -> u8 {
        FlagWdt1IrqOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlagWdt1ResetOccured {
    #[doc = "Not Triggered"]
    DISABLE = 0x0,
    #[doc = "Triggered"]
    ENABLE = 0x01,
}
impl FlagWdt1ResetOccured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlagWdt1ResetOccured {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlagWdt1ResetOccured {
    #[inline(always)]
    fn from(val: u8) -> FlagWdt1ResetOccured {
        FlagWdt1ResetOccured::from_bits(val)
    }
}
impl From<FlagWdt1ResetOccured> for u8 {
    #[inline(always)]
    fn from(val: FlagWdt1ResetOccured) -> u8 {
        FlagWdt1ResetOccured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy"]
    ENABLE = 0x0,
    #[doc = "Stall on FLASH busy"]
    DISABLE = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl FlexSpiclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkdivHalt {
        FlexSpiclkdivHalt::from_bits(val)
    }
}
impl From<FlexSpiclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkdivHalt) -> u8 {
        FlexSpiclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl FlexSpiclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkdivReset {
        FlexSpiclkdivReset::from_bits(val)
    }
}
impl From<FlexSpiclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkdivReset) -> u8 {
        FlexSpiclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl FlexSpiclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkdivUnstab {
        FlexSpiclkdivUnstab::from_bits(val)
    }
}
impl From<FlexSpiclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkdivUnstab) -> u8 {
        FlexSpiclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "pll1_clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
    #[doc = "No clock"]
    ENUM8 = 0x08,
    #[doc = "No clock"]
    ENUM9 = 0x09,
    #[doc = "No clock"]
    ENUM10 = 0x0a,
    #[doc = "No clock"]
    ENUM11 = 0x0b,
    #[doc = "No clock"]
    ENUM12 = 0x0c,
    #[doc = "No clock"]
    ENUM13 = 0x0d,
    #[doc = "No clock"]
    ENUM14 = 0x0e,
    #[doc = "No clock"]
    ENUM15 = 0x0f,
}
impl FlexSpiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkselSel {
        FlexSpiclkselSel::from_bits(val)
    }
}
impl From<FlexSpiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkselSel) -> u8 {
        FlexSpiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Flexcan0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0 {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0 {
        Flexcan0::from_bits(val)
    }
}
impl From<Flexcan0> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0) -> u8 {
        Flexcan0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Flexcan0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0Rst {
        Flexcan0Rst::from_bits(val)
    }
}
impl From<Flexcan0Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0Rst) -> u8 {
        Flexcan0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Flexcan0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivHalt {
        Flexcan0clkdivHalt::from_bits(val)
    }
}
impl From<Flexcan0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivHalt) -> u8 {
        Flexcan0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Flexcan0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivReset {
        Flexcan0clkdivReset::from_bits(val)
    }
}
impl From<Flexcan0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivReset) -> u8 {
        Flexcan0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Flexcan0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivUnstab {
        Flexcan0clkdivUnstab::from_bits(val)
    }
}
impl From<Flexcan0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivUnstab) -> u8 {
        Flexcan0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Flexcan0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkselSel {
        Flexcan0clkselSel::from_bits(val)
    }
}
impl From<Flexcan0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkselSel) -> u8 {
        Flexcan0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Flexcan1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1 {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1 {
        Flexcan1::from_bits(val)
    }
}
impl From<Flexcan1> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1) -> u8 {
        Flexcan1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Flexcan1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1Rst {
        Flexcan1Rst::from_bits(val)
    }
}
impl From<Flexcan1Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1Rst) -> u8 {
        Flexcan1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Flexcan1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivHalt {
        Flexcan1clkdivHalt::from_bits(val)
    }
}
impl From<Flexcan1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivHalt) -> u8 {
        Flexcan1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Flexcan1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivReset {
        Flexcan1clkdivReset::from_bits(val)
    }
}
impl From<Flexcan1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivReset) -> u8 {
        Flexcan1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Flexcan1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivUnstab {
        Flexcan1clkdivUnstab::from_bits(val)
    }
}
impl From<Flexcan1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivUnstab) -> u8 {
        Flexcan1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Flexcan1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkselSel {
        Flexcan1clkselSel::from_bits(val)
    }
}
impl From<Flexcan1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkselSel) -> u8 {
        Flexcan1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl FlexcommclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivHalt {
        FlexcommclkdivHalt::from_bits(val)
    }
}
impl From<FlexcommclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivHalt) -> u8 {
        FlexcommclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl FlexcommclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivReset {
        FlexcommclkdivReset::from_bits(val)
    }
}
impl From<FlexcommclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivReset) -> u8 {
        FlexcommclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl FlexcommclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivUnstab {
        FlexcommclkdivUnstab::from_bits(val)
    }
}
impl From<FlexcommclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivUnstab) -> u8 {
        FlexcommclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enable clock"]
    ENABLE = 0x01,
}
impl Flexio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio {
    #[inline(always)]
    fn from(val: u8) -> Flexio {
        Flexio::from_bits(val)
    }
}
impl From<Flexio> for u8 {
    #[inline(always)]
    fn from(val: Flexio) -> u8 {
        Flexio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl FlexioRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioRst {
    #[inline(always)]
    fn from(val: u8) -> FlexioRst {
        FlexioRst::from_bits(val)
    }
}
impl From<FlexioRst> for u8 {
    #[inline(always)]
    fn from(val: FlexioRst) -> u8 {
        FlexioRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl FlexioclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivHalt {
        FlexioclkdivHalt::from_bits(val)
    }
}
impl From<FlexioclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivHalt) -> u8 {
        FlexioclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl FlexioclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivReset {
        FlexioclkdivReset::from_bits(val)
    }
}
impl From<FlexioclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivReset) -> u8 {
        FlexioclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl FlexioclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivUnstab {
        FlexioclkdivUnstab::from_bits(val)
    }
}
impl From<FlexioclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivUnstab) -> u8 {
        FlexioclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl FlexioclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkselSel {
        FlexioclkselSel::from_bits(val)
    }
}
impl From<FlexioclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkselSel) -> u8 {
        FlexioclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Flexspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi {
    #[inline(always)]
    fn from(val: u8) -> Flexspi {
        Flexspi::from_bits(val)
    }
}
impl From<Flexspi> for u8 {
    #[inline(always)]
    fn from(val: Flexspi) -> u8 {
        Flexspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl FlexspiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiRst {
    #[inline(always)]
    fn from(val: u8) -> FlexspiRst {
        FlexspiRst::from_bits(val)
    }
}
impl From<FlexspiRst> for u8 {
    #[inline(always)]
    fn from(val: FlexspiRst) -> u8 {
        FlexspiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmc {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmc {
    #[inline(always)]
    fn from(val: u8) -> Fmc {
        Fmc::from_bits(val)
    }
}
impl From<Fmc> for u8 {
    #[inline(always)]
    fn from(val: Fmc) -> u8 {
        Fmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmu {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Fmu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmu {
    #[inline(always)]
    fn from(val: u8) -> Fmu {
        Fmu::from_bits(val)
    }
}
impl From<Fmu> for u8 {
    #[inline(always)]
    fn from(val: Fmu) -> u8 {
        Fmu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl FmuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuRst {
    #[inline(always)]
    fn from(val: u8) -> FmuRst {
        FmuRst::from_bits(val)
    }
}
impl From<FmuRst> for u8 {
    #[inline(always)]
    fn from(val: FmuRst) -> u8 {
        FmuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation"]
    ENABLE = 0x0,
    #[doc = "Forces no allocation"]
    DISABLE = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freqme {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Freqme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freqme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freqme {
    #[inline(always)]
    fn from(val: u8) -> Freqme {
        Freqme::from_bits(val)
    }
}
impl From<Freqme> for u8 {
    #[inline(always)]
    fn from(val: Freqme) -> u8 {
        Freqme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl FreqmeRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRst {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRst {
        FreqmeRst::from_bits(val)
    }
}
impl From<FreqmeRst> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRst) -> u8 {
        FreqmeRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro12mhzEna {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl Fro12mhzEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro12mhzEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro12mhzEna {
    #[inline(always)]
    fn from(val: u8) -> Fro12mhzEna {
        Fro12mhzEna::from_bits(val)
    }
}
impl From<Fro12mhzEna> for u8 {
    #[inline(always)]
    fn from(val: Fro12mhzEna) -> u8 {
        Fro12mhzEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mhzClkEna {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl Fro1mhzClkEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mhzClkEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mhzClkEna {
    #[inline(always)]
    fn from(val: u8) -> Fro1mhzClkEna {
        Fro1mhzClkEna::from_bits(val)
    }
}
impl From<Fro1mhzClkEna> for u8 {
    #[inline(always)]
    fn from(val: Fro1mhzClkEna) -> u8 {
        Fro1mhzClkEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mhzEna {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl Fro1mhzEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mhzEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mhzEna {
    #[inline(always)]
    fn from(val: u8) -> Fro1mhzEna {
        Fro1mhzEna::from_bits(val)
    }
}
impl From<Fro1mhzEna> for u8 {
    #[inline(always)]
    fn from(val: Fro1mhzEna) -> u8 {
        Fro1mhzEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FroHfEna {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl FroHfEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FroHfEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FroHfEna {
    #[inline(always)]
    fn from(val: u8) -> FroHfEna {
        FroHfEna::from_bits(val)
    }
}
impl From<FroHfEna> for u8 {
    #[inline(always)]
    fn from(val: FroHfEna) -> u8 {
        FroHfEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running, this bit is set to 0 when the register is written."]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl FrohfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivUnstab {
        FrohfdivUnstab::from_bits(val)
    }
}
impl From<FrohfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivUnstab) -> u8 {
        FrohfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gdet {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Gdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gdet {
    #[inline(always)]
    fn from(val: u8) -> Gdet {
        Gdet::from_bits(val)
    }
}
impl From<Gdet> for u8 {
    #[inline(always)]
    fn from(val: Gdet) -> u8 {
        Gdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetErrClr {
    #[doc = "Error status not cleared"]
    DISABLE = 0x0,
    #[doc = "Clears error status"]
    ENABLE = 0x01,
}
impl GdetErrClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetErrClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetErrClr {
    #[inline(always)]
    fn from(val: u8) -> GdetErrClr {
        GdetErrClr::from_bits(val)
    }
}
impl From<GdetErrClr> for u8 {
    #[inline(always)]
    fn from(val: GdetErrClr) -> u8 {
        GdetErrClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetEvtcntClr {
    #[doc = "Event counter not cleared"]
    DISABLE = 0x0,
    #[doc = "Clears event counter"]
    ENABLE = 0x01,
}
impl GdetEvtcntClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetEvtcntClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetEvtcntClr {
    #[inline(always)]
    fn from(val: u8) -> GdetEvtcntClr {
        GdetEvtcntClr::from_bits(val)
    }
}
impl From<GdetEvtcntClr> for u8 {
    #[inline(always)]
    fn from(val: GdetEvtcntClr) -> u8 {
        GdetEvtcntClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetIsoSw {
    #[doc = "Isolation is disabled"]
    DISABLE0 = 0x0,
    #[doc = "Isolation is disabled"]
    DISABLE1 = 0x01,
    #[doc = "Isolation is enabled. When both GDET0_CTRL/GDET1_CTRL GDET_ISO_SW are \"10\", isolation_on is asserted."]
    ENABLE = 0x02,
    #[doc = "Isolation is disabled"]
    DISABLE3 = 0x03,
}
impl GdetIsoSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetIsoSw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetIsoSw {
    #[inline(always)]
    fn from(val: u8) -> GdetIsoSw {
        GdetIsoSw::from_bits(val)
    }
}
impl From<GdetIsoSw> for u8 {
    #[inline(always)]
    fn from(val: GdetIsoSw) -> u8 {
        GdetIsoSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetRefclkEn {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl GdetRefclkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetRefclkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetRefclkEn {
    #[inline(always)]
    fn from(val: u8) -> GdetRefclkEn {
        GdetRefclkEn::from_bits(val)
    }
}
impl From<GdetRefclkEn> for u8 {
    #[inline(always)]
    fn from(val: GdetRefclkEn) -> u8 {
        GdetRefclkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetRefclkEnClr {
    #[doc = "No effect."]
    DISABLE = 0x0,
    #[doc = "Set to 0"]
    ENABLE = 0x01,
}
impl GdetRefclkEnClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetRefclkEnClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetRefclkEnClr {
    #[inline(always)]
    fn from(val: u8) -> GdetRefclkEnClr {
        GdetRefclkEnClr::from_bits(val)
    }
}
impl From<GdetRefclkEnClr> for u8 {
    #[inline(always)]
    fn from(val: GdetRefclkEnClr) -> u8 {
        GdetRefclkEnClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetRefclkEnSet {
    #[doc = "No effect."]
    DISABLE = 0x0,
    #[doc = "Set to 1"]
    ENABLE = 0x01,
}
impl GdetRefclkEnSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetRefclkEnSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetRefclkEnSet {
    #[inline(always)]
    fn from(val: u8) -> GdetRefclkEnSet {
        GdetRefclkEnSet::from_bits(val)
    }
}
impl From<GdetRefclkEnSet> for u8 {
    #[inline(always)]
    fn from(val: GdetRefclkEnSet) -> u8 {
        GdetRefclkEnSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Gpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0 {
        Gpio0::from_bits(val)
    }
}
impl From<Gpio0> for u8 {
    #[inline(always)]
    fn from(val: Gpio0) -> u8 {
        Gpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Gpio0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Rst {
        Gpio0Rst::from_bits(val)
    }
}
impl From<Gpio0Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Rst) -> u8 {
        Gpio0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Gpio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1 {
        Gpio1::from_bits(val)
    }
}
impl From<Gpio1> for u8 {
    #[inline(always)]
    fn from(val: Gpio1) -> u8 {
        Gpio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Gpio1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Rst {
        Gpio1Rst::from_bits(val)
    }
}
impl From<Gpio1Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Rst) -> u8 {
        Gpio1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Gpio2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2 {
        Gpio2::from_bits(val)
    }
}
impl From<Gpio2> for u8 {
    #[inline(always)]
    fn from(val: Gpio2) -> u8 {
        Gpio2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Gpio2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Rst {
        Gpio2Rst::from_bits(val)
    }
}
impl From<Gpio2Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Rst) -> u8 {
        Gpio2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Gpio3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3 {
        Gpio3::from_bits(val)
    }
}
impl From<Gpio3> for u8 {
    #[inline(always)]
    fn from(val: Gpio3) -> u8 {
        Gpio3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Gpio3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Rst {
        Gpio3Rst::from_bits(val)
    }
}
impl From<Gpio3Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Rst) -> u8 {
        Gpio3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Gpio4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4 {
        Gpio4::from_bits(val)
    }
}
impl From<Gpio4> for u8 {
    #[inline(always)]
    fn from(val: Gpio4) -> u8 {
        Gpio4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Gpio4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Rst {
        Gpio4Rst::from_bits(val)
    }
}
impl From<Gpio4Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Rst) -> u8 {
        Gpio4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl I3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0 {
    #[inline(always)]
    fn from(val: u8) -> I3c0 {
        I3c0::from_bits(val)
    }
}
impl From<I3c0> for u8 {
    #[inline(always)]
    fn from(val: I3c0) -> u8 {
        I3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl I3c0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c0Rst {
        I3c0Rst::from_bits(val)
    }
}
impl From<I3c0Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c0Rst) -> u8 {
        I3c0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl I3c0fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivHalt {
        I3c0fclkdivHalt::from_bits(val)
    }
}
impl From<I3c0fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivHalt) -> u8 {
        I3c0fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl I3c0fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivReset {
        I3c0fclkdivReset::from_bits(val)
    }
}
impl From<I3c0fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivReset) -> u8 {
        I3c0fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl I3c0fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivUnstab {
        I3c0fclkdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivUnstab) -> u8 {
        I3c0fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl I3c0fclksdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksdivHalt {
        I3c0fclksdivHalt::from_bits(val)
    }
}
impl From<I3c0fclksdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksdivHalt) -> u8 {
        I3c0fclksdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl I3c0fclksdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksdivReset {
        I3c0fclksdivReset::from_bits(val)
    }
}
impl From<I3c0fclksdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksdivReset) -> u8 {
        I3c0fclksdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl I3c0fclksdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksdivUnstab {
        I3c0fclksdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclksdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksdivUnstab) -> u8 {
        I3c0fclksdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkselSel {
        I3c0fclkselSel::from_bits(val)
    }
}
impl From<I3c0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkselSel) -> u8 {
        I3c0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksselSel {
    #[doc = "FRO_1M clock"]
    ENUM0 = 0x0,
    #[doc = "No clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c0fclksselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksselSel {
        I3c0fclksselSel::from_bits(val)
    }
}
impl From<I3c0fclksselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksselSel) -> u8 {
        I3c0fclksselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl I3c0fclkstcdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcdivHalt {
        I3c0fclkstcdivHalt::from_bits(val)
    }
}
impl From<I3c0fclkstcdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcdivHalt) -> u8 {
        I3c0fclkstcdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl I3c0fclkstcdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcdivReset {
        I3c0fclkstcdivReset::from_bits(val)
    }
}
impl From<I3c0fclkstcdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcdivReset) -> u8 {
        I3c0fclkstcdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl I3c0fclkstcdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcdivUnstab {
        I3c0fclkstcdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclkstcdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcdivUnstab) -> u8 {
        I3c0fclkstcdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcselSel {
    #[doc = "I3C0 functional clock I3C0FCLK"]
    ENUM0 = 0x0,
    #[doc = "FRO_1M clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c0fclkstcselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcselSel {
        I3c0fclkstcselSel::from_bits(val)
    }
}
impl From<I3c0fclkstcselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcselSel) -> u8 {
        I3c0fclkstcselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1 {
    #[inline(always)]
    fn from(val: u8) -> I3c1 {
        I3c1::from_bits(val)
    }
}
impl From<I3c1> for u8 {
    #[inline(always)]
    fn from(val: I3c1) -> u8 {
        I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl I3c1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c1Rst {
        I3c1Rst::from_bits(val)
    }
}
impl From<I3c1Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c1Rst) -> u8 {
        I3c1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl I3c1fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivHalt {
        I3c1fclkdivHalt::from_bits(val)
    }
}
impl From<I3c1fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivHalt) -> u8 {
        I3c1fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl I3c1fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivReset {
        I3c1fclkdivReset::from_bits(val)
    }
}
impl From<I3c1fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivReset) -> u8 {
        I3c1fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl I3c1fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivUnstab {
        I3c1fclkdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivUnstab) -> u8 {
        I3c1fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl I3c1fclksdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksdivHalt {
        I3c1fclksdivHalt::from_bits(val)
    }
}
impl From<I3c1fclksdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksdivHalt) -> u8 {
        I3c1fclksdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl I3c1fclksdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksdivReset {
        I3c1fclksdivReset::from_bits(val)
    }
}
impl From<I3c1fclksdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksdivReset) -> u8 {
        I3c1fclksdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl I3c1fclksdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksdivUnstab {
        I3c1fclksdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclksdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksdivUnstab) -> u8 {
        I3c1fclksdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkselSel {
        I3c1fclkselSel::from_bits(val)
    }
}
impl From<I3c1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkselSel) -> u8 {
        I3c1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksselSel {
    #[doc = "FRO_1M clock"]
    ENUM0 = 0x0,
    #[doc = "No clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c1fclksselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksselSel {
        I3c1fclksselSel::from_bits(val)
    }
}
impl From<I3c1fclksselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksselSel) -> u8 {
        I3c1fclksselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl I3c1fclkstcdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcdivHalt {
        I3c1fclkstcdivHalt::from_bits(val)
    }
}
impl From<I3c1fclkstcdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcdivHalt) -> u8 {
        I3c1fclkstcdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl I3c1fclkstcdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcdivReset {
        I3c1fclkstcdivReset::from_bits(val)
    }
}
impl From<I3c1fclkstcdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcdivReset) -> u8 {
        I3c1fclkstcdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl I3c1fclkstcdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcdivUnstab {
        I3c1fclkstcdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclkstcdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcdivUnstab) -> u8 {
        I3c1fclkstcdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcselSel {
    #[doc = "I3C1 functional clock I3C1FCLK"]
    ENUM0 = 0x0,
    #[doc = "FRO_1M clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c1fclkstcselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcselSel {
        I3c1fclkstcselSel::from_bits(val)
    }
}
impl From<I3c1fclkstcselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcselSel) -> u8 {
        I3c1fclkstcselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int0 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int0 {
    #[inline(always)]
    fn from(val: u8) -> Int0 {
        Int0::from_bits(val)
    }
}
impl From<Int0> for u8 {
    #[inline(always)]
    fn from(val: Int0) -> u8 {
        Int0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int1 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int1 {
    #[inline(always)]
    fn from(val: u8) -> Int1 {
        Int1::from_bits(val)
    }
}
impl From<Int1> for u8 {
    #[inline(always)]
    fn from(val: Int1) -> u8 {
        Int1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int10 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int10 {
    #[inline(always)]
    fn from(val: u8) -> Int10 {
        Int10::from_bits(val)
    }
}
impl From<Int10> for u8 {
    #[inline(always)]
    fn from(val: Int10) -> u8 {
        Int10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int11 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int11 {
    #[inline(always)]
    fn from(val: u8) -> Int11 {
        Int11::from_bits(val)
    }
}
impl From<Int11> for u8 {
    #[inline(always)]
    fn from(val: Int11) -> u8 {
        Int11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int12 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int12 {
    #[inline(always)]
    fn from(val: u8) -> Int12 {
        Int12::from_bits(val)
    }
}
impl From<Int12> for u8 {
    #[inline(always)]
    fn from(val: Int12) -> u8 {
        Int12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int13 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int13 {
    #[inline(always)]
    fn from(val: u8) -> Int13 {
        Int13::from_bits(val)
    }
}
impl From<Int13> for u8 {
    #[inline(always)]
    fn from(val: Int13) -> u8 {
        Int13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int14 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int14 {
    #[inline(always)]
    fn from(val: u8) -> Int14 {
        Int14::from_bits(val)
    }
}
impl From<Int14> for u8 {
    #[inline(always)]
    fn from(val: Int14) -> u8 {
        Int14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int15 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int15 {
    #[inline(always)]
    fn from(val: u8) -> Int15 {
        Int15::from_bits(val)
    }
}
impl From<Int15> for u8 {
    #[inline(always)]
    fn from(val: Int15) -> u8 {
        Int15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int16 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int16 {
    #[inline(always)]
    fn from(val: u8) -> Int16 {
        Int16::from_bits(val)
    }
}
impl From<Int16> for u8 {
    #[inline(always)]
    fn from(val: Int16) -> u8 {
        Int16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int17 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int17 {
    #[inline(always)]
    fn from(val: u8) -> Int17 {
        Int17::from_bits(val)
    }
}
impl From<Int17> for u8 {
    #[inline(always)]
    fn from(val: Int17) -> u8 {
        Int17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int18 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int18 {
    #[inline(always)]
    fn from(val: u8) -> Int18 {
        Int18::from_bits(val)
    }
}
impl From<Int18> for u8 {
    #[inline(always)]
    fn from(val: Int18) -> u8 {
        Int18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int19 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int19 {
    #[inline(always)]
    fn from(val: u8) -> Int19 {
        Int19::from_bits(val)
    }
}
impl From<Int19> for u8 {
    #[inline(always)]
    fn from(val: Int19) -> u8 {
        Int19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int2 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int2 {
    #[inline(always)]
    fn from(val: u8) -> Int2 {
        Int2::from_bits(val)
    }
}
impl From<Int2> for u8 {
    #[inline(always)]
    fn from(val: Int2) -> u8 {
        Int2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int20 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int20 {
    #[inline(always)]
    fn from(val: u8) -> Int20 {
        Int20::from_bits(val)
    }
}
impl From<Int20> for u8 {
    #[inline(always)]
    fn from(val: Int20) -> u8 {
        Int20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int21 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int21 {
    #[inline(always)]
    fn from(val: u8) -> Int21 {
        Int21::from_bits(val)
    }
}
impl From<Int21> for u8 {
    #[inline(always)]
    fn from(val: Int21) -> u8 {
        Int21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int22 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int22 {
    #[inline(always)]
    fn from(val: u8) -> Int22 {
        Int22::from_bits(val)
    }
}
impl From<Int22> for u8 {
    #[inline(always)]
    fn from(val: Int22) -> u8 {
        Int22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int23 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int23 {
    #[inline(always)]
    fn from(val: u8) -> Int23 {
        Int23::from_bits(val)
    }
}
impl From<Int23> for u8 {
    #[inline(always)]
    fn from(val: Int23) -> u8 {
        Int23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int3 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int3 {
    #[inline(always)]
    fn from(val: u8) -> Int3 {
        Int3::from_bits(val)
    }
}
impl From<Int3> for u8 {
    #[inline(always)]
    fn from(val: Int3) -> u8 {
        Int3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int4 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int4 {
    #[inline(always)]
    fn from(val: u8) -> Int4 {
        Int4::from_bits(val)
    }
}
impl From<Int4> for u8 {
    #[inline(always)]
    fn from(val: Int4) -> u8 {
        Int4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int5 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int5 {
    #[inline(always)]
    fn from(val: u8) -> Int5 {
        Int5::from_bits(val)
    }
}
impl From<Int5> for u8 {
    #[inline(always)]
    fn from(val: Int5) -> u8 {
        Int5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int6 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int6 {
    #[inline(always)]
    fn from(val: u8) -> Int6 {
        Int6::from_bits(val)
    }
}
impl From<Int6> for u8 {
    #[inline(always)]
    fn from(val: Int6) -> u8 {
        Int6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int7 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int7 {
    #[inline(always)]
    fn from(val: u8) -> Int7 {
        Int7::from_bits(val)
    }
}
impl From<Int7> for u8 {
    #[inline(always)]
    fn from(val: Int7) -> u8 {
        Int7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int8 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int8 {
    #[inline(always)]
    fn from(val: u8) -> Int8 {
        Int8::from_bits(val)
    }
}
impl From<Int8> for u8 {
    #[inline(always)]
    fn from(val: Int8) -> u8 {
        Int8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int9 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Int9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int9 {
    #[inline(always)]
    fn from(val: u8) -> Int9 {
        Int9::from_bits(val)
    }
}
impl From<Int9> for u8 {
    #[inline(always)]
    fn from(val: Int9) -> u8 {
        Int9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is consecutive."]
    NORMAL = 0x0,
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is interleaved. This setting is need for PKC L0 memory access."]
    INTERLEAVE = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intm {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Intm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intm {
    #[inline(always)]
    fn from(val: u8) -> Intm {
        Intm::from_bits(val)
    }
}
impl From<Intm> for u8 {
    #[inline(always)]
    fn from(val: Intm) -> u8 {
        Intm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irq {
    #[doc = "ETB interrupt is not asserted"]
    DISABLE = 0x0,
    #[doc = "ETB interrupt is asserted when ETB count expires. Write 1 to clear it."]
    ENABLE = 0x01,
}
impl Irq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irq {
    #[inline(always)]
    fn from(val: u8) -> Irq {
        Irq::from_bits(val)
    }
}
impl From<Irq> for u8 {
    #[inline(always)]
    fn from(val: Irq) -> u8 {
        Irq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyLoad {
    #[doc = "Key load sequence is disabled."]
    DISABLE = 0x0,
    #[doc = "Key load sequence is enabled."]
    ENABLE = 0x01,
}
impl KeyLoad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyLoad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyLoad {
    #[inline(always)]
    fn from(val: u8) -> KeyLoad {
        KeyLoad::from_bits(val)
    }
}
impl From<KeyLoad> for u8 {
    #[inline(always)]
    fn from(val: KeyLoad) -> u8 {
        KeyLoad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyRetainDone {
    #[doc = "Key save / load sequence has not completed."]
    NOT_DONE = 0x0,
    #[doc = "Key save / load sequence has completed."]
    DONE = 0x01,
}
impl KeyRetainDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyRetainDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyRetainDone {
    #[inline(always)]
    fn from(val: u8) -> KeyRetainDone {
        KeyRetainDone::from_bits(val)
    }
}
impl From<KeyRetainDone> for u8 {
    #[inline(always)]
    fn from(val: KeyRetainDone) -> u8 {
        KeyRetainDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyRetainValid {
    #[doc = "PUF key is not retained in VBAT domain."]
    NOT_VALID = 0x0,
    #[doc = "PUF key is retained in VBAT domain."]
    VALID = 0x01,
}
impl KeyRetainValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyRetainValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyRetainValid {
    #[inline(always)]
    fn from(val: u8) -> KeyRetainValid {
        KeyRetainValid::from_bits(val)
    }
}
impl From<KeyRetainValid> for u8 {
    #[inline(always)]
    fn from(val: KeyRetainValid) -> u8 {
        KeyRetainValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeySave {
    #[doc = "Key save sequence is disabled."]
    DISABLE = 0x0,
    #[doc = "Key save sequence is enabled."]
    ENABLE = 0x01,
}
impl KeySave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeySave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeySave {
    #[inline(always)]
    fn from(val: u8) -> KeySave {
        KeySave::from_bits(val)
    }
}
impl From<KeySave> for u8 {
    #[inline(always)]
    fn from(val: KeySave) -> u8 {
        KeySave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LimLpcacWtbf {
    #[doc = "Write buffer enabled when transaction is bufferable."]
    DISABLE = 0x0,
    #[doc = "Write buffer enabled when transaction is cacheable and bufferable"]
    ENABLE = 0x01,
}
impl LimLpcacWtbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LimLpcacWtbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LimLpcacWtbf {
    #[inline(always)]
    fn from(val: u8) -> LimLpcacWtbf {
        LimLpcacWtbf::from_bits(val)
    }
}
impl From<LimLpcacWtbf> for u8 {
    #[inline(always)]
    fn from(val: LimLpcacWtbf) -> u8 {
        LimLpcacWtbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disables write access to all registers"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enables write access to all registers"]
    ENABLE = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcacXom {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Enabled."]
    ENABLE = 0x01,
}
impl LpcacXom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcacXom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcacXom {
    #[inline(always)]
    fn from(val: u8) -> LpcacXom {
        LpcacXom::from_bits(val)
    }
}
impl From<LpcacXom> for u8 {
    #[inline(always)]
    fn from(val: LpcacXom) -> u8 {
        LpcacXom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mailbox {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Mailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mailbox {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mailbox {
    #[inline(always)]
    fn from(val: u8) -> Mailbox {
        Mailbox::from_bits(val)
    }
}
impl From<Mailbox> for u8 {
    #[inline(always)]
    fn from(val: Mailbox) -> u8 {
        Mailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MailboxRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl MailboxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MailboxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MailboxRst {
    #[inline(always)]
    fn from(val: u8) -> MailboxRst {
        MailboxRst::from_bits(val)
    }
}
impl From<MailboxRst> for u8 {
    #[inline(always)]
    fn from(val: MailboxRst) -> u8 {
        MailboxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Micfil {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Micfil {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Micfil {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Micfil {
    #[inline(always)]
    fn from(val: u8) -> Micfil {
        Micfil::from_bits(val)
    }
}
impl From<Micfil> for u8 {
    #[inline(always)]
    fn from(val: Micfil) -> u8 {
        Micfil::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl MicfilRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilRst {
    #[inline(always)]
    fn from(val: u8) -> MicfilRst {
        MicfilRst::from_bits(val)
    }
}
impl From<MicfilRst> for u8 {
    #[inline(always)]
    fn from(val: MicfilRst) -> u8 {
        MicfilRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl MicfilfclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivHalt {
        MicfilfclkdivHalt::from_bits(val)
    }
}
impl From<MicfilfclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivHalt) -> u8 {
        MicfilfclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl MicfilfclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivReset {
        MicfilfclkdivReset::from_bits(val)
    }
}
impl From<MicfilfclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivReset) -> u8 {
        MicfilfclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl MicfilfclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivUnstab {
        MicfilfclkdivUnstab::from_bits(val)
    }
}
impl From<MicfilfclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivUnstab) -> u8 {
        MicfilfclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkselSel {
    #[doc = "FRO_12M clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "PLL1_clk0 clock"]
    ENUM4 = 0x04,
    #[doc = "SAI0_MCLK clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
    #[doc = "SAI1_MCLK clock"]
    ENUM8 = 0x08,
    #[doc = "No clock"]
    ENUM9 = 0x09,
    #[doc = "No clock"]
    ENUM10 = 0x0a,
    #[doc = "No clock"]
    ENUM11 = 0x0b,
    #[doc = "No clock"]
    ENUM12 = 0x0c,
    #[doc = "No clock"]
    ENUM13 = 0x0d,
    #[doc = "No clock"]
    ENUM14 = 0x0e,
    #[doc = "No clock"]
    ENUM15 = 0x0f,
}
impl MicfilfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkselSel {
        MicfilfclkselSel::from_bits(val)
    }
}
impl From<MicfilfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkselSel) -> u8 {
        MicfilfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Mrt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt {
    #[inline(always)]
    fn from(val: u8) -> Mrt {
        Mrt::from_bits(val)
    }
}
impl From<Mrt> for u8 {
    #[inline(always)]
    fn from(val: Mrt) -> u8 {
        Mrt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrtRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl MrtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrtRst {
    #[inline(always)]
    fn from(val: u8) -> MrtRst {
        MrtRst::from_bits(val)
    }
}
impl From<MrtRst> for u8 {
    #[inline(always)]
    fn from(val: MrtRst) -> u8 {
        MrtRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mux {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mux {
    #[inline(always)]
    fn from(val: u8) -> Mux {
        Mux::from_bits(val)
    }
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(val: Mux) -> u8 {
        Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl MuxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRst {
    #[inline(always)]
    fn from(val: u8) -> MuxRst {
        MuxRst::from_bits(val)
    }
}
impl From<MuxRst> for u8 {
    #[inline(always)]
    fn from(val: MuxRst) -> u8 {
        MuxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NegSync {
    #[doc = "Negative glitch not detected"]
    DISABLE = 0x0,
    #[doc = "Negative glitch detected"]
    ENABLE = 0x01,
}
impl NegSync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NegSync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NegSync {
    #[inline(always)]
    fn from(val: u8) -> NegSync {
        NegSync::from_bits(val)
    }
}
impl From<NegSync> for u8 {
    #[inline(always)]
    fn from(val: NegSync) -> u8 {
        NegSync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmi {
    #[doc = "ETB NMI is not asserted"]
    DISABLE = 0x0,
    #[doc = "ETB NMI is asserted. Write 1 to clear it."]
    ENABLE = 0x01,
}
impl Nmi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmi {
    #[inline(always)]
    fn from(val: u8) -> Nmi {
        Nmi::from_bits(val)
    }
}
impl From<Nmi> for u8 {
    #[inline(always)]
    fn from(val: Nmi) -> u8 {
        Nmi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmiencpu0 {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl Nmiencpu0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmiencpu0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmiencpu0 {
    #[inline(always)]
    fn from(val: u8) -> Nmiencpu0 {
        Nmiencpu0::from_bits(val)
    }
}
impl From<Nmiencpu0> for u8 {
    #[inline(always)]
    fn from(val: Nmiencpu0) -> u8 {
        Nmiencpu0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmiencpu1 {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl Nmiencpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmiencpu1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmiencpu1 {
    #[inline(always)]
    fn from(val: u8) -> Nmiencpu1 {
        Nmiencpu1::from_bits(val)
    }
}
impl From<Nmiencpu1> for u8 {
    #[inline(always)]
    fn from(val: Nmiencpu1) -> u8 {
        Nmiencpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npu {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Npu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npu {
    #[inline(always)]
    fn from(val: u8) -> Npu {
        Npu::from_bits(val)
    }
}
impl From<Npu> for u8 {
    #[inline(always)]
    fn from(val: Npu) -> u8 {
        Npu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NpuRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl NpuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NpuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NpuRst {
    #[inline(always)]
    fn from(val: u8) -> NpuRst {
        NpuRst::from_bits(val)
    }
}
impl From<NpuRst> for u8 {
    #[inline(always)]
    fn from(val: NpuRst) -> u8 {
        NpuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Opamp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp0 {
    #[inline(always)]
    fn from(val: u8) -> Opamp0 {
        Opamp0::from_bits(val)
    }
}
impl From<Opamp0> for u8 {
    #[inline(always)]
    fn from(val: Opamp0) -> u8 {
        Opamp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Opamp0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp0Rst {
    #[inline(always)]
    fn from(val: u8) -> Opamp0Rst {
        Opamp0Rst::from_bits(val)
    }
}
impl From<Opamp0Rst> for u8 {
    #[inline(always)]
    fn from(val: Opamp0Rst) -> u8 {
        Opamp0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Opamp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp1 {
    #[inline(always)]
    fn from(val: u8) -> Opamp1 {
        Opamp1::from_bits(val)
    }
}
impl From<Opamp1> for u8 {
    #[inline(always)]
    fn from(val: Opamp1) -> u8 {
        Opamp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Opamp1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp1Rst {
    #[inline(always)]
    fn from(val: u8) -> Opamp1Rst {
        Opamp1Rst::from_bits(val)
    }
}
impl From<Opamp1Rst> for u8 {
    #[inline(always)]
    fn from(val: Opamp1Rst) -> u8 {
        Opamp1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Opamp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp2 {
    #[inline(always)]
    fn from(val: u8) -> Opamp2 {
        Opamp2::from_bits(val)
    }
}
impl From<Opamp2> for u8 {
    #[inline(always)]
    fn from(val: Opamp2) -> u8 {
        Opamp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Opamp2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp2Rst {
    #[inline(always)]
    fn from(val: u8) -> Opamp2Rst {
        Opamp2Rst::from_bits(val)
    }
}
impl From<Opamp2Rst> for u8 {
    #[inline(always)]
    fn from(val: Opamp2Rst) -> u8 {
        Opamp2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostimer {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Ostimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimer {
    #[inline(always)]
    fn from(val: u8) -> Ostimer {
        Ostimer::from_bits(val)
    }
}
impl From<Ostimer> for u8 {
    #[inline(always)]
    fn from(val: Ostimer) -> u8 {
        Ostimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl OstimerRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerRst {
    #[inline(always)]
    fn from(val: u8) -> OstimerRst {
        OstimerRst::from_bits(val)
    }
}
impl From<OstimerRst> for u8 {
    #[inline(always)]
    fn from(val: OstimerRst) -> u8 {
        OstimerRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerclkselSel {
    #[doc = "clk_16k\\[2\\]"]
    ENUM0 = 0x0,
    #[doc = "xtal32k\\[2\\]"]
    ENUM1 = 0x01,
    #[doc = "clk_1m clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
}
impl OstimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> OstimerclkselSel {
        OstimerclkselSel::from_bits(val)
    }
}
impl From<OstimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: OstimerclkselSel) -> u8 {
        OstimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParityFaultEn {
    #[doc = "Disables parity error report"]
    DISABLE = 0x0,
    #[doc = "Enables parity error report"]
    ENABLE = 0x01,
}
impl ParityFaultEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ParityFaultEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ParityFaultEn {
    #[inline(always)]
    fn from(val: u8) -> ParityFaultEn {
        ParityFaultEn::from_bits(val)
    }
}
impl From<ParityFaultEn> for u8 {
    #[inline(always)]
    fn from(val: ParityFaultEn) -> u8 {
        ParityFaultEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParityMissEn {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enables parity, miss on parity error"]
    ENABLE = 0x01,
}
impl ParityMissEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ParityMissEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ParityMissEn {
    #[inline(always)]
    fn from(val: u8) -> ParityMissEn {
        ParityMissEn::from_bits(val)
    }
}
impl From<ParityMissEn> for u8 {
    #[inline(always)]
    fn from(val: ParityMissEn) -> u8 {
        ParityMissEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhySel {
    #[doc = "Selects MII PHY Interface"]
    MII = 0x0,
    #[doc = "Selects RMII PHY Interface"]
    RMII = 0x01,
}
impl PhySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhySel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhySel {
    #[inline(always)]
    fn from(val: u8) -> PhySel {
        PhySel::from_bits(val)
    }
}
impl From<PhySel> for u8 {
    #[inline(always)]
    fn from(val: PhySel) -> u8 {
        PhySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pint {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Pint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pint {
    #[inline(always)]
    fn from(val: u8) -> Pint {
        Pint::from_bits(val)
    }
}
impl From<Pint> for u8 {
    #[inline(always)]
    fn from(val: Pint) -> u8 {
        Pint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl PintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintRst {
    #[inline(always)]
    fn from(val: u8) -> PintRst {
        PintRst::from_bits(val)
    }
}
impl From<PintRst> for u8 {
    #[inline(always)]
    fn from(val: PintRst) -> u8 {
        PintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pkc {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Pkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pkc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pkc {
    #[inline(always)]
    fn from(val: u8) -> Pkc {
        Pkc::from_bits(val)
    }
}
impl From<Pkc> for u8 {
    #[inline(always)]
    fn from(val: Pkc) -> u8 {
        Pkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcRam {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl PkcRam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcRam {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcRam {
    #[inline(always)]
    fn from(val: u8) -> PkcRam {
        PkcRam::from_bits(val)
    }
}
impl From<PkcRam> for u8 {
    #[inline(always)]
    fn from(val: PkcRam) -> u8 {
        PkcRam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl PkcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcRst {
    #[inline(always)]
    fn from(val: u8) -> PkcRst {
        PkcRst::from_bits(val)
    }
}
impl From<PkcRst> for u8 {
    #[inline(always)]
    fn from(val: PkcRst) -> u8 {
        PkcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Pll1clk0divHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divHalt {
        Pll1clk0divHalt::from_bits(val)
    }
}
impl From<Pll1clk0divHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divHalt) -> u8 {
        Pll1clk0divHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Pll1clk0divReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divReset {
        Pll1clk0divReset::from_bits(val)
    }
}
impl From<Pll1clk0divReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divReset) -> u8 {
        Pll1clk0divReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Pll1clk0divUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divUnstab {
        Pll1clk0divUnstab::from_bits(val)
    }
}
impl From<Pll1clk0divUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divUnstab) -> u8 {
        Pll1clk0divUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Pll1clk1divHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divHalt {
        Pll1clk1divHalt::from_bits(val)
    }
}
impl From<Pll1clk1divHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divHalt) -> u8 {
        Pll1clk1divHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Pll1clk1divReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divReset {
        Pll1clk1divReset::from_bits(val)
    }
}
impl From<Pll1clk1divReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divReset) -> u8 {
        Pll1clk1divReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Pll1clk1divUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divUnstab {
        Pll1clk1divUnstab::from_bits(val)
    }
}
impl From<Pll1clk1divUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divUnstab) -> u8 {
        Pll1clk1divUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl PllclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivHalt {
        PllclkdivHalt::from_bits(val)
    }
}
impl From<PllclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivHalt) -> u8 {
        PllclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl PllclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivReset {
        PllclkdivReset::from_bits(val)
    }
}
impl From<PllclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivReset) -> u8 {
        PllclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl PllclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivUnstab {
        PllclkdivUnstab::from_bits(val)
    }
}
impl From<PllclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivUnstab) -> u8 {
        PllclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivselSel {
    #[doc = "PLL0 clock"]
    ENUM0 = 0x0,
    #[doc = "pll1_clk0"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl PllclkdivselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivselSel {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivselSel {
        PllclkdivselSel::from_bits(val)
    }
}
impl From<PllclkdivselSel> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivselSel) -> u8 {
        PllclkdivselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PluDeglitchClkEna {
    #[doc = "Clock is not enabled"]
    DISABLE = 0x0,
    #[doc = "Clock is enabled"]
    ENABLE = 0x01,
}
impl PluDeglitchClkEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PluDeglitchClkEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PluDeglitchClkEna {
    #[inline(always)]
    fn from(val: u8) -> PluDeglitchClkEna {
        PluDeglitchClkEna::from_bits(val)
    }
}
impl From<PluDeglitchClkEna> for u8 {
    #[inline(always)]
    fn from(val: PluDeglitchClkEna) -> u8 {
        PluDeglitchClkEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PluLut {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl PluLut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PluLut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PluLut {
    #[inline(always)]
    fn from(val: u8) -> PluLut {
        PluLut::from_bits(val)
    }
}
impl From<PluLut> for u8 {
    #[inline(always)]
    fn from(val: PluLut) -> u8 {
        PluLut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PluRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl PluRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PluRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PluRst {
    #[inline(always)]
    fn from(val: u8) -> PluRst {
        PluRst::from_bits(val)
    }
}
impl From<PluRst> for u8 {
    #[inline(always)]
    fn from(val: PluRst) -> u8 {
        PluRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Port0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0 {
    #[inline(always)]
    fn from(val: u8) -> Port0 {
        Port0::from_bits(val)
    }
}
impl From<Port0> for u8 {
    #[inline(always)]
    fn from(val: Port0) -> u8 {
        Port0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Port0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0Rst {
    #[inline(always)]
    fn from(val: u8) -> Port0Rst {
        Port0Rst::from_bits(val)
    }
}
impl From<Port0Rst> for u8 {
    #[inline(always)]
    fn from(val: Port0Rst) -> u8 {
        Port0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Port1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1 {
    #[inline(always)]
    fn from(val: u8) -> Port1 {
        Port1::from_bits(val)
    }
}
impl From<Port1> for u8 {
    #[inline(always)]
    fn from(val: Port1) -> u8 {
        Port1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Port1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1Rst {
    #[inline(always)]
    fn from(val: u8) -> Port1Rst {
        Port1Rst::from_bits(val)
    }
}
impl From<Port1Rst> for u8 {
    #[inline(always)]
    fn from(val: Port1Rst) -> u8 {
        Port1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Port2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2 {
    #[inline(always)]
    fn from(val: u8) -> Port2 {
        Port2::from_bits(val)
    }
}
impl From<Port2> for u8 {
    #[inline(always)]
    fn from(val: Port2) -> u8 {
        Port2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Port2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2Rst {
    #[inline(always)]
    fn from(val: u8) -> Port2Rst {
        Port2Rst::from_bits(val)
    }
}
impl From<Port2Rst> for u8 {
    #[inline(always)]
    fn from(val: Port2Rst) -> u8 {
        Port2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Port3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3 {
    #[inline(always)]
    fn from(val: u8) -> Port3 {
        Port3::from_bits(val)
    }
}
impl From<Port3> for u8 {
    #[inline(always)]
    fn from(val: Port3) -> u8 {
        Port3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Port3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3Rst {
    #[inline(always)]
    fn from(val: u8) -> Port3Rst {
        Port3Rst::from_bits(val)
    }
}
impl From<Port3Rst> for u8 {
    #[inline(always)]
    fn from(val: Port3Rst) -> u8 {
        Port3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Port4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4 {
    #[inline(always)]
    fn from(val: u8) -> Port4 {
        Port4::from_bits(val)
    }
}
impl From<Port4> for u8 {
    #[inline(always)]
    fn from(val: Port4) -> u8 {
        Port4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Port4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4Rst {
    #[inline(always)]
    fn from(val: u8) -> Port4Rst {
        Port4Rst::from_bits(val)
    }
}
impl From<Port4Rst> for u8 {
    #[inline(always)]
    fn from(val: Port4Rst) -> u8 {
        Port4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PosSync {
    #[doc = "Positive glitch not detected"]
    DISABLE = 0x0,
    #[doc = "Positive glitch detected"]
    ENABLE = 0x01,
}
impl PosSync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PosSync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PosSync {
    #[inline(always)]
    fn from(val: u8) -> PosSync {
        PosSync::from_bits(val)
    }
}
impl From<PosSync> for u8 {
    #[inline(always)]
    fn from(val: PosSync) -> u8 {
        PosSync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pq {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Pq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pq {
    #[inline(always)]
    fn from(val: u8) -> Pq {
        Pq::from_bits(val)
    }
}
impl From<Pq> for u8 {
    #[inline(always)]
    fn from(val: Pq) -> u8 {
        Pq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PqRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl PqRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PqRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PqRst {
    #[inline(always)]
    fn from(val: u8) -> PqRst {
        PqRst::from_bits(val)
    }
}
impl From<PqRst> for u8 {
    #[inline(always)]
    fn from(val: PqRst) -> u8 {
        PqRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxI {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCoolfluxI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxI {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxI {
        PriCoolfluxI::from_bits(val)
    }
}
impl From<PriCoolfluxI> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxI) -> u8 {
        PriCoolfluxI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxX {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCoolfluxX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxX {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxX {
        PriCoolfluxX::from_bits(val)
    }
}
impl From<PriCoolfluxX> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxX) -> u8 {
        PriCoolfluxX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxYEspi {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxYEspi {
        PriCoolfluxYEspi::from_bits(val)
    }
}
impl From<PriCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxYEspi) -> u8 {
        PriCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Cbus {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Cbus {
        PriCpu0Cbus::from_bits(val)
    }
}
impl From<PriCpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Cbus) -> u8 {
        PriCpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Sbus {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Sbus {
        PriCpu0Sbus::from_bits(val)
    }
}
impl From<PriCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Sbus) -> u8 {
        PriCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu1CbusSmartDmaI {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu1CbusSmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu1CbusSmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu1CbusSmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> PriCpu1CbusSmartDmaI {
        PriCpu1CbusSmartDmaI::from_bits(val)
    }
}
impl From<PriCpu1CbusSmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: PriCpu1CbusSmartDmaI) -> u8 {
        PriCpu1CbusSmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu1SbusSmartDmaD {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu1SbusSmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu1SbusSmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu1SbusSmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> PriCpu1SbusSmartDmaD {
        PriCpu1SbusSmartDmaD::from_bits(val)
    }
}
impl From<PriCpu1SbusSmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: PriCpu1SbusSmartDmaD) -> u8 {
        PriCpu1SbusSmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriNpuD {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriNpuD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriNpuD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriNpuD {
    #[inline(always)]
    fn from(val: u8) -> PriNpuD {
        PriNpuD::from_bits(val)
    }
}
impl From<PriNpuD> for u8 {
    #[inline(always)]
    fn from(val: PriNpuD) -> u8 {
        PriNpuD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriNpuPq {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriNpuPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriNpuPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriNpuPq {
    #[inline(always)]
    fn from(val: u8) -> PriNpuPq {
        PriNpuPq::from_bits(val)
    }
}
impl From<PriNpuPq> for u8 {
    #[inline(always)]
    fn from(val: PriNpuPq) -> u8 {
        PriNpuPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriPkcEls {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriPkcEls {
    #[inline(always)]
    fn from(val: u8) -> PriPkcEls {
        PriPkcEls::from_bits(val)
    }
}
impl From<PriPkcEls> for u8 {
    #[inline(always)]
    fn from(val: PriPkcEls) -> u8 {
        PriPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbFsEnet {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> PriUsbFsEnet {
        PriUsbFsEnet::from_bits(val)
    }
}
impl From<PriUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: PriUsbFsEnet) -> u8 {
        PriUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbHs {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbHs {
    #[inline(always)]
    fn from(val: u8) -> PriUsbHs {
        PriUsbHs::from_bits(val)
    }
}
impl From<PriUsbHs> for u8 {
    #[inline(always)]
    fn from(val: PriUsbHs) -> u8 {
        PriUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsdhc {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsdhc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsdhc {
    #[inline(always)]
    fn from(val: u8) -> PriUsdhc {
        PriUsdhc::from_bits(val)
    }
}
impl From<PriUsdhc> for u8 {
    #[inline(always)]
    fn from(val: PriUsdhc) -> u8 {
        PriUsdhc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prot(u16);
impl Prot {
    #[doc = "For write operation to have an effect."]
    pub const PROT: Self = Self(0xc0c4);
}
impl Prot {
    pub const fn from_bits(val: u16) -> Prot {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Prot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xc0c4 => f.write_str("PROT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prot {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xc0c4 => defmt::write!(f, "PROT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Prot {
    #[inline(always)]
    fn from(val: u16) -> Prot {
        Prot::from_bits(val)
    }
}
impl From<Prot> for u16 {
    #[inline(always)]
    fn from(val: Prot) -> u16 {
        Prot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Puf {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Puf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Puf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Puf {
    #[inline(always)]
    fn from(val: u8) -> Puf {
        Puf::from_bits(val)
    }
}
impl From<Puf> for u8 {
    #[inline(always)]
    fn from(val: Puf) -> u8 {
        Puf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl PufRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufRst {
    #[inline(always)]
    fn from(val: u8) -> PufRst {
        PufRst::from_bits(val)
    }
}
impl From<PufRst> for u8 {
    #[inline(always)]
    fn from(val: PufRst) -> u8 {
        PufRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Pwm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0 {
    #[inline(always)]
    fn from(val: u8) -> Pwm0 {
        Pwm0::from_bits(val)
    }
}
impl From<Pwm0> for u8 {
    #[inline(always)]
    fn from(val: Pwm0) -> u8 {
        Pwm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Pwm0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0Rst {
    #[inline(always)]
    fn from(val: u8) -> Pwm0Rst {
        Pwm0Rst::from_bits(val)
    }
}
impl From<Pwm0Rst> for u8 {
    #[inline(always)]
    fn from(val: Pwm0Rst) -> u8 {
        Pwm0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Pwm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1 {
    #[inline(always)]
    fn from(val: u8) -> Pwm1 {
        Pwm1::from_bits(val)
    }
}
impl From<Pwm1> for u8 {
    #[inline(always)]
    fn from(val: Pwm1) -> u8 {
        Pwm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Pwm1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1Rst {
    #[inline(always)]
    fn from(val: u8) -> Pwm1Rst {
        Pwm1Rst::from_bits(val)
    }
}
impl From<Pwm1Rst> for u8 {
    #[inline(always)]
    fn from(val: Pwm1Rst) -> u8 {
        Pwm1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Qdc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0 {
    #[inline(always)]
    fn from(val: u8) -> Qdc0 {
        Qdc0::from_bits(val)
    }
}
impl From<Qdc0> for u8 {
    #[inline(always)]
    fn from(val: Qdc0) -> u8 {
        Qdc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Qdc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Qdc0Rst {
        Qdc0Rst::from_bits(val)
    }
}
impl From<Qdc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Qdc0Rst) -> u8 {
        Qdc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Qdc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1 {
    #[inline(always)]
    fn from(val: u8) -> Qdc1 {
        Qdc1::from_bits(val)
    }
}
impl From<Qdc1> for u8 {
    #[inline(always)]
    fn from(val: Qdc1) -> u8 {
        Qdc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Qdc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Qdc1Rst {
        Qdc1Rst::from_bits(val)
    }
}
impl From<Qdc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Qdc1Rst) -> u8 {
        Qdc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rama {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Rama {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rama {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rama {
    #[inline(always)]
    fn from(val: u8) -> Rama {
        Rama::from_bits(val)
    }
}
impl From<Rama> for u8 {
    #[inline(always)]
    fn from(val: Rama) -> u8 {
        Rama::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaEccEnable {
    #[doc = "ECC is disabled"]
    DISABLE = 0x0,
    #[doc = "ECC is enabled"]
    ENABLE = 0x01,
}
impl RamaEccEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaEccEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaEccEnable {
    #[inline(always)]
    fn from(val: u8) -> RamaEccEnable {
        RamaEccEnable::from_bits(val)
    }
}
impl From<RamaEccEnable> for u8 {
    #[inline(always)]
    fn from(val: RamaEccEnable) -> u8 {
        RamaEccEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambRamxEccEnable {
    #[doc = "ECC is disabled"]
    DISABLE = 0x0,
    #[doc = "ECC is enabled"]
    ENABLE = 0x01,
}
impl RambRamxEccEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambRamxEccEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambRamxEccEnable {
    #[inline(always)]
    fn from(val: u8) -> RambRamxEccEnable {
        RambRamxEccEnable::from_bits(val)
    }
}
impl From<RambRamxEccEnable> for u8 {
    #[inline(always)]
    fn from(val: RambRamxEccEnable) -> u8 {
        RambRamxEccEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdRamcEccEnable {
    #[doc = "ECC is disabled"]
    DISABLE = 0x0,
    #[doc = "ECC is enabled"]
    ENABLE = 0x01,
}
impl RamdRamcEccEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdRamcEccEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdRamcEccEnable {
    #[inline(always)]
    fn from(val: u8) -> RamdRamcEccEnable {
        RamdRamcEccEnable::from_bits(val)
    }
}
impl From<RamdRamcEccEnable> for u8 {
    #[inline(always)]
    fn from(val: RamdRamcEccEnable) -> u8 {
        RamdRamcEccEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamfRameEccEnable {
    #[doc = "ECC is disabled"]
    DISABLE = 0x0,
    #[doc = "ECC is enabled"]
    ENABLE = 0x01,
}
impl RamfRameEccEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamfRameEccEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamfRameEccEnable {
    #[inline(always)]
    fn from(val: u8) -> RamfRameEccEnable {
        RamfRameEccEnable::from_bits(val)
    }
}
impl From<RamfRameEccEnable> for u8 {
    #[inline(always)]
    fn from(val: RamfRameEccEnable) -> u8 {
        RamfRameEccEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ramx {
    #[doc = "Automatic clock gating is not overridden"]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Ramx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ramx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ramx {
    #[inline(always)]
    fn from(val: u8) -> Ramx {
        Ramx::from_bits(val)
    }
}
impl From<Ramx> for u8 {
    #[inline(always)]
    fn from(val: Ramx) -> u8 {
        Ramx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rlrq {
    #[doc = "No effect"]
    DISABLE = 0x0,
    #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
    ENABLE = 0x01,
}
impl Rlrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rlrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rlrq {
    #[inline(always)]
    fn from(val: u8) -> Rlrq {
        Rlrq::from_bits(val)
    }
}
impl From<Rlrq> for u8 {
    #[inline(always)]
    fn from(val: Rlrq) -> u8 {
        Rlrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rom {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Rom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rom {
    #[inline(always)]
    fn from(val: u8) -> Rom {
        Rom::from_bits(val)
    }
}
impl From<Rom> for u8 {
    #[inline(always)]
    fn from(val: Rom) -> u8 {
        Rom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomWait {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl RomWait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomWait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomWait {
    #[inline(always)]
    fn from(val: u8) -> RomWait {
        RomWait::from_bits(val)
    }
}
impl From<RomWait> for u8 {
    #[inline(always)]
    fn from(val: RomWait) -> u8 {
        RomWait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rspt {
    #[doc = "No response when the ETB count expires"]
    NO_RESP = 0x0,
    #[doc = "Generates a normal interrupt when the ETB count expires"]
    INTERRUPT = 0x01,
    #[doc = "Generates an NMI interrupt when the ETB count expires"]
    NMI = 0x02,
    #[doc = "Generates a debug halt when the ETB count expires via CPU0 CTICHIN\\[2\\]"]
    DEBUG_HALT = 0x03,
}
impl Rspt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rspt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rspt {
    #[inline(always)]
    fn from(val: u8) -> Rspt {
        Rspt::from_bits(val)
    }
}
impl From<Rspt> for u8 {
    #[inline(always)]
    fn from(val: Rspt) -> u8 {
        Rspt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Rtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc {
    #[inline(always)]
    fn from(val: u8) -> Rtc {
        Rtc::from_bits(val)
    }
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(val: Rtc) -> u8 {
        Rtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl RtcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRst {
    #[inline(always)]
    fn from(val: u8) -> RtcRst {
        RtcRst::from_bits(val)
    }
}
impl From<RtcRst> for u8 {
    #[inline(always)]
    fn from(val: RtcRst) -> u8 {
        RtcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Sai0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0 {
    #[inline(always)]
    fn from(val: u8) -> Sai0 {
        Sai0::from_bits(val)
    }
}
impl From<Sai0> for u8 {
    #[inline(always)]
    fn from(val: Sai0) -> u8 {
        Sai0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Sai0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0Rst {
    #[inline(always)]
    fn from(val: u8) -> Sai0Rst {
        Sai0Rst::from_bits(val)
    }
}
impl From<Sai0Rst> for u8 {
    #[inline(always)]
    fn from(val: Sai0Rst) -> u8 {
        Sai0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Sai0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivHalt {
        Sai0clkdivHalt::from_bits(val)
    }
}
impl From<Sai0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivHalt) -> u8 {
        Sai0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Sai0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivReset {
        Sai0clkdivReset::from_bits(val)
    }
}
impl From<Sai0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivReset) -> u8 {
        Sai0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Sai0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivUnstab {
        Sai0clkdivUnstab::from_bits(val)
    }
}
impl From<Sai0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivUnstab) -> u8 {
        Sai0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "PLL1_CLK0 clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Sai0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkselSel {
        Sai0clkselSel::from_bits(val)
    }
}
impl From<Sai0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkselSel) -> u8 {
        Sai0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Sai1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1 {
    #[inline(always)]
    fn from(val: u8) -> Sai1 {
        Sai1::from_bits(val)
    }
}
impl From<Sai1> for u8 {
    #[inline(always)]
    fn from(val: Sai1) -> u8 {
        Sai1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Sai1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Rst {
    #[inline(always)]
    fn from(val: u8) -> Sai1Rst {
        Sai1Rst::from_bits(val)
    }
}
impl From<Sai1Rst> for u8 {
    #[inline(always)]
    fn from(val: Sai1Rst) -> u8 {
        Sai1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Sai1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivHalt {
        Sai1clkdivHalt::from_bits(val)
    }
}
impl From<Sai1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivHalt) -> u8 {
        Sai1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Sai1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivReset {
        Sai1clkdivReset::from_bits(val)
    }
}
impl From<Sai1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivReset) -> u8 {
        Sai1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Sai1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivUnstab {
        Sai1clkdivUnstab::from_bits(val)
    }
}
impl From<Sai1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivUnstab) -> u8 {
        Sai1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "PLL1_CLK0 clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Sai1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkselSel {
        Sai1clkselSel::from_bits(val)
    }
}
impl From<Sai1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkselSel) -> u8 {
        Sai1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sb3 {
    #[doc = "customer fw load/update file."]
    CUSTOMER = 0x0,
    #[doc = "NXP Provisioning FW."]
    NXP = 0x01,
    #[doc = "ELS signed OEM Provisioning FW."]
    OEM = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sb3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sb3 {
    #[inline(always)]
    fn from(val: u8) -> Sb3 {
        Sb3::from_bits(val)
    }
}
impl From<Sb3> for u8 {
    #[inline(always)]
    fn from(val: Sb3) -> u8 {
        Sb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scg {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Scg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scg {
    #[inline(always)]
    fn from(val: u8) -> Scg {
        Scg::from_bits(val)
    }
}
impl From<Scg> for u8 {
    #[inline(always)]
    fn from(val: Scg) -> u8 {
        Scg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sct {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Sct {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sct {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sct {
    #[inline(always)]
    fn from(val: u8) -> Sct {
        Sct::from_bits(val)
    }
}
impl From<Sct> for u8 {
    #[inline(always)]
    fn from(val: Sct) -> u8 {
        Sct::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl SctRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctRst {
    #[inline(always)]
    fn from(val: u8) -> SctRst {
        SctRst::from_bits(val)
    }
}
impl From<SctRst> for u8 {
    #[inline(always)]
    fn from(val: SctRst) -> u8 {
        SctRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl SctclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivHalt {
        SctclkdivHalt::from_bits(val)
    }
}
impl From<SctclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivHalt) -> u8 {
        SctclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl SctclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivReset {
        SctclkdivReset::from_bits(val)
    }
}
impl From<SctclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivReset) -> u8 {
        SctclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl SctclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivUnstab {
        SctclkdivUnstab::from_bits(val)
    }
}
impl From<SctclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivUnstab) -> u8 {
        SctclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM_0X3 = 0x03,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X4 = 0x04,
    #[doc = "SAI0 MCLK_IN clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
    #[doc = "SAI1 MCLK_IN clock"]
    ENUM_0X8 = 0x08,
    #[doc = "No clock"]
    ENUM_0X9 = 0x09,
    #[doc = "No clock"]
    ENUM_0X_A = 0x0a,
    #[doc = "No clock"]
    ENUM_0X_B = 0x0b,
    #[doc = "No clock"]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock"]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock"]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock"]
    ENUM_0X_F = 0x0f,
}
impl SctclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctclkselSel {
        SctclkselSel::from_bits(val)
    }
}
impl From<SctclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctclkselSel) -> u8 {
        SctclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelCh0 {
    #[doc = "No trigger flow control"]
    DISABLE = 0x0,
    #[doc = "Trigger flow control"]
    ENABLE = 0x01,
}
impl SelCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelCh0 {
    #[inline(always)]
    fn from(val: u8) -> SelCh0 {
        SelCh0::from_bits(val)
    }
}
impl From<SelCh0> for u8 {
    #[inline(always)]
    fn from(val: SelCh0) -> u8 {
        SelCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelCh1 {
    #[doc = "No trigger flow control"]
    DISABLE = 0x0,
    #[doc = "Trigger flow control"]
    ENABLE = 0x01,
}
impl SelCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelCh1 {
    #[inline(always)]
    fn from(val: u8) -> SelCh1 {
        SelCh1::from_bits(val)
    }
}
impl From<SelCh1> for u8 {
    #[inline(always)]
    fn from(val: SelCh1) -> u8 {
        SelCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sema42 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Sema42 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sema42 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sema42 {
    #[inline(always)]
    fn from(val: u8) -> Sema42 {
        Sema42::from_bits(val)
    }
}
impl From<Sema42> for u8 {
    #[inline(always)]
    fn from(val: Sema42) -> u8 {
        Sema42::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sema42Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Sema42Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sema42Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sema42Rst {
    #[inline(always)]
    fn from(val: u8) -> Sema42Rst {
        Sema42Rst::from_bits(val)
    }
}
impl From<Sema42Rst> for u8 {
    #[inline(always)]
    fn from(val: Sema42Rst) -> u8 {
        Sema42Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Sinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc {
    #[inline(always)]
    fn from(val: u8) -> Sinc {
        Sinc::from_bits(val)
    }
}
impl From<Sinc> for u8 {
    #[inline(always)]
    fn from(val: Sinc) -> u8 {
        Sinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl SincRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincRst {
    #[inline(always)]
    fn from(val: u8) -> SincRst {
        SincRst::from_bits(val)
    }
}
impl From<SincRst> for u8 {
    #[inline(always)]
    fn from(val: SincRst) -> u8 {
        SincRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincfiltclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12Mhz clock"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl SincfiltclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincfiltclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincfiltclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SincfiltclkselSel {
        SincfiltclkselSel::from_bits(val)
    }
}
impl From<SincfiltclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SincfiltclkselSel) -> u8 {
        SincfiltclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl SlowclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivHalt {
        SlowclkdivHalt::from_bits(val)
    }
}
impl From<SlowclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivHalt) -> u8 {
        SlowclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl SlowclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivReset {
        SlowclkdivReset::from_bits(val)
    }
}
impl From<SlowclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivReset) -> u8 {
        SlowclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Sm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3 {
    #[inline(always)]
    fn from(val: u8) -> Sm3 {
        Sm3::from_bits(val)
    }
}
impl From<Sm3> for u8 {
    #[inline(always)]
    fn from(val: Sm3) -> u8 {
        Sm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Sm3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3Rst {
    #[inline(always)]
    fn from(val: u8) -> Sm3Rst {
        Sm3Rst::from_bits(val)
    }
}
impl From<Sm3Rst> for u8 {
    #[inline(always)]
    fn from(val: Sm3Rst) -> u8 {
        Sm3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDma {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl SmartDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDma {
    #[inline(always)]
    fn from(val: u8) -> SmartDma {
        SmartDma::from_bits(val)
    }
}
impl From<SmartDma> for u8 {
    #[inline(always)]
    fn from(val: SmartDma) -> u8 {
        SmartDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl SmartDmaRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaRst {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaRst {
        SmartDmaRst::from_bits(val)
    }
}
impl From<SmartDmaRst> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaRst) -> u8 {
        SmartDmaRst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessCpu0SecCode(u32);
impl SwdAccessCpu0SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SwdAccessCpu0SecCode {
    pub const fn from_bits(val: u32) -> SwdAccessCpu0SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessCpu0SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessCpu0SecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessCpu0SecCode {
        SwdAccessCpu0SecCode::from_bits(val)
    }
}
impl From<SwdAccessCpu0SecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessCpu0SecCode) -> u32 {
        SwdAccessCpu0SecCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessCpu1SecCode(u32);
impl SwdAccessCpu1SecCode {
    #[doc = "CPU1 DAP is not allowed"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Security code to allow CPU1 DAP"]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SwdAccessCpu1SecCode {
    pub const fn from_bits(val: u32) -> SwdAccessCpu1SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessCpu1SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu1SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessCpu1SecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessCpu1SecCode {
        SwdAccessCpu1SecCode::from_bits(val)
    }
}
impl From<SwdAccessCpu1SecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessCpu1SecCode) -> u32 {
        SwdAccessCpu1SecCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessDspSecCode(u32);
impl SwdAccessDspSecCode {
    #[doc = "DSP DAP is not allowed. Reading back register is read as 0x5."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Value to write to enable DSP SWD access. Reading back register is read as 0xA."]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SwdAccessDspSecCode {
    pub const fn from_bits(val: u32) -> SwdAccessDspSecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessDspSecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessDspSecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessDspSecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessDspSecCode {
        SwdAccessDspSecCode::from_bits(val)
    }
}
impl From<SwdAccessDspSecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessDspSecCode) -> u32 {
        SwdAccessDspSecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Halt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Systickclkdiv0Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Halt {
        Systickclkdiv0Halt::from_bits(val)
    }
}
impl From<Systickclkdiv0Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Halt) -> u8 {
        Systickclkdiv0Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Systickclkdiv0Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reset {
        Systickclkdiv0Reset::from_bits(val)
    }
}
impl From<Systickclkdiv0Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reset) -> u8 {
        Systickclkdiv0Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Unstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Systickclkdiv0Unstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Unstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Unstab {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Unstab {
        Systickclkdiv0Unstab::from_bits(val)
    }
}
impl From<Systickclkdiv0Unstab> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Unstab) -> u8 {
        Systickclkdiv0Unstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Halt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Systickclkdiv1Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Halt {
        Systickclkdiv1Halt::from_bits(val)
    }
}
impl From<Systickclkdiv1Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Halt) -> u8 {
        Systickclkdiv1Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Reset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Systickclkdiv1Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Reset {
        Systickclkdiv1Reset::from_bits(val)
    }
}
impl From<Systickclkdiv1Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Reset) -> u8 {
        Systickclkdiv1Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Unstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Systickclkdiv1Unstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Unstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Unstab {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Unstab {
        Systickclkdiv1Unstab::from_bits(val)
    }
}
impl From<Systickclkdiv1Unstab> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Unstab) -> u8 {
        Systickclkdiv1Unstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel0Sel {
    #[doc = "SYSTICKCLKDIV0 output"]
    ENUM_0X0 = 0x0,
    #[doc = "Clk 1 MHz clock"]
    ENUM_0X1 = 0x01,
    #[doc = "LP Oscillator clock"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "No clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Systickclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel0Sel {
        Systickclksel0Sel::from_bits(val)
    }
}
impl From<Systickclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel0Sel) -> u8 {
        Systickclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel1Sel {
    #[doc = "SYSTICKCLKDIV1 output"]
    ENUM_0X0 = 0x0,
    #[doc = "Clk 1 MHz clock"]
    ENUM_0X1 = 0x01,
    #[doc = "LP Oscillator clock"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "No clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Systickclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel1Sel {
        Systickclksel1Sel::from_bits(val)
    }
}
impl From<Systickclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel1Sel) -> u8 {
        Systickclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Timer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0 {
    #[inline(always)]
    fn from(val: u8) -> Timer0 {
        Timer0::from_bits(val)
    }
}
impl From<Timer0> for u8 {
    #[inline(always)]
    fn from(val: Timer0) -> u8 {
        Timer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Timer0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer0Rst {
        Timer0Rst::from_bits(val)
    }
}
impl From<Timer0Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer0Rst) -> u8 {
        Timer0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Timer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1 {
    #[inline(always)]
    fn from(val: u8) -> Timer1 {
        Timer1::from_bits(val)
    }
}
impl From<Timer1> for u8 {
    #[inline(always)]
    fn from(val: Timer1) -> u8 {
        Timer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Timer1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer1Rst {
        Timer1Rst::from_bits(val)
    }
}
impl From<Timer1Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer1Rst) -> u8 {
        Timer1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Timer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2 {
    #[inline(always)]
    fn from(val: u8) -> Timer2 {
        Timer2::from_bits(val)
    }
}
impl From<Timer2> for u8 {
    #[inline(always)]
    fn from(val: Timer2) -> u8 {
        Timer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Timer2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer2Rst {
        Timer2Rst::from_bits(val)
    }
}
impl From<Timer2Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer2Rst) -> u8 {
        Timer2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Timer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3 {
    #[inline(always)]
    fn from(val: u8) -> Timer3 {
        Timer3::from_bits(val)
    }
}
impl From<Timer3> for u8 {
    #[inline(always)]
    fn from(val: Timer3) -> u8 {
        Timer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Timer3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer3Rst {
        Timer3Rst::from_bits(val)
    }
}
impl From<Timer3Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer3Rst) -> u8 {
        Timer3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Timer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4 {
    #[inline(always)]
    fn from(val: u8) -> Timer4 {
        Timer4::from_bits(val)
    }
}
impl From<Timer4> for u8 {
    #[inline(always)]
    fn from(val: Timer4) -> u8 {
        Timer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4Rst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Timer4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer4Rst {
        Timer4Rst::from_bits(val)
    }
}
impl From<Timer4Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer4Rst) -> u8 {
        Timer4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl TraceclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivHalt {
        TraceclkdivHalt::from_bits(val)
    }
}
impl From<TraceclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivHalt) -> u8 {
        TraceclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl TraceclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReset {
        TraceclkdivReset::from_bits(val)
    }
}
impl From<TraceclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReset) -> u8 {
        TraceclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl TraceclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivUnstab {
        TraceclkdivUnstab::from_bits(val)
    }
}
impl From<TraceclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivUnstab) -> u8 {
        TraceclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkselSel {
    #[doc = "TRACECLKDIV output"]
    ENUM_0X0 = 0x0,
    #[doc = "Clk 1 MHz clock"]
    ENUM_0X1 = 0x01,
    #[doc = "LP Oscillator clock"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "No clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl TraceclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TraceclkselSel {
        TraceclkselSel::from_bits(val)
    }
}
impl From<TraceclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TraceclkselSel) -> u8 {
        TraceclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trng {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Trng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trng {
    #[inline(always)]
    fn from(val: u8) -> Trng {
        Trng::from_bits(val)
    }
}
impl From<Trng> for u8 {
    #[inline(always)]
    fn from(val: Trng) -> u8 {
        Trng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngRefclkEn {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl TrngRefclkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngRefclkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngRefclkEn {
    #[inline(always)]
    fn from(val: u8) -> TrngRefclkEn {
        TrngRefclkEn::from_bits(val)
    }
}
impl From<TrngRefclkEn> for u8 {
    #[inline(always)]
    fn from(val: TrngRefclkEn) -> u8 {
        TrngRefclkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngRefclkEnClr {
    #[doc = "No effect."]
    DISABLE = 0x0,
    #[doc = "Set to 0"]
    ENABLE = 0x01,
}
impl TrngRefclkEnClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngRefclkEnClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngRefclkEnClr {
    #[inline(always)]
    fn from(val: u8) -> TrngRefclkEnClr {
        TrngRefclkEnClr::from_bits(val)
    }
}
impl From<TrngRefclkEnClr> for u8 {
    #[inline(always)]
    fn from(val: TrngRefclkEnClr) -> u8 {
        TrngRefclkEnClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngRefclkEnSet {
    #[doc = "No effect."]
    DISABLE = 0x0,
    #[doc = "Set to 1"]
    ENABLE = 0x01,
}
impl TrngRefclkEnSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngRefclkEnSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngRefclkEnSet {
    #[inline(always)]
    fn from(val: u8) -> TrngRefclkEnSet {
        TrngRefclkEnSet::from_bits(val)
    }
}
impl From<TrngRefclkEnSet> for u8 {
    #[inline(always)]
    fn from(val: TrngRefclkEnSet) -> u8 {
        TrngRefclkEnSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl TrngRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngRst {
    #[inline(always)]
    fn from(val: u8) -> TrngRst {
        TrngRst::from_bits(val)
    }
}
impl From<TrngRst> for u8 {
    #[inline(always)]
    fn from(val: TrngRst) -> u8 {
        TrngRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tro {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Tro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tro {
    #[inline(always)]
    fn from(val: u8) -> Tro {
        Tro::from_bits(val)
    }
}
impl From<Tro> for u8 {
    #[inline(always)]
    fn from(val: Tro) -> u8 {
        Tro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TroRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl TroRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TroRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TroRst {
    #[inline(always)]
    fn from(val: u8) -> TroRst {
        TroRst::from_bits(val)
    }
}
impl From<TroRst> for u8 {
    #[inline(always)]
    fn from(val: TroRst) -> u8 {
        TroRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Tsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi {
    #[inline(always)]
    fn from(val: u8) -> Tsi {
        Tsi::from_bits(val)
    }
}
impl From<Tsi> for u8 {
    #[inline(always)]
    fn from(val: Tsi) -> u8 {
        Tsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl TsiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiRst {
    #[inline(always)]
    fn from(val: u8) -> TsiRst {
        TsiRst::from_bits(val)
    }
}
impl From<TsiRst> for u8 {
    #[inline(always)]
    fn from(val: TsiRst) -> u8 {
        TsiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl TsiclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TsiclkdivHalt {
        TsiclkdivHalt::from_bits(val)
    }
}
impl From<TsiclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TsiclkdivHalt) -> u8 {
        TsiclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl TsiclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TsiclkdivReset {
        TsiclkdivReset::from_bits(val)
    }
}
impl From<TsiclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TsiclkdivReset) -> u8 {
        TsiclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl TsiclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> TsiclkdivUnstab {
        TsiclkdivUnstab::from_bits(val)
    }
}
impl From<TsiclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: TsiclkdivUnstab) -> u8 {
        TsiclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "No clock"]
    ENUM_0X1 = 0x01,
    #[doc = "clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12Mhz clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl TsiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TsiclkselSel {
        TsiclkselSel::from_bits(val)
    }
}
impl From<TsiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TsiclkselSel) -> u8 {
        TsiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhc {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl USdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhc {
    #[inline(always)]
    fn from(val: u8) -> USdhc {
        USdhc::from_bits(val)
    }
}
impl From<USdhc> for u8 {
    #[inline(always)]
    fn from(val: USdhc) -> u8 {
        USdhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl USdhcclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkdivHalt {
        USdhcclkdivHalt::from_bits(val)
    }
}
impl From<USdhcclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkdivHalt) -> u8 {
        USdhcclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl USdhcclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkdivReset {
        USdhcclkdivReset::from_bits(val)
    }
}
impl From<USdhcclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkdivReset) -> u8 {
        USdhcclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl USdhcclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkdivUnstab {
        USdhcclkdivUnstab::from_bits(val)
    }
}
impl From<USdhcclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkdivUnstab) -> u8 {
        USdhcclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "pll1_clk1 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl USdhcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkselSel {
        USdhcclkselSel::from_bits(val)
    }
}
impl From<USdhcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkselSel) -> u8 {
        USdhcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers"]
    ENABLE = 0x0,
    #[doc = "Freezes all clock configuration registers update"]
    FREEZE = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0Fs {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Usb0Fs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0Fs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0Fs {
    #[inline(always)]
    fn from(val: u8) -> Usb0Fs {
        Usb0Fs::from_bits(val)
    }
}
impl From<Usb0Fs> for u8 {
    #[inline(always)]
    fn from(val: Usb0Fs) -> u8 {
        Usb0Fs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0FsDcd {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Usb0FsDcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0FsDcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0FsDcd {
    #[inline(always)]
    fn from(val: u8) -> Usb0FsDcd {
        Usb0FsDcd::from_bits(val)
    }
}
impl From<Usb0FsDcd> for u8 {
    #[inline(always)]
    fn from(val: Usb0FsDcd) -> u8 {
        Usb0FsDcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0FsDcdRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Usb0FsDcdRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0FsDcdRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0FsDcdRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0FsDcdRst {
        Usb0FsDcdRst::from_bits(val)
    }
}
impl From<Usb0FsDcdRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0FsDcdRst) -> u8 {
        Usb0FsDcdRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0FsRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl Usb0FsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0FsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0FsRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0FsRst {
        Usb0FsRst::from_bits(val)
    }
}
impl From<Usb0FsRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0FsRst) -> u8 {
        Usb0FsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Usb0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivHalt {
        Usb0clkdivHalt::from_bits(val)
    }
}
impl From<Usb0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivHalt) -> u8 {
        Usb0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Usb0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivReset {
        Usb0clkdivReset::from_bits(val)
    }
}
impl From<Usb0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivReset) -> u8 {
        Usb0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Usb0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivUnstab {
        Usb0clkdivUnstab::from_bits(val)
    }
}
impl From<Usb0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivUnstab) -> u8 {
        Usb0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "No clock"]
    ENUM_0X2 = 0x02,
    #[doc = "Clk 48 MHz clock"]
    ENUM_0X3 = 0x03,
    #[doc = "Clk_in"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Usb0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkselSel {
        Usb0clkselSel::from_bits(val)
    }
}
impl From<Usb0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkselSel) -> u8 {
        Usb0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHs {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl UsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHs {
    #[inline(always)]
    fn from(val: u8) -> UsbHs {
        UsbHs::from_bits(val)
    }
}
impl From<UsbHs> for u8 {
    #[inline(always)]
    fn from(val: UsbHs) -> u8 {
        UsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsPhy {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl UsbHsPhy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsPhy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsPhy {
    #[inline(always)]
    fn from(val: u8) -> UsbHsPhy {
        UsbHsPhy::from_bits(val)
    }
}
impl From<UsbHsPhy> for u8 {
    #[inline(always)]
    fn from(val: UsbHsPhy) -> u8 {
        UsbHsPhy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsPhyRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl UsbHsPhyRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsPhyRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsPhyRst {
    #[inline(always)]
    fn from(val: u8) -> UsbHsPhyRst {
        UsbHsPhyRst::from_bits(val)
    }
}
impl From<UsbHsPhyRst> for u8 {
    #[inline(always)]
    fn from(val: UsbHsPhyRst) -> u8 {
        UsbHsPhyRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl UsbHsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsRst {
    #[inline(always)]
    fn from(val: u8) -> UsbHsRst {
        UsbHsRst::from_bits(val)
    }
}
impl From<UsbHsRst> for u8 {
    #[inline(always)]
    fn from(val: UsbHsRst) -> u8 {
        UsbHsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsdhcRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl UsdhcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsdhcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsdhcRst {
    #[inline(always)]
    fn from(val: u8) -> UsdhcRst {
        UsdhcRst::from_bits(val)
    }
}
impl From<UsdhcRst> for u8 {
    #[inline(always)]
    fn from(val: UsdhcRst) -> u8 {
        UsdhcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Utick {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Utick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Utick {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Utick {
    #[inline(always)]
    fn from(val: u8) -> Utick {
        Utick::from_bits(val)
    }
}
impl From<Utick> for u8 {
    #[inline(always)]
    fn from(val: Utick) -> u8 {
        Utick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl UtickRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickRst {
    #[inline(always)]
    fn from(val: u8) -> UtickRst {
        UtickRst::from_bits(val)
    }
}
impl From<UtickRst> for u8 {
    #[inline(always)]
    fn from(val: UtickRst) -> u8 {
        UtickRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl UtickclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivHalt {
        UtickclkdivHalt::from_bits(val)
    }
}
impl From<UtickclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivHalt) -> u8 {
        UtickclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl UtickclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivReset {
        UtickclkdivReset::from_bits(val)
    }
}
impl From<UtickclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivReset) -> u8 {
        UtickclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl UtickclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivUnstab {
        UtickclkdivUnstab::from_bits(val)
    }
}
impl From<UtickclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivUnstab) -> u8 {
        UtickclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkselSel {
    #[doc = "clk_in"]
    ENUM0 = 0x0,
    #[doc = "xtal32k\\[2\\]"]
    ENUM1 = 0x01,
    #[doc = "clk_1m clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
}
impl UtickclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UtickclkselSel {
        UtickclkselSel::from_bits(val)
    }
}
impl From<UtickclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UtickclkselSel) -> u8 {
        UtickclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Vref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref {
    #[inline(always)]
    fn from(val: u8) -> Vref {
        Vref::from_bits(val)
    }
}
impl From<Vref> for u8 {
    #[inline(always)]
    fn from(val: Vref) -> u8 {
        Vref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VrefRst {
    #[doc = "Block is not reset"]
    RELEASED = 0x0,
    #[doc = "Block is reset"]
    ASSERTED = 0x01,
}
impl VrefRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VrefRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VrefRst {
    #[inline(always)]
    fn from(val: u8) -> VrefRst {
        VrefRst::from_bits(val)
    }
}
impl From<VrefRst> for u8 {
    #[inline(always)]
    fn from(val: VrefRst) -> u8 {
        VrefRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Wdt0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivHalt {
        Wdt0clkdivHalt::from_bits(val)
    }
}
impl From<Wdt0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivHalt) -> u8 {
        Wdt0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Wdt0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivReset {
        Wdt0clkdivReset::from_bits(val)
    }
}
impl From<Wdt0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivReset) -> u8 {
        Wdt0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Wdt0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivUnstab {
        Wdt0clkdivUnstab::from_bits(val)
    }
}
impl From<Wdt0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivUnstab) -> u8 {
        Wdt0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Wdt1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivHalt {
        Wdt1clkdivHalt::from_bits(val)
    }
}
impl From<Wdt1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivHalt) -> u8 {
        Wdt1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Wdt1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivReset {
        Wdt1clkdivReset::from_bits(val)
    }
}
impl From<Wdt1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivReset) -> u8 {
        Wdt1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Wdt1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivUnstab {
        Wdt1clkdivUnstab::from_bits(val)
    }
}
impl From<Wdt1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivUnstab) -> u8 {
        Wdt1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkselSel {
    #[doc = "FRO16K clock 2"]
    ENUM0 = 0x0,
    #[doc = "fro_hf_div clock"]
    ENUM1 = 0x01,
    #[doc = "clk_1m clock"]
    ENUM2 = 0x02,
    #[doc = "clk_1m clock"]
    ENUM3 = 0x03,
}
impl Wdt1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkselSel {
        Wdt1clkselSel::from_bits(val)
    }
}
impl From<Wdt1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkselSel) -> u8 {
        Wdt1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt0 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Wwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt0 {
        Wwdt0::from_bits(val)
    }
}
impl From<Wwdt0> for u8 {
    #[inline(always)]
    fn from(val: Wwdt0) -> u8 {
        Wwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1 {
    #[doc = "Disables clock"]
    DISABLE = 0x0,
    #[doc = "Enables clock"]
    ENABLE = 0x01,
}
impl Wwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1 {
        Wwdt1::from_bits(val)
    }
}
impl From<Wwdt1> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1) -> u8 {
        Wwdt1::to_bits(val)
    }
}
