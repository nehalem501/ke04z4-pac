#[doc = "Register `DATAHU` reader"]
pub struct R(crate::R<DATAHU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAHU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAHU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAHU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAHU` writer"]
pub struct W(crate::W<DATAHU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAHU_SPEC>;
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
impl From<crate::W<DATAHU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAHU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAHU` reader - DATAHU stores the fourth 8 bits of the 32 bit CRC"]
pub struct DATAHU_R(crate::FieldReader<u8, u8>);
impl DATAHU_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAHU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAHU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAHU` writer - DATAHU stores the fourth 8 bits of the 32 bit CRC"]
pub struct DATAHU_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datahu(&self) -> DATAHU_R {
        DATAHU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datahu(&mut self) -> DATAHU_W {
        DATAHU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_DATAHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datahu](index.html) module"]
pub struct DATAHU_SPEC;
impl crate::RegisterSpec for DATAHU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datahu::R](R) reader structure"]
impl crate::Readable for DATAHU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datahu::W](W) writer structure"]
impl crate::Writable for DATAHU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAHU to value 0xff"]
impl crate::Resettable for DATAHU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
