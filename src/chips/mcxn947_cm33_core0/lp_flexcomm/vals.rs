#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cm {
    #[doc = "Clear"]
    CLR = 0x0,
    #[doc = "Set"]
    SET = 0x01,
}
impl I2cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cm {
    #[inline(always)]
    fn from(val: u8) -> I2cm {
        I2cm::from_bits(val)
    }
}
impl From<I2cm> for u8 {
    #[inline(always)]
    fn from(val: I2cm) -> u8 {
        I2cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cpresent {
    #[doc = "Not supported"]
    FALSE = 0x0,
    #[doc = "Supported"]
    TRUE = 0x01,
}
impl I2cpresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cpresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cpresent {
    #[inline(always)]
    fn from(val: u8) -> I2cpresent {
        I2cpresent::from_bits(val)
    }
}
impl From<I2cpresent> for u8 {
    #[inline(always)]
    fn from(val: I2cpresent) -> u8 {
        I2cpresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cs {
    #[doc = "Clear"]
    CLR = 0x0,
    #[doc = "Set"]
    SET = 0x01,
}
impl I2cs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cs {
    #[inline(always)]
    fn from(val: u8) -> I2cs {
        I2cs::from_bits(val)
    }
}
impl From<I2cs> for u8 {
    #[inline(always)]
    fn from(val: I2cs) -> u8 {
        I2cs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lock {
    #[doc = "PERSEL is writable"]
    FALSE = 0x0,
    #[doc = "PERSEL is not writable"]
    TRUE = 0x01,
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
pub enum Persel {
    #[doc = "No peripheral selected"]
    NONE = 0x0,
    #[doc = "UART"]
    UART = 0x01,
    #[doc = "SPI"]
    SPI = 0x02,
    #[doc = "I2C"]
    I2C = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "UART and I2C"]
    UARTI2C = 0x07,
}
impl Persel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Persel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Persel {
    #[inline(always)]
    fn from(val: u8) -> Persel {
        Persel::from_bits(val)
    }
}
impl From<Persel> for u8 {
    #[inline(always)]
    fn from(val: Persel) -> u8 {
        Persel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spi {
    #[doc = "Clear"]
    CLR = 0x0,
    #[doc = "Set"]
    SET = 0x01,
}
impl Spi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi {
    #[inline(always)]
    fn from(val: u8) -> Spi {
        Spi::from_bits(val)
    }
}
impl From<Spi> for u8 {
    #[inline(always)]
    fn from(val: Spi) -> u8 {
        Spi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spipresent {
    #[doc = "Not supported"]
    FALSE = 0x0,
    #[doc = "Supported"]
    TRUE = 0x01,
}
impl Spipresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spipresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spipresent {
    #[inline(always)]
    fn from(val: u8) -> Spipresent {
        Spipresent::from_bits(val)
    }
}
impl From<Spipresent> for u8 {
    #[inline(always)]
    fn from(val: Spipresent) -> u8 {
        Spipresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartpresent {
    #[doc = "Not supported"]
    FALSE = 0x0,
    #[doc = "Supported"]
    TRUE = 0x01,
}
impl Uartpresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartpresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartpresent {
    #[inline(always)]
    fn from(val: u8) -> Uartpresent {
        Uartpresent::from_bits(val)
    }
}
impl From<Uartpresent> for u8 {
    #[inline(always)]
    fn from(val: Uartpresent) -> u8 {
        Uartpresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartrx {
    #[doc = "Clear"]
    CLR = 0x0,
    #[doc = "Set"]
    SET = 0x01,
}
impl Uartrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartrx {
    #[inline(always)]
    fn from(val: u8) -> Uartrx {
        Uartrx::from_bits(val)
    }
}
impl From<Uartrx> for u8 {
    #[inline(always)]
    fn from(val: Uartrx) -> u8 {
        Uartrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uarttx {
    #[doc = "Clear"]
    CLR = 0x0,
    #[doc = "Set"]
    SET = 0x01,
}
impl Uarttx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uarttx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uarttx {
    #[inline(always)]
    fn from(val: u8) -> Uarttx {
        Uarttx::from_bits(val)
    }
}
impl From<Uarttx> for u8 {
    #[inline(always)]
    fn from(val: Uarttx) -> u8 {
        Uarttx::to_bits(val)
    }
}
