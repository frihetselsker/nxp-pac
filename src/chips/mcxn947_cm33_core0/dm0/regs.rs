#[doc = "Command and Status Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csw(pub u32);
impl Csw {
    #[doc = "Resynchronization Request"]
    #[must_use]
    #[inline(always)]
    pub const fn resynch_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Resynchronization Request"]
    #[inline(always)]
    pub const fn set_resynch_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn req_pending(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request Pending"]
    #[inline(always)]
    pub const fn set_req_pending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DBGMB Overrun Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_or_err(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DBGMB Overrun Error"]
    #[inline(always)]
    pub const fn set_dbg_or_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AHB Overrun Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_or_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Overrun Error"]
    #[inline(always)]
    pub const fn set_ahb_or_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Soft Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn soft_reset(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Soft Reset"]
    #[inline(always)]
    pub const fn set_soft_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Chip Reset Request"]
    #[must_use]
    #[inline(always)]
    pub const fn chip_reset_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Reset Request"]
    #[inline(always)]
    pub const fn set_chip_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Csw {
    #[inline(always)]
    fn default() -> Csw {
        Csw(0)
    }
}
impl core::fmt::Debug for Csw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csw")
            .field("resynch_req", &self.resynch_req())
            .field("req_pending", &self.req_pending())
            .field("dbg_or_err", &self.dbg_or_err())
            .field("ahb_or_err", &self.ahb_or_err())
            .field("soft_reset", &self.soft_reset())
            .field("chip_reset_req", &self.chip_reset_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csw {{ resynch_req: {=bool:?}, req_pending: {=bool:?}, dbg_or_err: {=bool:?}, ahb_or_err: {=bool:?}, soft_reset: {=bool:?}, chip_reset_req: {=bool:?} }}",
            self.resynch_req(),
            self.req_pending(),
            self.dbg_or_err(),
            self.ahb_or_err(),
            self.soft_reset(),
            self.chip_reset_req()
        )
    }
}
#[doc = "Identification"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Identification Value"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Identification Value"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Id").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Id {{ id: {=u32:?} }}", self.id())
    }
}
#[doc = "Request Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Request(pub u32);
impl Request {
    #[doc = "Request Value"]
    #[must_use]
    #[inline(always)]
    pub const fn request(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Request Value"]
    #[inline(always)]
    pub const fn set_request(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Request {
    #[inline(always)]
    fn default() -> Request {
        Request(0)
    }
}
impl core::fmt::Debug for Request {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Request")
            .field("request", &self.request())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Request {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Request {{ request: {=u32:?} }}", self.request())
    }
}
#[doc = "Return Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Return(pub u32);
impl Return {
    #[doc = "Return Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ret(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Return Value"]
    #[inline(always)]
    pub const fn set_ret(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Return {
    #[inline(always)]
    fn default() -> Return {
        Return(0)
    }
}
impl core::fmt::Debug for Return {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Return").field("ret", &self.ret()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Return {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Return {{ ret: {=u32:?} }}", self.ret())
    }
}
