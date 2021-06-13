#[doc = "Register `CLKDIV` reader"]
pub struct R(crate::R<CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV` writer"]
pub struct W(crate::W<CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_SPEC>;
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
impl From<crate::W<CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock 3 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV3_A {
    #[doc = "0: Same as ICSOUTCLK."]
    _0 = 0,
    #[doc = "1: ICSOUTCLK divides by 2."]
    _1 = 1,
}
impl From<OUTDIV3_A> for bool {
    #[inline(always)]
    fn from(variant: OUTDIV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTDIV3` reader - Clock 3 output divider value"]
pub struct OUTDIV3_R(crate::FieldReader<bool, OUTDIV3_A>);
impl OUTDIV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTDIV3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV3_A {
        match self.bits {
            false => OUTDIV3_A::_0,
            true => OUTDIV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OUTDIV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OUTDIV3_A::_1
    }
}
impl core::ops::Deref for OUTDIV3_R {
    type Target = crate::FieldReader<bool, OUTDIV3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDIV3` writer - Clock 3 output divider value"]
pub struct OUTDIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Same as ICSOUTCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0)
    }
    #[doc = "ICSOUTCLK divides by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1)
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
#[doc = "Clock 2 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV2_A {
    #[doc = "0: Not divided from divider1."]
    _0 = 0,
    #[doc = "1: Divide by 2 from divider1."]
    _1 = 1,
}
impl From<OUTDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: OUTDIV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTDIV2` reader - Clock 2 output divider value"]
pub struct OUTDIV2_R(crate::FieldReader<bool, OUTDIV2_A>);
impl OUTDIV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTDIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV2_A {
        match self.bits {
            false => OUTDIV2_A::_0,
            true => OUTDIV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OUTDIV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OUTDIV2_A::_1
    }
}
impl core::ops::Deref for OUTDIV2_R {
    type Target = crate::FieldReader<bool, OUTDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDIV2` writer - Clock 2 output divider value"]
pub struct OUTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not divided from divider1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0)
    }
    #[doc = "Divide by 2 from divider1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1)
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
#[doc = "Clock 1 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTDIV1_A {
    #[doc = "0: Same as ICSOUTCLK."]
    _00 = 0,
    #[doc = "1: ICSOUTCLK divides by 2."]
    _01 = 1,
    #[doc = "2: ICSOUTCLK divides by 3."]
    _10 = 2,
    #[doc = "3: ICSOUTCLK divides by 4."]
    _11 = 3,
}
impl From<OUTDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTDIV1` reader - Clock 1 output divider value"]
pub struct OUTDIV1_R(crate::FieldReader<u8, OUTDIV1_A>);
impl OUTDIV1_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV1_A {
        match self.bits {
            0 => OUTDIV1_A::_00,
            1 => OUTDIV1_A::_01,
            2 => OUTDIV1_A::_10,
            3 => OUTDIV1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == OUTDIV1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == OUTDIV1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == OUTDIV1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == OUTDIV1_A::_11
    }
}
impl core::ops::Deref for OUTDIV1_R {
    type Target = crate::FieldReader<u8, OUTDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDIV1` writer - Clock 1 output divider value"]
pub struct OUTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Same as ICSOUTCLK."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_00)
    }
    #[doc = "ICSOUTCLK divides by 2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_01)
    }
    #[doc = "ICSOUTCLK divides by 3."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_10)
    }
    #[doc = "ICSOUTCLK divides by 4."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - Clock 3 output divider value"]
    #[inline(always)]
    pub fn outdiv3(&self) -> OUTDIV3_R {
        OUTDIV3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&self) -> OUTDIV2_R {
        OUTDIV2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&self) -> OUTDIV1_R {
        OUTDIV1_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 20 - Clock 3 output divider value"]
    #[inline(always)]
    pub fn outdiv3(&mut self) -> OUTDIV3_W {
        OUTDIV3_W { w: self }
    }
    #[doc = "Bit 24 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&mut self) -> OUTDIV2_W {
        OUTDIV2_W { w: self }
    }
    #[doc = "Bits 28:29 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&mut self) -> OUTDIV1_W {
        OUTDIV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](index.html) module"]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv::R](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
