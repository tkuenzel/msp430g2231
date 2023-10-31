#[doc = "Register `BCSCTL3` reader"]
pub type R = crate::R<BCSCTL3_SPEC>;
#[doc = "Register `BCSCTL3` writer"]
pub type W = crate::W<BCSCTL3_SPEC>;
#[doc = "Field `LFXT1OF` reader - Low/high Frequency Oscillator Fault Flag"]
pub type LFXT1OF_R = crate::BitReader;
#[doc = "Field `LFXT1OF` writer - Low/high Frequency Oscillator Fault Flag"]
pub type LFXT1OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XT2OF` reader - High frequency oscillator 2 fault flag"]
pub type XT2OF_R = crate::BitReader;
#[doc = "Field `XT2OF` writer - High frequency oscillator 2 fault flag"]
pub type XT2OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XCAP` reader - XIN/XOUT Cap 0"]
pub type XCAP_R = crate::FieldReader<XCAP_A>;
#[doc = "XIN/XOUT Cap 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XCAP_A {
    #[doc = "0: XIN/XOUT Cap : 0 pF"]
    XCAP_0 = 0,
    #[doc = "1: XIN/XOUT Cap : 6 pF"]
    XCAP_1 = 1,
    #[doc = "2: XIN/XOUT Cap : 10 pF"]
    XCAP_2 = 2,
    #[doc = "3: XIN/XOUT Cap : 12.5 pF"]
    XCAP_3 = 3,
}
impl From<XCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: XCAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XCAP_A {
    type Ux = u8;
}
impl XCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XCAP_A {
        match self.bits {
            0 => XCAP_A::XCAP_0,
            1 => XCAP_A::XCAP_1,
            2 => XCAP_A::XCAP_2,
            3 => XCAP_A::XCAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "XIN/XOUT Cap : 0 pF"]
    #[inline(always)]
    pub fn is_xcap_0(&self) -> bool {
        *self == XCAP_A::XCAP_0
    }
    #[doc = "XIN/XOUT Cap : 6 pF"]
    #[inline(always)]
    pub fn is_xcap_1(&self) -> bool {
        *self == XCAP_A::XCAP_1
    }
    #[doc = "XIN/XOUT Cap : 10 pF"]
    #[inline(always)]
    pub fn is_xcap_2(&self) -> bool {
        *self == XCAP_A::XCAP_2
    }
    #[doc = "XIN/XOUT Cap : 12.5 pF"]
    #[inline(always)]
    pub fn is_xcap_3(&self) -> bool {
        *self == XCAP_A::XCAP_3
    }
}
#[doc = "Field `XCAP` writer - XIN/XOUT Cap 0"]
pub type XCAP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, XCAP_A>;
impl<'a, REG, const O: u8> XCAP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XIN/XOUT Cap : 0 pF"]
    #[inline(always)]
    pub fn xcap_0(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_0)
    }
    #[doc = "XIN/XOUT Cap : 6 pF"]
    #[inline(always)]
    pub fn xcap_1(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_1)
    }
    #[doc = "XIN/XOUT Cap : 10 pF"]
    #[inline(always)]
    pub fn xcap_2(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_2)
    }
    #[doc = "XIN/XOUT Cap : 12.5 pF"]
    #[inline(always)]
    pub fn xcap_3(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_3)
    }
}
#[doc = "Field `LFXT1S` reader - Mode 0 for LFXT1 (XTS = 0)"]
pub type LFXT1S_R = crate::FieldReader<LFXT1S_A>;
#[doc = "Mode 0 for LFXT1 (XTS = 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFXT1S_A {
    #[doc = "0: Mode 0 for LFXT1 : Normal operation"]
    LFXT1S_0 = 0,
    #[doc = "1: Mode 1 for LFXT1 : Reserved"]
    LFXT1S_1 = 1,
    #[doc = "2: Mode 2 for LFXT1 : VLO"]
    LFXT1S_2 = 2,
    #[doc = "3: Mode 3 for LFXT1 : Digital input signal"]
    LFXT1S_3 = 3,
}
impl From<LFXT1S_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXT1S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFXT1S_A {
    type Ux = u8;
}
impl LFXT1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFXT1S_A {
        match self.bits {
            0 => LFXT1S_A::LFXT1S_0,
            1 => LFXT1S_A::LFXT1S_1,
            2 => LFXT1S_A::LFXT1S_2,
            3 => LFXT1S_A::LFXT1S_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode 0 for LFXT1 : Normal operation"]
    #[inline(always)]
    pub fn is_lfxt1s_0(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_0
    }
    #[doc = "Mode 1 for LFXT1 : Reserved"]
    #[inline(always)]
    pub fn is_lfxt1s_1(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_1
    }
    #[doc = "Mode 2 for LFXT1 : VLO"]
    #[inline(always)]
    pub fn is_lfxt1s_2(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_2
    }
    #[doc = "Mode 3 for LFXT1 : Digital input signal"]
    #[inline(always)]
    pub fn is_lfxt1s_3(&self) -> bool {
        *self == LFXT1S_A::LFXT1S_3
    }
}
#[doc = "Field `LFXT1S` writer - Mode 0 for LFXT1 (XTS = 0)"]
pub type LFXT1S_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LFXT1S_A>;
impl<'a, REG, const O: u8> LFXT1S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0 for LFXT1 : Normal operation"]
    #[inline(always)]
    pub fn lfxt1s_0(self) -> &'a mut crate::W<REG> {
        self.variant(LFXT1S_A::LFXT1S_0)
    }
    #[doc = "Mode 1 for LFXT1 : Reserved"]
    #[inline(always)]
    pub fn lfxt1s_1(self) -> &'a mut crate::W<REG> {
        self.variant(LFXT1S_A::LFXT1S_1)
    }
    #[doc = "Mode 2 for LFXT1 : VLO"]
    #[inline(always)]
    pub fn lfxt1s_2(self) -> &'a mut crate::W<REG> {
        self.variant(LFXT1S_A::LFXT1S_2)
    }
    #[doc = "Mode 3 for LFXT1 : Digital input signal"]
    #[inline(always)]
    pub fn lfxt1s_3(self) -> &'a mut crate::W<REG> {
        self.variant(LFXT1S_A::LFXT1S_3)
    }
}
#[doc = "Field `XT2S` reader - Mode 0 for XT2"]
pub type XT2S_R = crate::FieldReader<XT2S_A>;
#[doc = "Mode 0 for XT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XT2S_A {
    #[doc = "0: Mode 0 for XT2 : 0.4 - 1 MHz"]
    XT2S_0 = 0,
    #[doc = "1: Mode 1 for XT2 : 1 - 4 MHz"]
    XT2S_1 = 1,
    #[doc = "2: Mode 2 for XT2 : 2 - 16 MHz"]
    XT2S_2 = 2,
    #[doc = "3: Mode 3 for XT2 : Digital input signal"]
    XT2S_3 = 3,
}
impl From<XT2S_A> for u8 {
    #[inline(always)]
    fn from(variant: XT2S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XT2S_A {
    type Ux = u8;
}
impl XT2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XT2S_A {
        match self.bits {
            0 => XT2S_A::XT2S_0,
            1 => XT2S_A::XT2S_1,
            2 => XT2S_A::XT2S_2,
            3 => XT2S_A::XT2S_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
    #[inline(always)]
    pub fn is_xt2s_0(&self) -> bool {
        *self == XT2S_A::XT2S_0
    }
    #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
    #[inline(always)]
    pub fn is_xt2s_1(&self) -> bool {
        *self == XT2S_A::XT2S_1
    }
    #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
    #[inline(always)]
    pub fn is_xt2s_2(&self) -> bool {
        *self == XT2S_A::XT2S_2
    }
    #[doc = "Mode 3 for XT2 : Digital input signal"]
    #[inline(always)]
    pub fn is_xt2s_3(&self) -> bool {
        *self == XT2S_A::XT2S_3
    }
}
#[doc = "Field `XT2S` writer - Mode 0 for XT2"]
pub type XT2S_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, XT2S_A>;
impl<'a, REG, const O: u8> XT2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
    #[inline(always)]
    pub fn xt2s_0(self) -> &'a mut crate::W<REG> {
        self.variant(XT2S_A::XT2S_0)
    }
    #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
    #[inline(always)]
    pub fn xt2s_1(self) -> &'a mut crate::W<REG> {
        self.variant(XT2S_A::XT2S_1)
    }
    #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
    #[inline(always)]
    pub fn xt2s_2(self) -> &'a mut crate::W<REG> {
        self.variant(XT2S_A::XT2S_2)
    }
    #[doc = "Mode 3 for XT2 : Digital input signal"]
    #[inline(always)]
    pub fn xt2s_3(self) -> &'a mut crate::W<REG> {
        self.variant(XT2S_A::XT2S_3)
    }
}
impl R {
    #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxt1of(&self) -> LFXT1OF_R {
        LFXT1OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
    #[inline(always)]
    pub fn xt2of(&self) -> XT2OF_R {
        XT2OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap(&self) -> XCAP_R {
        XCAP_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
    #[inline(always)]
    pub fn lfxt1s(&self) -> LFXT1S_R {
        LFXT1S_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Mode 0 for XT2"]
    #[inline(always)]
    pub fn xt2s(&self) -> XT2S_R {
        XT2S_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxt1of(&mut self) -> LFXT1OF_W<BCSCTL3_SPEC, 0> {
        LFXT1OF_W::new(self)
    }
    #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
    #[inline(always)]
    #[must_use]
    pub fn xt2of(&mut self) -> XT2OF_W<BCSCTL3_SPEC, 1> {
        XT2OF_W::new(self)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
    #[inline(always)]
    #[must_use]
    pub fn xcap(&mut self) -> XCAP_W<BCSCTL3_SPEC, 2> {
        XCAP_W::new(self)
    }
    #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn lfxt1s(&mut self) -> LFXT1S_W<BCSCTL3_SPEC, 4> {
        LFXT1S_W::new(self)
    }
    #[doc = "Bits 6:7 - Mode 0 for XT2"]
    #[inline(always)]
    #[must_use]
    pub fn xt2s(&mut self) -> XT2S_W<BCSCTL3_SPEC, 6> {
        XT2S_W::new(self)
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
#[doc = "Basic Clock System Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCSCTL3_SPEC;
impl crate::RegisterSpec for BCSCTL3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcsctl3::R`](R) reader structure"]
impl crate::Readable for BCSCTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcsctl3::W`](W) writer structure"]
impl crate::Writable for BCSCTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCSCTL3 to value 0"]
impl crate::Resettable for BCSCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
