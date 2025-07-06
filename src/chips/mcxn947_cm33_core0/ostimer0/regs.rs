#[doc = "Local Capture High for CPU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptureH(pub u32);
impl CaptureH {
    #[doc = "EVTimer Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "EVTimer Capture Value"]
    #[inline(always)]
    pub const fn set_capture_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for CaptureH {
    #[inline(always)]
    fn default() -> CaptureH {
        CaptureH(0)
    }
}
impl core::fmt::Debug for CaptureH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CaptureH")
            .field("capture_value", &self.capture_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CaptureH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CaptureH {{ capture_value: {=u16:?} }}",
            self.capture_value()
        )
    }
}
#[doc = "Local Capture Low for CPU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptureL(pub u32);
impl CaptureL {
    #[doc = "EVTimer Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "EVTimer Capture Value"]
    #[inline(always)]
    pub const fn set_capture_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CaptureL {
    #[inline(always)]
    fn default() -> CaptureL {
        CaptureL(0)
    }
}
impl core::fmt::Debug for CaptureL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CaptureL")
            .field("capture_value", &self.capture_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CaptureL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CaptureL {{ capture_value: {=u32:?} }}",
            self.capture_value()
        )
    }
}
#[doc = "EVTIMER High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtimerh(pub u32);
impl Evtimerh {
    #[doc = "EVTimer Count Value"]
    #[must_use]
    #[inline(always)]
    pub const fn evtimer_count_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "EVTimer Count Value"]
    #[inline(always)]
    pub const fn set_evtimer_count_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Evtimerh {
    #[inline(always)]
    fn default() -> Evtimerh {
        Evtimerh(0)
    }
}
impl core::fmt::Debug for Evtimerh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Evtimerh")
            .field("evtimer_count_value", &self.evtimer_count_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evtimerh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Evtimerh {{ evtimer_count_value: {=u16:?} }}",
            self.evtimer_count_value()
        )
    }
}
#[doc = "EVTIMER Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtimerl(pub u32);
impl Evtimerl {
    #[doc = "EVTimer Count Value"]
    #[must_use]
    #[inline(always)]
    pub const fn evtimer_count_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "EVTimer Count Value"]
    #[inline(always)]
    pub const fn set_evtimer_count_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Evtimerl {
    #[inline(always)]
    fn default() -> Evtimerl {
        Evtimerl(0)
    }
}
impl core::fmt::Debug for Evtimerl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Evtimerl")
            .field("evtimer_count_value", &self.evtimer_count_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evtimerl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Evtimerl {{ evtimer_count_value: {=u32:?} }}",
            self.evtimer_count_value()
        )
    }
}
#[doc = "Local Match High for CPU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MatchH(pub u32);
impl MatchH {
    #[doc = "EVTimer Match Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "EVTimer Match Value"]
    #[inline(always)]
    pub const fn set_match_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for MatchH {
    #[inline(always)]
    fn default() -> MatchH {
        MatchH(0)
    }
}
impl core::fmt::Debug for MatchH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MatchH")
            .field("match_value", &self.match_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MatchH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MatchH {{ match_value: {=u16:?} }}", self.match_value())
    }
}
#[doc = "Local Match Low for CPU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MatchL(pub u32);
impl MatchL {
    #[doc = "EVTimer Match Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "EVTimer Match Value"]
    #[inline(always)]
    pub const fn set_match_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MatchL {
    #[inline(always)]
    fn default() -> MatchL {
        MatchL(0)
    }
}
impl core::fmt::Debug for MatchL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MatchL")
            .field("match_value", &self.match_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MatchL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MatchL {{ match_value: {=u32:?} }}", self.match_value())
    }
}
#[doc = "OSTIMER Control for CPU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OseventCtrl(pub u32);
impl OseventCtrl {
    #[doc = "Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer_intrflag(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub const fn set_ostimer_intrflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt or Wake-Up Request"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer_intena(&self) -> super::vals::OstimerIntena {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OstimerIntena::from_bits(val as u8)
    }
    #[doc = "Interrupt or Wake-Up Request"]
    #[inline(always)]
    pub const fn set_ostimer_intena(&mut self, val: super::vals::OstimerIntena) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "EVTimer Match Write Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn match_wr_rdy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EVTimer Match Write Ready"]
    #[inline(always)]
    pub const fn set_match_wr_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for OseventCtrl {
    #[inline(always)]
    fn default() -> OseventCtrl {
        OseventCtrl(0)
    }
}
impl core::fmt::Debug for OseventCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OseventCtrl")
            .field("ostimer_intrflag", &self.ostimer_intrflag())
            .field("ostimer_intena", &self.ostimer_intena())
            .field("match_wr_rdy", &self.match_wr_rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OseventCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OseventCtrl {{ ostimer_intrflag: {=bool:?}, ostimer_intena: {:?}, match_wr_rdy: {=bool:?} }}",
            self.ostimer_intrflag(),
            self.ostimer_intena(),
            self.match_wr_rdy()
        )
    }
}
