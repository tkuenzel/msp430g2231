#[doc = "Reader of register USICTL0"]
pub type R = crate::R<u8, super::USICTL0>;
#[doc = "Writer for register USICTL0"]
pub type W = crate::W<u8, super::USICTL0>;
#[doc = "Register USICTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::USICTL0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USISWRST`"]
pub type USISWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USISWRST`"]
pub struct USISWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USISWRST_W<'a> {
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
#[doc = "Reader of field `USIOE`"]
pub type USIOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIOE`"]
pub struct USIOE_W<'a> {
    w: &'a mut W,
}
impl<'a> USIOE_W<'a> {
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
#[doc = "Reader of field `USIGE`"]
pub type USIGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIGE`"]
pub struct USIGE_W<'a> {
    w: &'a mut W,
}
impl<'a> USIGE_W<'a> {
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
#[doc = "Reader of field `USIMST`"]
pub type USIMST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIMST`"]
pub struct USIMST_W<'a> {
    w: &'a mut W,
}
impl<'a> USIMST_W<'a> {
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
#[doc = "Reader of field `USILSB`"]
pub type USILSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USILSB`"]
pub struct USILSB_W<'a> {
    w: &'a mut W,
}
impl<'a> USILSB_W<'a> {
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
#[doc = "Reader of field `USIPE5`"]
pub type USIPE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIPE5`"]
pub struct USIPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> USIPE5_W<'a> {
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
#[doc = "Reader of field `USIPE6`"]
pub type USIPE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIPE6`"]
pub struct USIPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> USIPE6_W<'a> {
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
#[doc = "Reader of field `USIPE7`"]
pub type USIPE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIPE7`"]
pub struct USIPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> USIPE7_W<'a> {
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
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&self) -> USISWRST_R {
        USISWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&self) -> USIOE_R {
        USIOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&self) -> USIGE_R {
        USIGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&self) -> USIMST_R {
        USIMST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&self) -> USILSB_R {
        USILSB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&self) -> USIPE5_R {
        USIPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&self) -> USIPE6_R {
        USIPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&self) -> USIPE7_R {
        USIPE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&mut self) -> USISWRST_W {
        USISWRST_W { w: self }
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&mut self) -> USIOE_W {
        USIOE_W { w: self }
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&mut self) -> USIGE_W {
        USIGE_W { w: self }
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&mut self) -> USIMST_W {
        USIMST_W { w: self }
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&mut self) -> USILSB_W {
        USILSB_W { w: self }
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&mut self) -> USIPE5_W {
        USIPE5_W { w: self }
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&mut self) -> USIPE6_W {
        USIPE6_W { w: self }
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&mut self) -> USIPE7_W {
        USIPE7_W { w: self }
    }
}
