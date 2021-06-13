#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S` writer"]
pub struct W(crate::W<S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_SPEC>;
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
impl From<crate::W<S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Mode Fault Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error detected"]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Master Mode Fault Flag"]
pub struct MODF_R(crate::FieldReader<bool, MODF_A>);
impl MODF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODF_A::_1
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, MODF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI Transmit Buffer Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTEF_A {
    #[doc = "0: SPI transmit buffer not empty"]
    _0 = 0,
    #[doc = "1: SPI transmit buffer empty"]
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPTEF` reader - SPI Transmit Buffer Empty Flag"]
pub struct SPTEF_R(crate::FieldReader<bool, SPTEF_A>);
impl SPTEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPTEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPTEF_A::_1
    }
}
impl core::ops::Deref for SPTEF_R {
    type Target = crate::FieldReader<bool, SPTEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMF_A {
    #[doc = "0: Value in the receive data buffer does not match the value in the M register"]
    _0 = 0,
    #[doc = "1: Value in the receive data buffer matches the value in the M register"]
    _1 = 1,
}
impl From<SPMF_A> for bool {
    #[inline(always)]
    fn from(variant: SPMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPMF` reader - SPI Match Flag"]
pub struct SPMF_R(crate::FieldReader<bool, SPMF_A>);
impl SPMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMF_A {
        match self.bits {
            false => SPMF_A::_0,
            true => SPMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPMF_A::_1
    }
}
impl core::ops::Deref for SPMF_R {
    type Target = crate::FieldReader<bool, SPMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPMF` writer - SPI Match Flag"]
pub struct SPMF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPMF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Value in the receive data buffer does not match the value in the M register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMF_A::_0)
    }
    #[doc = "Value in the receive data buffer matches the value in the M register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "SPI Read Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRF_A {
    #[doc = "0: No data available in the receive data buffer"]
    _0 = 0,
    #[doc = "1: Data available in the receive data buffer"]
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRF` reader - SPI Read Buffer Full Flag"]
pub struct SPRF_R(crate::FieldReader<bool, SPRF_A>);
impl SPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPRF_A::_1
    }
}
impl core::ops::Deref for SPRF_R {
    type Target = crate::FieldReader<bool, SPRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Master Mode Fault Flag"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI Match Flag"]
    #[inline(always)]
    pub fn spmf(&self) -> SPMF_R {
        SPMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI Read Buffer Full Flag"]
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - SPI Match Flag"]
    #[inline(always)]
    pub fn spmf(&mut self) -> SPMF_W {
        SPMF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s::W](W) writer structure"]
impl crate::Writable for S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S to value 0x20"]
impl crate::Resettable for S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
