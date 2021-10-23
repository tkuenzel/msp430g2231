#[doc = "Reader of register USICKCTL"]
pub type R = crate::R<u8, super::USICKCTL>;
#[doc = "Writer for register USICKCTL"]
pub type W = crate::W<u8, super::USICKCTL>;
#[doc = "Register USICKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::USICKCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USISWCLK`"]
pub type USISWCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USISWCLK`"]
pub struct USISWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USISWCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `USICKPL`"]
pub type USICKPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICKPL`"]
pub struct USICKPL_W<'a> {
    w: &'a mut W,
}
impl<'a> USICKPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "USI Clock Source Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USISSEL_A {
    #[doc = "0: USI  Clock Source: 0"]
    USISSEL_0 = 0,
    #[doc = "1: USI  Clock Source: 1"]
    USISSEL_1 = 1,
    #[doc = "2: USI  Clock Source: 2"]
    USISSEL_2 = 2,
    #[doc = "3: USI  Clock Source: 3"]
    USISSEL_3 = 3,
    #[doc = "4: USI  Clock Source: 4"]
    USISSEL_4 = 4,
    #[doc = "5: USI  Clock Source: 5"]
    USISSEL_5 = 5,
    #[doc = "6: USI  Clock Source: 6"]
    USISSEL_6 = 6,
    #[doc = "7: USI  Clock Source: 7"]
    USISSEL_7 = 7,
}
impl From<USISSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USISSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USISSEL`"]
pub type USISSEL_R = crate::R<u8, USISSEL_A>;
impl USISSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USISSEL_A {
        match self.bits {
            0 => USISSEL_A::USISSEL_0,
            1 => USISSEL_A::USISSEL_1,
            2 => USISSEL_A::USISSEL_2,
            3 => USISSEL_A::USISSEL_3,
            4 => USISSEL_A::USISSEL_4,
            5 => USISSEL_A::USISSEL_5,
            6 => USISSEL_A::USISSEL_6,
            7 => USISSEL_A::USISSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USISSEL_0`"]
    #[inline(always)]
    pub fn is_usissel_0(&self) -> bool {
        *self == USISSEL_A::USISSEL_0
    }
    #[doc = "Checks if the value of the field is `USISSEL_1`"]
    #[inline(always)]
    pub fn is_usissel_1(&self) -> bool {
        *self == USISSEL_A::USISSEL_1
    }
    #[doc = "Checks if the value of the field is `USISSEL_2`"]
    #[inline(always)]
    pub fn is_usissel_2(&self) -> bool {
        *self == USISSEL_A::USISSEL_2
    }
    #[doc = "Checks if the value of the field is `USISSEL_3`"]
    #[inline(always)]
    pub fn is_usissel_3(&self) -> bool {
        *self == USISSEL_A::USISSEL_3
    }
    #[doc = "Checks if the value of the field is `USISSEL_4`"]
    #[inline(always)]
    pub fn is_usissel_4(&self) -> bool {
        *self == USISSEL_A::USISSEL_4
    }
    #[doc = "Checks if the value of the field is `USISSEL_5`"]
    #[inline(always)]
    pub fn is_usissel_5(&self) -> bool {
        *self == USISSEL_A::USISSEL_5
    }
    #[doc = "Checks if the value of the field is `USISSEL_6`"]
    #[inline(always)]
    pub fn is_usissel_6(&self) -> bool {
        *self == USISSEL_A::USISSEL_6
    }
    #[doc = "Checks if the value of the field is `USISSEL_7`"]
    #[inline(always)]
    pub fn is_usissel_7(&self) -> bool {
        *self == USISSEL_A::USISSEL_7
    }
}
#[doc = "Write proxy for field `USISSEL`"]
pub struct USISSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USISSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USISSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USI Clock Source: 0"]
    #[inline(always)]
    pub fn usissel_0(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_0)
    }
    #[doc = "USI Clock Source: 1"]
    #[inline(always)]
    pub fn usissel_1(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_1)
    }
    #[doc = "USI Clock Source: 2"]
    #[inline(always)]
    pub fn usissel_2(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_2)
    }
    #[doc = "USI Clock Source: 3"]
    #[inline(always)]
    pub fn usissel_3(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_3)
    }
    #[doc = "USI Clock Source: 4"]
    #[inline(always)]
    pub fn usissel_4(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_4)
    }
    #[doc = "USI Clock Source: 5"]
    #[inline(always)]
    pub fn usissel_5(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_5)
    }
    #[doc = "USI Clock Source: 6"]
    #[inline(always)]
    pub fn usissel_6(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_6)
    }
    #[doc = "USI Clock Source: 7"]
    #[inline(always)]
    pub fn usissel_7(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u8) & 0x07) << 2);
        self.w
    }
}
#[doc = "USI Clock Divider 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USIDIV_A {
    #[doc = "0: USI  Clock Divider: 0"]
    USIDIV_0 = 0,
    #[doc = "1: USI  Clock Divider: 1"]
    USIDIV_1 = 1,
    #[doc = "2: USI  Clock Divider: 2"]
    USIDIV_2 = 2,
    #[doc = "3: USI  Clock Divider: 3"]
    USIDIV_3 = 3,
    #[doc = "4: USI  Clock Divider: 4"]
    USIDIV_4 = 4,
    #[doc = "5: USI  Clock Divider: 5"]
    USIDIV_5 = 5,
    #[doc = "6: USI  Clock Divider: 6"]
    USIDIV_6 = 6,
    #[doc = "7: USI  Clock Divider: 7"]
    USIDIV_7 = 7,
}
impl From<USIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: USIDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USIDIV`"]
pub type USIDIV_R = crate::R<u8, USIDIV_A>;
impl USIDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIDIV_A {
        match self.bits {
            0 => USIDIV_A::USIDIV_0,
            1 => USIDIV_A::USIDIV_1,
            2 => USIDIV_A::USIDIV_2,
            3 => USIDIV_A::USIDIV_3,
            4 => USIDIV_A::USIDIV_4,
            5 => USIDIV_A::USIDIV_5,
            6 => USIDIV_A::USIDIV_6,
            7 => USIDIV_A::USIDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USIDIV_0`"]
    #[inline(always)]
    pub fn is_usidiv_0(&self) -> bool {
        *self == USIDIV_A::USIDIV_0
    }
    #[doc = "Checks if the value of the field is `USIDIV_1`"]
    #[inline(always)]
    pub fn is_usidiv_1(&self) -> bool {
        *self == USIDIV_A::USIDIV_1
    }
    #[doc = "Checks if the value of the field is `USIDIV_2`"]
    #[inline(always)]
    pub fn is_usidiv_2(&self) -> bool {
        *self == USIDIV_A::USIDIV_2
    }
    #[doc = "Checks if the value of the field is `USIDIV_3`"]
    #[inline(always)]
    pub fn is_usidiv_3(&self) -> bool {
        *self == USIDIV_A::USIDIV_3
    }
    #[doc = "Checks if the value of the field is `USIDIV_4`"]
    #[inline(always)]
    pub fn is_usidiv_4(&self) -> bool {
        *self == USIDIV_A::USIDIV_4
    }
    #[doc = "Checks if the value of the field is `USIDIV_5`"]
    #[inline(always)]
    pub fn is_usidiv_5(&self) -> bool {
        *self == USIDIV_A::USIDIV_5
    }
    #[doc = "Checks if the value of the field is `USIDIV_6`"]
    #[inline(always)]
    pub fn is_usidiv_6(&self) -> bool {
        *self == USIDIV_A::USIDIV_6
    }
    #[doc = "Checks if the value of the field is `USIDIV_7`"]
    #[inline(always)]
    pub fn is_usidiv_7(&self) -> bool {
        *self == USIDIV_A::USIDIV_7
    }
}
#[doc = "Write proxy for field `USIDIV`"]
pub struct USIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USIDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USI Clock Divider: 0"]
    #[inline(always)]
    pub fn usidiv_0(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_0)
    }
    #[doc = "USI Clock Divider: 1"]
    #[inline(always)]
    pub fn usidiv_1(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_1)
    }
    #[doc = "USI Clock Divider: 2"]
    #[inline(always)]
    pub fn usidiv_2(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_2)
    }
    #[doc = "USI Clock Divider: 3"]
    #[inline(always)]
    pub fn usidiv_3(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_3)
    }
    #[doc = "USI Clock Divider: 4"]
    #[inline(always)]
    pub fn usidiv_4(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_4)
    }
    #[doc = "USI Clock Divider: 5"]
    #[inline(always)]
    pub fn usidiv_5(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_5)
    }
    #[doc = "USI Clock Divider: 6"]
    #[inline(always)]
    pub fn usidiv_6(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_6)
    }
    #[doc = "USI Clock Divider: 7"]
    #[inline(always)]
    pub fn usidiv_7(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u8) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USI Software Clock"]
    #[inline(always)]
    pub fn usiswclk(&self) -> USISWCLK_R {
        USISWCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
    #[inline(always)]
    pub fn usickpl(&self) -> USICKPL_R {
        USICKPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - USI Clock Source Select 2"]
    #[inline(always)]
    pub fn usissel(&self) -> USISSEL_R {
        USISSEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - USI Clock Divider 2"]
    #[inline(always)]
    pub fn usidiv(&self) -> USIDIV_R {
        USIDIV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Clock"]
    #[inline(always)]
    pub fn usiswclk(&mut self) -> USISWCLK_W {
        USISWCLK_W { w: self }
    }
    #[doc = "Bit 1 - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
    #[inline(always)]
    pub fn usickpl(&mut self) -> USICKPL_W {
        USICKPL_W { w: self }
    }
    #[doc = "Bits 2:4 - USI Clock Source Select 2"]
    #[inline(always)]
    pub fn usissel(&mut self) -> USISSEL_W {
        USISSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - USI Clock Divider 2"]
    #[inline(always)]
    pub fn usidiv(&mut self) -> USIDIV_W {
        USIDIV_W { w: self }
    }
}
