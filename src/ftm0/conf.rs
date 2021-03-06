#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMTOF` reader - TOF Frequency"]
pub struct NUMTOF_R(crate::FieldReader<u8, u8>);
impl NUMTOF_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUMTOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMTOF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMTOF` writer - TOF Frequency"]
pub struct NUMTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMTOF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `BDMMODE` reader - Debug Mode"]
pub struct BDMMODE_R(crate::FieldReader<u8, u8>);
impl BDMMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BDMMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDMMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMMODE` writer - Debug Mode"]
pub struct BDMMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Global Time Base Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEEN_A {
    #[doc = "0: Use of an external global time base is disabled."]
    _0 = 0,
    #[doc = "1: Use of an external global time base is enabled."]
    _1 = 1,
}
impl From<GTBEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBEEN` reader - Global Time Base Enable"]
pub struct GTBEEN_R(crate::FieldReader<bool, GTBEEN_A>);
impl GTBEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTBEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEEN_A {
        match self.bits {
            false => GTBEEN_A::_0,
            true => GTBEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GTBEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GTBEEN_A::_1
    }
}
impl core::ops::Deref for GTBEEN_R {
    type Target = crate::FieldReader<bool, GTBEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTBEEN` writer - Global Time Base Enable"]
pub struct GTBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use of an external global time base is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEEN_A::_0)
    }
    #[doc = "Use of an external global time base is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Global Time Base Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEOUT_A {
    #[doc = "0: A global time base signal generation is disabled."]
    _0 = 0,
    #[doc = "1: A global time base signal generation is enabled."]
    _1 = 1,
}
impl From<GTBEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBEOUT` reader - Global Time Base Output"]
pub struct GTBEOUT_R(crate::FieldReader<bool, GTBEOUT_A>);
impl GTBEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTBEOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEOUT_A {
        match self.bits {
            false => GTBEOUT_A::_0,
            true => GTBEOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GTBEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GTBEOUT_A::_1
    }
}
impl core::ops::Deref for GTBEOUT_R {
    type Target = crate::FieldReader<bool, GTBEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTBEOUT` writer - Global Time Base Output"]
pub struct GTBEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A global time base signal generation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEOUT_A::_0)
    }
    #[doc = "A global time base signal generation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEOUT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline(always)]
    pub fn numtof(&self) -> NUMTOF_R {
        NUMTOF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn bdmmode(&self) -> BDMMODE_R {
        BDMMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&self) -> GTBEOUT_R {
        GTBEOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline(always)]
    pub fn numtof(&mut self) -> NUMTOF_W {
        NUMTOF_W { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn bdmmode(&mut self) -> BDMMODE_W {
        BDMMODE_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&mut self) -> GTBEEN_W {
        GTBEEN_W { w: self }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&mut self) -> GTBEOUT_W {
        GTBEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
