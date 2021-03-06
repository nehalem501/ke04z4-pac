#[doc = "Register `SC4` reader"]
pub struct R(crate::R<SC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC4` writer"]
pub struct W(crate::W<SC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC4_SPEC>;
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
impl From<crate::W<SC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFDEP_A {
    #[doc = "0: FIFO is disabled."]
    _000 = 0,
    #[doc = "1: 2-level FIFO is enabled."]
    _001 = 1,
    #[doc = "2: 3-level FIFO is enabled.."]
    _010 = 2,
    #[doc = "3: 4-level FIFO is enabled."]
    _011 = 3,
    #[doc = "4: 5-level FIFO is enabled."]
    _100 = 4,
    #[doc = "5: 6-level FIFO is enabled."]
    _101 = 5,
    #[doc = "6: 7-level FIFO is enabled."]
    _110 = 6,
    #[doc = "7: 8-level FIFO is enabled."]
    _111 = 7,
}
impl From<AFDEP_A> for u8 {
    #[inline(always)]
    fn from(variant: AFDEP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AFDEP` reader - FIFO Depth"]
pub struct AFDEP_R(crate::FieldReader<u8, AFDEP_A>);
impl AFDEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFDEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFDEP_A {
        match self.bits {
            0 => AFDEP_A::_000,
            1 => AFDEP_A::_001,
            2 => AFDEP_A::_010,
            3 => AFDEP_A::_011,
            4 => AFDEP_A::_100,
            5 => AFDEP_A::_101,
            6 => AFDEP_A::_110,
            7 => AFDEP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == AFDEP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == AFDEP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == AFDEP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == AFDEP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == AFDEP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == AFDEP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == AFDEP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == AFDEP_A::_111
    }
}
impl core::ops::Deref for AFDEP_R {
    type Target = crate::FieldReader<u8, AFDEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFDEP` writer - FIFO Depth"]
pub struct AFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> AFDEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFDEP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FIFO is disabled."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(AFDEP_A::_000)
    }
    #[doc = "2-level FIFO is enabled."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(AFDEP_A::_001)
    }
    #[doc = "3-level FIFO is enabled.."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(AFDEP_A::_010)
    }
    #[doc = "4-level FIFO is enabled."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(AFDEP_A::_011)
    }
    #[doc = "5-level FIFO is enabled."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(AFDEP_A::_100)
    }
    #[doc = "6-level FIFO is enabled."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(AFDEP_A::_101)
    }
    #[doc = "7-level FIFO is enabled."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(AFDEP_A::_110)
    }
    #[doc = "8-level FIFO is enabled."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(AFDEP_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Compare Function Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFSEL_A {
    #[doc = "0: OR all of compare trigger."]
    _0 = 0,
    #[doc = "1: AND all of compare trigger."]
    _1 = 1,
}
impl From<ACFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ACFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFSEL` reader - Compare Function Selection"]
pub struct ACFSEL_R(crate::FieldReader<bool, ACFSEL_A>);
impl ACFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFSEL_A {
        match self.bits {
            false => ACFSEL_A::_0,
            true => ACFSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACFSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACFSEL_A::_1
    }
}
impl core::ops::Deref for ACFSEL_R {
    type Target = crate::FieldReader<bool, ACFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACFSEL` writer - Compare Function Selection"]
pub struct ACFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OR all of compare trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFSEL_A::_0)
    }
    #[doc = "AND all of compare trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "FIFO Scan Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASCANE_A {
    #[doc = "0: FIFO scan mode disabled."]
    _0 = 0,
    #[doc = "1: FIFO scan mode enabled."]
    _1 = 1,
}
impl From<ASCANE_A> for bool {
    #[inline(always)]
    fn from(variant: ASCANE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCANE` reader - FIFO Scan Mode Enable"]
pub struct ASCANE_R(crate::FieldReader<bool, ASCANE_A>);
impl ASCANE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASCANE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCANE_A {
        match self.bits {
            false => ASCANE_A::_0,
            true => ASCANE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ASCANE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ASCANE_A::_1
    }
}
impl core::ops::Deref for ASCANE_R {
    type Target = crate::FieldReader<bool, ASCANE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASCANE` writer - FIFO Scan Mode Enable"]
pub struct ASCANE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASCANE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASCANE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO scan mode disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCANE_A::_0)
    }
    #[doc = "FIFO scan mode enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCANE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Hardware Trigger Multiple Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTRGME_A {
    #[doc = "0: One hardware trigger pulse triggers one conversion."]
    _0 = 0,
    #[doc = "1: One hardware trigger pulse triggers multiple conversions in fifo mode."]
    _1 = 1,
}
impl From<HTRGME_A> for bool {
    #[inline(always)]
    fn from(variant: HTRGME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTRGME` reader - Hardware Trigger Multiple Conversion Enable"]
pub struct HTRGME_R(crate::FieldReader<bool, HTRGME_A>);
impl HTRGME_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTRGME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTRGME_A {
        match self.bits {
            false => HTRGME_A::_0,
            true => HTRGME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HTRGME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HTRGME_A::_1
    }
}
impl core::ops::Deref for HTRGME_R {
    type Target = crate::FieldReader<bool, HTRGME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTRGME` writer - Hardware Trigger Multiple Conversion Enable"]
pub struct HTRGME_W<'a> {
    w: &'a mut W,
}
impl<'a> HTRGME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTRGME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One hardware trigger pulse triggers one conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTRGME_A::_0)
    }
    #[doc = "One hardware trigger pulse triggers multiple conversions in fifo mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTRGME_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FIFO Depth"]
    #[inline(always)]
    pub fn afdep(&self) -> AFDEP_R {
        AFDEP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - Compare Function Selection"]
    #[inline(always)]
    pub fn acfsel(&self) -> ACFSEL_R {
        ACFSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FIFO Scan Mode Enable"]
    #[inline(always)]
    pub fn ascane(&self) -> ASCANE_R {
        ASCANE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Hardware Trigger Multiple Conversion Enable"]
    #[inline(always)]
    pub fn htrgme(&self) -> HTRGME_R {
        HTRGME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO Depth"]
    #[inline(always)]
    pub fn afdep(&mut self) -> AFDEP_W {
        AFDEP_W { w: self }
    }
    #[doc = "Bit 5 - Compare Function Selection"]
    #[inline(always)]
    pub fn acfsel(&mut self) -> ACFSEL_W {
        ACFSEL_W { w: self }
    }
    #[doc = "Bit 6 - FIFO Scan Mode Enable"]
    #[inline(always)]
    pub fn ascane(&mut self) -> ASCANE_W {
        ASCANE_W { w: self }
    }
    #[doc = "Bit 8 - Hardware Trigger Multiple Conversion Enable"]
    #[inline(always)]
    pub fn htrgme(&mut self) -> HTRGME_W {
        HTRGME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc4](index.html) module"]
pub struct SC4_SPEC;
impl crate::RegisterSpec for SC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc4::R](R) reader structure"]
impl crate::Readable for SC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc4::W](W) writer structure"]
impl crate::Writable for SC4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC4 to value 0"]
impl crate::Resettable for SC4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
