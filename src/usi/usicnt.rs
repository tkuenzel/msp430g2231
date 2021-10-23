#[doc = "Reader of register USICNT"]
pub type R = crate::R<u8, super::USICNT>;
#[doc = "Writer for register USICNT"]
pub type W = crate::W<u8, super::USICNT>;
#[doc = "Register USICNT `reset()`'s with value 0"]
impl crate::ResetValue for super::USICNT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USICNT0`"]
pub type USICNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICNT0`"]
pub struct USICNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> USICNT0_W<'a> {
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
#[doc = "Reader of field `USICNT1`"]
pub type USICNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICNT1`"]
pub struct USICNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> USICNT1_W<'a> {
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
#[doc = "Reader of field `USICNT2`"]
pub type USICNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICNT2`"]
pub struct USICNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> USICNT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `USICNT3`"]
pub type USICNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICNT3`"]
pub struct USICNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> USICNT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `USICNT4`"]
pub type USICNT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICNT4`"]
pub struct USICNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> USICNT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `USIIFGCC`"]
pub type USIIFGCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIIFGCC`"]
pub struct USIIFGCC_W<'a> {
    w: &'a mut W,
}
impl<'a> USIIFGCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `USI16B`"]
pub type USI16B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USI16B`"]
pub struct USI16B_W<'a> {
    w: &'a mut W,
}
impl<'a> USI16B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `USISCLREL`"]
pub type USISCLREL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USISCLREL`"]
pub struct USISCLREL_W<'a> {
    w: &'a mut W,
}
impl<'a> USISCLREL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&self) -> USICNT0_R {
        USICNT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&self) -> USICNT1_R {
        USICNT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&self) -> USICNT2_R {
        USICNT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&self) -> USICNT3_R {
        USICNT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&self) -> USICNT4_R {
        USICNT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&self) -> USIIFGCC_R {
        USIIFGCC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&self) -> USI16B_R {
        USI16B_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&self) -> USISCLREL_R {
        USISCLREL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&mut self) -> USICNT0_W {
        USICNT0_W { w: self }
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&mut self) -> USICNT1_W {
        USICNT1_W { w: self }
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&mut self) -> USICNT2_W {
        USICNT2_W { w: self }
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&mut self) -> USICNT3_W {
        USICNT3_W { w: self }
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&mut self) -> USICNT4_W {
        USICNT4_W { w: self }
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&mut self) -> USIIFGCC_W {
        USIIFGCC_W { w: self }
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&mut self) -> USI16B_W {
        USI16B_W { w: self }
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&mut self) -> USISCLREL_W {
        USISCLREL_W { w: self }
    }
}
