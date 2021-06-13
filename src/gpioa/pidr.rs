#[doc = "Register `PIDR` reader"]
pub struct R(crate::R<PIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR` writer"]
pub struct W(crate::W<PIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR_SPEC>;
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
impl From<crate::W<PIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID0_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID0_A> for bool {
    #[inline(always)]
    fn from(variant: PID0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID0` reader - Port Input Disable"]
pub struct PID0_R(crate::FieldReader<bool, PID0_A>);
impl PID0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID0_A {
        match self.bits {
            false => PID0_A::_0,
            true => PID0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID0_A::_1
    }
}
impl core::ops::Deref for PID0_R {
    type Target = crate::FieldReader<bool, PID0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID0` writer - Port Input Disable"]
pub struct PID0_W<'a> {
    w: &'a mut W,
}
impl<'a> PID0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID0_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID1_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID1_A> for bool {
    #[inline(always)]
    fn from(variant: PID1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID1` reader - Port Input Disable"]
pub struct PID1_R(crate::FieldReader<bool, PID1_A>);
impl PID1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID1_A {
        match self.bits {
            false => PID1_A::_0,
            true => PID1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID1_A::_1
    }
}
impl core::ops::Deref for PID1_R {
    type Target = crate::FieldReader<bool, PID1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID1` writer - Port Input Disable"]
pub struct PID1_W<'a> {
    w: &'a mut W,
}
impl<'a> PID1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID1_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID2_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID2_A> for bool {
    #[inline(always)]
    fn from(variant: PID2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID2` reader - Port Input Disable"]
pub struct PID2_R(crate::FieldReader<bool, PID2_A>);
impl PID2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID2_A {
        match self.bits {
            false => PID2_A::_0,
            true => PID2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID2_A::_1
    }
}
impl core::ops::Deref for PID2_R {
    type Target = crate::FieldReader<bool, PID2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID2` writer - Port Input Disable"]
pub struct PID2_W<'a> {
    w: &'a mut W,
}
impl<'a> PID2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID2_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID3_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID3_A> for bool {
    #[inline(always)]
    fn from(variant: PID3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID3` reader - Port Input Disable"]
pub struct PID3_R(crate::FieldReader<bool, PID3_A>);
impl PID3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID3_A {
        match self.bits {
            false => PID3_A::_0,
            true => PID3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID3_A::_1
    }
}
impl core::ops::Deref for PID3_R {
    type Target = crate::FieldReader<bool, PID3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID3` writer - Port Input Disable"]
pub struct PID3_W<'a> {
    w: &'a mut W,
}
impl<'a> PID3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID3_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID4_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID4_A> for bool {
    #[inline(always)]
    fn from(variant: PID4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID4` reader - Port Input Disable"]
pub struct PID4_R(crate::FieldReader<bool, PID4_A>);
impl PID4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID4_A {
        match self.bits {
            false => PID4_A::_0,
            true => PID4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID4_A::_1
    }
}
impl core::ops::Deref for PID4_R {
    type Target = crate::FieldReader<bool, PID4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID4` writer - Port Input Disable"]
pub struct PID4_W<'a> {
    w: &'a mut W,
}
impl<'a> PID4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID4_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID5_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID5_A> for bool {
    #[inline(always)]
    fn from(variant: PID5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID5` reader - Port Input Disable"]
pub struct PID5_R(crate::FieldReader<bool, PID5_A>);
impl PID5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID5_A {
        match self.bits {
            false => PID5_A::_0,
            true => PID5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID5_A::_1
    }
}
impl core::ops::Deref for PID5_R {
    type Target = crate::FieldReader<bool, PID5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID5` writer - Port Input Disable"]
pub struct PID5_W<'a> {
    w: &'a mut W,
}
impl<'a> PID5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID5_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID5_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID6_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID6_A> for bool {
    #[inline(always)]
    fn from(variant: PID6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID6` reader - Port Input Disable"]
pub struct PID6_R(crate::FieldReader<bool, PID6_A>);
impl PID6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID6_A {
        match self.bits {
            false => PID6_A::_0,
            true => PID6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID6_A::_1
    }
}
impl core::ops::Deref for PID6_R {
    type Target = crate::FieldReader<bool, PID6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID6` writer - Port Input Disable"]
pub struct PID6_W<'a> {
    w: &'a mut W,
}
impl<'a> PID6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID6_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID6_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID7_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID7_A> for bool {
    #[inline(always)]
    fn from(variant: PID7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID7` reader - Port Input Disable"]
pub struct PID7_R(crate::FieldReader<bool, PID7_A>);
impl PID7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID7_A {
        match self.bits {
            false => PID7_A::_0,
            true => PID7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID7_A::_1
    }
}
impl core::ops::Deref for PID7_R {
    type Target = crate::FieldReader<bool, PID7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID7` writer - Port Input Disable"]
pub struct PID7_W<'a> {
    w: &'a mut W,
}
impl<'a> PID7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID7_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID7_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID8_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID8_A> for bool {
    #[inline(always)]
    fn from(variant: PID8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID8` reader - Port Input Disable"]
pub struct PID8_R(crate::FieldReader<bool, PID8_A>);
impl PID8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID8_A {
        match self.bits {
            false => PID8_A::_0,
            true => PID8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID8_A::_1
    }
}
impl core::ops::Deref for PID8_R {
    type Target = crate::FieldReader<bool, PID8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID8` writer - Port Input Disable"]
pub struct PID8_W<'a> {
    w: &'a mut W,
}
impl<'a> PID8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID8_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID8_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID9_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID9_A> for bool {
    #[inline(always)]
    fn from(variant: PID9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID9` reader - Port Input Disable"]
pub struct PID9_R(crate::FieldReader<bool, PID9_A>);
impl PID9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID9_A {
        match self.bits {
            false => PID9_A::_0,
            true => PID9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID9_A::_1
    }
}
impl core::ops::Deref for PID9_R {
    type Target = crate::FieldReader<bool, PID9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID9` writer - Port Input Disable"]
pub struct PID9_W<'a> {
    w: &'a mut W,
}
impl<'a> PID9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID9_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID9_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID10_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID10_A> for bool {
    #[inline(always)]
    fn from(variant: PID10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID10` reader - Port Input Disable"]
pub struct PID10_R(crate::FieldReader<bool, PID10_A>);
impl PID10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID10_A {
        match self.bits {
            false => PID10_A::_0,
            true => PID10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID10_A::_1
    }
}
impl core::ops::Deref for PID10_R {
    type Target = crate::FieldReader<bool, PID10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID10` writer - Port Input Disable"]
pub struct PID10_W<'a> {
    w: &'a mut W,
}
impl<'a> PID10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID10_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID11_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID11_A> for bool {
    #[inline(always)]
    fn from(variant: PID11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID11` reader - Port Input Disable"]
pub struct PID11_R(crate::FieldReader<bool, PID11_A>);
impl PID11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID11_A {
        match self.bits {
            false => PID11_A::_0,
            true => PID11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID11_A::_1
    }
}
impl core::ops::Deref for PID11_R {
    type Target = crate::FieldReader<bool, PID11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID11` writer - Port Input Disable"]
pub struct PID11_W<'a> {
    w: &'a mut W,
}
impl<'a> PID11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID11_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID11_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID12_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID12_A> for bool {
    #[inline(always)]
    fn from(variant: PID12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID12` reader - Port Input Disable"]
pub struct PID12_R(crate::FieldReader<bool, PID12_A>);
impl PID12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID12_A {
        match self.bits {
            false => PID12_A::_0,
            true => PID12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID12_A::_1
    }
}
impl core::ops::Deref for PID12_R {
    type Target = crate::FieldReader<bool, PID12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID12` writer - Port Input Disable"]
pub struct PID12_W<'a> {
    w: &'a mut W,
}
impl<'a> PID12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID12_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID12_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID13_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID13_A> for bool {
    #[inline(always)]
    fn from(variant: PID13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID13` reader - Port Input Disable"]
pub struct PID13_R(crate::FieldReader<bool, PID13_A>);
impl PID13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID13_A {
        match self.bits {
            false => PID13_A::_0,
            true => PID13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID13_A::_1
    }
}
impl core::ops::Deref for PID13_R {
    type Target = crate::FieldReader<bool, PID13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID13` writer - Port Input Disable"]
pub struct PID13_W<'a> {
    w: &'a mut W,
}
impl<'a> PID13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID13_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID13_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID14_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID14_A> for bool {
    #[inline(always)]
    fn from(variant: PID14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID14` reader - Port Input Disable"]
pub struct PID14_R(crate::FieldReader<bool, PID14_A>);
impl PID14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID14_A {
        match self.bits {
            false => PID14_A::_0,
            true => PID14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID14_A::_1
    }
}
impl core::ops::Deref for PID14_R {
    type Target = crate::FieldReader<bool, PID14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID14` writer - Port Input Disable"]
pub struct PID14_W<'a> {
    w: &'a mut W,
}
impl<'a> PID14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID14_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID14_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID15_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID15_A> for bool {
    #[inline(always)]
    fn from(variant: PID15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID15` reader - Port Input Disable"]
pub struct PID15_R(crate::FieldReader<bool, PID15_A>);
impl PID15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID15_A {
        match self.bits {
            false => PID15_A::_0,
            true => PID15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID15_A::_1
    }
}
impl core::ops::Deref for PID15_R {
    type Target = crate::FieldReader<bool, PID15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID15` writer - Port Input Disable"]
pub struct PID15_W<'a> {
    w: &'a mut W,
}
impl<'a> PID15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID15_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID15_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID16_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID16_A> for bool {
    #[inline(always)]
    fn from(variant: PID16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID16` reader - Port Input Disable"]
pub struct PID16_R(crate::FieldReader<bool, PID16_A>);
impl PID16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID16_A {
        match self.bits {
            false => PID16_A::_0,
            true => PID16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID16_A::_1
    }
}
impl core::ops::Deref for PID16_R {
    type Target = crate::FieldReader<bool, PID16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID16` writer - Port Input Disable"]
pub struct PID16_W<'a> {
    w: &'a mut W,
}
impl<'a> PID16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID16_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID16_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID17_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID17_A> for bool {
    #[inline(always)]
    fn from(variant: PID17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID17` reader - Port Input Disable"]
pub struct PID17_R(crate::FieldReader<bool, PID17_A>);
impl PID17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID17_A {
        match self.bits {
            false => PID17_A::_0,
            true => PID17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID17_A::_1
    }
}
impl core::ops::Deref for PID17_R {
    type Target = crate::FieldReader<bool, PID17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID17` writer - Port Input Disable"]
pub struct PID17_W<'a> {
    w: &'a mut W,
}
impl<'a> PID17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID17_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID17_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID18_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID18_A> for bool {
    #[inline(always)]
    fn from(variant: PID18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID18` reader - Port Input Disable"]
pub struct PID18_R(crate::FieldReader<bool, PID18_A>);
impl PID18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID18_A {
        match self.bits {
            false => PID18_A::_0,
            true => PID18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID18_A::_1
    }
}
impl core::ops::Deref for PID18_R {
    type Target = crate::FieldReader<bool, PID18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID18` writer - Port Input Disable"]
pub struct PID18_W<'a> {
    w: &'a mut W,
}
impl<'a> PID18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID18_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID18_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID19_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID19_A> for bool {
    #[inline(always)]
    fn from(variant: PID19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID19` reader - Port Input Disable"]
pub struct PID19_R(crate::FieldReader<bool, PID19_A>);
impl PID19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID19_A {
        match self.bits {
            false => PID19_A::_0,
            true => PID19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID19_A::_1
    }
}
impl core::ops::Deref for PID19_R {
    type Target = crate::FieldReader<bool, PID19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID19` writer - Port Input Disable"]
pub struct PID19_W<'a> {
    w: &'a mut W,
}
impl<'a> PID19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID19_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID19_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID20_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID20_A> for bool {
    #[inline(always)]
    fn from(variant: PID20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID20` reader - Port Input Disable"]
pub struct PID20_R(crate::FieldReader<bool, PID20_A>);
impl PID20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID20_A {
        match self.bits {
            false => PID20_A::_0,
            true => PID20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID20_A::_1
    }
}
impl core::ops::Deref for PID20_R {
    type Target = crate::FieldReader<bool, PID20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID20` writer - Port Input Disable"]
pub struct PID20_W<'a> {
    w: &'a mut W,
}
impl<'a> PID20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID20_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID20_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID21_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID21_A> for bool {
    #[inline(always)]
    fn from(variant: PID21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID21` reader - Port Input Disable"]
pub struct PID21_R(crate::FieldReader<bool, PID21_A>);
impl PID21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID21_A {
        match self.bits {
            false => PID21_A::_0,
            true => PID21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID21_A::_1
    }
}
impl core::ops::Deref for PID21_R {
    type Target = crate::FieldReader<bool, PID21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID21` writer - Port Input Disable"]
pub struct PID21_W<'a> {
    w: &'a mut W,
}
impl<'a> PID21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID21_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID21_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID22_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID22_A> for bool {
    #[inline(always)]
    fn from(variant: PID22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID22` reader - Port Input Disable"]
pub struct PID22_R(crate::FieldReader<bool, PID22_A>);
impl PID22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID22_A {
        match self.bits {
            false => PID22_A::_0,
            true => PID22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID22_A::_1
    }
}
impl core::ops::Deref for PID22_R {
    type Target = crate::FieldReader<bool, PID22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID22` writer - Port Input Disable"]
pub struct PID22_W<'a> {
    w: &'a mut W,
}
impl<'a> PID22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID22_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID22_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID23_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID23_A> for bool {
    #[inline(always)]
    fn from(variant: PID23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID23` reader - Port Input Disable"]
pub struct PID23_R(crate::FieldReader<bool, PID23_A>);
impl PID23_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID23_A {
        match self.bits {
            false => PID23_A::_0,
            true => PID23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID23_A::_1
    }
}
impl core::ops::Deref for PID23_R {
    type Target = crate::FieldReader<bool, PID23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID23` writer - Port Input Disable"]
pub struct PID23_W<'a> {
    w: &'a mut W,
}
impl<'a> PID23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID23_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID23_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID24_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID24_A> for bool {
    #[inline(always)]
    fn from(variant: PID24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID24` reader - Port Input Disable"]
pub struct PID24_R(crate::FieldReader<bool, PID24_A>);
impl PID24_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID24_A {
        match self.bits {
            false => PID24_A::_0,
            true => PID24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID24_A::_1
    }
}
impl core::ops::Deref for PID24_R {
    type Target = crate::FieldReader<bool, PID24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID24` writer - Port Input Disable"]
pub struct PID24_W<'a> {
    w: &'a mut W,
}
impl<'a> PID24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID24_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID24_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID25_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID25_A> for bool {
    #[inline(always)]
    fn from(variant: PID25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID25` reader - Port Input Disable"]
pub struct PID25_R(crate::FieldReader<bool, PID25_A>);
impl PID25_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID25_A {
        match self.bits {
            false => PID25_A::_0,
            true => PID25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID25_A::_1
    }
}
impl core::ops::Deref for PID25_R {
    type Target = crate::FieldReader<bool, PID25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID25` writer - Port Input Disable"]
pub struct PID25_W<'a> {
    w: &'a mut W,
}
impl<'a> PID25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID25_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID25_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID26_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID26_A> for bool {
    #[inline(always)]
    fn from(variant: PID26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID26` reader - Port Input Disable"]
pub struct PID26_R(crate::FieldReader<bool, PID26_A>);
impl PID26_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID26_A {
        match self.bits {
            false => PID26_A::_0,
            true => PID26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID26_A::_1
    }
}
impl core::ops::Deref for PID26_R {
    type Target = crate::FieldReader<bool, PID26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID26` writer - Port Input Disable"]
pub struct PID26_W<'a> {
    w: &'a mut W,
}
impl<'a> PID26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID26_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID26_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID27_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID27_A> for bool {
    #[inline(always)]
    fn from(variant: PID27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID27` reader - Port Input Disable"]
pub struct PID27_R(crate::FieldReader<bool, PID27_A>);
impl PID27_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID27_A {
        match self.bits {
            false => PID27_A::_0,
            true => PID27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID27_A::_1
    }
}
impl core::ops::Deref for PID27_R {
    type Target = crate::FieldReader<bool, PID27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID27` writer - Port Input Disable"]
pub struct PID27_W<'a> {
    w: &'a mut W,
}
impl<'a> PID27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID27_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID27_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID28_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID28_A> for bool {
    #[inline(always)]
    fn from(variant: PID28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID28` reader - Port Input Disable"]
pub struct PID28_R(crate::FieldReader<bool, PID28_A>);
impl PID28_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID28_A {
        match self.bits {
            false => PID28_A::_0,
            true => PID28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID28_A::_1
    }
}
impl core::ops::Deref for PID28_R {
    type Target = crate::FieldReader<bool, PID28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID28` writer - Port Input Disable"]
pub struct PID28_W<'a> {
    w: &'a mut W,
}
impl<'a> PID28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID28_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID28_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID29_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID29_A> for bool {
    #[inline(always)]
    fn from(variant: PID29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID29` reader - Port Input Disable"]
pub struct PID29_R(crate::FieldReader<bool, PID29_A>);
impl PID29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID29_A {
        match self.bits {
            false => PID29_A::_0,
            true => PID29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID29_A::_1
    }
}
impl core::ops::Deref for PID29_R {
    type Target = crate::FieldReader<bool, PID29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID29` writer - Port Input Disable"]
pub struct PID29_W<'a> {
    w: &'a mut W,
}
impl<'a> PID29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID29_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID29_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID30_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID30_A> for bool {
    #[inline(always)]
    fn from(variant: PID30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID30` reader - Port Input Disable"]
pub struct PID30_R(crate::FieldReader<bool, PID30_A>);
impl PID30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID30_A {
        match self.bits {
            false => PID30_A::_0,
            true => PID30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID30_A::_1
    }
}
impl core::ops::Deref for PID30_R {
    type Target = crate::FieldReader<bool, PID30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID30` writer - Port Input Disable"]
pub struct PID30_W<'a> {
    w: &'a mut W,
}
impl<'a> PID30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID30_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID30_A::_1)
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
#[doc = "Port Input Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID31_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID31_A> for bool {
    #[inline(always)]
    fn from(variant: PID31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID31` reader - Port Input Disable"]
pub struct PID31_R(crate::FieldReader<bool, PID31_A>);
impl PID31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID31_A {
        match self.bits {
            false => PID31_A::_0,
            true => PID31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID31_A::_1
    }
}
impl core::ops::Deref for PID31_R {
    type Target = crate::FieldReader<bool, PID31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID31` writer - Port Input Disable"]
pub struct PID31_W<'a> {
    w: &'a mut W,
}
impl<'a> PID31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID31_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID31_A::_1)
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
    #[doc = "Bit 0 - Port Input Disable"]
    #[inline(always)]
    pub fn pid0(&self) -> PID0_R {
        PID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Input Disable"]
    #[inline(always)]
    pub fn pid1(&self) -> PID1_R {
        PID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Input Disable"]
    #[inline(always)]
    pub fn pid2(&self) -> PID2_R {
        PID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Input Disable"]
    #[inline(always)]
    pub fn pid3(&self) -> PID3_R {
        PID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Input Disable"]
    #[inline(always)]
    pub fn pid4(&self) -> PID4_R {
        PID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Input Disable"]
    #[inline(always)]
    pub fn pid5(&self) -> PID5_R {
        PID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Input Disable"]
    #[inline(always)]
    pub fn pid6(&self) -> PID6_R {
        PID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Input Disable"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Input Disable"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Input Disable"]
    #[inline(always)]
    pub fn pid9(&self) -> PID9_R {
        PID9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Input Disable"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Input Disable"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Input Disable"]
    #[inline(always)]
    pub fn pid12(&self) -> PID12_R {
        PID12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Input Disable"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Input Disable"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Input Disable"]
    #[inline(always)]
    pub fn pid15(&self) -> PID15_R {
        PID15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Input Disable"]
    #[inline(always)]
    pub fn pid16(&self) -> PID16_R {
        PID16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Input Disable"]
    #[inline(always)]
    pub fn pid17(&self) -> PID17_R {
        PID17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Input Disable"]
    #[inline(always)]
    pub fn pid18(&self) -> PID18_R {
        PID18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Input Disable"]
    #[inline(always)]
    pub fn pid19(&self) -> PID19_R {
        PID19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Input Disable"]
    #[inline(always)]
    pub fn pid20(&self) -> PID20_R {
        PID20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Input Disable"]
    #[inline(always)]
    pub fn pid21(&self) -> PID21_R {
        PID21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Input Disable"]
    #[inline(always)]
    pub fn pid22(&self) -> PID22_R {
        PID22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Input Disable"]
    #[inline(always)]
    pub fn pid23(&self) -> PID23_R {
        PID23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Input Disable"]
    #[inline(always)]
    pub fn pid24(&self) -> PID24_R {
        PID24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Input Disable"]
    #[inline(always)]
    pub fn pid25(&self) -> PID25_R {
        PID25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Input Disable"]
    #[inline(always)]
    pub fn pid26(&self) -> PID26_R {
        PID26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Input Disable"]
    #[inline(always)]
    pub fn pid27(&self) -> PID27_R {
        PID27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Input Disable"]
    #[inline(always)]
    pub fn pid28(&self) -> PID28_R {
        PID28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Input Disable"]
    #[inline(always)]
    pub fn pid29(&self) -> PID29_R {
        PID29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Input Disable"]
    #[inline(always)]
    pub fn pid30(&self) -> PID30_R {
        PID30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid31(&self) -> PID31_R {
        PID31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Input Disable"]
    #[inline(always)]
    pub fn pid0(&mut self) -> PID0_W {
        PID0_W { w: self }
    }
    #[doc = "Bit 1 - Port Input Disable"]
    #[inline(always)]
    pub fn pid1(&mut self) -> PID1_W {
        PID1_W { w: self }
    }
    #[doc = "Bit 2 - Port Input Disable"]
    #[inline(always)]
    pub fn pid2(&mut self) -> PID2_W {
        PID2_W { w: self }
    }
    #[doc = "Bit 3 - Port Input Disable"]
    #[inline(always)]
    pub fn pid3(&mut self) -> PID3_W {
        PID3_W { w: self }
    }
    #[doc = "Bit 4 - Port Input Disable"]
    #[inline(always)]
    pub fn pid4(&mut self) -> PID4_W {
        PID4_W { w: self }
    }
    #[doc = "Bit 5 - Port Input Disable"]
    #[inline(always)]
    pub fn pid5(&mut self) -> PID5_W {
        PID5_W { w: self }
    }
    #[doc = "Bit 6 - Port Input Disable"]
    #[inline(always)]
    pub fn pid6(&mut self) -> PID6_W {
        PID6_W { w: self }
    }
    #[doc = "Bit 7 - Port Input Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> PID7_W {
        PID7_W { w: self }
    }
    #[doc = "Bit 8 - Port Input Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PID8_W {
        PID8_W { w: self }
    }
    #[doc = "Bit 9 - Port Input Disable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> PID9_W {
        PID9_W { w: self }
    }
    #[doc = "Bit 10 - Port Input Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PID10_W {
        PID10_W { w: self }
    }
    #[doc = "Bit 11 - Port Input Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PID11_W {
        PID11_W { w: self }
    }
    #[doc = "Bit 12 - Port Input Disable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> PID12_W {
        PID12_W { w: self }
    }
    #[doc = "Bit 13 - Port Input Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PID13_W {
        PID13_W { w: self }
    }
    #[doc = "Bit 14 - Port Input Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PID14_W {
        PID14_W { w: self }
    }
    #[doc = "Bit 15 - Port Input Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PID15_W {
        PID15_W { w: self }
    }
    #[doc = "Bit 16 - Port Input Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> PID16_W {
        PID16_W { w: self }
    }
    #[doc = "Bit 17 - Port Input Disable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> PID17_W {
        PID17_W { w: self }
    }
    #[doc = "Bit 18 - Port Input Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> PID18_W {
        PID18_W { w: self }
    }
    #[doc = "Bit 19 - Port Input Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> PID19_W {
        PID19_W { w: self }
    }
    #[doc = "Bit 20 - Port Input Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> PID20_W {
        PID20_W { w: self }
    }
    #[doc = "Bit 21 - Port Input Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> PID21_W {
        PID21_W { w: self }
    }
    #[doc = "Bit 22 - Port Input Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> PID22_W {
        PID22_W { w: self }
    }
    #[doc = "Bit 23 - Port Input Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> PID23_W {
        PID23_W { w: self }
    }
    #[doc = "Bit 24 - Port Input Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> PID24_W {
        PID24_W { w: self }
    }
    #[doc = "Bit 25 - Port Input Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> PID25_W {
        PID25_W { w: self }
    }
    #[doc = "Bit 26 - Port Input Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> PID26_W {
        PID26_W { w: self }
    }
    #[doc = "Bit 27 - Port Input Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> PID27_W {
        PID27_W { w: self }
    }
    #[doc = "Bit 28 - Port Input Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> PID28_W {
        PID28_W { w: self }
    }
    #[doc = "Bit 29 - Port Input Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> PID29_W {
        PID29_W { w: self }
    }
    #[doc = "Bit 30 - Port Input Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> PID30_W {
        PID30_W { w: self }
    }
    #[doc = "Bit 31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> PID31_W {
        PID31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Input Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr](index.html) module"]
pub struct PIDR_SPEC;
impl crate::RegisterSpec for PIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr::R](R) reader structure"]
impl crate::Readable for PIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr::W](W) writer structure"]
impl crate::Writable for PIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIDR to value 0xffff_ffff"]
impl crate::Resettable for PIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
