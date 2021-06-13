#[doc = "Register `SC5` reader"]
pub struct R(crate::R<SC5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC5` writer"]
pub struct W(crate::W<SC5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC5_SPEC>;
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
impl From<crate::W<SC5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hardware Trigger Mask Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTRGMASKSEL_A {
    #[doc = "0: Hardware trigger mask with HTRGMASKE."]
    _0 = 0,
    #[doc = "1: Hardware trigger mask automatically when data fifo is not empty."]
    _1 = 1,
}
impl From<HTRGMASKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HTRGMASKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTRGMASKSEL` reader - Hardware Trigger Mask Mode Select"]
pub struct HTRGMASKSEL_R(crate::FieldReader<bool, HTRGMASKSEL_A>);
impl HTRGMASKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTRGMASKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTRGMASKSEL_A {
        match self.bits {
            false => HTRGMASKSEL_A::_0,
            true => HTRGMASKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HTRGMASKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HTRGMASKSEL_A::_1
    }
}
impl core::ops::Deref for HTRGMASKSEL_R {
    type Target = crate::FieldReader<bool, HTRGMASKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTRGMASKSEL` writer - Hardware Trigger Mask Mode Select"]
pub struct HTRGMASKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HTRGMASKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTRGMASKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware trigger mask with HTRGMASKE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTRGMASKSEL_A::_0)
    }
    #[doc = "Hardware trigger mask automatically when data fifo is not empty."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTRGMASKSEL_A::_1)
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
#[doc = "Hardware Trigger Mask Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTRGMASKE_A {
    #[doc = "0: Hardware trigger mask disable."]
    _0 = 0,
    #[doc = "1: Hardware trigger mask enable and hardware trigger cannot trigger ADC conversion.."]
    _1 = 1,
}
impl From<HTRGMASKE_A> for bool {
    #[inline(always)]
    fn from(variant: HTRGMASKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTRGMASKE` reader - Hardware Trigger Mask Enable"]
pub struct HTRGMASKE_R(crate::FieldReader<bool, HTRGMASKE_A>);
impl HTRGMASKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTRGMASKE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTRGMASKE_A {
        match self.bits {
            false => HTRGMASKE_A::_0,
            true => HTRGMASKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HTRGMASKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HTRGMASKE_A::_1
    }
}
impl core::ops::Deref for HTRGMASKE_R {
    type Target = crate::FieldReader<bool, HTRGMASKE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTRGMASKE` writer - Hardware Trigger Mask Enable"]
pub struct HTRGMASKE_W<'a> {
    w: &'a mut W,
}
impl<'a> HTRGMASKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTRGMASKE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware trigger mask disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTRGMASKE_A::_0)
    }
    #[doc = "Hardware trigger mask enable and hardware trigger cannot trigger ADC conversion.."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTRGMASKE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Hardware Trigger Mask Mode Select"]
    #[inline(always)]
    pub fn htrgmasksel(&self) -> HTRGMASKSEL_R {
        HTRGMASKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Trigger Mask Enable"]
    #[inline(always)]
    pub fn htrgmaske(&self) -> HTRGMASKE_R {
        HTRGMASKE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Trigger Mask Mode Select"]
    #[inline(always)]
    pub fn htrgmasksel(&mut self) -> HTRGMASKSEL_W {
        HTRGMASKSEL_W { w: self }
    }
    #[doc = "Bit 1 - Hardware Trigger Mask Enable"]
    #[inline(always)]
    pub fn htrgmaske(&mut self) -> HTRGMASKE_W {
        HTRGMASKE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc5](index.html) module"]
pub struct SC5_SPEC;
impl crate::RegisterSpec for SC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc5::R](R) reader structure"]
impl crate::Readable for SC5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc5::W](W) writer structure"]
impl crate::Writable for SC5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC5 to value 0"]
impl crate::Resettable for SC5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
