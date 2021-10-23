#[doc = "Reader of register USICTL1"]
pub type R = crate::R<u8, super::USICTL1>;
#[doc = "Writer for register USICTL1"]
pub type W = crate::W<u8, super::USICTL1>;
#[doc = "Register USICTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::USICTL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USIIFG`"]
pub type USIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIIFG`"]
pub struct USIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USIIFG_W<'a> {
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
#[doc = "Reader of field `USISTTIFG`"]
pub type USISTTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USISTTIFG`"]
pub struct USISTTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USISTTIFG_W<'a> {
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
#[doc = "Reader of field `USISTP`"]
pub type USISTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USISTP`"]
pub struct USISTP_W<'a> {
    w: &'a mut W,
}
impl<'a> USISTP_W<'a> {
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
#[doc = "Reader of field `USIAL`"]
pub type USIAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIAL`"]
pub struct USIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> USIAL_W<'a> {
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
#[doc = "Reader of field `USIIE`"]
pub type USIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USIIE`"]
pub struct USIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USIIE_W<'a> {
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
#[doc = "Reader of field `USISTTIE`"]
pub type USISTTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USISTTIE`"]
pub struct USISTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USISTTIE_W<'a> {
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
#[doc = "Reader of field `USII2C`"]
pub type USII2C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USII2C`"]
pub struct USII2C_W<'a> {
    w: &'a mut W,
}
impl<'a> USII2C_W<'a> {
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
#[doc = "Reader of field `USICKPH`"]
pub type USICKPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USICKPH`"]
pub struct USICKPH_W<'a> {
    w: &'a mut W,
}
impl<'a> USICKPH_W<'a> {
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
    #[doc = "Bit 0 - USI Counter Interrupt Flag"]
    #[inline(always)]
    pub fn usiifg(&self) -> USIIFG_R {
        USIIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USI START Condition interrupt Flag"]
    #[inline(always)]
    pub fn usisttifg(&self) -> USISTTIFG_R {
        USISTTIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USI STOP Condition received"]
    #[inline(always)]
    pub fn usistp(&self) -> USISTP_R {
        USISTP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USI Arbitration Lost"]
    #[inline(always)]
    pub fn usial(&self) -> USIAL_R {
        USIAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USI Counter Interrupt enable"]
    #[inline(always)]
    pub fn usiie(&self) -> USIIE_R {
        USIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USI START Condition interrupt enable"]
    #[inline(always)]
    pub fn usisttie(&self) -> USISTTIE_R {
        USISTTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USI I2C Mode"]
    #[inline(always)]
    pub fn usii2c(&self) -> USII2C_R {
        USII2C_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USI Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn usickph(&self) -> USICKPH_R {
        USICKPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Counter Interrupt Flag"]
    #[inline(always)]
    pub fn usiifg(&mut self) -> USIIFG_W {
        USIIFG_W { w: self }
    }
    #[doc = "Bit 1 - USI START Condition interrupt Flag"]
    #[inline(always)]
    pub fn usisttifg(&mut self) -> USISTTIFG_W {
        USISTTIFG_W { w: self }
    }
    #[doc = "Bit 2 - USI STOP Condition received"]
    #[inline(always)]
    pub fn usistp(&mut self) -> USISTP_W {
        USISTP_W { w: self }
    }
    #[doc = "Bit 3 - USI Arbitration Lost"]
    #[inline(always)]
    pub fn usial(&mut self) -> USIAL_W {
        USIAL_W { w: self }
    }
    #[doc = "Bit 4 - USI Counter Interrupt enable"]
    #[inline(always)]
    pub fn usiie(&mut self) -> USIIE_W {
        USIIE_W { w: self }
    }
    #[doc = "Bit 5 - USI START Condition interrupt enable"]
    #[inline(always)]
    pub fn usisttie(&mut self) -> USISTTIE_W {
        USISTTIE_W { w: self }
    }
    #[doc = "Bit 6 - USI I2C Mode"]
    #[inline(always)]
    pub fn usii2c(&mut self) -> USII2C_W {
        USII2C_W { w: self }
    }
    #[doc = "Bit 7 - USI Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn usickph(&mut self) -> USICKPH_W {
        USICKPH_W { w: self }
    }
}
