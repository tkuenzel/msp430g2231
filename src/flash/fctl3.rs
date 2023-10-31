#[doc = "Register `FCTL3` reader"]
pub type R = crate::R<FCTL3_SPEC>;
#[doc = "Register `FCTL3` writer"]
pub type W = crate::W<FCTL3_SPEC>;
#[doc = "Field `BUSY` reader - Flash busy: 1"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Flash busy: 1"]
pub type BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEYV` reader - Flash Key violation flag"]
pub type KEYV_R = crate::BitReader;
#[doc = "Field `KEYV` writer - Flash Key violation flag"]
pub type KEYV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCVIFG` reader - Flash Access violation flag"]
pub type ACCVIFG_R = crate::BitReader;
#[doc = "Field `ACCVIFG` writer - Flash Access violation flag"]
pub type ACCVIFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAIT` reader - Wait flag for segment write"]
pub type WAIT_R = crate::BitReader;
#[doc = "Field `WAIT` writer - Wait flag for segment write"]
pub type WAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK` reader - Lock bit: 1 - Flash is locked (read only)"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock bit: 1 - Flash is locked (read only)"]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMEX` reader - Flash Emergency Exit"]
pub type EMEX_R = crate::BitReader;
#[doc = "Field `EMEX` writer - Flash Emergency Exit"]
pub type EMEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKA` reader - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LOCKA_R = crate::BitReader;
#[doc = "Field `LOCKA` writer - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LOCKA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAIL` reader - Last Program or Erase failed"]
pub type FAIL_R = crate::BitReader;
#[doc = "Field `FAIL` writer - Last Program or Erase failed"]
pub type FAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FWKEY` reader - FCTL3 Password"]
pub type FWKEY_R = crate::FieldReader<FWKEYR_A>;
#[doc = "FCTL3 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYR_A {
    #[doc = "150: Value always read from the FCTL3 Password register"]
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
    #[doc = "Value always read from the FCTL3 Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == FWKEYR_A::PASSWORD
    }
}
#[doc = "FCTL3 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYW_AW {
    #[doc = "165: Value which must be written to the FCTL3 Password register"]
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
#[doc = "Field `FWKEY` writer - FCTL3 Password"]
pub type FWKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, FWKEYW_AW>;
impl<'a, REG, const O: u8> FWKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FCTL3 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FWKEYW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    pub fn keyv(&self) -> KEYV_R {
        KEYV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    pub fn accvifg(&self) -> ACCVIFG_R {
        ACCVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    pub fn emex(&self) -> EMEX_R {
        EMEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Last Program or Erase failed"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - FCTL3 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FWKEY_R {
        FWKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<FCTL3_SPEC, 0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn keyv(&mut self) -> KEYV_W<FCTL3_SPEC, 1> {
        KEYV_W::new(self)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn accvifg(&mut self) -> ACCVIFG_W<FCTL3_SPEC, 2> {
        ACCVIFG_W::new(self)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<FCTL3_SPEC, 3> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<FCTL3_SPEC, 4> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    #[must_use]
    pub fn emex(&mut self) -> EMEX_W<FCTL3_SPEC, 5> {
        EMEX_W::new(self)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<FCTL3_SPEC, 6> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 7 - Last Program or Erase failed"]
    #[inline(always)]
    #[must_use]
    pub fn fail(&mut self) -> FAIL_W<FCTL3_SPEC, 7> {
        FAIL_W::new(self)
    }
    #[doc = "Bits 8:15 - FCTL3 Password"]
    #[inline(always)]
    #[must_use]
    pub fn fwkey(&mut self) -> FWKEY_W<FCTL3_SPEC, 8> {
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
#[doc = "FLASH Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL3_SPEC;
impl crate::RegisterSpec for FCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl3::R`](R) reader structure"]
impl crate::Readable for FCTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl3::W`](W) writer structure"]
impl crate::Writable for FCTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL3 to value 0"]
impl crate::Resettable for FCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
