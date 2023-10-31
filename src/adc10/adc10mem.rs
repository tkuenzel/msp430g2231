#[doc = "Register `ADC10MEM` reader"]
pub type R = crate::R<ADC10MEM_SPEC>;
#[doc = "Register `ADC10MEM` writer"]
pub type W = crate::W<ADC10MEM_SPEC>;
#[doc = "Field `ADC10MEM` reader - ADC10 Memory register"]
pub type ADC10MEM_R = crate::FieldReader<u16>;
#[doc = "Field `ADC10MEM` writer - ADC10 Memory register"]
pub type ADC10MEM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&self) -> ADC10MEM_R {
        ADC10MEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    #[must_use]
    pub fn adc10mem(&mut self) -> ADC10MEM_W<ADC10MEM_SPEC, 0> {
        ADC10MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC10 Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC10MEM_SPEC;
impl crate::RegisterSpec for ADC10MEM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10mem::R`](R) reader structure"]
impl crate::Readable for ADC10MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc10mem::W`](W) writer structure"]
impl crate::Writable for ADC10MEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC10MEM to value 0"]
impl crate::Resettable for ADC10MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
