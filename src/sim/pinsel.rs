#[doc = "Register `PINSEL` reader"]
pub struct R(crate::R<PINSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINSEL` writer"]
pub struct W(crate::W<PINSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PINSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C0 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0PS_A {
    #[doc = "0: I2C0_SCL and I2C0_SDA are mapped on PTA3 and PTA2, respectively."]
    _0 = 0,
    #[doc = "1: I2C0_SCL and I2C0_SDA are mapped on PTB7 and PTB6, respectively."]
    _1 = 1,
}
impl From<I2C0PS_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0PS` reader - I2C0 Port Pin Select"]
pub struct I2C0PS_R(crate::FieldReader<bool, I2C0PS_A>);
impl I2C0PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0PS_A {
        match self.bits {
            false => I2C0PS_A::_0,
            true => I2C0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C0PS_A::_1
    }
}
impl core::ops::Deref for I2C0PS_R {
    type Target = crate::FieldReader<bool, I2C0PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0PS` writer - I2C0 Port Pin Select"]
pub struct I2C0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0PS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C0_SCL and I2C0_SDA are mapped on PTA3 and PTA2, respectively."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0PS_A::_0)
    }
    #[doc = "I2C0_SCL and I2C0_SDA are mapped on PTB7 and PTB6, respectively."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0PS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "SPI0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0PS_A {
    #[doc = "0: SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTB2, PTB3, PTB4, and PTB5."]
    _0 = 0,
    #[doc = "1: SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTA6, PTA7, PTB1, and PTB0."]
    _1 = 1,
}
impl From<SPI0PS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0PS` reader - SPI0 Pin Select"]
pub struct SPI0PS_R(crate::FieldReader<bool, SPI0PS_A>);
impl SPI0PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0PS_A {
        match self.bits {
            false => SPI0PS_A::_0,
            true => SPI0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI0PS_A::_1
    }
}
impl core::ops::Deref for SPI0PS_R {
    type Target = crate::FieldReader<bool, SPI0PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0PS` writer - SPI0 Pin Select"]
pub struct SPI0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0PS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTB2, PTB3, PTB4, and PTB5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0PS_A::_0)
    }
    #[doc = "SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTA6, PTA7, PTB1, and PTB0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0PS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "UART0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0PS_A {
    #[doc = "0: UART0_RX and UART0_TX are mapped on PTB0 and PTB1."]
    _0 = 0,
    #[doc = "1: UART0_RX and UART0_TX are mapped on PTA2 and PTA3."]
    _1 = 1,
}
impl From<UART0PS_A> for bool {
    #[inline(always)]
    fn from(variant: UART0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0PS` reader - UART0 Pin Select"]
pub struct UART0PS_R(crate::FieldReader<bool, UART0PS_A>);
impl UART0PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0PS_A {
        match self.bits {
            false => UART0PS_A::_0,
            true => UART0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART0PS_A::_1
    }
}
impl core::ops::Deref for UART0PS_R {
    type Target = crate::FieldReader<bool, UART0PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0PS` writer - UART0 Pin Select"]
pub struct UART0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0PS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART0_RX and UART0_TX are mapped on PTB0 and PTB1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0PS_A::_0)
    }
    #[doc = "UART0_RX and UART0_TX are mapped on PTA2 and PTA3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0PS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "FTM0_CH0 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0PS0_A {
    #[doc = "0: FTM0_CH0 channels are mapped on PTA0."]
    _0 = 0,
    #[doc = "1: FTM0_CH0 channels are mapped on PTB2."]
    _1 = 1,
}
impl From<FTM0PS0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM0PS0` reader - FTM0_CH0 Port Pin Select"]
pub struct FTM0PS0_R(crate::FieldReader<bool, FTM0PS0_A>);
impl FTM0PS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM0PS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0PS0_A {
        match self.bits {
            false => FTM0PS0_A::_0,
            true => FTM0PS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM0PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM0PS0_A::_1
    }
}
impl core::ops::Deref for FTM0PS0_R {
    type Target = crate::FieldReader<bool, FTM0PS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0PS0` writer - FTM0_CH0 Port Pin Select"]
pub struct FTM0PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0PS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM0_CH0 channels are mapped on PTA0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0PS0_A::_0)
    }
    #[doc = "FTM0_CH0 channels are mapped on PTB2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0PS0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "FTM0_CH1 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0PS1_A {
    #[doc = "0: FTM0_CH1 channels are mapped on PTA1."]
    _0 = 0,
    #[doc = "1: FTM0_CH1 channels are mapped on PTB3."]
    _1 = 1,
}
impl From<FTM0PS1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM0PS1` reader - FTM0_CH1 Port Pin Select"]
pub struct FTM0PS1_R(crate::FieldReader<bool, FTM0PS1_A>);
impl FTM0PS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM0PS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0PS1_A {
        match self.bits {
            false => FTM0PS1_A::_0,
            true => FTM0PS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM0PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM0PS1_A::_1
    }
}
impl core::ops::Deref for FTM0PS1_R {
    type Target = crate::FieldReader<bool, FTM0PS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0PS1` writer - FTM0_CH1 Port Pin Select"]
pub struct FTM0PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0PS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM0_CH1 channels are mapped on PTA1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0PS1_A::_0)
    }
    #[doc = "FTM0_CH1 channels are mapped on PTB3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0PS1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "FTM2_CH2 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS2_A {
    #[doc = "0: FTM2_CH2 channels are mapped on PTC2."]
    _0 = 0,
    #[doc = "1: FTM2_CH2 channels are mapped on PTC4."]
    _1 = 1,
}
impl From<FTM2PS2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2PS2` reader - FTM2_CH2 Port Pin Select"]
pub struct FTM2PS2_R(crate::FieldReader<bool, FTM2PS2_A>);
impl FTM2PS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM2PS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS2_A {
        match self.bits {
            false => FTM2PS2_A::_0,
            true => FTM2PS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM2PS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM2PS2_A::_1
    }
}
impl core::ops::Deref for FTM2PS2_R {
    type Target = crate::FieldReader<bool, FTM2PS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2PS2` writer - FTM2_CH2 Port Pin Select"]
pub struct FTM2PS2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM2_CH2 channels are mapped on PTC2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_0)
    }
    #[doc = "FTM2_CH2 channels are mapped on PTC4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "FTM2_CH3 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS3_A {
    #[doc = "0: FTM2_CH3 channels are mapped on PTC3."]
    _0 = 0,
    #[doc = "1: FTM2_CH3 channels are mapped on PTC5."]
    _1 = 1,
}
impl From<FTM2PS3_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2PS3` reader - FTM2_CH3 Port Pin Select"]
pub struct FTM2PS3_R(crate::FieldReader<bool, FTM2PS3_A>);
impl FTM2PS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM2PS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS3_A {
        match self.bits {
            false => FTM2PS3_A::_0,
            true => FTM2PS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM2PS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM2PS3_A::_1
    }
}
impl core::ops::Deref for FTM2PS3_R {
    type Target = crate::FieldReader<bool, FTM2PS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2PS3` writer - FTM2_CH3 Port Pin Select"]
pub struct FTM2PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM2_CH3 channels are mapped on PTC3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_0)
    }
    #[doc = "FTM2_CH3 channels are mapped on PTC5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "FTM0 TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0CLKPS_A {
    #[doc = "0: Selects TCLK1 for FTM0 module."]
    _0 = 0,
    #[doc = "1: Selects TCLK2 for FTM0 module."]
    _1 = 1,
}
impl From<FTM0CLKPS_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0CLKPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM0CLKPS` reader - FTM0 TCLK Pin Select"]
pub struct FTM0CLKPS_R(crate::FieldReader<bool, FTM0CLKPS_A>);
impl FTM0CLKPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM0CLKPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0CLKPS_A {
        match self.bits {
            false => FTM0CLKPS_A::_0,
            true => FTM0CLKPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM0CLKPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM0CLKPS_A::_1
    }
}
impl core::ops::Deref for FTM0CLKPS_R {
    type Target = crate::FieldReader<bool, FTM0CLKPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0CLKPS` writer - FTM0 TCLK Pin Select"]
pub struct FTM0CLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0CLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0CLKPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects TCLK1 for FTM0 module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0CLKPS_A::_0)
    }
    #[doc = "Selects TCLK2 for FTM0 module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0CLKPS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "FTM2 TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CLKPS_A {
    #[doc = "0: Selects TCLK1 for FTM2 module."]
    _0 = 0,
    #[doc = "1: Selects TCLK2 for FTM2 module."]
    _1 = 1,
}
impl From<FTM2CLKPS_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2CLKPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2CLKPS` reader - FTM2 TCLK Pin Select"]
pub struct FTM2CLKPS_R(crate::FieldReader<bool, FTM2CLKPS_A>);
impl FTM2CLKPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM2CLKPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CLKPS_A {
        match self.bits {
            false => FTM2CLKPS_A::_0,
            true => FTM2CLKPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM2CLKPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM2CLKPS_A::_1
    }
}
impl core::ops::Deref for FTM2CLKPS_R {
    type Target = crate::FieldReader<bool, FTM2CLKPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2CLKPS` writer - FTM2 TCLK Pin Select"]
pub struct FTM2CLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CLKPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects TCLK1 for FTM2 module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CLKPS_A::_0)
    }
    #[doc = "Selects TCLK2 for FTM2 module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CLKPS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "PWT TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTCLKPS_A {
    #[doc = "0: Selects TCLK1 for PWT module."]
    _0 = 0,
    #[doc = "1: Selects TCLK2 for PWT module."]
    _1 = 1,
}
impl From<PWTCLKPS_A> for bool {
    #[inline(always)]
    fn from(variant: PWTCLKPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWTCLKPS` reader - PWT TCLK Pin Select"]
pub struct PWTCLKPS_R(crate::FieldReader<bool, PWTCLKPS_A>);
impl PWTCLKPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWTCLKPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTCLKPS_A {
        match self.bits {
            false => PWTCLKPS_A::_0,
            true => PWTCLKPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWTCLKPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWTCLKPS_A::_1
    }
}
impl core::ops::Deref for PWTCLKPS_R {
    type Target = crate::FieldReader<bool, PWTCLKPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWTCLKPS` writer - PWT TCLK Pin Select"]
pub struct PWTCLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTCLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTCLKPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects TCLK1 for PWT module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTCLKPS_A::_0)
    }
    #[doc = "Selects TCLK2 for PWT module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTCLKPS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - I2C0 Port Pin Select"]
    #[inline(always)]
    pub fn i2c0ps(&self) -> I2C0PS_R {
        I2C0PS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI0 Pin Select"]
    #[inline(always)]
    pub fn spi0ps(&self) -> SPI0PS_R {
        SPI0PS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART0 Pin Select"]
    #[inline(always)]
    pub fn uart0ps(&self) -> UART0PS_R {
        UART0PS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FTM0_CH0 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps0(&self) -> FTM0PS0_R {
        FTM0PS0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FTM0_CH1 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps1(&self) -> FTM0PS1_R {
        FTM0PS1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FTM2_CH2 Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps2(&self) -> FTM2PS2_R {
        FTM2PS2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FTM2_CH3 Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps3(&self) -> FTM2PS3_R {
        FTM2PS3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 28 - FTM0 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm0clkps(&self) -> FTM0CLKPS_R {
        FTM0CLKPS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - FTM2 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm2clkps(&self) -> FTM2CLKPS_R {
        FTM2CLKPS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PWT TCLK Pin Select"]
    #[inline(always)]
    pub fn pwtclkps(&self) -> PWTCLKPS_R {
        PWTCLKPS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - I2C0 Port Pin Select"]
    #[inline(always)]
    pub fn i2c0ps(&mut self) -> I2C0PS_W {
        I2C0PS_W { w: self }
    }
    #[doc = "Bit 6 - SPI0 Pin Select"]
    #[inline(always)]
    pub fn spi0ps(&mut self) -> SPI0PS_W {
        SPI0PS_W { w: self }
    }
    #[doc = "Bit 7 - UART0 Pin Select"]
    #[inline(always)]
    pub fn uart0ps(&mut self) -> UART0PS_W {
        UART0PS_W { w: self }
    }
    #[doc = "Bit 8 - FTM0_CH0 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps0(&mut self) -> FTM0PS0_W {
        FTM0PS0_W { w: self }
    }
    #[doc = "Bit 9 - FTM0_CH1 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps1(&mut self) -> FTM0PS1_W {
        FTM0PS1_W { w: self }
    }
    #[doc = "Bit 14 - FTM2_CH2 Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps2(&mut self) -> FTM2PS2_W {
        FTM2PS2_W { w: self }
    }
    #[doc = "Bit 15 - FTM2_CH3 Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps3(&mut self) -> FTM2PS3_W {
        FTM2PS3_W { w: self }
    }
    #[doc = "Bit 28 - FTM0 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm0clkps(&mut self) -> FTM0CLKPS_W {
        FTM0CLKPS_W { w: self }
    }
    #[doc = "Bit 30 - FTM2 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm2clkps(&mut self) -> FTM2CLKPS_W {
        FTM2CLKPS_W { w: self }
    }
    #[doc = "Bit 31 - PWT TCLK Pin Select"]
    #[inline(always)]
    pub fn pwtclkps(&mut self) -> PWTCLKPS_W {
        PWTCLKPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel](index.html) module"]
pub struct PINSEL_SPEC;
impl crate::RegisterSpec for PINSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinsel::R](R) reader structure"]
impl crate::Readable for PINSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinsel::W](W) writer structure"]
impl crate::Writable for PINSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINSEL to value 0"]
impl crate::Resettable for PINSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
