#[doc = "Register `CNTL` reader"]
pub struct R(crate::R<CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL` writer"]
pub struct W(crate::W<CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_SPEC>;
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
impl From<crate::W<CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTLOW` reader - Low byte of the Watchdog Counter"]
pub struct CNTLOW_R(crate::FieldReader<u8, u8>);
impl CNTLOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTLOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTLOW` writer - Low byte of the Watchdog Counter"]
pub struct CNTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&self) -> CNTLOW_R {
        CNTLOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&mut self) -> CNTLOW_W {
        CNTLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counter Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](index.html) module"]
pub struct CNTL_SPEC;
impl crate::RegisterSpec for CNTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cntl::R](R) reader structure"]
impl crate::Readable for CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl::W](W) writer structure"]
impl crate::Writable for CNTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
