#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features implemented"]
    pub const STANDARD: Self = Self(0x0);
    #[doc = "Support for DMA/Trigger generation from wake-up pins and filters enabled. Support for external pin/filter detection during all power modes enabled."]
    pub const FILT_ALL_PWR: Self = Self(0x01);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("STANDARD"),
            0x01 => f.write_str("FILT_ALL_PWR"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STANDARD"),
            0x01 => defmt::write!(f, "FILT_ALL_PWR"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtc1 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    _RESERVED_3 = 0x03,
}
impl Filtc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtc1 {
    #[inline(always)]
    fn from(val: u8) -> Filtc1 {
        Filtc1::from_bits(val)
    }
}
impl From<Filtc1> for u8 {
    #[inline(always)]
    fn from(val: Filtc1) -> u8 {
        Filtc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtc2 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    _RESERVED_3 = 0x03,
}
impl Filtc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtc2 {
    #[inline(always)]
    fn from(val: u8) -> Filtc2 {
        Filtc2::from_bits(val)
    }
}
impl From<Filtc2> for u8 {
    #[inline(always)]
    fn from(val: Filtc2) -> u8 {
        Filtc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filte1 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (Detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (Detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (Detect on any edge)"]
    EN_ANY = 0x03,
}
impl Filte1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filte1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filte1 {
    #[inline(always)]
    fn from(val: u8) -> Filte1 {
        Filte1::from_bits(val)
    }
}
impl From<Filte1> for u8 {
    #[inline(always)]
    fn from(val: Filte1) -> u8 {
        Filte1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filte2 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (Detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (Detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (Detect on any edge)"]
    EN_ANY = 0x03,
}
impl Filte2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filte2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filte2 {
    #[inline(always)]
    fn from(val: u8) -> Filte2 {
        Filte2::from_bits(val)
    }
}
impl From<Filte2> for u8 {
    #[inline(always)]
    fn from(val: Filte2) -> u8 {
        Filte2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtf1 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Filtf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtf1 {
    #[inline(always)]
    fn from(val: u8) -> Filtf1 {
        Filtf1::from_bits(val)
    }
}
impl From<Filtf1> for u8 {
    #[inline(always)]
    fn from(val: Filtf1) -> u8 {
        Filtf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtf2 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Filtf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtf2 {
    #[inline(always)]
    fn from(val: u8) -> Filtf2 {
        Filtf2::from_bits(val)
    }
}
impl From<Filtf2> for u8 {
    #[inline(always)]
    fn from(val: Filtf2) -> u8 {
        Filtf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtm1 {
    #[doc = "Active only during Power Down/Deep Power Down mode"]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes"]
    ANY_PWR = 0x01,
}
impl Filtm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtm1 {
    #[inline(always)]
    fn from(val: u8) -> Filtm1 {
        Filtm1::from_bits(val)
    }
}
impl From<Filtm1> for u8 {
    #[inline(always)]
    fn from(val: Filtm1) -> u8 {
        Filtm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtm2 {
    #[doc = "Active only during Power Down/Deep Power Down mode"]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes"]
    ANY_PWR = 0x01,
}
impl Filtm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtm2 {
    #[inline(always)]
    fn from(val: u8) -> Filtm2 {
        Filtm2::from_bits(val)
    }
}
impl From<Filtm2> for u8 {
    #[inline(always)]
    fn from(val: Filtm2) -> u8 {
        Filtm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude0 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude0 {
    #[inline(always)]
    fn from(val: u8) -> Wude0 {
        Wude0::from_bits(val)
    }
}
impl From<Wude0> for u8 {
    #[inline(always)]
    fn from(val: Wude0) -> u8 {
        Wude0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude1 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude1 {
    #[inline(always)]
    fn from(val: u8) -> Wude1 {
        Wude1::from_bits(val)
    }
}
impl From<Wude1> for u8 {
    #[inline(always)]
    fn from(val: Wude1) -> u8 {
        Wude1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude2 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude2 {
    #[inline(always)]
    fn from(val: u8) -> Wude2 {
        Wude2::from_bits(val)
    }
}
impl From<Wude2> for u8 {
    #[inline(always)]
    fn from(val: Wude2) -> u8 {
        Wude2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude3 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude3 {
    #[inline(always)]
    fn from(val: u8) -> Wude3 {
        Wude3::from_bits(val)
    }
}
impl From<Wude3> for u8 {
    #[inline(always)]
    fn from(val: Wude3) -> u8 {
        Wude3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude4 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude4 {
    #[inline(always)]
    fn from(val: u8) -> Wude4 {
        Wude4::from_bits(val)
    }
}
impl From<Wude4> for u8 {
    #[inline(always)]
    fn from(val: Wude4) -> u8 {
        Wude4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude5 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude5 {
    #[inline(always)]
    fn from(val: u8) -> Wude5 {
        Wude5::from_bits(val)
    }
}
impl From<Wude5> for u8 {
    #[inline(always)]
    fn from(val: Wude5) -> u8 {
        Wude5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude6 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude6 {
    #[inline(always)]
    fn from(val: u8) -> Wude6 {
        Wude6::from_bits(val)
    }
}
impl From<Wude6> for u8 {
    #[inline(always)]
    fn from(val: Wude6) -> u8 {
        Wude6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude7 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude7 {
    #[inline(always)]
    fn from(val: u8) -> Wude7 {
        Wude7::from_bits(val)
    }
}
impl From<Wude7> for u8 {
    #[inline(always)]
    fn from(val: Wude7) -> u8 {
        Wude7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude8 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude8 {
    #[inline(always)]
    fn from(val: u8) -> Wude8 {
        Wude8::from_bits(val)
    }
}
impl From<Wude8> for u8 {
    #[inline(always)]
    fn from(val: Wude8) -> u8 {
        Wude8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wude9 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wude9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wude9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wude9 {
    #[inline(always)]
    fn from(val: u8) -> Wude9 {
        Wude9::from_bits(val)
    }
}
impl From<Wude9> for u8 {
    #[inline(always)]
    fn from(val: Wude9) -> u8 {
        Wude9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf0 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf0 {
    #[inline(always)]
    fn from(val: u8) -> Wuf0 {
        Wuf0::from_bits(val)
    }
}
impl From<Wuf0> for u8 {
    #[inline(always)]
    fn from(val: Wuf0) -> u8 {
        Wuf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf1 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf1 {
    #[inline(always)]
    fn from(val: u8) -> Wuf1 {
        Wuf1::from_bits(val)
    }
}
impl From<Wuf1> for u8 {
    #[inline(always)]
    fn from(val: Wuf1) -> u8 {
        Wuf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf10 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf10 {
    #[inline(always)]
    fn from(val: u8) -> Wuf10 {
        Wuf10::from_bits(val)
    }
}
impl From<Wuf10> for u8 {
    #[inline(always)]
    fn from(val: Wuf10) -> u8 {
        Wuf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf11 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf11 {
    #[inline(always)]
    fn from(val: u8) -> Wuf11 {
        Wuf11::from_bits(val)
    }
}
impl From<Wuf11> for u8 {
    #[inline(always)]
    fn from(val: Wuf11) -> u8 {
        Wuf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf12 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf12 {
    #[inline(always)]
    fn from(val: u8) -> Wuf12 {
        Wuf12::from_bits(val)
    }
}
impl From<Wuf12> for u8 {
    #[inline(always)]
    fn from(val: Wuf12) -> u8 {
        Wuf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf13 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf13 {
    #[inline(always)]
    fn from(val: u8) -> Wuf13 {
        Wuf13::from_bits(val)
    }
}
impl From<Wuf13> for u8 {
    #[inline(always)]
    fn from(val: Wuf13) -> u8 {
        Wuf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf14 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf14 {
    #[inline(always)]
    fn from(val: u8) -> Wuf14 {
        Wuf14::from_bits(val)
    }
}
impl From<Wuf14> for u8 {
    #[inline(always)]
    fn from(val: Wuf14) -> u8 {
        Wuf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf15 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf15 {
    #[inline(always)]
    fn from(val: u8) -> Wuf15 {
        Wuf15::from_bits(val)
    }
}
impl From<Wuf15> for u8 {
    #[inline(always)]
    fn from(val: Wuf15) -> u8 {
        Wuf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf16 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf16 {
    #[inline(always)]
    fn from(val: u8) -> Wuf16 {
        Wuf16::from_bits(val)
    }
}
impl From<Wuf16> for u8 {
    #[inline(always)]
    fn from(val: Wuf16) -> u8 {
        Wuf16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf17 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf17 {
    #[inline(always)]
    fn from(val: u8) -> Wuf17 {
        Wuf17::from_bits(val)
    }
}
impl From<Wuf17> for u8 {
    #[inline(always)]
    fn from(val: Wuf17) -> u8 {
        Wuf17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf18 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf18 {
    #[inline(always)]
    fn from(val: u8) -> Wuf18 {
        Wuf18::from_bits(val)
    }
}
impl From<Wuf18> for u8 {
    #[inline(always)]
    fn from(val: Wuf18) -> u8 {
        Wuf18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf19 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf19 {
    #[inline(always)]
    fn from(val: u8) -> Wuf19 {
        Wuf19::from_bits(val)
    }
}
impl From<Wuf19> for u8 {
    #[inline(always)]
    fn from(val: Wuf19) -> u8 {
        Wuf19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf2 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf2 {
    #[inline(always)]
    fn from(val: u8) -> Wuf2 {
        Wuf2::from_bits(val)
    }
}
impl From<Wuf2> for u8 {
    #[inline(always)]
    fn from(val: Wuf2) -> u8 {
        Wuf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf20 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf20 {
    #[inline(always)]
    fn from(val: u8) -> Wuf20 {
        Wuf20::from_bits(val)
    }
}
impl From<Wuf20> for u8 {
    #[inline(always)]
    fn from(val: Wuf20) -> u8 {
        Wuf20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf21 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf21 {
    #[inline(always)]
    fn from(val: u8) -> Wuf21 {
        Wuf21::from_bits(val)
    }
}
impl From<Wuf21> for u8 {
    #[inline(always)]
    fn from(val: Wuf21) -> u8 {
        Wuf21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf22 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf22 {
    #[inline(always)]
    fn from(val: u8) -> Wuf22 {
        Wuf22::from_bits(val)
    }
}
impl From<Wuf22> for u8 {
    #[inline(always)]
    fn from(val: Wuf22) -> u8 {
        Wuf22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf23 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf23 {
    #[inline(always)]
    fn from(val: u8) -> Wuf23 {
        Wuf23::from_bits(val)
    }
}
impl From<Wuf23> for u8 {
    #[inline(always)]
    fn from(val: Wuf23) -> u8 {
        Wuf23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf24 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf24 {
    #[inline(always)]
    fn from(val: u8) -> Wuf24 {
        Wuf24::from_bits(val)
    }
}
impl From<Wuf24> for u8 {
    #[inline(always)]
    fn from(val: Wuf24) -> u8 {
        Wuf24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf25 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf25 {
    #[inline(always)]
    fn from(val: u8) -> Wuf25 {
        Wuf25::from_bits(val)
    }
}
impl From<Wuf25> for u8 {
    #[inline(always)]
    fn from(val: Wuf25) -> u8 {
        Wuf25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf26 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf26 {
    #[inline(always)]
    fn from(val: u8) -> Wuf26 {
        Wuf26::from_bits(val)
    }
}
impl From<Wuf26> for u8 {
    #[inline(always)]
    fn from(val: Wuf26) -> u8 {
        Wuf26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf27 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf27 {
    #[inline(always)]
    fn from(val: u8) -> Wuf27 {
        Wuf27::from_bits(val)
    }
}
impl From<Wuf27> for u8 {
    #[inline(always)]
    fn from(val: Wuf27) -> u8 {
        Wuf27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf28 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf28 {
    #[inline(always)]
    fn from(val: u8) -> Wuf28 {
        Wuf28::from_bits(val)
    }
}
impl From<Wuf28> for u8 {
    #[inline(always)]
    fn from(val: Wuf28) -> u8 {
        Wuf28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf29 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf29 {
    #[inline(always)]
    fn from(val: u8) -> Wuf29 {
        Wuf29::from_bits(val)
    }
}
impl From<Wuf29> for u8 {
    #[inline(always)]
    fn from(val: Wuf29) -> u8 {
        Wuf29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf3 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf3 {
    #[inline(always)]
    fn from(val: u8) -> Wuf3 {
        Wuf3::from_bits(val)
    }
}
impl From<Wuf3> for u8 {
    #[inline(always)]
    fn from(val: Wuf3) -> u8 {
        Wuf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf30 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf30 {
    #[inline(always)]
    fn from(val: u8) -> Wuf30 {
        Wuf30::from_bits(val)
    }
}
impl From<Wuf30> for u8 {
    #[inline(always)]
    fn from(val: Wuf30) -> u8 {
        Wuf30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf31 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf31 {
    #[inline(always)]
    fn from(val: u8) -> Wuf31 {
        Wuf31::from_bits(val)
    }
}
impl From<Wuf31> for u8 {
    #[inline(always)]
    fn from(val: Wuf31) -> u8 {
        Wuf31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf4 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf4 {
    #[inline(always)]
    fn from(val: u8) -> Wuf4 {
        Wuf4::from_bits(val)
    }
}
impl From<Wuf4> for u8 {
    #[inline(always)]
    fn from(val: Wuf4) -> u8 {
        Wuf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf5 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf5 {
    #[inline(always)]
    fn from(val: u8) -> Wuf5 {
        Wuf5::from_bits(val)
    }
}
impl From<Wuf5> for u8 {
    #[inline(always)]
    fn from(val: Wuf5) -> u8 {
        Wuf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf6 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf6 {
    #[inline(always)]
    fn from(val: u8) -> Wuf6 {
        Wuf6::from_bits(val)
    }
}
impl From<Wuf6> for u8 {
    #[inline(always)]
    fn from(val: Wuf6) -> u8 {
        Wuf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf7 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf7 {
    #[inline(always)]
    fn from(val: u8) -> Wuf7 {
        Wuf7::from_bits(val)
    }
}
impl From<Wuf7> for u8 {
    #[inline(always)]
    fn from(val: Wuf7) -> u8 {
        Wuf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf8 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf8 {
    #[inline(always)]
    fn from(val: u8) -> Wuf8 {
        Wuf8::from_bits(val)
    }
}
impl From<Wuf8> for u8 {
    #[inline(always)]
    fn from(val: Wuf8) -> u8 {
        Wuf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuf9 {
    #[doc = "No"]
    NO_FLAG = 0x0,
    #[doc = "Yes"]
    FLAG = 0x01,
}
impl Wuf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuf9 {
    #[inline(always)]
    fn from(val: u8) -> Wuf9 {
        Wuf9::from_bits(val)
    }
}
impl From<Wuf9> for u8 {
    #[inline(always)]
    fn from(val: Wuf9) -> u8 {
        Wuf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume0 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume0 {
    #[inline(always)]
    fn from(val: u8) -> Wume0 {
        Wume0::from_bits(val)
    }
}
impl From<Wume0> for u8 {
    #[inline(always)]
    fn from(val: Wume0) -> u8 {
        Wume0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume1 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume1 {
    #[inline(always)]
    fn from(val: u8) -> Wume1 {
        Wume1::from_bits(val)
    }
}
impl From<Wume1> for u8 {
    #[inline(always)]
    fn from(val: Wume1) -> u8 {
        Wume1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume2 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume2 {
    #[inline(always)]
    fn from(val: u8) -> Wume2 {
        Wume2::from_bits(val)
    }
}
impl From<Wume2> for u8 {
    #[inline(always)]
    fn from(val: Wume2) -> u8 {
        Wume2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume3 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume3 {
    #[inline(always)]
    fn from(val: u8) -> Wume3 {
        Wume3::from_bits(val)
    }
}
impl From<Wume3> for u8 {
    #[inline(always)]
    fn from(val: Wume3) -> u8 {
        Wume3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume4 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume4 {
    #[inline(always)]
    fn from(val: u8) -> Wume4 {
        Wume4::from_bits(val)
    }
}
impl From<Wume4> for u8 {
    #[inline(always)]
    fn from(val: Wume4) -> u8 {
        Wume4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume5 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume5 {
    #[inline(always)]
    fn from(val: u8) -> Wume5 {
        Wume5::from_bits(val)
    }
}
impl From<Wume5> for u8 {
    #[inline(always)]
    fn from(val: Wume5) -> u8 {
        Wume5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume6 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume6 {
    #[inline(always)]
    fn from(val: u8) -> Wume6 {
        Wume6::from_bits(val)
    }
}
impl From<Wume6> for u8 {
    #[inline(always)]
    fn from(val: Wume6) -> u8 {
        Wume6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume7 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume7 {
    #[inline(always)]
    fn from(val: u8) -> Wume7 {
        Wume7::from_bits(val)
    }
}
impl From<Wume7> for u8 {
    #[inline(always)]
    fn from(val: Wume7) -> u8 {
        Wume7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume8 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume8 {
    #[inline(always)]
    fn from(val: u8) -> Wume8 {
        Wume8::from_bits(val)
    }
}
impl From<Wume8> for u8 {
    #[inline(always)]
    fn from(val: Wume8) -> u8 {
        Wume8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wume9 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Wume9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wume9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wume9 {
    #[inline(always)]
    fn from(val: u8) -> Wume9 {
        Wume9::from_bits(val)
    }
}
impl From<Wume9> for u8 {
    #[inline(always)]
    fn from(val: Wume9) -> u8 {
        Wume9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc0 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc0 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc0 {
        Wupdc0::from_bits(val)
    }
}
impl From<Wupdc0> for u8 {
    #[inline(always)]
    fn from(val: Wupdc0) -> u8 {
        Wupdc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc1 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc1 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc1 {
        Wupdc1::from_bits(val)
    }
}
impl From<Wupdc1> for u8 {
    #[inline(always)]
    fn from(val: Wupdc1) -> u8 {
        Wupdc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc10 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc10 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc10 {
        Wupdc10::from_bits(val)
    }
}
impl From<Wupdc10> for u8 {
    #[inline(always)]
    fn from(val: Wupdc10) -> u8 {
        Wupdc10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc11 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc11 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc11 {
        Wupdc11::from_bits(val)
    }
}
impl From<Wupdc11> for u8 {
    #[inline(always)]
    fn from(val: Wupdc11) -> u8 {
        Wupdc11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc12 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc12 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc12 {
        Wupdc12::from_bits(val)
    }
}
impl From<Wupdc12> for u8 {
    #[inline(always)]
    fn from(val: Wupdc12) -> u8 {
        Wupdc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc13 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc13 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc13 {
        Wupdc13::from_bits(val)
    }
}
impl From<Wupdc13> for u8 {
    #[inline(always)]
    fn from(val: Wupdc13) -> u8 {
        Wupdc13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc14 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc14 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc14 {
        Wupdc14::from_bits(val)
    }
}
impl From<Wupdc14> for u8 {
    #[inline(always)]
    fn from(val: Wupdc14) -> u8 {
        Wupdc14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc15 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc15 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc15 {
        Wupdc15::from_bits(val)
    }
}
impl From<Wupdc15> for u8 {
    #[inline(always)]
    fn from(val: Wupdc15) -> u8 {
        Wupdc15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc16 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc16 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc16 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc16 {
        Wupdc16::from_bits(val)
    }
}
impl From<Wupdc16> for u8 {
    #[inline(always)]
    fn from(val: Wupdc16) -> u8 {
        Wupdc16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc17 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc17 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc17 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc17 {
        Wupdc17::from_bits(val)
    }
}
impl From<Wupdc17> for u8 {
    #[inline(always)]
    fn from(val: Wupdc17) -> u8 {
        Wupdc17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc18 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc18 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc18 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc18 {
        Wupdc18::from_bits(val)
    }
}
impl From<Wupdc18> for u8 {
    #[inline(always)]
    fn from(val: Wupdc18) -> u8 {
        Wupdc18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc19 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc19 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc19 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc19 {
        Wupdc19::from_bits(val)
    }
}
impl From<Wupdc19> for u8 {
    #[inline(always)]
    fn from(val: Wupdc19) -> u8 {
        Wupdc19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc2 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc2 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc2 {
        Wupdc2::from_bits(val)
    }
}
impl From<Wupdc2> for u8 {
    #[inline(always)]
    fn from(val: Wupdc2) -> u8 {
        Wupdc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc20 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc20 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc20 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc20 {
        Wupdc20::from_bits(val)
    }
}
impl From<Wupdc20> for u8 {
    #[inline(always)]
    fn from(val: Wupdc20) -> u8 {
        Wupdc20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc21 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc21 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc21 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc21 {
        Wupdc21::from_bits(val)
    }
}
impl From<Wupdc21> for u8 {
    #[inline(always)]
    fn from(val: Wupdc21) -> u8 {
        Wupdc21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc22 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc22 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc22 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc22 {
        Wupdc22::from_bits(val)
    }
}
impl From<Wupdc22> for u8 {
    #[inline(always)]
    fn from(val: Wupdc22) -> u8 {
        Wupdc22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc23 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc23 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc23 {
        Wupdc23::from_bits(val)
    }
}
impl From<Wupdc23> for u8 {
    #[inline(always)]
    fn from(val: Wupdc23) -> u8 {
        Wupdc23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc24 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc24 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc24 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc24 {
        Wupdc24::from_bits(val)
    }
}
impl From<Wupdc24> for u8 {
    #[inline(always)]
    fn from(val: Wupdc24) -> u8 {
        Wupdc24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc25 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc25 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc25 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc25 {
        Wupdc25::from_bits(val)
    }
}
impl From<Wupdc25> for u8 {
    #[inline(always)]
    fn from(val: Wupdc25) -> u8 {
        Wupdc25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc26 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc26 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc26 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc26 {
        Wupdc26::from_bits(val)
    }
}
impl From<Wupdc26> for u8 {
    #[inline(always)]
    fn from(val: Wupdc26) -> u8 {
        Wupdc26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc27 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc27 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc27 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc27 {
        Wupdc27::from_bits(val)
    }
}
impl From<Wupdc27> for u8 {
    #[inline(always)]
    fn from(val: Wupdc27) -> u8 {
        Wupdc27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc28 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc28 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc28 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc28 {
        Wupdc28::from_bits(val)
    }
}
impl From<Wupdc28> for u8 {
    #[inline(always)]
    fn from(val: Wupdc28) -> u8 {
        Wupdc28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc29 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc29 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc29 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc29 {
        Wupdc29::from_bits(val)
    }
}
impl From<Wupdc29> for u8 {
    #[inline(always)]
    fn from(val: Wupdc29) -> u8 {
        Wupdc29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc3 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc3 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc3 {
        Wupdc3::from_bits(val)
    }
}
impl From<Wupdc3> for u8 {
    #[inline(always)]
    fn from(val: Wupdc3) -> u8 {
        Wupdc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc30 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc30 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc30 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc30 {
        Wupdc30::from_bits(val)
    }
}
impl From<Wupdc30> for u8 {
    #[inline(always)]
    fn from(val: Wupdc30) -> u8 {
        Wupdc30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc31 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc31 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc31 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc31 {
        Wupdc31::from_bits(val)
    }
}
impl From<Wupdc31> for u8 {
    #[inline(always)]
    fn from(val: Wupdc31) -> u8 {
        Wupdc31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc4 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc4 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc4 {
        Wupdc4::from_bits(val)
    }
}
impl From<Wupdc4> for u8 {
    #[inline(always)]
    fn from(val: Wupdc4) -> u8 {
        Wupdc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc5 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc5 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc5 {
        Wupdc5::from_bits(val)
    }
}
impl From<Wupdc5> for u8 {
    #[inline(always)]
    fn from(val: Wupdc5) -> u8 {
        Wupdc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc6 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc6 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc6 {
        Wupdc6::from_bits(val)
    }
}
impl From<Wupdc6> for u8 {
    #[inline(always)]
    fn from(val: Wupdc6) -> u8 {
        Wupdc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc7 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc7 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc7 {
        Wupdc7::from_bits(val)
    }
}
impl From<Wupdc7> for u8 {
    #[inline(always)]
    fn from(val: Wupdc7) -> u8 {
        Wupdc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc8 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc8 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc8 {
        Wupdc8::from_bits(val)
    }
}
impl From<Wupdc8> for u8 {
    #[inline(always)]
    fn from(val: Wupdc8) -> u8 {
        Wupdc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc9 {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "DMA request"]
    DMA_REQ = 0x01,
    #[doc = "Trigger event"]
    TRIGGER = 0x02,
    #[doc = "Reserved"]
    RES = 0x03,
}
impl Wupdc9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc9 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc9 {
        Wupdc9::from_bits(val)
    }
}
impl From<Wupdc9> for u8 {
    #[inline(always)]
    fn from(val: Wupdc9) -> u8 {
        Wupdc9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe0 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe0 {
    #[inline(always)]
    fn from(val: u8) -> Wupe0 {
        Wupe0::from_bits(val)
    }
}
impl From<Wupe0> for u8 {
    #[inline(always)]
    fn from(val: Wupe0) -> u8 {
        Wupe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe1 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe1 {
    #[inline(always)]
    fn from(val: u8) -> Wupe1 {
        Wupe1::from_bits(val)
    }
}
impl From<Wupe1> for u8 {
    #[inline(always)]
    fn from(val: Wupe1) -> u8 {
        Wupe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe10 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe10 {
    #[inline(always)]
    fn from(val: u8) -> Wupe10 {
        Wupe10::from_bits(val)
    }
}
impl From<Wupe10> for u8 {
    #[inline(always)]
    fn from(val: Wupe10) -> u8 {
        Wupe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe11 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe11 {
    #[inline(always)]
    fn from(val: u8) -> Wupe11 {
        Wupe11::from_bits(val)
    }
}
impl From<Wupe11> for u8 {
    #[inline(always)]
    fn from(val: Wupe11) -> u8 {
        Wupe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe12 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe12 {
    #[inline(always)]
    fn from(val: u8) -> Wupe12 {
        Wupe12::from_bits(val)
    }
}
impl From<Wupe12> for u8 {
    #[inline(always)]
    fn from(val: Wupe12) -> u8 {
        Wupe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe13 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe13 {
    #[inline(always)]
    fn from(val: u8) -> Wupe13 {
        Wupe13::from_bits(val)
    }
}
impl From<Wupe13> for u8 {
    #[inline(always)]
    fn from(val: Wupe13) -> u8 {
        Wupe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe14 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe14 {
    #[inline(always)]
    fn from(val: u8) -> Wupe14 {
        Wupe14::from_bits(val)
    }
}
impl From<Wupe14> for u8 {
    #[inline(always)]
    fn from(val: Wupe14) -> u8 {
        Wupe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe15 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe15 {
    #[inline(always)]
    fn from(val: u8) -> Wupe15 {
        Wupe15::from_bits(val)
    }
}
impl From<Wupe15> for u8 {
    #[inline(always)]
    fn from(val: Wupe15) -> u8 {
        Wupe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe16 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe16 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe16 {
    #[inline(always)]
    fn from(val: u8) -> Wupe16 {
        Wupe16::from_bits(val)
    }
}
impl From<Wupe16> for u8 {
    #[inline(always)]
    fn from(val: Wupe16) -> u8 {
        Wupe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe17 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe17 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe17 {
    #[inline(always)]
    fn from(val: u8) -> Wupe17 {
        Wupe17::from_bits(val)
    }
}
impl From<Wupe17> for u8 {
    #[inline(always)]
    fn from(val: Wupe17) -> u8 {
        Wupe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe18 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe18 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe18 {
    #[inline(always)]
    fn from(val: u8) -> Wupe18 {
        Wupe18::from_bits(val)
    }
}
impl From<Wupe18> for u8 {
    #[inline(always)]
    fn from(val: Wupe18) -> u8 {
        Wupe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe19 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe19 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe19 {
    #[inline(always)]
    fn from(val: u8) -> Wupe19 {
        Wupe19::from_bits(val)
    }
}
impl From<Wupe19> for u8 {
    #[inline(always)]
    fn from(val: Wupe19) -> u8 {
        Wupe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe2 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe2 {
    #[inline(always)]
    fn from(val: u8) -> Wupe2 {
        Wupe2::from_bits(val)
    }
}
impl From<Wupe2> for u8 {
    #[inline(always)]
    fn from(val: Wupe2) -> u8 {
        Wupe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe20 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe20 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe20 {
    #[inline(always)]
    fn from(val: u8) -> Wupe20 {
        Wupe20::from_bits(val)
    }
}
impl From<Wupe20> for u8 {
    #[inline(always)]
    fn from(val: Wupe20) -> u8 {
        Wupe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe21 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe21 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe21 {
    #[inline(always)]
    fn from(val: u8) -> Wupe21 {
        Wupe21::from_bits(val)
    }
}
impl From<Wupe21> for u8 {
    #[inline(always)]
    fn from(val: Wupe21) -> u8 {
        Wupe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe22 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe22 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe22 {
    #[inline(always)]
    fn from(val: u8) -> Wupe22 {
        Wupe22::from_bits(val)
    }
}
impl From<Wupe22> for u8 {
    #[inline(always)]
    fn from(val: Wupe22) -> u8 {
        Wupe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe23 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe23 {
    #[inline(always)]
    fn from(val: u8) -> Wupe23 {
        Wupe23::from_bits(val)
    }
}
impl From<Wupe23> for u8 {
    #[inline(always)]
    fn from(val: Wupe23) -> u8 {
        Wupe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe24 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe24 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe24 {
    #[inline(always)]
    fn from(val: u8) -> Wupe24 {
        Wupe24::from_bits(val)
    }
}
impl From<Wupe24> for u8 {
    #[inline(always)]
    fn from(val: Wupe24) -> u8 {
        Wupe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe25 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe25 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe25 {
    #[inline(always)]
    fn from(val: u8) -> Wupe25 {
        Wupe25::from_bits(val)
    }
}
impl From<Wupe25> for u8 {
    #[inline(always)]
    fn from(val: Wupe25) -> u8 {
        Wupe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe26 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe26 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe26 {
    #[inline(always)]
    fn from(val: u8) -> Wupe26 {
        Wupe26::from_bits(val)
    }
}
impl From<Wupe26> for u8 {
    #[inline(always)]
    fn from(val: Wupe26) -> u8 {
        Wupe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe27 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe27 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe27 {
    #[inline(always)]
    fn from(val: u8) -> Wupe27 {
        Wupe27::from_bits(val)
    }
}
impl From<Wupe27> for u8 {
    #[inline(always)]
    fn from(val: Wupe27) -> u8 {
        Wupe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe28 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe28 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe28 {
    #[inline(always)]
    fn from(val: u8) -> Wupe28 {
        Wupe28::from_bits(val)
    }
}
impl From<Wupe28> for u8 {
    #[inline(always)]
    fn from(val: Wupe28) -> u8 {
        Wupe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe29 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe29 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe29 {
    #[inline(always)]
    fn from(val: u8) -> Wupe29 {
        Wupe29::from_bits(val)
    }
}
impl From<Wupe29> for u8 {
    #[inline(always)]
    fn from(val: Wupe29) -> u8 {
        Wupe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe3 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe3 {
    #[inline(always)]
    fn from(val: u8) -> Wupe3 {
        Wupe3::from_bits(val)
    }
}
impl From<Wupe3> for u8 {
    #[inline(always)]
    fn from(val: Wupe3) -> u8 {
        Wupe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe30 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe30 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe30 {
    #[inline(always)]
    fn from(val: u8) -> Wupe30 {
        Wupe30::from_bits(val)
    }
}
impl From<Wupe30> for u8 {
    #[inline(always)]
    fn from(val: Wupe30) -> u8 {
        Wupe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe31 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe31 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe31 {
    #[inline(always)]
    fn from(val: u8) -> Wupe31 {
        Wupe31::from_bits(val)
    }
}
impl From<Wupe31> for u8 {
    #[inline(always)]
    fn from(val: Wupe31) -> u8 {
        Wupe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe4 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe4 {
    #[inline(always)]
    fn from(val: u8) -> Wupe4 {
        Wupe4::from_bits(val)
    }
}
impl From<Wupe4> for u8 {
    #[inline(always)]
    fn from(val: Wupe4) -> u8 {
        Wupe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe5 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe5 {
    #[inline(always)]
    fn from(val: u8) -> Wupe5 {
        Wupe5::from_bits(val)
    }
}
impl From<Wupe5> for u8 {
    #[inline(always)]
    fn from(val: Wupe5) -> u8 {
        Wupe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe6 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe6 {
    #[inline(always)]
    fn from(val: u8) -> Wupe6 {
        Wupe6::from_bits(val)
    }
}
impl From<Wupe6> for u8 {
    #[inline(always)]
    fn from(val: Wupe6) -> u8 {
        Wupe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe7 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe7 {
    #[inline(always)]
    fn from(val: u8) -> Wupe7 {
        Wupe7::from_bits(val)
    }
}
impl From<Wupe7> for u8 {
    #[inline(always)]
    fn from(val: Wupe7) -> u8 {
        Wupe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe8 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe8 {
    #[inline(always)]
    fn from(val: u8) -> Wupe8 {
        Wupe8::from_bits(val)
    }
}
impl From<Wupe8> for u8 {
    #[inline(always)]
    fn from(val: Wupe8) -> u8 {
        Wupe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe9 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (detect on rising edge or high level)"]
    EN_RISE_HI = 0x01,
    #[doc = "Enable (detect on falling edge or low level)"]
    EN_FALL_LO = 0x02,
    #[doc = "Enable (detect on any edge)"]
    EN_ANY = 0x03,
}
impl Wupe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe9 {
    #[inline(always)]
    fn from(val: u8) -> Wupe9 {
        Wupe9::from_bits(val)
    }
}
impl From<Wupe9> for u8 {
    #[inline(always)]
    fn from(val: Wupe9) -> u8 {
        Wupe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc0 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc0 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc0 {
        Wupmc0::from_bits(val)
    }
}
impl From<Wupmc0> for u8 {
    #[inline(always)]
    fn from(val: Wupmc0) -> u8 {
        Wupmc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc1 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc1 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc1 {
        Wupmc1::from_bits(val)
    }
}
impl From<Wupmc1> for u8 {
    #[inline(always)]
    fn from(val: Wupmc1) -> u8 {
        Wupmc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc10 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc10 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc10 {
        Wupmc10::from_bits(val)
    }
}
impl From<Wupmc10> for u8 {
    #[inline(always)]
    fn from(val: Wupmc10) -> u8 {
        Wupmc10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc11 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc11 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc11 {
        Wupmc11::from_bits(val)
    }
}
impl From<Wupmc11> for u8 {
    #[inline(always)]
    fn from(val: Wupmc11) -> u8 {
        Wupmc11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc12 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc12 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc12 {
        Wupmc12::from_bits(val)
    }
}
impl From<Wupmc12> for u8 {
    #[inline(always)]
    fn from(val: Wupmc12) -> u8 {
        Wupmc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc13 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc13 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc13 {
        Wupmc13::from_bits(val)
    }
}
impl From<Wupmc13> for u8 {
    #[inline(always)]
    fn from(val: Wupmc13) -> u8 {
        Wupmc13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc14 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc14 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc14 {
        Wupmc14::from_bits(val)
    }
}
impl From<Wupmc14> for u8 {
    #[inline(always)]
    fn from(val: Wupmc14) -> u8 {
        Wupmc14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc15 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc15 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc15 {
        Wupmc15::from_bits(val)
    }
}
impl From<Wupmc15> for u8 {
    #[inline(always)]
    fn from(val: Wupmc15) -> u8 {
        Wupmc15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc16 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc16 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc16 {
        Wupmc16::from_bits(val)
    }
}
impl From<Wupmc16> for u8 {
    #[inline(always)]
    fn from(val: Wupmc16) -> u8 {
        Wupmc16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc17 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc17 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc17 {
        Wupmc17::from_bits(val)
    }
}
impl From<Wupmc17> for u8 {
    #[inline(always)]
    fn from(val: Wupmc17) -> u8 {
        Wupmc17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc18 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc18 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc18 {
        Wupmc18::from_bits(val)
    }
}
impl From<Wupmc18> for u8 {
    #[inline(always)]
    fn from(val: Wupmc18) -> u8 {
        Wupmc18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc19 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc19 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc19 {
        Wupmc19::from_bits(val)
    }
}
impl From<Wupmc19> for u8 {
    #[inline(always)]
    fn from(val: Wupmc19) -> u8 {
        Wupmc19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc2 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc2 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc2 {
        Wupmc2::from_bits(val)
    }
}
impl From<Wupmc2> for u8 {
    #[inline(always)]
    fn from(val: Wupmc2) -> u8 {
        Wupmc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc20 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc20 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc20 {
        Wupmc20::from_bits(val)
    }
}
impl From<Wupmc20> for u8 {
    #[inline(always)]
    fn from(val: Wupmc20) -> u8 {
        Wupmc20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc21 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc21 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc21 {
        Wupmc21::from_bits(val)
    }
}
impl From<Wupmc21> for u8 {
    #[inline(always)]
    fn from(val: Wupmc21) -> u8 {
        Wupmc21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc22 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc22 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc22 {
        Wupmc22::from_bits(val)
    }
}
impl From<Wupmc22> for u8 {
    #[inline(always)]
    fn from(val: Wupmc22) -> u8 {
        Wupmc22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc23 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc23 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc23 {
        Wupmc23::from_bits(val)
    }
}
impl From<Wupmc23> for u8 {
    #[inline(always)]
    fn from(val: Wupmc23) -> u8 {
        Wupmc23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc24 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc24 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc24 {
        Wupmc24::from_bits(val)
    }
}
impl From<Wupmc24> for u8 {
    #[inline(always)]
    fn from(val: Wupmc24) -> u8 {
        Wupmc24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc25 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc25 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc25 {
        Wupmc25::from_bits(val)
    }
}
impl From<Wupmc25> for u8 {
    #[inline(always)]
    fn from(val: Wupmc25) -> u8 {
        Wupmc25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc26 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc26 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc26 {
        Wupmc26::from_bits(val)
    }
}
impl From<Wupmc26> for u8 {
    #[inline(always)]
    fn from(val: Wupmc26) -> u8 {
        Wupmc26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc27 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc27 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc27 {
        Wupmc27::from_bits(val)
    }
}
impl From<Wupmc27> for u8 {
    #[inline(always)]
    fn from(val: Wupmc27) -> u8 {
        Wupmc27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc28 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc28 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc28 {
        Wupmc28::from_bits(val)
    }
}
impl From<Wupmc28> for u8 {
    #[inline(always)]
    fn from(val: Wupmc28) -> u8 {
        Wupmc28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc29 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc29 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc29 {
        Wupmc29::from_bits(val)
    }
}
impl From<Wupmc29> for u8 {
    #[inline(always)]
    fn from(val: Wupmc29) -> u8 {
        Wupmc29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc3 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc3 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc3 {
        Wupmc3::from_bits(val)
    }
}
impl From<Wupmc3> for u8 {
    #[inline(always)]
    fn from(val: Wupmc3) -> u8 {
        Wupmc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc30 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc30 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc30 {
        Wupmc30::from_bits(val)
    }
}
impl From<Wupmc30> for u8 {
    #[inline(always)]
    fn from(val: Wupmc30) -> u8 {
        Wupmc30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc31 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc31 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc31 {
        Wupmc31::from_bits(val)
    }
}
impl From<Wupmc31> for u8 {
    #[inline(always)]
    fn from(val: Wupmc31) -> u8 {
        Wupmc31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc4 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc4 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc4 {
        Wupmc4::from_bits(val)
    }
}
impl From<Wupmc4> for u8 {
    #[inline(always)]
    fn from(val: Wupmc4) -> u8 {
        Wupmc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc5 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc5 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc5 {
        Wupmc5::from_bits(val)
    }
}
impl From<Wupmc5> for u8 {
    #[inline(always)]
    fn from(val: Wupmc5) -> u8 {
        Wupmc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc6 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc6 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc6 {
        Wupmc6::from_bits(val)
    }
}
impl From<Wupmc6> for u8 {
    #[inline(always)]
    fn from(val: Wupmc6) -> u8 {
        Wupmc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc7 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc7 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc7 {
        Wupmc7::from_bits(val)
    }
}
impl From<Wupmc7> for u8 {
    #[inline(always)]
    fn from(val: Wupmc7) -> u8 {
        Wupmc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc8 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc8 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc8 {
        Wupmc8::from_bits(val)
    }
}
impl From<Wupmc8> for u8 {
    #[inline(always)]
    fn from(val: Wupmc8) -> u8 {
        Wupmc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc9 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LOW_PWR_ONLY = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    ANY_PWR = 0x01,
}
impl Wupmc9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc9 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc9 {
        Wupmc9::from_bits(val)
    }
}
impl From<Wupmc9> for u8 {
    #[inline(always)]
    fn from(val: Wupmc9) -> u8 {
        Wupmc9::to_bits(val)
    }
}
