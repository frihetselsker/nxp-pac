#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugEn {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl DebugEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugEn {
    #[inline(always)]
    fn from(val: u8) -> DebugEn {
        DebugEn::from_bits(val)
    }
}
impl From<DebugEn> for u8 {
    #[inline(always)]
    fn from(val: DebugEn) -> u8 {
        DebugEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lock {
    #[doc = "No Lock"]
    NO_LOCK = 0x0,
    #[doc = "Lock"]
    LOCK = 0x01,
}
impl Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wden {
    #[doc = "Timer stopped"]
    STOP = 0x0,
    #[doc = "Timer running"]
    RUN = 0x01,
}
impl Wden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wden {
    #[inline(always)]
    fn from(val: u8) -> Wden {
        Wden::from_bits(val)
    }
}
impl From<Wden> for u8 {
    #[inline(always)]
    fn from(val: Wden) -> u8 {
        Wden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdint {
    #[doc = "No flag"]
    NO_FLAG = 0x0,
    #[doc = "Flag"]
    FLAG = 0x01,
}
impl Wdint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdint {
    #[inline(always)]
    fn from(val: u8) -> Wdint {
        Wdint::from_bits(val)
    }
}
impl From<Wdint> for u8 {
    #[inline(always)]
    fn from(val: Wdint) -> u8 {
        Wdint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdprotect {
    #[doc = "Flexible"]
    FLEXIBLE = 0x0,
    #[doc = "Threshold"]
    THRESHOLD = 0x01,
}
impl Wdprotect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdprotect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdprotect {
    #[inline(always)]
    fn from(val: u8) -> Wdprotect {
        Wdprotect::from_bits(val)
    }
}
impl From<Wdprotect> for u8 {
    #[inline(always)]
    fn from(val: Wdprotect) -> u8 {
        Wdprotect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdreset {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl Wdreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdreset {
    #[inline(always)]
    fn from(val: u8) -> Wdreset {
        Wdreset::from_bits(val)
    }
}
impl From<Wdreset> for u8 {
    #[inline(always)]
    fn from(val: Wdreset) -> u8 {
        Wdreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdtof {
    #[doc = "Watchdog event has not occurred."]
    CLEAR = 0x0,
    #[doc = "Watchdog event has occurred (causes a chip reset if WDRESET = 1)."]
    RESET = 0x01,
}
impl Wdtof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdtof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdtof {
    #[inline(always)]
    fn from(val: u8) -> Wdtof {
        Wdtof::from_bits(val)
    }
}
impl From<Wdtof> for u8 {
    #[inline(always)]
    fn from(val: Wdtof) -> u8 {
        Wdtof::to_bits(val)
    }
}
