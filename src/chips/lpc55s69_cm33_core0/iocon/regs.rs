#[doc = "Digital I/O control for port 0 pins PIO0_0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio00(pub u32);
impl Pio00 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio00 {
    #[inline(always)]
    fn default() -> Pio00 {
        Pio00(0)
    }
}
impl core::fmt::Debug for Pio00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio00")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio00 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio01(pub u32);
impl Pio01 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio01 {
    #[inline(always)]
    fn default() -> Pio01 {
        Pio01(0)
    }
}
impl core::fmt::Debug for Pio01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio01")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio01 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio010(pub u32);
impl Pio010 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio010 {
    #[inline(always)]
    fn default() -> Pio010 {
        Pio010(0)
    }
}
impl core::fmt::Debug for Pio010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio010")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio010 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio010 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio011(pub u32);
impl Pio011 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio011 {
    #[inline(always)]
    fn default() -> Pio011 {
        Pio011(0)
    }
}
impl core::fmt::Debug for Pio011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio011")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio011 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio011 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio012(pub u32);
impl Pio012 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio012 {
    #[inline(always)]
    fn default() -> Pio012 {
        Pio012(0)
    }
}
impl core::fmt::Debug for Pio012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio012")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio012 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio012 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio013(pub u32);
impl Pio013 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ssel(&self) -> super::vals::PioSsel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PioSsel::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_ssel(&mut self, val: super::vals::PioSsel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filteroff(&self) -> super::vals::PioFilteroff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PioFilteroff::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_filteroff(&mut self, val: super::vals::PioFilteroff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ecs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn egp(&self) -> super::vals::PioEgp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PioEgp::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_egp(&mut self, val: super::vals::PioEgp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cfilter(&self) -> super::vals::PioI2cfilter {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PioI2cfilter::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_i2cfilter(&mut self, val: super::vals::PioI2cfilter) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pio013 {
    #[inline(always)]
    fn default() -> Pio013 {
        Pio013(0)
    }
}
impl core::fmt::Debug for Pio013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio013")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("ssel", &self.ssel())
            .field("filteroff", &self.filteroff())
            .field("ecs", &self.ecs())
            .field("egp", &self.egp())
            .field("i2cfilter", &self.i2cfilter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio013 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio013 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, ssel: {:?}, filteroff: {:?}, ecs: {=bool:?}, egp: {:?}, i2cfilter: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.ssel(),
            self.filteroff(),
            self.ecs(),
            self.egp(),
            self.i2cfilter()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio014(pub u32);
impl Pio014 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ssel(&self) -> super::vals::PioSsel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PioSsel::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_ssel(&mut self, val: super::vals::PioSsel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filteroff(&self) -> super::vals::PioFilteroff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PioFilteroff::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_filteroff(&mut self, val: super::vals::PioFilteroff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ecs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn egp(&self) -> super::vals::PioEgp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PioEgp::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_egp(&mut self, val: super::vals::PioEgp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cfilter(&self) -> super::vals::PioI2cfilter {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PioI2cfilter::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_i2cfilter(&mut self, val: super::vals::PioI2cfilter) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pio014 {
    #[inline(always)]
    fn default() -> Pio014 {
        Pio014(0)
    }
}
impl core::fmt::Debug for Pio014 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio014")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("ssel", &self.ssel())
            .field("filteroff", &self.filteroff())
            .field("ecs", &self.ecs())
            .field("egp", &self.egp())
            .field("i2cfilter", &self.i2cfilter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio014 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio014 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, ssel: {:?}, filteroff: {:?}, ecs: {=bool:?}, egp: {:?}, i2cfilter: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.ssel(),
            self.filteroff(),
            self.ecs(),
            self.egp(),
            self.i2cfilter()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio015(pub u32);
impl Pio015 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio015 {
    #[inline(always)]
    fn default() -> Pio015 {
        Pio015(0)
    }
}
impl core::fmt::Debug for Pio015 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio015")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio015 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio015 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio016(pub u32);
impl Pio016 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio016 {
    #[inline(always)]
    fn default() -> Pio016 {
        Pio016(0)
    }
}
impl core::fmt::Debug for Pio016 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio016")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio016 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio016 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio017(pub u32);
impl Pio017 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio017 {
    #[inline(always)]
    fn default() -> Pio017 {
        Pio017(0)
    }
}
impl core::fmt::Debug for Pio017 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio017")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio017 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio017 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio018(pub u32);
impl Pio018 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio018 {
    #[inline(always)]
    fn default() -> Pio018 {
        Pio018(0)
    }
}
impl core::fmt::Debug for Pio018 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio018")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio018 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio018 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio019(pub u32);
impl Pio019 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio019 {
    #[inline(always)]
    fn default() -> Pio019 {
        Pio019(0)
    }
}
impl core::fmt::Debug for Pio019 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio019")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio019 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio019 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio02(pub u32);
impl Pio02 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio02 {
    #[inline(always)]
    fn default() -> Pio02 {
        Pio02(0)
    }
}
impl core::fmt::Debug for Pio02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio02")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio02 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio020(pub u32);
impl Pio020 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio020 {
    #[inline(always)]
    fn default() -> Pio020 {
        Pio020(0)
    }
}
impl core::fmt::Debug for Pio020 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio020")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio020 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio020 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio021(pub u32);
impl Pio021 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio021 {
    #[inline(always)]
    fn default() -> Pio021 {
        Pio021(0)
    }
}
impl core::fmt::Debug for Pio021 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio021")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio021 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio021 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio022(pub u32);
impl Pio022 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio022 {
    #[inline(always)]
    fn default() -> Pio022 {
        Pio022(0)
    }
}
impl core::fmt::Debug for Pio022 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio022")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio022 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio022 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio023(pub u32);
impl Pio023 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio023 {
    #[inline(always)]
    fn default() -> Pio023 {
        Pio023(0)
    }
}
impl core::fmt::Debug for Pio023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio023")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio023 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio024(pub u32);
impl Pio024 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio024 {
    #[inline(always)]
    fn default() -> Pio024 {
        Pio024(0)
    }
}
impl core::fmt::Debug for Pio024 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio024")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio024 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio024 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio025(pub u32);
impl Pio025 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio025 {
    #[inline(always)]
    fn default() -> Pio025 {
        Pio025(0)
    }
}
impl core::fmt::Debug for Pio025 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio025")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio025 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio025 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio026(pub u32);
impl Pio026 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio026 {
    #[inline(always)]
    fn default() -> Pio026 {
        Pio026(0)
    }
}
impl core::fmt::Debug for Pio026 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio026")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio026 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio026 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio027(pub u32);
impl Pio027 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio027 {
    #[inline(always)]
    fn default() -> Pio027 {
        Pio027(0)
    }
}
impl core::fmt::Debug for Pio027 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio027")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio027 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio027 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio028(pub u32);
impl Pio028 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio028 {
    #[inline(always)]
    fn default() -> Pio028 {
        Pio028(0)
    }
}
impl core::fmt::Debug for Pio028 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio028")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio028 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio028 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio029(pub u32);
impl Pio029 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio029 {
    #[inline(always)]
    fn default() -> Pio029 {
        Pio029(0)
    }
}
impl core::fmt::Debug for Pio029 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio029")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio029 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio029 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio03(pub u32);
impl Pio03 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio03 {
    #[inline(always)]
    fn default() -> Pio03 {
        Pio03(0)
    }
}
impl core::fmt::Debug for Pio03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio03")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio03 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio030(pub u32);
impl Pio030 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio030 {
    #[inline(always)]
    fn default() -> Pio030 {
        Pio030(0)
    }
}
impl core::fmt::Debug for Pio030 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio030")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio030 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio030 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio031(pub u32);
impl Pio031 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio031 {
    #[inline(always)]
    fn default() -> Pio031 {
        Pio031(0)
    }
}
impl core::fmt::Debug for Pio031 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio031")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio031 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio031 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio04(pub u32);
impl Pio04 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio04 {
    #[inline(always)]
    fn default() -> Pio04 {
        Pio04(0)
    }
}
impl core::fmt::Debug for Pio04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio04")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio04 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio04 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio05(pub u32);
impl Pio05 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio05 {
    #[inline(always)]
    fn default() -> Pio05 {
        Pio05(0)
    }
}
impl core::fmt::Debug for Pio05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio05")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio05 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio05 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio06(pub u32);
impl Pio06 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio06 {
    #[inline(always)]
    fn default() -> Pio06 {
        Pio06(0)
    }
}
impl core::fmt::Debug for Pio06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio06")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio06 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio06 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio07(pub u32);
impl Pio07 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio07 {
    #[inline(always)]
    fn default() -> Pio07 {
        Pio07(0)
    }
}
impl core::fmt::Debug for Pio07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio07")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio07 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio07 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio08(pub u32);
impl Pio08 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio08 {
    #[inline(always)]
    fn default() -> Pio08 {
        Pio08(0)
    }
}
impl core::fmt::Debug for Pio08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio08")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio08 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio08 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio09(pub u32);
impl Pio09 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio09 {
    #[inline(always)]
    fn default() -> Pio09 {
        Pio09(0)
    }
}
impl core::fmt::Debug for Pio09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio09")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio09 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio09 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio10(pub u32);
impl Pio10 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio10 {
    #[inline(always)]
    fn default() -> Pio10 {
        Pio10(0)
    }
}
impl core::fmt::Debug for Pio10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio10")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio10 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio11(pub u32);
impl Pio11 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio11 {
    #[inline(always)]
    fn default() -> Pio11 {
        Pio11(0)
    }
}
impl core::fmt::Debug for Pio11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio11")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio11 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio110(pub u32);
impl Pio110 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio110 {
    #[inline(always)]
    fn default() -> Pio110 {
        Pio110(0)
    }
}
impl core::fmt::Debug for Pio110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio110")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio110 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio110 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio111(pub u32);
impl Pio111 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio111 {
    #[inline(always)]
    fn default() -> Pio111 {
        Pio111(0)
    }
}
impl core::fmt::Debug for Pio111 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio111")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio111 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio111 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio112(pub u32);
impl Pio112 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio112 {
    #[inline(always)]
    fn default() -> Pio112 {
        Pio112(0)
    }
}
impl core::fmt::Debug for Pio112 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio112")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio112 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio112 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio113(pub u32);
impl Pio113 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio113 {
    #[inline(always)]
    fn default() -> Pio113 {
        Pio113(0)
    }
}
impl core::fmt::Debug for Pio113 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio113")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio113 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio113 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio114(pub u32);
impl Pio114 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio114 {
    #[inline(always)]
    fn default() -> Pio114 {
        Pio114(0)
    }
}
impl core::fmt::Debug for Pio114 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio114")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio114 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio114 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio115(pub u32);
impl Pio115 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio115 {
    #[inline(always)]
    fn default() -> Pio115 {
        Pio115(0)
    }
}
impl core::fmt::Debug for Pio115 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio115")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio115 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio115 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio116(pub u32);
impl Pio116 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio116 {
    #[inline(always)]
    fn default() -> Pio116 {
        Pio116(0)
    }
}
impl core::fmt::Debug for Pio116 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio116")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio116 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio116 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio117(pub u32);
impl Pio117 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio117 {
    #[inline(always)]
    fn default() -> Pio117 {
        Pio117(0)
    }
}
impl core::fmt::Debug for Pio117 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio117")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio117 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio117 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio118(pub u32);
impl Pio118 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio118 {
    #[inline(always)]
    fn default() -> Pio118 {
        Pio118(0)
    }
}
impl core::fmt::Debug for Pio118 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio118")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio118 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio118 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio119(pub u32);
impl Pio119 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio119 {
    #[inline(always)]
    fn default() -> Pio119 {
        Pio119(0)
    }
}
impl core::fmt::Debug for Pio119 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio119")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio119 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio119 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio12(pub u32);
impl Pio12 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio12 {
    #[inline(always)]
    fn default() -> Pio12 {
        Pio12(0)
    }
}
impl core::fmt::Debug for Pio12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio12")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio12 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio120(pub u32);
impl Pio120 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio120 {
    #[inline(always)]
    fn default() -> Pio120 {
        Pio120(0)
    }
}
impl core::fmt::Debug for Pio120 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio120")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio120 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio120 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio121(pub u32);
impl Pio121 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio121 {
    #[inline(always)]
    fn default() -> Pio121 {
        Pio121(0)
    }
}
impl core::fmt::Debug for Pio121 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio121")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio121 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio121 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio122(pub u32);
impl Pio122 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio122 {
    #[inline(always)]
    fn default() -> Pio122 {
        Pio122(0)
    }
}
impl core::fmt::Debug for Pio122 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio122")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio122 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio122 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio123(pub u32);
impl Pio123 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio123 {
    #[inline(always)]
    fn default() -> Pio123 {
        Pio123(0)
    }
}
impl core::fmt::Debug for Pio123 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio123")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio123 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio123 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio124(pub u32);
impl Pio124 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio124 {
    #[inline(always)]
    fn default() -> Pio124 {
        Pio124(0)
    }
}
impl core::fmt::Debug for Pio124 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio124")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio124 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio124 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio125(pub u32);
impl Pio125 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio125 {
    #[inline(always)]
    fn default() -> Pio125 {
        Pio125(0)
    }
}
impl core::fmt::Debug for Pio125 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio125")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio125 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio125 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio126(pub u32);
impl Pio126 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio126 {
    #[inline(always)]
    fn default() -> Pio126 {
        Pio126(0)
    }
}
impl core::fmt::Debug for Pio126 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio126")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio126 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio126 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio127(pub u32);
impl Pio127 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio127 {
    #[inline(always)]
    fn default() -> Pio127 {
        Pio127(0)
    }
}
impl core::fmt::Debug for Pio127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio127")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio127 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio128(pub u32);
impl Pio128 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio128 {
    #[inline(always)]
    fn default() -> Pio128 {
        Pio128(0)
    }
}
impl core::fmt::Debug for Pio128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio128")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio128 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio128 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio129(pub u32);
impl Pio129 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio129 {
    #[inline(always)]
    fn default() -> Pio129 {
        Pio129(0)
    }
}
impl core::fmt::Debug for Pio129 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio129")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio129 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio129 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio13(pub u32);
impl Pio13 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio13 {
    #[inline(always)]
    fn default() -> Pio13 {
        Pio13(0)
    }
}
impl core::fmt::Debug for Pio13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio13")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio13 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio130(pub u32);
impl Pio130 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio130 {
    #[inline(always)]
    fn default() -> Pio130 {
        Pio130(0)
    }
}
impl core::fmt::Debug for Pio130 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio130")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio130 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio130 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio131(pub u32);
impl Pio131 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio131 {
    #[inline(always)]
    fn default() -> Pio131 {
        Pio131(0)
    }
}
impl core::fmt::Debug for Pio131 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio131")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio131 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio131 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio14(pub u32);
impl Pio14 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio14 {
    #[inline(always)]
    fn default() -> Pio14 {
        Pio14(0)
    }
}
impl core::fmt::Debug for Pio14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio14")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio14 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio15(pub u32);
impl Pio15 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio15 {
    #[inline(always)]
    fn default() -> Pio15 {
        Pio15(0)
    }
}
impl core::fmt::Debug for Pio15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio15")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio15 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio16(pub u32);
impl Pio16 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio16 {
    #[inline(always)]
    fn default() -> Pio16 {
        Pio16(0)
    }
}
impl core::fmt::Debug for Pio16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio16")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio16 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio17(pub u32);
impl Pio17 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio17 {
    #[inline(always)]
    fn default() -> Pio17 {
        Pio17(0)
    }
}
impl core::fmt::Debug for Pio17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio17")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio17 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio18(pub u32);
impl Pio18 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio18 {
    #[inline(always)]
    fn default() -> Pio18 {
        Pio18(0)
    }
}
impl core::fmt::Debug for Pio18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio18")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio18 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio19(pub u32);
impl Pio19 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio19 {
    #[inline(always)]
    fn default() -> Pio19 {
        Pio19(0)
    }
}
impl core::fmt::Debug for Pio19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio19")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio19 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
