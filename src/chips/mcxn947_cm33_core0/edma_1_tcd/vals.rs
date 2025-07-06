#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwc {
    #[doc = "No eDMA engine stalls"]
    NO_STALL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    ENGINE_STALLS_FOUR = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    ENGINE_STALLS_EIGHT = 0x03,
}
impl Bwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwc {
    #[inline(always)]
    fn from(val: u8) -> Bwc {
        Bwc::from_bits(val)
    }
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(val: Bwc) -> u8 {
        Bwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dae {
    #[doc = "No destination address configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DADDR field"]
    ERROR = 0x01,
}
impl Dae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dae {
    #[inline(always)]
    fn from(val: u8) -> Dae {
        Dae::from_bits(val)
    }
}
impl From<Dae> for u8 {
    #[inline(always)]
    fn from(val: Dae) -> u8 {
        Dae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbe {
    #[doc = "No destination bus error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was bus error on destination write"]
    ERROR = 0x01,
}
impl Dbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbe {
    #[inline(always)]
    fn from(val: u8) -> Dbe {
        Dbe::from_bits(val)
    }
}
impl From<Dbe> for u8 {
    #[inline(always)]
    fn from(val: Dbe) -> u8 {
        Dbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doe {
    #[doc = "No destination offset configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DOFF field"]
    ERROR = 0x01,
}
impl Doe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doe {
    #[inline(always)]
    fn from(val: u8) -> Doe {
        Doe::from_bits(val)
    }
}
impl From<Doe> for u8 {
    #[inline(always)]
    fn from(val: Doe) -> u8 {
        Doe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpa {
    #[doc = "Channel can suspend a lower-priority channel"]
    SUSPEND = 0x0,
    #[doc = "Channel cannot suspend any other channel, regardless of channel priority"]
    CANNOT_SUSPEND = 0x01,
}
impl Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dpa {
        Dpa::from_bits(val)
    }
}
impl From<Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dpa) -> u8 {
        Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dreq {
    #[doc = "No operation"]
    CHANNEL_NOT_AFFECTED = 0x0,
    #[doc = "Clear the ERQ field to 0 upon major loop completion, thus disabling hardware service requests"]
    ERQ_FIELD_CLEAR = 0x01,
}
impl Dreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dreq {
    #[inline(always)]
    fn from(val: u8) -> Dreq {
        Dreq::from_bits(val)
    }
}
impl From<Dreq> for u8 {
    #[inline(always)]
    fn from(val: Dreq) -> u8 {
        Dreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Earq {
    #[doc = "Disable asynchronous DMA request for the channel"]
    DISABLE = 0x0,
    #[doc = "Enable asynchronous DMA request for the channel"]
    ENABLE = 0x01,
}
impl Earq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Earq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Earq {
    #[inline(always)]
    fn from(val: u8) -> Earq {
        Earq::from_bits(val)
    }
}
impl From<Earq> for u8 {
    #[inline(always)]
    fn from(val: Earq) -> u8 {
        Earq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ebw {
    #[doc = "Buffered writes on system bus disabled"]
    DISABLE = 0x0,
    #[doc = "Buffered writes on system bus enabled"]
    ENABLE = 0x01,
}
impl Ebw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ebw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ebw {
    #[inline(always)]
    fn from(val: u8) -> Ebw {
        Ebw::from_bits(val)
    }
}
impl From<Ebw> for u8 {
    #[inline(always)]
    fn from(val: Ebw) -> u8 {
        Ebw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ecp {
    #[doc = "Channel cannot be suspended by a higher-priority channel's service request"]
    CANNOT_SUSPEND = 0x0,
    #[doc = "Channel can be temporarily suspended by a higher-priority channel's service request"]
    SUSPEND = 0x01,
}
impl Ecp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ecp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ecp {
    #[inline(always)]
    fn from(val: u8) -> Ecp {
        Ecp::from_bits(val)
    }
}
impl From<Ecp> for u8 {
    #[inline(always)]
    fn from(val: Ecp) -> u8 {
        Ecp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eei {
    #[doc = "Error signal for corresponding channel does not generate error interrupt"]
    NO_ERROR = 0x0,
    #[doc = "Assertion of error signal for corresponding channel generates error interrupt request"]
    ERROR = 0x01,
}
impl Eei {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eei {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eei {
    #[inline(always)]
    fn from(val: u8) -> Eei {
        Eei::from_bits(val)
    }
}
impl From<Eei> for u8 {
    #[inline(always)]
    fn from(val: Eei) -> u8 {
        Eei::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eeop {
    #[doc = "End-of-packet operation disabled"]
    DISABLE = 0x0,
    #[doc = "End-of-packet hardware input signal enabled"]
    ENABLE = 0x01,
}
impl Eeop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eeop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eeop {
    #[inline(always)]
    fn from(val: u8) -> Eeop {
        Eeop::from_bits(val)
    }
}
impl From<Eeop> for u8 {
    #[inline(always)]
    fn from(val: Eeop) -> u8 {
        Eeop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emi {
    #[doc = "Master ID replication is disabled"]
    DISABLE = 0x0,
    #[doc = "Master ID replication is enabled"]
    ENABLE = 0x01,
}
impl Emi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emi {
    #[inline(always)]
    fn from(val: u8) -> Emi {
        Emi::from_bits(val)
    }
}
impl From<Emi> for u8 {
    #[inline(always)]
    fn from(val: Emi) -> u8 {
        Emi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erq {
    #[doc = "DMA hardware request signal for corresponding channel disabled"]
    DISABLE = 0x0,
    #[doc = "DMA hardware request signal for corresponding channel enabled"]
    ENABLE = 0x01,
}
impl Erq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erq {
    #[inline(always)]
    fn from(val: u8) -> Erq {
        Erq::from_bits(val)
    }
}
impl From<Erq> for u8 {
    #[inline(always)]
    fn from(val: Erq) -> u8 {
        Erq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Err {
    #[doc = "An error in this channel has not occurred"]
    NO_ERROR = 0x0,
    #[doc = "An error in this channel has occurred"]
    ERROR = 0x01,
}
impl Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Err {
    #[inline(always)]
    fn from(val: u8) -> Err {
        Err::from_bits(val)
    }
}
impl From<Err> for u8 {
    #[inline(always)]
    fn from(val: Err) -> u8 {
        Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esda {
    #[doc = "Ability to store destination address to system memory disabled"]
    DISABLE = 0x0,
    #[doc = "Ability to store destination address to system memory enabled"]
    ENABLE = 0x01,
}
impl Esda {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esda {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esda {
    #[inline(always)]
    fn from(val: u8) -> Esda {
        Esda::from_bits(val)
    }
}
impl From<Esda> for u8 {
    #[inline(always)]
    fn from(val: Esda) -> u8 {
        Esda::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esg {
    #[doc = "Current channel's TCD is normal format"]
    NORMAL_FORMAT = 0x0,
    #[doc = "Current channel's TCD specifies scatter/gather format."]
    SCATTER_GATHER_FORMAT = 0x01,
}
impl Esg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esg {
    #[inline(always)]
    fn from(val: u8) -> Esg {
        Esg::from_bits(val)
    }
}
impl From<Esg> for u8 {
    #[inline(always)]
    fn from(val: Esg) -> u8 {
        Esg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int {
    #[doc = "Interrupt request for corresponding channel cleared"]
    INTERRUPT_CLEARED = 0x0,
    #[doc = "Interrupt request for corresponding channel active"]
    INTERRUPT_ACTIVE = 0x01,
}
impl Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int {
    #[inline(always)]
    fn from(val: u8) -> Int {
        Int::from_bits(val)
    }
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(val: Int) -> u8 {
        Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inthalf {
    #[doc = "Halfway point interrupt disabled"]
    DISABLE = 0x0,
    #[doc = "Halfway point interrupt enabled"]
    ENABLE = 0x01,
}
impl Inthalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inthalf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inthalf {
    #[inline(always)]
    fn from(val: u8) -> Inthalf {
        Inthalf::from_bits(val)
    }
}
impl From<Inthalf> for u8 {
    #[inline(always)]
    fn from(val: Inthalf) -> u8 {
        Inthalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intmajor {
    #[doc = "End-of-major loop interrupt disabled"]
    DISABLE = 0x0,
    #[doc = "End-of-major loop interrupt enabled"]
    ENABLE = 0x01,
}
impl Intmajor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intmajor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intmajor {
    #[inline(always)]
    fn from(val: u8) -> Intmajor {
        Intmajor::from_bits(val)
    }
}
impl From<Intmajor> for u8 {
    #[inline(always)]
    fn from(val: Intmajor) -> u8 {
        Intmajor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Majorelink {
    #[doc = "Channel-to-channel linking disabled"]
    DISABLE = 0x0,
    #[doc = "Channel-to-channel linking enabled"]
    ENABLE = 0x01,
}
impl Majorelink {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Majorelink {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Majorelink {
    #[inline(always)]
    fn from(val: u8) -> Majorelink {
        Majorelink::from_bits(val)
    }
}
impl From<Majorelink> for u8 {
    #[inline(always)]
    fn from(val: Majorelink) -> u8 {
        Majorelink::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce {
    #[doc = "No NBYTES/CITER configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields"]
    ERROR = 0x01,
}
impl Nce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce {
    #[inline(always)]
    fn from(val: u8) -> Nce {
        Nce::from_bits(val)
    }
}
impl From<Nce> for u8 {
    #[inline(always)]
    fn from(val: Nce) -> u8 {
        Nce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pal {
    #[doc = "User protection level for DMA transfers"]
    USER_PROTECTION = 0x0,
    #[doc = "Privileged protection level for DMA transfers"]
    PRIVILEGED_PROTECTION = 0x01,
}
impl Pal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pal {
    #[inline(always)]
    fn from(val: u8) -> Pal {
        Pal::from_bits(val)
    }
}
impl From<Pal> for u8 {
    #[inline(always)]
    fn from(val: Pal) -> u8 {
        Pal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sae {
    #[doc = "No source address configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_SADDR field"]
    ERROR = 0x01,
}
impl Sae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sae {
    #[inline(always)]
    fn from(val: u8) -> Sae {
        Sae::from_bits(val)
    }
}
impl From<Sae> for u8 {
    #[inline(always)]
    fn from(val: Sae) -> u8 {
        Sae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbe {
    #[doc = "No source bus error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was bus error on source read"]
    ERROR = 0x01,
}
impl Sbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbe {
    #[inline(always)]
    fn from(val: u8) -> Sbe {
        Sbe::from_bits(val)
    }
}
impl From<Sbe> for u8 {
    #[inline(always)]
    fn from(val: Sbe) -> u8 {
        Sbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sec {
    #[doc = "Nonsecure protection level for DMA transfers"]
    NONSECURE_PROTECTION = 0x0,
    #[doc = "Secure protection level for DMA transfers"]
    SECURE_PROTECTION = 0x01,
}
impl Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sec {
    #[inline(always)]
    fn from(val: u8) -> Sec {
        Sec::from_bits(val)
    }
}
impl From<Sec> for u8 {
    #[inline(always)]
    fn from(val: Sec) -> u8 {
        Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sge {
    #[doc = "No scatter/gather configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field"]
    ERROR = 0x01,
}
impl Sge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sge {
    #[inline(always)]
    fn from(val: u8) -> Sge {
        Sge::from_bits(val)
    }
}
impl From<Sge> for u8 {
    #[inline(always)]
    fn from(val: Sge) -> u8 {
        Sge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod {
    #[doc = "Source address modulo feature disabled"]
    DISABLE = 0x0,
    #[doc = "Source address modulo feature enabled for any non-zero value \\[1-31\\]"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Smod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soe {
    #[doc = "No source offset configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_SOFF field"]
    ERROR = 0x01,
}
impl Soe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soe {
    #[inline(always)]
    fn from(val: u8) -> Soe {
        Soe::from_bits(val)
    }
}
impl From<Soe> for u8 {
    #[inline(always)]
    fn from(val: Soe) -> u8 {
        Soe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssize {
    #[doc = "8-bit"]
    EIGHT_BIT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR_BIT = 0x03,
    #[doc = "16-byte"]
    SIXTEEN_BYTE = 0x04,
    #[doc = "32-byte"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ssize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssize {
    #[inline(always)]
    fn from(val: u8) -> Ssize {
        Ssize::from_bits(val)
    }
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(val: Ssize) -> u8 {
        Ssize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Channel not explicitly started"]
    CHANNEL_NOT_STARTED = 0x0,
    #[doc = "Channel explicitly started via a software-initiated service request"]
    CHANNEL_STARTED = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdBiterElinknoElink {
    #[doc = "Channel-to-channel linking disabled"]
    DISABLE = 0x0,
    #[doc = "Channel-to-channel linking enabled"]
    ENABLE = 0x01,
}
impl TcdBiterElinknoElink {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdBiterElinknoElink {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdBiterElinknoElink {
    #[inline(always)]
    fn from(val: u8) -> TcdBiterElinknoElink {
        TcdBiterElinknoElink::from_bits(val)
    }
}
impl From<TcdBiterElinknoElink> for u8 {
    #[inline(always)]
    fn from(val: TcdBiterElinknoElink) -> u8 {
        TcdBiterElinknoElink::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdBiterElinkyesElink {
    #[doc = "Channel-to-channel linking disabled"]
    DISABLE = 0x0,
    #[doc = "Channel-to-channel linking enabled"]
    ENABLE = 0x01,
}
impl TcdBiterElinkyesElink {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdBiterElinkyesElink {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdBiterElinkyesElink {
    #[inline(always)]
    fn from(val: u8) -> TcdBiterElinkyesElink {
        TcdBiterElinkyesElink::from_bits(val)
    }
}
impl From<TcdBiterElinkyesElink> for u8 {
    #[inline(always)]
    fn from(val: TcdBiterElinkyesElink) -> u8 {
        TcdBiterElinkyesElink::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdCiterElinknoElink {
    #[doc = "Channel-to-channel linking disabled"]
    DISABLE = 0x0,
    #[doc = "Channel-to-channel linking enabled"]
    ENABLE = 0x01,
}
impl TcdCiterElinknoElink {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdCiterElinknoElink {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdCiterElinknoElink {
    #[inline(always)]
    fn from(val: u8) -> TcdCiterElinknoElink {
        TcdCiterElinknoElink::from_bits(val)
    }
}
impl From<TcdCiterElinknoElink> for u8 {
    #[inline(always)]
    fn from(val: TcdCiterElinknoElink) -> u8 {
        TcdCiterElinknoElink::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdCiterElinkyesElink {
    #[doc = "Channel-to-channel linking disabled"]
    DISABLE = 0x0,
    #[doc = "Channel-to-channel linking enabled"]
    ENABLE = 0x01,
}
impl TcdCiterElinkyesElink {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdCiterElinkyesElink {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdCiterElinkyesElink {
    #[inline(always)]
    fn from(val: u8) -> TcdCiterElinkyesElink {
        TcdCiterElinkyesElink::from_bits(val)
    }
}
impl From<TcdCiterElinkyesElink> for u8 {
    #[inline(always)]
    fn from(val: TcdCiterElinkyesElink) -> u8 {
        TcdCiterElinkyesElink::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffnoDmloe {
    #[doc = "Minor loop offset not applied to DADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to DADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffnoDmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffnoDmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffnoDmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffnoDmloe {
        TcdNbytesMloffnoDmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffnoDmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffnoDmloe) -> u8 {
        TcdNbytesMloffnoDmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffnoSmloe {
    #[doc = "Minor loop offset not applied to SADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to SADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffnoSmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffnoSmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffnoSmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffnoSmloe {
        TcdNbytesMloffnoSmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffnoSmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffnoSmloe) -> u8 {
        TcdNbytesMloffnoSmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffyesDmloe {
    #[doc = "Minor loop offset not applied to DADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to DADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffyesDmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffyesDmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffyesDmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffyesDmloe {
        TcdNbytesMloffyesDmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffyesDmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffyesDmloe) -> u8 {
        TcdNbytesMloffyesDmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffyesSmloe {
    #[doc = "Minor loop offset not applied to SADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to SADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffyesSmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffyesSmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffyesSmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffyesSmloe {
        TcdNbytesMloffyesSmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffyesSmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffyesSmloe) -> u8 {
        TcdNbytesMloffyesSmloe::to_bits(val)
    }
}
