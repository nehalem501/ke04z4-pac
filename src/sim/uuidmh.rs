#[doc = "Register `UUIDMH` reader"]
pub struct R(crate::R<UUIDMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUIDMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUIDMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUIDMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Universally Unique Identifier"]
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Universally Unique Identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Universally Unique Identifier Middle High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidmh](index.html) module"]
pub struct UUIDMH_SPEC;
impl crate::RegisterSpec for UUIDMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuidmh::R](R) reader structure"]
impl crate::Readable for UUIDMH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UUIDMH to value 0"]
impl crate::Resettable for UUIDMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
