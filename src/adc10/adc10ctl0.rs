#[doc = "Register `ADC10CTL0` reader"]
pub type R = crate::R<ADC10CTL0_SPEC>;
#[doc = "Register `ADC10CTL0` writer"]
pub type W = crate::W<ADC10CTL0_SPEC>;
#[doc = "Field `ADC10SC` reader - ADC10 Start Conversion"]
pub type ADC10SC_R = crate::BitReader;
#[doc = "Field `ADC10SC` writer - ADC10 Start Conversion"]
pub type ADC10SC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENC` reader - ADC10 Enable Conversion"]
pub type ENC_R = crate::BitReader;
#[doc = "Field `ENC` writer - ADC10 Enable Conversion"]
pub type ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10IFG` reader - ADC10 Interrupt Flag"]
pub type ADC10IFG_R = crate::BitReader;
#[doc = "Field `ADC10IFG` writer - ADC10 Interrupt Flag"]
pub type ADC10IFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10IE` reader - ADC10 Interrupt Enalbe"]
pub type ADC10IE_R = crate::BitReader;
#[doc = "Field `ADC10IE` writer - ADC10 Interrupt Enalbe"]
pub type ADC10IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10ON` reader - ADC10 On/Enable"]
pub type ADC10ON_R = crate::BitReader;
#[doc = "Field `ADC10ON` writer - ADC10 On/Enable"]
pub type ADC10ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFON` reader - ADC10 Reference on"]
pub type REFON_R = crate::BitReader;
#[doc = "Field `REFON` writer - ADC10 Reference on"]
pub type REFON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REF2_5V` reader - ADC10 Ref 0:1.5V / 1:2.5V"]
pub type REF2_5V_R = crate::BitReader;
#[doc = "Field `REF2_5V` writer - ADC10 Ref 0:1.5V / 1:2.5V"]
pub type REF2_5V_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSC` reader - ADC10 Multiple SampleConversion"]
pub type MSC_R = crate::BitReader;
#[doc = "Field `MSC` writer - ADC10 Multiple SampleConversion"]
pub type MSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFBURST` reader - ADC10 Reference Burst Mode"]
pub type REFBURST_R = crate::BitReader;
#[doc = "Field `REFBURST` writer - ADC10 Reference Burst Mode"]
pub type REFBURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFOUT` reader - ADC10 Enalbe output of Ref."]
pub type REFOUT_R = crate::BitReader;
#[doc = "Field `REFOUT` writer - ADC10 Enalbe output of Ref."]
pub type REFOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10SR` reader - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
pub type ADC10SR_R = crate::BitReader;
#[doc = "Field `ADC10SR` writer - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
pub type ADC10SR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC10SHT` reader - ADC10 Sample Hold Select Bit: 0"]
pub type ADC10SHT_R = crate::FieldReader<ADC10SHT_A>;
#[doc = "ADC10 Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC10SHT_A {
    #[doc = "0: 4 x ADC10CLKs"]
    ADC10SHT_0 = 0,
    #[doc = "1: 8 x ADC10CLKs"]
    ADC10SHT_1 = 1,
    #[doc = "2: 16 x ADC10CLKs"]
    ADC10SHT_2 = 2,
    #[doc = "3: 64 x ADC10CLKs"]
    ADC10SHT_3 = 3,
}
impl From<ADC10SHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SHT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC10SHT_A {
    type Ux = u8;
}
impl ADC10SHT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC10SHT_A {
        match self.bits {
            0 => ADC10SHT_A::ADC10SHT_0,
            1 => ADC10SHT_A::ADC10SHT_1,
            2 => ADC10SHT_A::ADC10SHT_2,
            3 => ADC10SHT_A::ADC10SHT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "4 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_0(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_0
    }
    #[doc = "8 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_1(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_1
    }
    #[doc = "16 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_2(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_2
    }
    #[doc = "64 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_3(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_3
    }
}
#[doc = "Field `ADC10SHT` writer - ADC10 Sample Hold Select Bit: 0"]
pub type ADC10SHT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ADC10SHT_A>;
impl<'a, REG, const O: u8> ADC10SHT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC10SHT_A::ADC10SHT_0)
    }
    #[doc = "8 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC10SHT_A::ADC10SHT_1)
    }
    #[doc = "16 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC10SHT_A::ADC10SHT_2)
    }
    #[doc = "64 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC10SHT_A::ADC10SHT_3)
    }
}
#[doc = "Field `SREF` reader - ADC10 Reference Select Bit: 0"]
pub type SREF_R = crate::FieldReader<SREF_A>;
#[doc = "ADC10 Reference Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SREF_A {
    #[doc = "0: VR+ = AVCC and VR- = AVSS"]
    SREF_0 = 0,
    #[doc = "1: VR+ = VREF+ and VR- = AVSS"]
    SREF_1 = 1,
    #[doc = "2: VR+ = VEREF+ and VR- = AVSS"]
    SREF_2 = 2,
    #[doc = "3: VR+ = VEREF+ and VR- = AVSS"]
    SREF_3 = 3,
    #[doc = "4: VR+ = AVCC and VR- = VREF-/VEREF-"]
    SREF_4 = 4,
    #[doc = "5: VR+ = VREF+ and VR- = VREF-/VEREF-"]
    SREF_5 = 5,
    #[doc = "6: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    SREF_6 = 6,
    #[doc = "7: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    SREF_7 = 7,
}
impl From<SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SREF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SREF_A {
    type Ux = u8;
}
impl SREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SREF_A {
        match self.bits {
            0 => SREF_A::SREF_0,
            1 => SREF_A::SREF_1,
            2 => SREF_A::SREF_2,
            3 => SREF_A::SREF_3,
            4 => SREF_A::SREF_4,
            5 => SREF_A::SREF_5,
            6 => SREF_A::SREF_6,
            7 => SREF_A::SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_0(&self) -> bool {
        *self == SREF_A::SREF_0
    }
    #[doc = "VR+ = VREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_1(&self) -> bool {
        *self == SREF_A::SREF_1
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_2(&self) -> bool {
        *self == SREF_A::SREF_2
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_3(&self) -> bool {
        *self == SREF_A::SREF_3
    }
    #[doc = "VR+ = AVCC and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_4(&self) -> bool {
        *self == SREF_A::SREF_4
    }
    #[doc = "VR+ = VREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_5(&self) -> bool {
        *self == SREF_A::SREF_5
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_6(&self) -> bool {
        *self == SREF_A::SREF_6
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_7(&self) -> bool {
        *self == SREF_A::SREF_7
    }
}
#[doc = "Field `SREF` writer - ADC10 Reference Select Bit: 0"]
pub type SREF_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SREF_A>;
impl<'a, REG, const O: u8> SREF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_0(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_0)
    }
    #[doc = "VR+ = VREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_1(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_1)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_2(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_2)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_3(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_3)
    }
    #[doc = "VR+ = AVCC and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_4(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_4)
    }
    #[doc = "VR+ = VREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_5(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_5)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_6(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_6)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_7(self) -> &'a mut crate::W<REG> {
        self.variant(SREF_A::SREF_7)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&self) -> ADC10SC_R {
        ADC10SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&self) -> ADC10IFG_R {
        ADC10IFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&self) -> ADC10IE_R {
        ADC10IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&self) -> ADC10ON_R {
        ADC10ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&self) -> REF2_5V_R {
        REF2_5V_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&self) -> REFBURST_R {
        REFBURST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&self) -> ADC10SR_R {
        ADC10SR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&self) -> ADC10SHT_R {
        ADC10SHT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&self) -> SREF_R {
        SREF_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn adc10sc(&mut self) -> ADC10SC_W<ADC10CTL0_SPEC, 0> {
        ADC10SC_W::new(self)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<ADC10CTL0_SPEC, 1> {
        ENC_W::new(self)
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc10ifg(&mut self) -> ADC10IFG_W<ADC10CTL0_SPEC, 2> {
        ADC10IFG_W::new(self)
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    #[must_use]
    pub fn adc10ie(&mut self) -> ADC10IE_W<ADC10CTL0_SPEC, 3> {
        ADC10IE_W::new(self)
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc10on(&mut self) -> ADC10ON_W<ADC10CTL0_SPEC, 4> {
        ADC10ON_W::new(self)
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    #[must_use]
    pub fn refon(&mut self) -> REFON_W<ADC10CTL0_SPEC, 5> {
        REFON_W::new(self)
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    #[must_use]
    pub fn ref2_5v(&mut self) -> REF2_5V_W<ADC10CTL0_SPEC, 6> {
        REF2_5V_W::new(self)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<ADC10CTL0_SPEC, 7> {
        MSC_W::new(self)
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn refburst(&mut self) -> REFBURST_W<ADC10CTL0_SPEC, 8> {
        REFBURST_W::new(self)
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    #[must_use]
    pub fn refout(&mut self) -> REFOUT_W<ADC10CTL0_SPEC, 9> {
        REFOUT_W::new(self)
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    #[must_use]
    pub fn adc10sr(&mut self) -> ADC10SR_W<ADC10CTL0_SPEC, 10> {
        ADC10SR_W::new(self)
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc10sht(&mut self) -> ADC10SHT_W<ADC10CTL0_SPEC, 11> {
        ADC10SHT_W::new(self)
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn sref(&mut self) -> SREF_W<ADC10CTL0_SPEC, 13> {
        SREF_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC10 Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC10CTL0_SPEC;
impl crate::RegisterSpec for ADC10CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10ctl0::R`](R) reader structure"]
impl crate::Readable for ADC10CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc10ctl0::W`](W) writer structure"]
impl crate::Writable for ADC10CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC10CTL0 to value 0"]
impl crate::Resettable for ADC10CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
