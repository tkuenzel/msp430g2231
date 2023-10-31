#[doc = "Register `ADC10DTC0` reader"]
pub type R = crate::R<ADC10DTC0_SPEC>;
#[doc = "Register `ADC10DTC0` writer"]
pub type W = crate::W<ADC10DTC0_SPEC>;
#[doc = "Field `ADC10FETCH` reader - This bit should normally be reset"]
pub type ADC10FETCH_R = crate::BitReader;
#[doc = "Field `ADC10FETCH` writer - This bit should normally be reset"]
pub type ADC10FETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10B1` reader - ADC10 block one"]
pub type ADC10B1_R = crate::BitReader;
#[doc = "Field `ADC10B1` writer - ADC10 block one"]
pub type ADC10B1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10CT` reader - ADC10 continuous transfer"]
pub type ADC10CT_R = crate::BitReader;
#[doc = "Field `ADC10CT` writer - ADC10 continuous transfer"]
pub type ADC10CT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10TB` reader - ADC10 two-block mode"]
pub type ADC10TB_R = crate::BitReader;
#[doc = "Field `ADC10TB` writer - ADC10 two-block mode"]
pub type ADC10TB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&self) -> ADC10FETCH_R {
        ADC10FETCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&self) -> ADC10B1_R {
        ADC10B1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&self) -> ADC10CT_R {
        ADC10CT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&self) -> ADC10TB_R {
        ADC10TB_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc10fetch(&mut self) -> ADC10FETCH_W<ADC10DTC0_SPEC, 0> {
        ADC10FETCH_W::new(self)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    #[must_use]
    pub fn adc10b1(&mut self) -> ADC10B1_W<ADC10DTC0_SPEC, 1> {
        ADC10B1_W::new(self)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    #[must_use]
    pub fn adc10ct(&mut self) -> ADC10CT_W<ADC10DTC0_SPEC, 2> {
        ADC10CT_W::new(self)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc10tb(&mut self) -> ADC10TB_W<ADC10DTC0_SPEC, 3> {
        ADC10TB_W::new(self)
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
#[doc = "ADC10 Data Transfer Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10dtc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10dtc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC10DTC0_SPEC;
impl crate::RegisterSpec for ADC10DTC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10dtc0::R`](R) reader structure"]
impl crate::Readable for ADC10DTC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc10dtc0::W`](W) writer structure"]
impl crate::Writable for ADC10DTC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC10DTC0 to value 0"]
impl crate::Resettable for ADC10DTC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
