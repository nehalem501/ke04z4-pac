#[doc = "Register `R2` reader"]
pub struct R(crate::R<R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPW` reader - Negative Pulse Width. It is suggested to use half-word (16-bit) or word (32-bit) to read out this value."]
pub struct NPW_R(crate::FieldReader<u16, u16>);
impl NPW_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWTC` reader - PWT Counter. It is suggested to use half-word (16-bit) or word (32-bit) to read out this value."]
pub struct PWTC_R(crate::FieldReader<u16, u16>);
impl PWTC_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWTC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Negative Pulse Width. It is suggested to use half-word (16-bit) or word (32-bit) to read out this value."]
    #[inline(always)]
    pub fn npw(&self) -> NPW_R {
        NPW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PWT Counter. It is suggested to use half-word (16-bit) or word (32-bit) to read out this value."]
    #[inline(always)]
    pub fn pwtc(&self) -> PWTC_R {
        PWTC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Pulse Width Timer Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2](index.html) module"]
pub struct R2_SPEC;
impl crate::RegisterSpec for R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r2::R](R) reader structure"]
impl crate::Readable for R2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R2 to value 0"]
impl crate::Resettable for R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
