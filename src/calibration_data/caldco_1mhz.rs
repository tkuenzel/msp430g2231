#[doc = "Register `CALDCO_1MHZ` reader"]
pub type R = crate::R<CALDCO_1MHZ_SPEC>;
#[doc = "Register `CALDCO_1MHZ` writer"]
pub type W = crate::W<CALDCO_1MHZ_SPEC>;
#[doc = "Field `CALDCO_1MHZ` reader - DCOCTL Calibration Data register"]
pub type CALDCO_1MHZ_R = crate::FieldReader;
#[doc = "Field `CALDCO_1MHZ` writer - DCOCTL Calibration Data register"]
pub type CALDCO_1MHZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data register"]
    #[inline(always)]
    pub fn caldco_1mhz(&self) -> CALDCO_1MHZ_R {
        CALDCO_1MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data register"]
    #[inline(always)]
    #[must_use]
    pub fn caldco_1mhz(&mut self) -> CALDCO_1MHZ_W<CALDCO_1MHZ_SPEC, 0> {
        CALDCO_1MHZ_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caldco_1mhz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caldco_1mhz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALDCO_1MHZ_SPEC;
impl crate::RegisterSpec for CALDCO_1MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_1mhz::R`](R) reader structure"]
impl crate::Readable for CALDCO_1MHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`caldco_1mhz::W`](W) writer structure"]
impl crate::Writable for CALDCO_1MHZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALDCO_1MHZ to value 0"]
impl crate::Resettable for CALDCO_1MHZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
