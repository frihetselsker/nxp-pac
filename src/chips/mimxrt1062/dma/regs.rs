#[doc = "Clear DONE Status Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdne(pub u8);
impl Cdne {
    #[doc = "Clear DONE field"]
    #[must_use]
    #[inline(always)]
    pub const fn cdne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear DONE field"]
    #[inline(always)]
    pub const fn set_cdne(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clears All DONE fields"]
    #[must_use]
    #[inline(always)]
    pub const fn cadn(&self) -> super::vals::Cadn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cadn::from_bits(val as u8)
    }
    #[doc = "Clears All DONE fields"]
    #[inline(always)]
    pub const fn set_cadn(&mut self, val: super::vals::Cadn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CdneNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CdneNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CdneNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cdne {
    #[inline(always)]
    fn default() -> Cdne {
        Cdne(0)
    }
}
impl core::fmt::Debug for Cdne {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdne")
            .field("cdne", &self.cdne())
            .field("cadn", &self.cadn())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdne {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdne {{ cdne: {=u8:?}, cadn: {:?}, nop: {:?} }}",
            self.cdne(),
            self.cadn(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceei(pub u8);
impl Ceei {
    #[doc = "Clear Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ceei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_ceei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Enable Error Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn caee(&self) -> super::vals::Caee {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caee::from_bits(val as u8)
    }
    #[doc = "Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub const fn set_caee(&mut self, val: super::vals::Caee) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CeeiNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CeeiNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CeeiNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ceei {
    #[inline(always)]
    fn default() -> Ceei {
        Ceei(0)
    }
}
impl core::fmt::Debug for Ceei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ceei")
            .field("ceei", &self.ceei())
            .field("caee", &self.caee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ceei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ceei {{ ceei: {=u8:?}, caee: {:?}, nop: {:?} }}",
            self.ceei(),
            self.caee(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerq(pub u8);
impl Cerq {
    #[doc = "Clear Enable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cerq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Enable Request"]
    #[inline(always)]
    pub const fn set_cerq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Enable Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn caer(&self) -> super::vals::Caer {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caer::from_bits(val as u8)
    }
    #[doc = "Clear All Enable Requests"]
    #[inline(always)]
    pub const fn set_caer(&mut self, val: super::vals::Caer) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CerqNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CerqNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CerqNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerq {
    #[inline(always)]
    fn default() -> Cerq {
        Cerq(0)
    }
}
impl core::fmt::Debug for Cerq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerq")
            .field("cerq", &self.cerq())
            .field("caer", &self.caer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerq {{ cerq: {=u8:?}, caer: {:?}, nop: {:?} }}",
            self.cerq(),
            self.caer(),
            self.nop()
        )
    }
}
#[doc = "Clear Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerr(pub u8);
impl Cerr {
    #[doc = "Clear Error Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn cerr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Error Indicator"]
    #[inline(always)]
    pub const fn set_cerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Error Indicators"]
    #[must_use]
    #[inline(always)]
    pub const fn caei(&self) -> super::vals::Caei {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caei::from_bits(val as u8)
    }
    #[doc = "Clear All Error Indicators"]
    #[inline(always)]
    pub const fn set_caei(&mut self, val: super::vals::Caei) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CerrNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CerrNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CerrNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerr {
    #[inline(always)]
    fn default() -> Cerr {
        Cerr(0)
    }
}
impl core::fmt::Debug for Cerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerr")
            .field("cerr", &self.cerr())
            .field("caei", &self.caei())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerr {{ cerr: {=u8:?}, caei: {:?}, nop: {:?} }}",
            self.cerr(),
            self.caei(),
            self.nop()
        )
    }
}
#[doc = "Clear Interrupt Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cint(pub u8);
impl Cint {
    #[doc = "Clear Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Interrupt Request"]
    #[inline(always)]
    pub const fn set_cint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Interrupt Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn cair(&self) -> super::vals::Cair {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cair::from_bits(val as u8)
    }
    #[doc = "Clear All Interrupt Requests"]
    #[inline(always)]
    pub const fn set_cair(&mut self, val: super::vals::Cair) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CintNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CintNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CintNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cint {
    #[inline(always)]
    fn default() -> Cint {
        Cint(0)
    }
}
impl core::fmt::Debug for Cint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cint")
            .field("cint", &self.cint())
            .field("cair", &self.cair())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cint {{ cint: {=u8:?}, cair: {:?}, nop: {:?} }}",
            self.cint(),
            self.cair(),
            self.nop()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Enable Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug"]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Round Robin Group Arbitration"]
    #[must_use]
    #[inline(always)]
    pub const fn erga(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Group Arbitration"]
    #[inline(always)]
    pub const fn set_erga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Halt On Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hoe(&self) -> super::vals::Hoe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hoe::from_bits(val as u8)
    }
    #[doc = "Halt On Error"]
    #[inline(always)]
    pub const fn set_hoe(&mut self, val: super::vals::Hoe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt eDMA Operations"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Halt eDMA Operations"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Continuous Link Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn clm(&self) -> super::vals::Clm {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Clm::from_bits(val as u8)
    }
    #[doc = "Continuous Link Mode"]
    #[inline(always)]
    pub const fn set_clm(&mut self, val: super::vals::Clm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Minor Loop Mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn emlm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Minor Loop Mapping"]
    #[inline(always)]
    pub const fn set_emlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel Group 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn grp0pri(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Group 0 Priority"]
    #[inline(always)]
    pub const fn set_grp0pri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel Group 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn grp1pri(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Group 1 Priority"]
    #[inline(always)]
    pub const fn set_grp1pri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::CrEcx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrEcx::from_bits(val as u8)
    }
    #[doc = "Error Cancel Transfer"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::CrEcx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> super::vals::Cx {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer"]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: super::vals::Cx) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "eDMA Active Status"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "eDMA Active Status"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("erga", &self.erga())
            .field("hoe", &self.hoe())
            .field("halt", &self.halt())
            .field("clm", &self.clm())
            .field("emlm", &self.emlm())
            .field("grp0pri", &self.grp0pri())
            .field("grp1pri", &self.grp1pri())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ edbg: {=bool:?}, erca: {=bool:?}, erga: {=bool:?}, hoe: {:?}, halt: {:?}, clm: {:?}, emlm: {=bool:?}, grp0pri: {=bool:?}, grp1pri: {=bool:?}, ecx: {:?}, cx: {:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.erga(),
            self.hoe(),
            self.halt(),
            self.clm(),
            self.emlm(),
            self.grp0pri(),
            self.grp1pri(),
            self.ecx(),
            self.cx(),
            self.active()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri(pub u8);
impl Dchpri {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority"]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::DchpriDpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DchpriDpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::DchpriDpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri {
    #[inline(always)]
    fn default() -> Dchpri {
        Dchpri(0)
    }
}
impl core::fmt::Debug for Dchpri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Enable Asynchronous Request in Stop"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ears(pub u32);
impl Ears {
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub const fn set_edreq(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ears {
    #[inline(always)]
    fn default() -> Ears {
        Ears(0)
    }
}
impl core::fmt::Debug for Ears {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ears")
            .field("edreq[0]", &self.edreq(0usize))
            .field("edreq[1]", &self.edreq(1usize))
            .field("edreq[2]", &self.edreq(2usize))
            .field("edreq[3]", &self.edreq(3usize))
            .field("edreq[4]", &self.edreq(4usize))
            .field("edreq[5]", &self.edreq(5usize))
            .field("edreq[6]", &self.edreq(6usize))
            .field("edreq[7]", &self.edreq(7usize))
            .field("edreq[8]", &self.edreq(8usize))
            .field("edreq[9]", &self.edreq(9usize))
            .field("edreq[10]", &self.edreq(10usize))
            .field("edreq[11]", &self.edreq(11usize))
            .field("edreq[12]", &self.edreq(12usize))
            .field("edreq[13]", &self.edreq(13usize))
            .field("edreq[14]", &self.edreq(14usize))
            .field("edreq[15]", &self.edreq(15usize))
            .field("edreq[16]", &self.edreq(16usize))
            .field("edreq[17]", &self.edreq(17usize))
            .field("edreq[18]", &self.edreq(18usize))
            .field("edreq[19]", &self.edreq(19usize))
            .field("edreq[20]", &self.edreq(20usize))
            .field("edreq[21]", &self.edreq(21usize))
            .field("edreq[22]", &self.edreq(22usize))
            .field("edreq[23]", &self.edreq(23usize))
            .field("edreq[24]", &self.edreq(24usize))
            .field("edreq[25]", &self.edreq(25usize))
            .field("edreq[26]", &self.edreq(26usize))
            .field("edreq[27]", &self.edreq(27usize))
            .field("edreq[28]", &self.edreq(28usize))
            .field("edreq[29]", &self.edreq(29usize))
            .field("edreq[30]", &self.edreq(30usize))
            .field("edreq[31]", &self.edreq(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ears {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ears {{ edreq[0]: {=bool:?}, edreq[1]: {=bool:?}, edreq[2]: {=bool:?}, edreq[3]: {=bool:?}, edreq[4]: {=bool:?}, edreq[5]: {=bool:?}, edreq[6]: {=bool:?}, edreq[7]: {=bool:?}, edreq[8]: {=bool:?}, edreq[9]: {=bool:?}, edreq[10]: {=bool:?}, edreq[11]: {=bool:?}, edreq[12]: {=bool:?}, edreq[13]: {=bool:?}, edreq[14]: {=bool:?}, edreq[15]: {=bool:?}, edreq[16]: {=bool:?}, edreq[17]: {=bool:?}, edreq[18]: {=bool:?}, edreq[19]: {=bool:?}, edreq[20]: {=bool:?}, edreq[21]: {=bool:?}, edreq[22]: {=bool:?}, edreq[23]: {=bool:?}, edreq[24]: {=bool:?}, edreq[25]: {=bool:?}, edreq[26]: {=bool:?}, edreq[27]: {=bool:?}, edreq[28]: {=bool:?}, edreq[29]: {=bool:?}, edreq[30]: {=bool:?}, edreq[31]: {=bool:?} }}",
            self.edreq(0usize),
            self.edreq(1usize),
            self.edreq(2usize),
            self.edreq(3usize),
            self.edreq(4usize),
            self.edreq(5usize),
            self.edreq(6usize),
            self.edreq(7usize),
            self.edreq(8usize),
            self.edreq(9usize),
            self.edreq(10usize),
            self.edreq(11usize),
            self.edreq(12usize),
            self.edreq(13usize),
            self.edreq(14usize),
            self.edreq(15usize),
            self.edreq(16usize),
            self.edreq(17usize),
            self.edreq(18usize),
            self.edreq(19usize),
            self.edreq(20usize),
            self.edreq(21usize),
            self.edreq(22usize),
            self.edreq(23usize),
            self.edreq(24usize),
            self.edreq(25usize),
            self.edreq(26usize),
            self.edreq(27usize),
            self.edreq(28usize),
            self.edreq(29usize),
            self.edreq(30usize),
            self.edreq(31usize)
        )
    }
}
#[doc = "Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eei(pub u32);
impl Eei {
    #[doc = "Enable Error Interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn eei(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 0"]
    #[inline(always)]
    pub const fn set_eei(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Eei {
    #[inline(always)]
    fn default() -> Eei {
        Eei(0)
    }
}
impl core::fmt::Debug for Eei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eei")
            .field("eei[0]", &self.eei(0usize))
            .field("eei[1]", &self.eei(1usize))
            .field("eei[2]", &self.eei(2usize))
            .field("eei[3]", &self.eei(3usize))
            .field("eei[4]", &self.eei(4usize))
            .field("eei[5]", &self.eei(5usize))
            .field("eei[6]", &self.eei(6usize))
            .field("eei[7]", &self.eei(7usize))
            .field("eei[8]", &self.eei(8usize))
            .field("eei[9]", &self.eei(9usize))
            .field("eei[10]", &self.eei(10usize))
            .field("eei[11]", &self.eei(11usize))
            .field("eei[12]", &self.eei(12usize))
            .field("eei[13]", &self.eei(13usize))
            .field("eei[14]", &self.eei(14usize))
            .field("eei[15]", &self.eei(15usize))
            .field("eei[16]", &self.eei(16usize))
            .field("eei[17]", &self.eei(17usize))
            .field("eei[18]", &self.eei(18usize))
            .field("eei[19]", &self.eei(19usize))
            .field("eei[20]", &self.eei(20usize))
            .field("eei[21]", &self.eei(21usize))
            .field("eei[22]", &self.eei(22usize))
            .field("eei[23]", &self.eei(23usize))
            .field("eei[24]", &self.eei(24usize))
            .field("eei[25]", &self.eei(25usize))
            .field("eei[26]", &self.eei(26usize))
            .field("eei[27]", &self.eei(27usize))
            .field("eei[28]", &self.eei(28usize))
            .field("eei[29]", &self.eei(29usize))
            .field("eei[30]", &self.eei(30usize))
            .field("eei[31]", &self.eei(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eei {{ eei[0]: {=bool:?}, eei[1]: {=bool:?}, eei[2]: {=bool:?}, eei[3]: {=bool:?}, eei[4]: {=bool:?}, eei[5]: {=bool:?}, eei[6]: {=bool:?}, eei[7]: {=bool:?}, eei[8]: {=bool:?}, eei[9]: {=bool:?}, eei[10]: {=bool:?}, eei[11]: {=bool:?}, eei[12]: {=bool:?}, eei[13]: {=bool:?}, eei[14]: {=bool:?}, eei[15]: {=bool:?}, eei[16]: {=bool:?}, eei[17]: {=bool:?}, eei[18]: {=bool:?}, eei[19]: {=bool:?}, eei[20]: {=bool:?}, eei[21]: {=bool:?}, eei[22]: {=bool:?}, eei[23]: {=bool:?}, eei[24]: {=bool:?}, eei[25]: {=bool:?}, eei[26]: {=bool:?}, eei[27]: {=bool:?}, eei[28]: {=bool:?}, eei[29]: {=bool:?}, eei[30]: {=bool:?}, eei[31]: {=bool:?} }}",
            self.eei(0usize),
            self.eei(1usize),
            self.eei(2usize),
            self.eei(3usize),
            self.eei(4usize),
            self.eei(5usize),
            self.eei(6usize),
            self.eei(7usize),
            self.eei(8usize),
            self.eei(9usize),
            self.eei(10usize),
            self.eei(11usize),
            self.eei(12usize),
            self.eei(13usize),
            self.eei(14usize),
            self.eei(15usize),
            self.eei(16usize),
            self.eei(17usize),
            self.eei(18usize),
            self.eei(19usize),
            self.eei(20usize),
            self.eei(21usize),
            self.eei(22usize),
            self.eei(23usize),
            self.eei(24usize),
            self.eei(25usize),
            self.eei(26usize),
            self.eei(27usize),
            self.eei(28usize),
            self.eei(29usize),
            self.eei(30usize),
            self.eei(31usize)
        )
    }
}
#[doc = "Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erq(pub u32);
impl Erq {
    #[doc = "Enable DMA Request 0"]
    #[must_use]
    #[inline(always)]
    pub const fn erq(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 0"]
    #[inline(always)]
    pub const fn set_erq(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Erq {
    #[inline(always)]
    fn default() -> Erq {
        Erq(0)
    }
}
impl core::fmt::Debug for Erq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erq")
            .field("erq[0]", &self.erq(0usize))
            .field("erq[1]", &self.erq(1usize))
            .field("erq[2]", &self.erq(2usize))
            .field("erq[3]", &self.erq(3usize))
            .field("erq[4]", &self.erq(4usize))
            .field("erq[5]", &self.erq(5usize))
            .field("erq[6]", &self.erq(6usize))
            .field("erq[7]", &self.erq(7usize))
            .field("erq[8]", &self.erq(8usize))
            .field("erq[9]", &self.erq(9usize))
            .field("erq[10]", &self.erq(10usize))
            .field("erq[11]", &self.erq(11usize))
            .field("erq[12]", &self.erq(12usize))
            .field("erq[13]", &self.erq(13usize))
            .field("erq[14]", &self.erq(14usize))
            .field("erq[15]", &self.erq(15usize))
            .field("erq[16]", &self.erq(16usize))
            .field("erq[17]", &self.erq(17usize))
            .field("erq[18]", &self.erq(18usize))
            .field("erq[19]", &self.erq(19usize))
            .field("erq[20]", &self.erq(20usize))
            .field("erq[21]", &self.erq(21usize))
            .field("erq[22]", &self.erq(22usize))
            .field("erq[23]", &self.erq(23usize))
            .field("erq[24]", &self.erq(24usize))
            .field("erq[25]", &self.erq(25usize))
            .field("erq[26]", &self.erq(26usize))
            .field("erq[27]", &self.erq(27usize))
            .field("erq[28]", &self.erq(28usize))
            .field("erq[29]", &self.erq(29usize))
            .field("erq[30]", &self.erq(30usize))
            .field("erq[31]", &self.erq(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erq {{ erq[0]: {=bool:?}, erq[1]: {=bool:?}, erq[2]: {=bool:?}, erq[3]: {=bool:?}, erq[4]: {=bool:?}, erq[5]: {=bool:?}, erq[6]: {=bool:?}, erq[7]: {=bool:?}, erq[8]: {=bool:?}, erq[9]: {=bool:?}, erq[10]: {=bool:?}, erq[11]: {=bool:?}, erq[12]: {=bool:?}, erq[13]: {=bool:?}, erq[14]: {=bool:?}, erq[15]: {=bool:?}, erq[16]: {=bool:?}, erq[17]: {=bool:?}, erq[18]: {=bool:?}, erq[19]: {=bool:?}, erq[20]: {=bool:?}, erq[21]: {=bool:?}, erq[22]: {=bool:?}, erq[23]: {=bool:?}, erq[24]: {=bool:?}, erq[25]: {=bool:?}, erq[26]: {=bool:?}, erq[27]: {=bool:?}, erq[28]: {=bool:?}, erq[29]: {=bool:?}, erq[30]: {=bool:?}, erq[31]: {=bool:?} }}",
            self.erq(0usize),
            self.erq(1usize),
            self.erq(2usize),
            self.erq(3usize),
            self.erq(4usize),
            self.erq(5usize),
            self.erq(6usize),
            self.erq(7usize),
            self.erq(8usize),
            self.erq(9usize),
            self.erq(10usize),
            self.erq(11usize),
            self.erq(12usize),
            self.erq(13usize),
            self.erq(14usize),
            self.erq(15usize),
            self.erq(16usize),
            self.erq(17usize),
            self.erq(18usize),
            self.erq(19usize),
            self.erq(20usize),
            self.erq(21usize),
            self.erq(22usize),
            self.erq(23usize),
            self.erq(24usize),
            self.erq(25usize),
            self.erq(26usize),
            self.erq(27usize),
            self.erq(28usize),
            self.erq(29usize),
            self.erq(30usize),
            self.erq(31usize)
        )
    }
}
#[doc = "Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Err(pub u32);
impl Err {
    #[doc = "Error In Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 0"]
    #[inline(always)]
    pub const fn set_err(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Err {
    #[inline(always)]
    fn default() -> Err {
        Err(0)
    }
}
impl core::fmt::Debug for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Err")
            .field("err[0]", &self.err(0usize))
            .field("err[1]", &self.err(1usize))
            .field("err[2]", &self.err(2usize))
            .field("err[3]", &self.err(3usize))
            .field("err[4]", &self.err(4usize))
            .field("err[5]", &self.err(5usize))
            .field("err[6]", &self.err(6usize))
            .field("err[7]", &self.err(7usize))
            .field("err[8]", &self.err(8usize))
            .field("err[9]", &self.err(9usize))
            .field("err[10]", &self.err(10usize))
            .field("err[11]", &self.err(11usize))
            .field("err[12]", &self.err(12usize))
            .field("err[13]", &self.err(13usize))
            .field("err[14]", &self.err(14usize))
            .field("err[15]", &self.err(15usize))
            .field("err[16]", &self.err(16usize))
            .field("err[17]", &self.err(17usize))
            .field("err[18]", &self.err(18usize))
            .field("err[19]", &self.err(19usize))
            .field("err[20]", &self.err(20usize))
            .field("err[21]", &self.err(21usize))
            .field("err[22]", &self.err(22usize))
            .field("err[23]", &self.err(23usize))
            .field("err[24]", &self.err(24usize))
            .field("err[25]", &self.err(25usize))
            .field("err[26]", &self.err(26usize))
            .field("err[27]", &self.err(27usize))
            .field("err[28]", &self.err(28usize))
            .field("err[29]", &self.err(29usize))
            .field("err[30]", &self.err(30usize))
            .field("err[31]", &self.err(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Err {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Err {{ err[0]: {=bool:?}, err[1]: {=bool:?}, err[2]: {=bool:?}, err[3]: {=bool:?}, err[4]: {=bool:?}, err[5]: {=bool:?}, err[6]: {=bool:?}, err[7]: {=bool:?}, err[8]: {=bool:?}, err[9]: {=bool:?}, err[10]: {=bool:?}, err[11]: {=bool:?}, err[12]: {=bool:?}, err[13]: {=bool:?}, err[14]: {=bool:?}, err[15]: {=bool:?}, err[16]: {=bool:?}, err[17]: {=bool:?}, err[18]: {=bool:?}, err[19]: {=bool:?}, err[20]: {=bool:?}, err[21]: {=bool:?}, err[22]: {=bool:?}, err[23]: {=bool:?}, err[24]: {=bool:?}, err[25]: {=bool:?}, err[26]: {=bool:?}, err[27]: {=bool:?}, err[28]: {=bool:?}, err[29]: {=bool:?}, err[30]: {=bool:?}, err[31]: {=bool:?} }}",
            self.err(0usize),
            self.err(1usize),
            self.err(2usize),
            self.err(3usize),
            self.err(4usize),
            self.err(5usize),
            self.err(6usize),
            self.err(7usize),
            self.err(8usize),
            self.err(9usize),
            self.err(10usize),
            self.err(11usize),
            self.err(12usize),
            self.err(13usize),
            self.err(14usize),
            self.err(15usize),
            self.err(16usize),
            self.err(17usize),
            self.err(18usize),
            self.err(19usize),
            self.err(20usize),
            self.err(21usize),
            self.err(22usize),
            self.err(23usize),
            self.err(24usize),
            self.err(25usize),
            self.err(26usize),
            self.err(27usize),
            self.err(28usize),
            self.err(29usize),
            self.err(30usize),
            self.err(31usize)
        )
    }
}
#[doc = "Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Es(pub u32);
impl Es {
    #[doc = "Destination Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Bus Error"]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Bus Error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Offset Error"]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Error"]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source Offset Error"]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Error"]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Channel Priority Error"]
    #[must_use]
    #[inline(always)]
    pub const fn cpe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Priority Error"]
    #[inline(always)]
    pub const fn set_cpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Group Priority Error"]
    #[must_use]
    #[inline(always)]
    pub const fn gpe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Group Priority Error"]
    #[inline(always)]
    pub const fn set_gpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transfer Canceled"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::EsEcx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::EsEcx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Logical OR of all ERR status fields"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Logical OR of all ERR status fields"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Es {
    #[inline(always)]
    fn default() -> Es {
        Es(0)
    }
}
impl core::fmt::Debug for Es {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Es")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("errchn", &self.errchn())
            .field("cpe", &self.cpe())
            .field("gpe", &self.gpe())
            .field("ecx", &self.ecx())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Es {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Es {{ dbe: {=bool:?}, sbe: {=bool:?}, sge: {=bool:?}, nce: {=bool:?}, doe: {=bool:?}, dae: {=bool:?}, soe: {=bool:?}, sae: {=bool:?}, errchn: {=u8:?}, cpe: {=bool:?}, gpe: {=bool:?}, ecx: {:?}, vld: {=bool:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.errchn(),
            self.cpe(),
            self.gpe(),
            self.ecx(),
            self.vld()
        )
    }
}
#[doc = "Hardware Request Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrs(pub u32);
impl Hrs {
    #[doc = "Hardware Request Status Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 0"]
    #[inline(always)]
    pub const fn set_hrs(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Hrs {
    #[inline(always)]
    fn default() -> Hrs {
        Hrs(0)
    }
}
impl core::fmt::Debug for Hrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrs")
            .field("hrs[0]", &self.hrs(0usize))
            .field("hrs[1]", &self.hrs(1usize))
            .field("hrs[2]", &self.hrs(2usize))
            .field("hrs[3]", &self.hrs(3usize))
            .field("hrs[4]", &self.hrs(4usize))
            .field("hrs[5]", &self.hrs(5usize))
            .field("hrs[6]", &self.hrs(6usize))
            .field("hrs[7]", &self.hrs(7usize))
            .field("hrs[8]", &self.hrs(8usize))
            .field("hrs[9]", &self.hrs(9usize))
            .field("hrs[10]", &self.hrs(10usize))
            .field("hrs[11]", &self.hrs(11usize))
            .field("hrs[12]", &self.hrs(12usize))
            .field("hrs[13]", &self.hrs(13usize))
            .field("hrs[14]", &self.hrs(14usize))
            .field("hrs[15]", &self.hrs(15usize))
            .field("hrs[16]", &self.hrs(16usize))
            .field("hrs[17]", &self.hrs(17usize))
            .field("hrs[18]", &self.hrs(18usize))
            .field("hrs[19]", &self.hrs(19usize))
            .field("hrs[20]", &self.hrs(20usize))
            .field("hrs[21]", &self.hrs(21usize))
            .field("hrs[22]", &self.hrs(22usize))
            .field("hrs[23]", &self.hrs(23usize))
            .field("hrs[24]", &self.hrs(24usize))
            .field("hrs[25]", &self.hrs(25usize))
            .field("hrs[26]", &self.hrs(26usize))
            .field("hrs[27]", &self.hrs(27usize))
            .field("hrs[28]", &self.hrs(28usize))
            .field("hrs[29]", &self.hrs(29usize))
            .field("hrs[30]", &self.hrs(30usize))
            .field("hrs[31]", &self.hrs(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hrs {{ hrs[0]: {=bool:?}, hrs[1]: {=bool:?}, hrs[2]: {=bool:?}, hrs[3]: {=bool:?}, hrs[4]: {=bool:?}, hrs[5]: {=bool:?}, hrs[6]: {=bool:?}, hrs[7]: {=bool:?}, hrs[8]: {=bool:?}, hrs[9]: {=bool:?}, hrs[10]: {=bool:?}, hrs[11]: {=bool:?}, hrs[12]: {=bool:?}, hrs[13]: {=bool:?}, hrs[14]: {=bool:?}, hrs[15]: {=bool:?}, hrs[16]: {=bool:?}, hrs[17]: {=bool:?}, hrs[18]: {=bool:?}, hrs[19]: {=bool:?}, hrs[20]: {=bool:?}, hrs[21]: {=bool:?}, hrs[22]: {=bool:?}, hrs[23]: {=bool:?}, hrs[24]: {=bool:?}, hrs[25]: {=bool:?}, hrs[26]: {=bool:?}, hrs[27]: {=bool:?}, hrs[28]: {=bool:?}, hrs[29]: {=bool:?}, hrs[30]: {=bool:?}, hrs[31]: {=bool:?} }}",
            self.hrs(0usize),
            self.hrs(1usize),
            self.hrs(2usize),
            self.hrs(3usize),
            self.hrs(4usize),
            self.hrs(5usize),
            self.hrs(6usize),
            self.hrs(7usize),
            self.hrs(8usize),
            self.hrs(9usize),
            self.hrs(10usize),
            self.hrs(11usize),
            self.hrs(12usize),
            self.hrs(13usize),
            self.hrs(14usize),
            self.hrs(15usize),
            self.hrs(16usize),
            self.hrs(17usize),
            self.hrs(18usize),
            self.hrs(19usize),
            self.hrs(20usize),
            self.hrs(21usize),
            self.hrs(22usize),
            self.hrs(23usize),
            self.hrs(24usize),
            self.hrs(25usize),
            self.hrs(26usize),
            self.hrs(27usize),
            self.hrs(28usize),
            self.hrs(29usize),
            self.hrs(30usize),
            self.hrs(31usize)
        )
    }
}
#[doc = "Interrupt Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Interrupt Request 0"]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 0"]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Request 1"]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 1"]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Request 2"]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 2"]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Request 3"]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 3"]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Request 4"]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 4"]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Request 5"]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 5"]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Request 6"]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 6"]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Request 7"]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 7"]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Request 8"]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 8"]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Request 9"]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 9"]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Request 10"]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 10"]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Request 11"]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 11"]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Request 12"]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 12"]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Request 13"]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 13"]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Request 14"]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 14"]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Request 15"]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 15"]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Request 16"]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 16"]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Request 17"]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 17"]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Request 18"]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 18"]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Request 19"]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 19"]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Request 20"]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 20"]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Request 21"]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 21"]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Request 22"]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 22"]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Request 23"]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 23"]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Request 24"]
    #[must_use]
    #[inline(always)]
    pub const fn int24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 24"]
    #[inline(always)]
    pub const fn set_int24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Request 25"]
    #[must_use]
    #[inline(always)]
    pub const fn int25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 25"]
    #[inline(always)]
    pub const fn set_int25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Request 26"]
    #[must_use]
    #[inline(always)]
    pub const fn int26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 26"]
    #[inline(always)]
    pub const fn set_int26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Request 27"]
    #[must_use]
    #[inline(always)]
    pub const fn int27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 27"]
    #[inline(always)]
    pub const fn set_int27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Request 28"]
    #[must_use]
    #[inline(always)]
    pub const fn int28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 28"]
    #[inline(always)]
    pub const fn set_int28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Request 29"]
    #[must_use]
    #[inline(always)]
    pub const fn int29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 29"]
    #[inline(always)]
    pub const fn set_int29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Request 30"]
    #[must_use]
    #[inline(always)]
    pub const fn int30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 30"]
    #[inline(always)]
    pub const fn set_int30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Request 31"]
    #[must_use]
    #[inline(always)]
    pub const fn int31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 31"]
    #[inline(always)]
    pub const fn set_int31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .field("int24", &self.int24())
            .field("int25", &self.int25())
            .field("int26", &self.int26())
            .field("int27", &self.int27())
            .field("int28", &self.int28())
            .field("int29", &self.int29())
            .field("int30", &self.int30())
            .field("int31", &self.int31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Int {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?}, int24: {=bool:?}, int25: {=bool:?}, int26: {=bool:?}, int27: {=bool:?}, int28: {=bool:?}, int29: {=bool:?}, int30: {=bool:?}, int31: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23(),
            self.int24(),
            self.int25(),
            self.int26(),
            self.int27(),
            self.int28(),
            self.int29(),
            self.int30(),
            self.int31()
        )
    }
}
#[doc = "Set Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seei(pub u8);
impl Seei {
    #[doc = "Set Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn seei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_seei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All Enable Error Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn saee(&self) -> super::vals::Saee {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Saee::from_bits(val as u8)
    }
    #[doc = "Set All Enable Error Interrupts"]
    #[inline(always)]
    pub const fn set_saee(&mut self, val: super::vals::Saee) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SeeiNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SeeiNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SeeiNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Seei {
    #[inline(always)]
    fn default() -> Seei {
        Seei(0)
    }
}
impl core::fmt::Debug for Seei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seei")
            .field("seei", &self.seei())
            .field("saee", &self.saee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seei {{ seei: {=u8:?}, saee: {:?}, nop: {:?} }}",
            self.seei(),
            self.saee(),
            self.nop()
        )
    }
}
#[doc = "Set Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serq(pub u8);
impl Serq {
    #[doc = "Set Enable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn serq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set Enable Request"]
    #[inline(always)]
    pub const fn set_serq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All Enable Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn saer(&self) -> super::vals::Saer {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Saer::from_bits(val as u8)
    }
    #[doc = "Set All Enable Requests"]
    #[inline(always)]
    pub const fn set_saer(&mut self, val: super::vals::Saer) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SerqNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SerqNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SerqNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Serq {
    #[inline(always)]
    fn default() -> Serq {
        Serq(0)
    }
}
impl core::fmt::Debug for Serq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serq")
            .field("serq", &self.serq())
            .field("saer", &self.saer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Serq {{ serq: {=u8:?}, saer: {:?}, nop: {:?} }}",
            self.serq(),
            self.saer(),
            self.nop()
        )
    }
}
#[doc = "Set START Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssrt(pub u8);
impl Ssrt {
    #[doc = "Set START field"]
    #[must_use]
    #[inline(always)]
    pub const fn ssrt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set START field"]
    #[inline(always)]
    pub const fn set_ssrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All START fields (activates all channels)"]
    #[must_use]
    #[inline(always)]
    pub const fn sast(&self) -> super::vals::Sast {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sast::from_bits(val as u8)
    }
    #[doc = "Set All START fields (activates all channels)"]
    #[inline(always)]
    pub const fn set_sast(&mut self, val: super::vals::Sast) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SsrtNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SsrtNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SsrtNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ssrt {
    #[inline(always)]
    fn default() -> Ssrt {
        Ssrt(0)
    }
}
impl core::fmt::Debug for Ssrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssrt")
            .field("ssrt", &self.ssrt())
            .field("sast", &self.sast())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrt {{ ssrt: {=u8:?}, sast: {:?}, nop: {:?} }}",
            self.ssrt(),
            self.sast(),
            self.nop()
        )
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::TcdAttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::TcdAttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::TcdAttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for TcdAttr {
    #[inline(always)]
    fn default() -> TcdAttr {
        TcdAttr(0)
    }
}
impl core::fmt::Debug for TcdAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdAttr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdAttr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdAttr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkno {
    #[inline(always)]
    fn default() -> TcdBiterElinkno {
        TcdBiterElinkno(0)
    }
}
impl core::fmt::Debug for TcdBiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkyes {
    #[inline(always)]
    fn default() -> TcdBiterElinkyes {
        TcdBiterElinkyes(0)
    }
}
impl core::fmt::Debug for TcdBiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkno {
    #[inline(always)]
    fn default() -> TcdCiterElinkno {
        TcdCiterElinkno(0)
    }
}
impl core::fmt::Debug for TcdCiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkyes {
    #[inline(always)]
    fn default() -> TcdCiterElinkyes {
        TcdCiterElinkyes(0)
    }
}
impl core::fmt::Debug for TcdCiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::TcdCsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TcdCsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::TcdCsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::TcdCsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::TcdCsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::TcdCsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for TcdCsr {
    #[inline(always)]
    fn default() -> TcdCsr {
        TcdCsr(0)
    }
}
impl core::fmt::Debug for TcdCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCsr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCsr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDaddr(pub u32);
impl TcdDaddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdDaddr {
    #[inline(always)]
    fn default() -> TcdDaddr {
        TcdDaddr(0)
    }
}
impl core::fmt::Debug for TcdDaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDaddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDaddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDlastsga(pub u32);
impl TcdDlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdDlastsga {
    #[inline(always)]
    fn default() -> TcdDlastsga {
        TcdDlastsga(0)
    }
}
impl core::fmt::Debug for TcdDlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDoff(pub u16);
impl TcdDoff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TcdDoff {
    #[inline(always)]
    fn default() -> TcdDoff {
        TcdDoff(0)
    }
}
impl core::fmt::Debug for TcdDoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDoff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDoff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMlno(pub u32);
impl TcdNbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdNbytesMlno {
    #[inline(always)]
    fn default() -> TcdNbytesMlno {
        TcdNbytesMlno(0)
    }
}
impl core::fmt::Debug for TcdNbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdNbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffno {
    #[inline(always)]
    fn default() -> TcdNbytesMloffno {
        TcdNbytesMloffno(0)
    }
}
impl core::fmt::Debug for TcdNbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffyes {
    #[inline(always)]
    fn default() -> TcdNbytesMloffyes {
        TcdNbytesMloffyes(0)
    }
}
impl core::fmt::Debug for TcdNbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSaddr(pub u32);
impl TcdSaddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdSaddr {
    #[inline(always)]
    fn default() -> TcdSaddr {
        TcdSaddr(0)
    }
}
impl core::fmt::Debug for TcdSaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSaddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSaddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSlast(pub u32);
impl TcdSlast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdSlast {
    #[inline(always)]
    fn default() -> TcdSlast {
        TcdSlast(0)
    }
}
impl core::fmt::Debug for TcdSlast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSlast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSlast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSlast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSoff(pub u16);
impl TcdSoff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TcdSoff {
    #[inline(always)]
    fn default() -> TcdSoff {
        TcdSoff(0)
    }
}
impl core::fmt::Debug for TcdSoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSoff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSoff {{ soff: {=u16:?} }}", self.soff())
    }
}
