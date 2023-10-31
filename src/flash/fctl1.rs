#[doc = "Register `FCTL1` reader"]
pub type R = crate::R<FCTL1_SPEC>;
#[doc = "Register `FCTL1` writer"]
pub type W = crate::W<FCTL1_SPEC>;
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type ERASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MERAS_R = crate::BitReader;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MERAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WRT_R = crate::BitReader;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BLKWRT_R = crate::BitReader;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BLKWRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FWKEY` reader - FCTL1 Password"]
pub type FWKEY_R = crate::FieldReader<FWKEYR_A>;
#[doc = "FCTL1 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYR_A {
    #[doc = "150: Value always read from the FCTL1 Password register"]
    PASSWORD = 150,
}
impl From<FWKEYR_A> for u8 {
    #[inline(always)]
    fn from(variant: FWKEYR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FWKEYR_A {
    type Ux = u8;
}
impl FWKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FWKEYR_A> {
        match self.bits {
            150 => Some(FWKEYR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Value always read from the FCTL1 Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == FWKEYR_A::PASSWORD
    }
}
#[doc = "FCTL1 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYW_AW {
    #[doc = "165: Value which must be written to the FCTL1 Password register"]
    PASSWORD = 165,
}
impl From<FWKEYW_AW> for u8 {
    #[inline(always)]
    fn from(variant: FWKEYW_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FWKEYW_AW {
    type Ux = u8;
}
#[doc = "Field `FWKEY` writer - FCTL1 Password"]
pub type FWKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, FWKEYW_AW>;
impl<'a, REG, const O: u8> FWKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FCTL1 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FWKEYW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MERAS_R {
        MERAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WRT_R {
        WRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BLKWRT_R {
        BLKWRT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - FCTL1 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FWKEY_R {
        FWKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<FCTL1_SPEC, 1> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn meras(&mut self) -> MERAS_W<FCTL1_SPEC, 2> {
        MERAS_W::new(self)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    #[must_use]
    pub fn wrt(&mut self) -> WRT_W<FCTL1_SPEC, 6> {
        WRT_W::new(self)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    #[must_use]
    pub fn blkwrt(&mut self) -> BLKWRT_W<FCTL1_SPEC, 7> {
        BLKWRT_W::new(self)
    }
    #[doc = "Bits 8:15 - FCTL1 Password"]
    #[inline(always)]
    #[must_use]
    pub fn fwkey(&mut self) -> FWKEY_W<FCTL1_SPEC, 8> {
        FWKEY_W::new(self)
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
#[doc = "FLASH Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL1_SPEC;
impl crate::RegisterSpec for FCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl1::R`](R) reader structure"]
impl crate::Readable for FCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl1::W`](W) writer structure"]
impl crate::Writable for FCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for FCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
