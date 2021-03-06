#[doc = "Register `UUIDML` reader"]
pub struct R(crate::R<UUIDML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUIDML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUIDML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUIDML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Universally Unique Identifier"]
pub struct ID_R(crate::FieldReader<u32, u32>);
impl ID_R {
    pub(crate) fn new(bits: u32) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Universally Unique Identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Universally Unique Identifier Middle Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidml](index.html) module"]
pub struct UUIDML_SPEC;
impl crate::RegisterSpec for UUIDML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuidml::R](R) reader structure"]
impl crate::Readable for UUIDML_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UUIDML to value 0"]
impl crate::Resettable for UUIDML_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
