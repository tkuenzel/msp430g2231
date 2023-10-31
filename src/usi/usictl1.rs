#[doc = "Register `USICTL1` reader"]
pub type R = crate::R<USICTL1_SPEC>;
#[doc = "Register `USICTL1` writer"]
pub type W = crate::W<USICTL1_SPEC>;
#[doc = "Field `USIIFG` reader - USI Counter Interrupt Flag"]
pub type USIIFG_R = crate::BitReader;
#[doc = "Field `USIIFG` writer - USI Counter Interrupt Flag"]
pub type USIIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USISTTIFG` reader - USI START Condition interrupt Flag"]
pub type USISTTIFG_R = crate::BitReader;
#[doc = "Field `USISTTIFG` writer - USI START Condition interrupt Flag"]
pub type USISTTIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USISTP` reader - USI STOP Condition received"]
pub type USISTP_R = crate::BitReader;
#[doc = "Field `USISTP` writer - USI STOP Condition received"]
pub type USISTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USIAL` reader - USI Arbitration Lost"]
pub type USIAL_R = crate::BitReader;
#[doc = "Field `USIAL` writer - USI Arbitration Lost"]
pub type USIAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USIIE` reader - USI Counter Interrupt enable"]
pub type USIIE_R = crate::BitReader;
#[doc = "Field `USIIE` writer - USI Counter Interrupt enable"]
pub type USIIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USISTTIE` reader - USI START Condition interrupt enable"]
pub type USISTTIE_R = crate::BitReader;
#[doc = "Field `USISTTIE` writer - USI START Condition interrupt enable"]
pub type USISTTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USII2C` reader - USI I2C Mode"]
pub type USII2C_R = crate::BitReader;
#[doc = "Field `USII2C` writer - USI I2C Mode"]
pub type USII2C_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USICKPH` reader - USI Sync. Mode: Clock Phase"]
pub type USICKPH_R = crate::BitReader;
#[doc = "Field `USICKPH` writer - USI Sync. Mode: Clock Phase"]
pub type USICKPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USI Counter Interrupt Flag"]
    #[inline(always)]
    pub fn usiifg(&self) -> USIIFG_R {
        USIIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI START Condition interrupt Flag"]
    #[inline(always)]
    pub fn usisttifg(&self) -> USISTTIFG_R {
        USISTTIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI STOP Condition received"]
    #[inline(always)]
    pub fn usistp(&self) -> USISTP_R {
        USISTP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Arbitration Lost"]
    #[inline(always)]
    pub fn usial(&self) -> USIAL_R {
        USIAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI Counter Interrupt enable"]
    #[inline(always)]
    pub fn usiie(&self) -> USIIE_R {
        USIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI START Condition interrupt enable"]
    #[inline(always)]
    pub fn usisttie(&self) -> USISTTIE_R {
        USISTTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI I2C Mode"]
    #[inline(always)]
    pub fn usii2c(&self) -> USII2C_R {
        USII2C_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn usickph(&self) -> USICKPH_R {
        USICKPH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Counter Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usiifg(&mut self) -> USIIFG_W<USICTL1_SPEC, 0> {
        USIIFG_W::new(self)
    }
    #[doc = "Bit 1 - USI START Condition interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usisttifg(&mut self) -> USISTTIFG_W<USICTL1_SPEC, 1> {
        USISTTIFG_W::new(self)
    }
    #[doc = "Bit 2 - USI STOP Condition received"]
    #[inline(always)]
    #[must_use]
    pub fn usistp(&mut self) -> USISTP_W<USICTL1_SPEC, 2> {
        USISTP_W::new(self)
    }
    #[doc = "Bit 3 - USI Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn usial(&mut self) -> USIAL_W<USICTL1_SPEC, 3> {
        USIAL_W::new(self)
    }
    #[doc = "Bit 4 - USI Counter Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usiie(&mut self) -> USIIE_W<USICTL1_SPEC, 4> {
        USIIE_W::new(self)
    }
    #[doc = "Bit 5 - USI START Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usisttie(&mut self) -> USISTTIE_W<USICTL1_SPEC, 5> {
        USISTTIE_W::new(self)
    }
    #[doc = "Bit 6 - USI I2C Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usii2c(&mut self) -> USII2C_W<USICTL1_SPEC, 6> {
        USII2C_W::new(self)
    }
    #[doc = "Bit 7 - USI Sync. Mode: Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn usickph(&mut self) -> USICKPH_W<USICTL1_SPEC, 7> {
        USICKPH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USI Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usictl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usictl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USICTL1_SPEC;
impl crate::RegisterSpec for USICTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usictl1::R`](R) reader structure"]
impl crate::Readable for USICTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usictl1::W`](W) writer structure"]
impl crate::Writable for USICTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USICTL1 to value 0"]
impl crate::Resettable for USICTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
