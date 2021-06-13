#[doc = "Register `HDRVE` reader"]
pub struct R(crate::R<HDRVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDRVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDRVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDRVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDRVE` writer"]
pub struct W(crate::W<HDRVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDRVE_SPEC>;
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
impl From<crate::W<HDRVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDRVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "High Current Drive Capability of PTB5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTB5_A {
    #[doc = "0: PTB5 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTB5 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTB5_A> for bool {
    #[inline(always)]
    fn from(variant: PTB5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTB5` reader - High Current Drive Capability of PTB5"]
pub struct PTB5_R(crate::FieldReader<bool, PTB5_A>);
impl PTB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTB5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTB5_A {
        match self.bits {
            false => PTB5_A::_0,
            true => PTB5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTB5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTB5_A::_1
    }
}
impl core::ops::Deref for PTB5_R {
    type Target = crate::FieldReader<bool, PTB5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTB5` writer - High Current Drive Capability of PTB5"]
pub struct PTB5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTB5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTB5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PTB5 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTB5_A::_0)
    }
    #[doc = "PTB5 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTB5_A::_1)
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
#[doc = "High Current Drive Capability of PTC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTC1_A {
    #[doc = "0: PTC1 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTC1 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTC1_A> for bool {
    #[inline(always)]
    fn from(variant: PTC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTC1` reader - High Current Drive Capability of PTC1"]
pub struct PTC1_R(crate::FieldReader<bool, PTC1_A>);
impl PTC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTC1_A {
        match self.bits {
            false => PTC1_A::_0,
            true => PTC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTC1_A::_1
    }
}
impl core::ops::Deref for PTC1_R {
    type Target = crate::FieldReader<bool, PTC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTC1` writer - High Current Drive Capability of PTC1"]
pub struct PTC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTC1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PTC1 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTC1_A::_0)
    }
    #[doc = "PTC1 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTC1_A::_1)
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
#[doc = "High Current Drive Capability of PTC5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTC5_A {
    #[doc = "0: PTC5 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTC5 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTC5_A> for bool {
    #[inline(always)]
    fn from(variant: PTC5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTC5` reader - High Current Drive Capability of PTC5"]
pub struct PTC5_R(crate::FieldReader<bool, PTC5_A>);
impl PTC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTC5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTC5_A {
        match self.bits {
            false => PTC5_A::_0,
            true => PTC5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTC5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTC5_A::_1
    }
}
impl core::ops::Deref for PTC5_R {
    type Target = crate::FieldReader<bool, PTC5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTC5` writer - High Current Drive Capability of PTC5"]
pub struct PTC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTC5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PTC5 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTC5_A::_0)
    }
    #[doc = "PTC5 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTC5_A::_1)
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
impl R {
    #[doc = "Bit 1 - High Current Drive Capability of PTB5"]
    #[inline(always)]
    pub fn ptb5(&self) -> PTB5_R {
        PTB5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Current Drive Capability of PTC1"]
    #[inline(always)]
    pub fn ptc1(&self) -> PTC1_R {
        PTC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Current Drive Capability of PTC5"]
    #[inline(always)]
    pub fn ptc5(&self) -> PTC5_R {
        PTC5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - High Current Drive Capability of PTB5"]
    #[inline(always)]
    pub fn ptb5(&mut self) -> PTB5_W {
        PTB5_W { w: self }
    }
    #[doc = "Bit 2 - High Current Drive Capability of PTC1"]
    #[inline(always)]
    pub fn ptc1(&mut self) -> PTC1_W {
        PTC1_W { w: self }
    }
    #[doc = "Bit 3 - High Current Drive Capability of PTC5"]
    #[inline(always)]
    pub fn ptc5(&mut self) -> PTC5_W {
        PTC5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port High Drive Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdrve](index.html) module"]
pub struct HDRVE_SPEC;
impl crate::RegisterSpec for HDRVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdrve::R](R) reader structure"]
impl crate::Readable for HDRVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hdrve::W](W) writer structure"]
impl crate::Writable for HDRVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDRVE to value 0"]
impl crate::Resettable for HDRVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
