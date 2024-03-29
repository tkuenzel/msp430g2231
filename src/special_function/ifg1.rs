#[doc = "Register `IFG1` reader"]
pub type R = crate::R<IFG1_SPEC>;
#[doc = "Register `IFG1` writer"]
pub type W = crate::W<IFG1_SPEC>;
#[doc = "Field `WDTIFG` reader - Watchdog Interrupt Flag"]
pub type WDTIFG_R = crate::BitReader;
#[doc = "Field `WDTIFG` writer - Watchdog Interrupt Flag"]
pub type WDTIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OFIFG` reader - Osc. Fault Interrupt Flag"]
pub type OFIFG_R = crate::BitReader;
#[doc = "Field `OFIFG` writer - Osc. Fault Interrupt Flag"]
pub type OFIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORIFG` reader - Power On Interrupt Flag"]
pub type PORIFG_R = crate::BitReader;
#[doc = "Field `PORIFG` writer - Power On Interrupt Flag"]
pub type PORIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIFG` reader - Reset Interrupt Flag"]
pub type RSTIFG_R = crate::BitReader;
#[doc = "Field `RSTIFG` writer - Reset Interrupt Flag"]
pub type RSTIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NMIIFG_R = crate::BitReader;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NMIIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&self) -> PORIFG_R {
        PORIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&self) -> RSTIFG_R {
        RSTIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdtifg(&mut self) -> WDTIFG_W<IFG1_SPEC, 0> {
        WDTIFG_W::new(self)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ofifg(&mut self) -> OFIFG_W<IFG1_SPEC, 1> {
        OFIFG_W::new(self)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn porifg(&mut self) -> PORIFG_W<IFG1_SPEC, 2> {
        PORIFG_W::new(self)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstifg(&mut self) -> RSTIFG_W<IFG1_SPEC, 3> {
        RSTIFG_W::new(self)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nmiifg(&mut self) -> NMIIFG_W<IFG1_SPEC, 4> {
        NMIIFG_W::new(self)
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
#[doc = "Interrupt Flag 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFG1_SPEC;
impl crate::RegisterSpec for IFG1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifg1::R`](R) reader structure"]
impl crate::Readable for IFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifg1::W`](W) writer structure"]
impl crate::Writable for IFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFG1 to value 0"]
impl crate::Resettable for IFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
