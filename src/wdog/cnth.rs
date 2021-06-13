#[doc = "Register `CNTH` reader"]
pub struct R(crate::R<CNTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTH` writer"]
pub struct W(crate::W<CNTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTH_SPEC>;
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
impl From<crate::W<CNTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTHIGH` reader - High byte of the Watchdog Counter"]
pub struct CNTHIGH_R(crate::FieldReader<u8, u8>);
impl CNTHIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTHIGH` writer - High byte of the Watchdog Counter"]
pub struct CNTHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&self) -> CNTHIGH_R {
        CNTHIGH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&mut self) -> CNTHIGH_W {
        CNTHIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counter Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](index.html) module"]
pub struct CNTH_SPEC;
impl crate::RegisterSpec for CNTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cnth::R](R) reader structure"]
impl crate::Readable for CNTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnth::W](W) writer structure"]
impl crate::Writable for CNTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
