#[doc = "Register `BCSCTL2` reader"]
pub type R = crate::R<BCSCTL2_SPEC>;
#[doc = "Register `BCSCTL2` writer"]
pub type W = crate::W<BCSCTL2_SPEC>;
#[doc = "Field `DIVS` reader - SMCLK Divider 0"]
pub type DIVS_R = crate::FieldReader<DIVS_A>;
#[doc = "SMCLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: SMCLK Divider 0: /1"]
    DIVS_0 = 0,
    #[doc = "1: SMCLK Divider 1: /2"]
    DIVS_1 = 1,
    #[doc = "2: SMCLK Divider 2: /4"]
    DIVS_2 = 2,
    #[doc = "3: SMCLK Divider 3: /8"]
    DIVS_3 = 3,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVS_A {
    type Ux = u8;
}
impl DIVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::DIVS_0,
            1 => DIVS_A::DIVS_1,
            2 => DIVS_A::DIVS_2,
            3 => DIVS_A::DIVS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "SMCLK Divider 0: /1"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == DIVS_A::DIVS_0
    }
    #[doc = "SMCLK Divider 1: /2"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == DIVS_A::DIVS_1
    }
    #[doc = "SMCLK Divider 2: /4"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == DIVS_A::DIVS_2
    }
    #[doc = "SMCLK Divider 3: /8"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == DIVS_A::DIVS_3
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider 0"]
pub type DIVS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DIVS_A>;
impl<'a, REG, const O: u8> DIVS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK Divider 0: /1"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_0)
    }
    #[doc = "SMCLK Divider 1: /2"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_1)
    }
    #[doc = "SMCLK Divider 2: /4"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_2)
    }
    #[doc = "SMCLK Divider 3: /8"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_3)
    }
}
#[doc = "Field `SELS` reader - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
pub type SELS_R = crate::BitReader;
#[doc = "Field `SELS` writer - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
pub type SELS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVM` reader - MCLK Divider 0"]
pub type DIVM_R = crate::FieldReader<DIVM_A>;
#[doc = "MCLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: MCLK Divider 0: /1"]
    DIVM_0 = 0,
    #[doc = "1: MCLK Divider 1: /2"]
    DIVM_1 = 1,
    #[doc = "2: MCLK Divider 2: /4"]
    DIVM_2 = 2,
    #[doc = "3: MCLK Divider 3: /8"]
    DIVM_3 = 3,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVM_A {
    type Ux = u8;
}
impl DIVM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::DIVM_0,
            1 => DIVM_A::DIVM_1,
            2 => DIVM_A::DIVM_2,
            3 => DIVM_A::DIVM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK Divider 0: /1"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == DIVM_A::DIVM_0
    }
    #[doc = "MCLK Divider 1: /2"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == DIVM_A::DIVM_1
    }
    #[doc = "MCLK Divider 2: /4"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == DIVM_A::DIVM_2
    }
    #[doc = "MCLK Divider 3: /8"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == DIVM_A::DIVM_3
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider 0"]
pub type DIVM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DIVM_A>;
impl<'a, REG, const O: u8> DIVM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Divider 0: /1"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_0)
    }
    #[doc = "MCLK Divider 1: /2"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_1)
    }
    #[doc = "MCLK Divider 2: /4"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_2)
    }
    #[doc = "MCLK Divider 3: /8"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_3)
    }
}
#[doc = "Field `SELM` reader - MCLK Source Select 0"]
pub type SELM_R = crate::FieldReader<SELM_A>;
#[doc = "MCLK Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELM_A {
    #[doc = "0: MCLK Source Select 0: DCOCLK"]
    SELM_0 = 0,
    #[doc = "1: MCLK Source Select 1: DCOCLK"]
    SELM_1 = 1,
    #[doc = "2: MCLK Source Select 2: XT2CLK/LFXTCLK"]
    SELM_2 = 2,
    #[doc = "3: MCLK Source Select 3: LFXTCLK"]
    SELM_3 = 3,
}
impl From<SELM_A> for u8 {
    #[inline(always)]
    fn from(variant: SELM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELM_A {
    type Ux = u8;
}
impl SELM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELM_A {
        match self.bits {
            0 => SELM_A::SELM_0,
            1 => SELM_A::SELM_1,
            2 => SELM_A::SELM_2,
            3 => SELM_A::SELM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK Source Select 0: DCOCLK"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        *self == SELM_A::SELM_0
    }
    #[doc = "MCLK Source Select 1: DCOCLK"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        *self == SELM_A::SELM_1
    }
    #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        *self == SELM_A::SELM_2
    }
    #[doc = "MCLK Source Select 3: LFXTCLK"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        *self == SELM_A::SELM_3
    }
}
#[doc = "Field `SELM` writer - MCLK Source Select 0"]
pub type SELM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SELM_A>;
impl<'a, REG, const O: u8> SELM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Source Select 0: DCOCLK"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_0)
    }
    #[doc = "MCLK Source Select 1: DCOCLK"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_1)
    }
    #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_2)
    }
    #[doc = "MCLK Source Select 3: LFXTCLK"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_3)
    }
}
impl R {
    #[doc = "Bits 1:2 - SMCLK Divider 0"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MCLK Divider 0"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 1:2 - SMCLK Divider 0"]
    #[inline(always)]
    #[must_use]
    pub fn divs(&mut self) -> DIVS_W<BCSCTL2_SPEC, 1> {
        DIVS_W::new(self)
    }
    #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sels(&mut self) -> SELS_W<BCSCTL2_SPEC, 3> {
        SELS_W::new(self)
    }
    #[doc = "Bits 4:5 - MCLK Divider 0"]
    #[inline(always)]
    #[must_use]
    pub fn divm(&mut self) -> DIVM_W<BCSCTL2_SPEC, 4> {
        DIVM_W::new(self)
    }
    #[doc = "Bits 6:7 - MCLK Source Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn selm(&mut self) -> SELM_W<BCSCTL2_SPEC, 6> {
        SELM_W::new(self)
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
#[doc = "Basic Clock System Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCSCTL2_SPEC;
impl crate::RegisterSpec for BCSCTL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcsctl2::R`](R) reader structure"]
impl crate::Readable for BCSCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcsctl2::W`](W) writer structure"]
impl crate::Writable for BCSCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCSCTL2 to value 0"]
impl crate::Resettable for BCSCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
