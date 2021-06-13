#[doc = "Register `FCNFG` reader"]
pub struct R(crate::R<FCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCNFG` writer"]
pub struct W(crate::W<FCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCNFG_SPEC>;
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
impl From<crate::W<FCNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debugger Mass Erase Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQ_A {
    #[doc = "0: No request or request complete"]
    _0 = 0,
    #[doc = "1: Request to run the Erase All Blocks command verify the erased state program the security byte in the Flash Configuration Field to the unsecure state release MCU security by setting FSEC\\[SEC\\]
to the unsecure state"]
    _1 = 1,
}
impl From<ERSAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERSAREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSAREQ` reader - Debugger Mass Erase Request"]
pub struct ERSAREQ_R(crate::FieldReader<bool, ERSAREQ_A>);
impl ERSAREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERSAREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSAREQ_A {
        match self.bits {
            false => ERSAREQ_A::_0,
            true => ERSAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERSAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERSAREQ_A::_1
    }
}
impl core::ops::Deref for ERSAREQ_R {
    type Target = crate::FieldReader<bool, ERSAREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Command complete interrupt is disabled."]
    _0 = 0,
    #[doc = "1: An interrupt will be requested whenever the CCIF flag in the FSTAT register is set."]
    _1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Command Complete Interrupt Enable"]
pub struct CCIE_R(crate::FieldReader<bool, CCIE_A>);
impl CCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::_0,
            true => CCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CCIE_A::_1
    }
}
impl core::ops::Deref for CCIE_R {
    type Target = crate::FieldReader<bool, CCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCIE` writer - Command Complete Interrupt Enable"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Command complete interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIE_A::_0)
    }
    #[doc = "An interrupt will be requested whenever the CCIF flag in the FSTAT register is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Debugger Mass Erase Request"]
    #[inline(always)]
    pub fn ersareq(&self) -> ERSAREQ_R {
        ERSAREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](index.html) module"]
pub struct FCNFG_SPEC;
impl crate::RegisterSpec for FCNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcnfg::R](R) reader structure"]
impl crate::Readable for FCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](W) writer structure"]
impl crate::Writable for FCNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCNFG to value 0"]
impl crate::Resettable for FCNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
