#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USI Control Register 0"]
    pub usictl0: USICTL0,
    #[doc = "0x01 - USI Control Register 1"]
    pub usictl1: USICTL1,
    #[doc = "0x02 - USI Clock Control Register"]
    pub usickctl: USICKCTL,
    #[doc = "0x03 - USI Bit Counter Register"]
    pub usicnt: USICNT,
    #[doc = "0x04 - USI Low Byte Shift Register"]
    pub usisrl: USISRL,
    #[doc = "0x05 - USI High Byte Shift Register"]
    pub usisrh: USISRH,
}
#[doc = "USI Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usictl0](usictl0) module"]
pub type USICTL0 = crate::Reg<u8, _USICTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USICTL0;
#[doc = "`read()` method returns [usictl0::R](usictl0::R) reader structure"]
impl crate::Readable for USICTL0 {}
#[doc = "`write(|w| ..)` method takes [usictl0::W](usictl0::W) writer structure"]
impl crate::Writable for USICTL0 {}
#[doc = "USI Control Register 0"]
pub mod usictl0;
#[doc = "USI Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usictl1](usictl1) module"]
pub type USICTL1 = crate::Reg<u8, _USICTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USICTL1;
#[doc = "`read()` method returns [usictl1::R](usictl1::R) reader structure"]
impl crate::Readable for USICTL1 {}
#[doc = "`write(|w| ..)` method takes [usictl1::W](usictl1::W) writer structure"]
impl crate::Writable for USICTL1 {}
#[doc = "USI Control Register 1"]
pub mod usictl1;
#[doc = "USI Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usickctl](usickctl) module"]
pub type USICKCTL = crate::Reg<u8, _USICKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USICKCTL;
#[doc = "`read()` method returns [usickctl::R](usickctl::R) reader structure"]
impl crate::Readable for USICKCTL {}
#[doc = "`write(|w| ..)` method takes [usickctl::W](usickctl::W) writer structure"]
impl crate::Writable for USICKCTL {}
#[doc = "USI Clock Control Register"]
pub mod usickctl;
#[doc = "USI Bit Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usicnt](usicnt) module"]
pub type USICNT = crate::Reg<u8, _USICNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USICNT;
#[doc = "`read()` method returns [usicnt::R](usicnt::R) reader structure"]
impl crate::Readable for USICNT {}
#[doc = "`write(|w| ..)` method takes [usicnt::W](usicnt::W) writer structure"]
impl crate::Writable for USICNT {}
#[doc = "USI Bit Counter Register"]
pub mod usicnt;
#[doc = "USI Low Byte Shift Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usisrl](usisrl) module"]
pub type USISRL = crate::Reg<u8, _USISRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USISRL;
#[doc = "`read()` method returns [usisrl::R](usisrl::R) reader structure"]
impl crate::Readable for USISRL {}
#[doc = "`write(|w| ..)` method takes [usisrl::W](usisrl::W) writer structure"]
impl crate::Writable for USISRL {}
#[doc = "USI Low Byte Shift Register"]
pub mod usisrl;
#[doc = "USI High Byte Shift Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usisrh](usisrh) module"]
pub type USISRH = crate::Reg<u8, _USISRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USISRH;
#[doc = "`read()` method returns [usisrh::R](usisrh::R) reader structure"]
impl crate::Readable for USISRH {}
#[doc = "`write(|w| ..)` method takes [usisrh::W](usisrh::W) writer structure"]
impl crate::Writable for USISRH {}
#[doc = "USI High Byte Shift Register"]
pub mod usisrh;
