#[doc = "Register `IOFLT` reader"]
pub struct R(crate::R<IOFLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOFLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOFLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOFLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOFLT` writer"]
pub struct W(crate::W<IOFLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOFLT_SPEC>;
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
impl From<crate::W<IOFLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOFLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter Selection for Input from PTA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTA_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTA_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTA` reader - Filter Selection for Input from PTA"]
pub struct FLTA_R(crate::FieldReader<u8, FLTA_A>);
impl FLTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTA_A {
        match self.bits {
            0 => FLTA_A::_00,
            1 => FLTA_A::_01,
            2 => FLTA_A::_10,
            3 => FLTA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTA_A::_11
    }
}
impl core::ops::Deref for FLTA_R {
    type Target = crate::FieldReader<u8, FLTA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTA` writer - Filter Selection for Input from PTA"]
pub struct FLTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTA_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTA_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTA_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTA_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTA_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTB_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTB_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTB` reader - Filter Selection for Input from PTB"]
pub struct FLTB_R(crate::FieldReader<u8, FLTB_A>);
impl FLTB_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTB_A {
        match self.bits {
            0 => FLTB_A::_00,
            1 => FLTB_A::_01,
            2 => FLTB_A::_10,
            3 => FLTB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTB_A::_11
    }
}
impl core::ops::Deref for FLTB_R {
    type Target = crate::FieldReader<u8, FLTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTB` writer - Filter Selection for Input from PTB"]
pub struct FLTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTB_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTB_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTB_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTB_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTB_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTC_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTC_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTC` reader - Filter Selection for Input from PTC"]
pub struct FLTC_R(crate::FieldReader<u8, FLTC_A>);
impl FLTC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTC_A {
        match self.bits {
            0 => FLTC_A::_00,
            1 => FLTC_A::_01,
            2 => FLTC_A::_10,
            3 => FLTC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTC_A::_11
    }
}
impl core::ops::Deref for FLTC_R {
    type Target = crate::FieldReader<u8, FLTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTC` writer - Filter Selection for Input from PTC"]
pub struct FLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTC_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTC_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTC_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Filter Selection For Input from SCL/SDA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTIIC_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select BUSCLK"]
    _11 = 3,
}
impl From<FLTIIC_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTIIC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTIIC` reader - Filter Selection For Input from SCL/SDA"]
pub struct FLTIIC_R(crate::FieldReader<u8, FLTIIC_A>);
impl FLTIIC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTIIC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTIIC_A {
        match self.bits {
            0 => FLTIIC_A::_00,
            1 => FLTIIC_A::_01,
            2 => FLTIIC_A::_10,
            3 => FLTIIC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTIIC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTIIC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTIIC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTIIC_A::_11
    }
}
impl core::ops::Deref for FLTIIC_R {
    type Target = crate::FieldReader<u8, FLTIIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTIIC` writer - Filter Selection For Input from SCL/SDA"]
pub struct FLTIIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTIIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTIIC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTIIC_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTIIC_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTIIC_A::_10)
    }
    #[doc = "Select BUSCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTIIC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Filter Selection For Input from FTM0CH0/FTM0CH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTFTM0_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select FLTDIV3"]
    _11 = 3,
}
impl From<FLTFTM0_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTFTM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTFTM0` reader - Filter Selection For Input from FTM0CH0/FTM0CH1"]
pub struct FLTFTM0_R(crate::FieldReader<u8, FLTFTM0_A>);
impl FLTFTM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTFTM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTFTM0_A {
        match self.bits {
            0 => FLTFTM0_A::_00,
            1 => FLTFTM0_A::_01,
            2 => FLTFTM0_A::_10,
            3 => FLTFTM0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTFTM0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTFTM0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTFTM0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTFTM0_A::_11
    }
}
impl core::ops::Deref for FLTFTM0_R {
    type Target = crate::FieldReader<u8, FLTFTM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTFTM0` writer - Filter Selection For Input from FTM0CH0/FTM0CH1"]
pub struct FLTFTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTFTM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTFTM0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_10)
    }
    #[doc = "Select FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Filter Selection For Input from PWT_IN1/PWT_IN0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTPWT_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select FLTDIV3"]
    _11 = 3,
}
impl From<FLTPWT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTPWT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTPWT` reader - Filter Selection For Input from PWT_IN1/PWT_IN0"]
pub struct FLTPWT_R(crate::FieldReader<u8, FLTPWT_A>);
impl FLTPWT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTPWT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTPWT_A {
        match self.bits {
            0 => FLTPWT_A::_00,
            1 => FLTPWT_A::_01,
            2 => FLTPWT_A::_10,
            3 => FLTPWT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTPWT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTPWT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTPWT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTPWT_A::_11
    }
}
impl core::ops::Deref for FLTPWT_R {
    type Target = crate::FieldReader<u8, FLTPWT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTPWT` writer - Filter Selection For Input from PWT_IN1/PWT_IN0"]
pub struct FLTPWT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTPWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTPWT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTPWT_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTPWT_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTPWT_A::_10)
    }
    #[doc = "Select FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTPWT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Filter Selection for Input from RESET/IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTRST_A {
    #[doc = "0: No filter."]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTRST_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTRST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTRST` reader - Filter Selection for Input from RESET/IRQ"]
pub struct FLTRST_R(crate::FieldReader<u8, FLTRST_A>);
impl FLTRST_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTRST_A {
        match self.bits {
            0 => FLTRST_A::_00,
            1 => FLTRST_A::_01,
            2 => FLTRST_A::_10,
            3 => FLTRST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTRST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTRST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTRST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTRST_A::_11
    }
}
impl core::ops::Deref for FLTRST_R {
    type Target = crate::FieldReader<u8, FLTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTRST` writer - Filter Selection for Input from RESET/IRQ"]
pub struct FLTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTRST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTRST_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTRST_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTRST_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTRST_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Filter selection for Input from KBI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTKBI0_A {
    #[doc = "0: No filter."]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTKBI0_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTKBI0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTKBI0` reader - Filter selection for Input from KBI0"]
pub struct FLTKBI0_R(crate::FieldReader<u8, FLTKBI0_A>);
impl FLTKBI0_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTKBI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTKBI0_A {
        match self.bits {
            0 => FLTKBI0_A::_00,
            1 => FLTKBI0_A::_01,
            2 => FLTKBI0_A::_10,
            3 => FLTKBI0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTKBI0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTKBI0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTKBI0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTKBI0_A::_11
    }
}
impl core::ops::Deref for FLTKBI0_R {
    type Target = crate::FieldReader<u8, FLTKBI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTKBI0` writer - Filter selection for Input from KBI0"]
pub struct FLTKBI0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTKBI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTKBI0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Filter Selection for Input from KBI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTKBI1_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTKBI1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTKBI1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTKBI1` reader - Filter Selection for Input from KBI1"]
pub struct FLTKBI1_R(crate::FieldReader<u8, FLTKBI1_A>);
impl FLTKBI1_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTKBI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTKBI1_A {
        match self.bits {
            0 => FLTKBI1_A::_00,
            1 => FLTKBI1_A::_01,
            2 => FLTKBI1_A::_10,
            3 => FLTKBI1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTKBI1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTKBI1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTKBI1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTKBI1_A::_11
    }
}
impl core::ops::Deref for FLTKBI1_R {
    type Target = crate::FieldReader<u8, FLTKBI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTKBI1` writer - Filter Selection for Input from KBI1"]
pub struct FLTKBI1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTKBI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTKBI1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Filter Selection for Input from NMI\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTNMI_A {
    #[doc = "0: No filter."]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTNMI_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTNMI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTNMI` reader - Filter Selection for Input from NMI"]
pub struct FLTNMI_R(crate::FieldReader<u8, FLTNMI_A>);
impl FLTNMI_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTNMI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTNMI_A {
        match self.bits {
            0 => FLTNMI_A::_00,
            1 => FLTNMI_A::_01,
            2 => FLTNMI_A::_10,
            3 => FLTNMI_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTNMI_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTNMI_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTNMI_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTNMI_A::_11
    }
}
impl core::ops::Deref for FLTNMI_R {
    type Target = crate::FieldReader<u8, FLTNMI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTNMI` writer - Filter Selection for Input from NMI"]
pub struct FLTNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTNMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTNMI_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTNMI_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTNMI_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTNMI_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTNMI_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Filter Division Set 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTDIV1_A {
    #[doc = "0: BUSCLK/2"]
    _00 = 0,
    #[doc = "1: BUSCLK/4"]
    _01 = 1,
    #[doc = "2: BUSCLK/8"]
    _10 = 2,
    #[doc = "3: BUSCLK/16"]
    _11 = 3,
}
impl From<FLTDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTDIV1` reader - Filter Division Set 1"]
pub struct FLTDIV1_R(crate::FieldReader<u8, FLTDIV1_A>);
impl FLTDIV1_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTDIV1_A {
        match self.bits {
            0 => FLTDIV1_A::_00,
            1 => FLTDIV1_A::_01,
            2 => FLTDIV1_A::_10,
            3 => FLTDIV1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLTDIV1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLTDIV1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLTDIV1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLTDIV1_A::_11
    }
}
impl core::ops::Deref for FLTDIV1_R {
    type Target = crate::FieldReader<u8, FLTDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTDIV1` writer - Filter Division Set 1"]
pub struct FLTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "BUSCLK/2"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_00)
    }
    #[doc = "BUSCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_01)
    }
    #[doc = "BUSCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_10)
    }
    #[doc = "BUSCLK/16"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Filter Division Set 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTDIV2_A {
    #[doc = "0: BUSCLK/32"]
    _000 = 0,
    #[doc = "1: BUSCLK/64"]
    _001 = 1,
    #[doc = "2: BUSCLK/128"]
    _010 = 2,
    #[doc = "3: BUSCLK/256"]
    _011 = 3,
    #[doc = "4: BUSCLK/512"]
    _100 = 4,
    #[doc = "5: BUSCLK/1024"]
    _101 = 5,
    #[doc = "6: BUSCLK/2048"]
    _110 = 6,
    #[doc = "7: BUSCLK/4096"]
    _111 = 7,
}
impl From<FLTDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTDIV2` reader - Filter Division Set 2"]
pub struct FLTDIV2_R(crate::FieldReader<u8, FLTDIV2_A>);
impl FLTDIV2_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTDIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTDIV2_A {
        match self.bits {
            0 => FLTDIV2_A::_000,
            1 => FLTDIV2_A::_001,
            2 => FLTDIV2_A::_010,
            3 => FLTDIV2_A::_011,
            4 => FLTDIV2_A::_100,
            5 => FLTDIV2_A::_101,
            6 => FLTDIV2_A::_110,
            7 => FLTDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FLTDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FLTDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == FLTDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == FLTDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == FLTDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == FLTDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == FLTDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == FLTDIV2_A::_111
    }
}
impl core::ops::Deref for FLTDIV2_R {
    type Target = crate::FieldReader<u8, FLTDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTDIV2` writer - Filter Division Set 2"]
pub struct FLTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTDIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "BUSCLK/32"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_000)
    }
    #[doc = "BUSCLK/64"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_001)
    }
    #[doc = "BUSCLK/128"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_010)
    }
    #[doc = "BUSCLK/256"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_011)
    }
    #[doc = "BUSCLK/512"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_100)
    }
    #[doc = "BUSCLK/1024"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_101)
    }
    #[doc = "BUSCLK/2048"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_110)
    }
    #[doc = "BUSCLK/4096"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | ((value as u32 & 0x07) << 26);
        self.w
    }
}
#[doc = "Filter Division Set 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTDIV3_A {
    #[doc = "0: LPOCLK"]
    _000 = 0,
    #[doc = "1: LPOCLK/2"]
    _001 = 1,
    #[doc = "2: LPOCLK/4"]
    _010 = 2,
    #[doc = "3: LPOCLK/8"]
    _011 = 3,
    #[doc = "4: LPOCLK/16"]
    _100 = 4,
    #[doc = "5: LPOCLK/32"]
    _101 = 5,
    #[doc = "6: LPOCLK/64"]
    _110 = 6,
    #[doc = "7: LPOCLK/128"]
    _111 = 7,
}
impl From<FLTDIV3_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTDIV3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTDIV3` reader - Filter Division Set 3"]
pub struct FLTDIV3_R(crate::FieldReader<u8, FLTDIV3_A>);
impl FLTDIV3_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTDIV3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTDIV3_A {
        match self.bits {
            0 => FLTDIV3_A::_000,
            1 => FLTDIV3_A::_001,
            2 => FLTDIV3_A::_010,
            3 => FLTDIV3_A::_011,
            4 => FLTDIV3_A::_100,
            5 => FLTDIV3_A::_101,
            6 => FLTDIV3_A::_110,
            7 => FLTDIV3_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FLTDIV3_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FLTDIV3_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == FLTDIV3_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == FLTDIV3_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == FLTDIV3_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == FLTDIV3_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == FLTDIV3_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == FLTDIV3_A::_111
    }
}
impl core::ops::Deref for FLTDIV3_R {
    type Target = crate::FieldReader<u8, FLTDIV3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTDIV3` writer - Filter Division Set 3"]
pub struct FLTDIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTDIV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTDIV3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "LPOCLK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_000)
    }
    #[doc = "LPOCLK/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_001)
    }
    #[doc = "LPOCLK/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_010)
    }
    #[doc = "LPOCLK/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_011)
    }
    #[doc = "LPOCLK/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_100)
    }
    #[doc = "LPOCLK/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_101)
    }
    #[doc = "LPOCLK/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_110)
    }
    #[doc = "LPOCLK/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter Selection for Input from PTA"]
    #[inline(always)]
    pub fn flta(&self) -> FLTA_R {
        FLTA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Filter Selection for Input from PTB"]
    #[inline(always)]
    pub fn fltb(&self) -> FLTB_R {
        FLTB_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Filter Selection for Input from PTC"]
    #[inline(always)]
    pub fn fltc(&self) -> FLTC_R {
        FLTC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Filter Selection For Input from SCL/SDA"]
    #[inline(always)]
    pub fn fltiic(&self) -> FLTIIC_R {
        FLTIIC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Filter Selection For Input from FTM0CH0/FTM0CH1"]
    #[inline(always)]
    pub fn fltftm0(&self) -> FLTFTM0_R {
        FLTFTM0_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Filter Selection For Input from PWT_IN1/PWT_IN0"]
    #[inline(always)]
    pub fn fltpwt(&self) -> FLTPWT_R {
        FLTPWT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Filter Selection for Input from RESET/IRQ"]
    #[inline(always)]
    pub fn fltrst(&self) -> FLTRST_R {
        FLTRST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Filter selection for Input from KBI0"]
    #[inline(always)]
    pub fn fltkbi0(&self) -> FLTKBI0_R {
        FLTKBI0_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Filter Selection for Input from KBI1"]
    #[inline(always)]
    pub fn fltkbi1(&self) -> FLTKBI1_R {
        FLTKBI1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Filter Selection for Input from NMI"]
    #[inline(always)]
    pub fn fltnmi(&self) -> FLTNMI_R {
        FLTNMI_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Filter Division Set 1"]
    #[inline(always)]
    pub fn fltdiv1(&self) -> FLTDIV1_R {
        FLTDIV1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28 - Filter Division Set 2"]
    #[inline(always)]
    pub fn fltdiv2(&self) -> FLTDIV2_R {
        FLTDIV2_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Filter Division Set 3"]
    #[inline(always)]
    pub fn fltdiv3(&self) -> FLTDIV3_R {
        FLTDIV3_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter Selection for Input from PTA"]
    #[inline(always)]
    pub fn flta(&mut self) -> FLTA_W {
        FLTA_W { w: self }
    }
    #[doc = "Bits 2:3 - Filter Selection for Input from PTB"]
    #[inline(always)]
    pub fn fltb(&mut self) -> FLTB_W {
        FLTB_W { w: self }
    }
    #[doc = "Bits 4:5 - Filter Selection for Input from PTC"]
    #[inline(always)]
    pub fn fltc(&mut self) -> FLTC_W {
        FLTC_W { w: self }
    }
    #[doc = "Bits 10:11 - Filter Selection For Input from SCL/SDA"]
    #[inline(always)]
    pub fn fltiic(&mut self) -> FLTIIC_W {
        FLTIIC_W { w: self }
    }
    #[doc = "Bits 12:13 - Filter Selection For Input from FTM0CH0/FTM0CH1"]
    #[inline(always)]
    pub fn fltftm0(&mut self) -> FLTFTM0_W {
        FLTFTM0_W { w: self }
    }
    #[doc = "Bits 14:15 - Filter Selection For Input from PWT_IN1/PWT_IN0"]
    #[inline(always)]
    pub fn fltpwt(&mut self) -> FLTPWT_W {
        FLTPWT_W { w: self }
    }
    #[doc = "Bits 16:17 - Filter Selection for Input from RESET/IRQ"]
    #[inline(always)]
    pub fn fltrst(&mut self) -> FLTRST_W {
        FLTRST_W { w: self }
    }
    #[doc = "Bits 18:19 - Filter selection for Input from KBI0"]
    #[inline(always)]
    pub fn fltkbi0(&mut self) -> FLTKBI0_W {
        FLTKBI0_W { w: self }
    }
    #[doc = "Bits 20:21 - Filter Selection for Input from KBI1"]
    #[inline(always)]
    pub fn fltkbi1(&mut self) -> FLTKBI1_W {
        FLTKBI1_W { w: self }
    }
    #[doc = "Bits 22:23 - Filter Selection for Input from NMI"]
    #[inline(always)]
    pub fn fltnmi(&mut self) -> FLTNMI_W {
        FLTNMI_W { w: self }
    }
    #[doc = "Bits 24:25 - Filter Division Set 1"]
    #[inline(always)]
    pub fn fltdiv1(&mut self) -> FLTDIV1_W {
        FLTDIV1_W { w: self }
    }
    #[doc = "Bits 26:28 - Filter Division Set 2"]
    #[inline(always)]
    pub fn fltdiv2(&mut self) -> FLTDIV2_W {
        FLTDIV2_W { w: self }
    }
    #[doc = "Bits 29:31 - Filter Division Set 3"]
    #[inline(always)]
    pub fn fltdiv3(&mut self) -> FLTDIV3_W {
        FLTDIV3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioflt](index.html) module"]
pub struct IOFLT_SPEC;
impl crate::RegisterSpec for IOFLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioflt::R](R) reader structure"]
impl crate::Readable for IOFLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioflt::W](W) writer structure"]
impl crate::Writable for IOFLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOFLT to value 0x00c0_0000"]
impl crate::Resettable for IOFLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00c0_0000
    }
}
