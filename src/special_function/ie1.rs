#[doc = "Register `IE1` reader"]
pub type R = crate::R<IE1_SPEC>;
#[doc = "Register `IE1` writer"]
pub type W = crate::W<IE1_SPEC>;
#[doc = "Field `WDTIE` reader - Watchdog Interrupt Enable"]
pub type WDTIE_R = crate::BitReader;
#[doc = "Field `WDTIE` writer - Watchdog Interrupt Enable"]
pub type WDTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OFIE` reader - Osc. Fault Interrupt Enable"]
pub type OFIE_R = crate::BitReader;
#[doc = "Field `OFIE` writer - Osc. Fault Interrupt Enable"]
pub type OFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub type NMIIE_R = crate::BitReader;
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub type NMIIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_R = crate::BitReader;
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtie(&mut self) -> WDTIE_W<IE1_SPEC, 0> {
        WDTIE_W::new(self)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ofie(&mut self) -> OFIE_W<IE1_SPEC, 1> {
        OFIE_W::new(self)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmiie(&mut self) -> NMIIE_W<IE1_SPEC, 4> {
        NMIIE_W::new(self)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn accvie(&mut self) -> ACCVIE_W<IE1_SPEC, 5> {
        ACCVIE_W::new(self)
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
#[doc = "Interrupt Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE1_SPEC;
impl crate::RegisterSpec for IE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ie1::R`](R) reader structure"]
impl crate::Readable for IE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie1::W`](W) writer structure"]
impl crate::Writable for IE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE1 to value 0"]
impl crate::Resettable for IE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
