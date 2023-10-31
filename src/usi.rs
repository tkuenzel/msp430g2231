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
#[doc = "USICTL0 (rw) register accessor: USI Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usictl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usictl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usictl0`]
module"]
pub type USICTL0 = crate::Reg<usictl0::USICTL0_SPEC>;
#[doc = "USI Control Register 0"]
pub mod usictl0;
#[doc = "USICTL1 (rw) register accessor: USI Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usictl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usictl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usictl1`]
module"]
pub type USICTL1 = crate::Reg<usictl1::USICTL1_SPEC>;
#[doc = "USI Control Register 1"]
pub mod usictl1;
#[doc = "USICKCTL (rw) register accessor: USI Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usickctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usickctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usickctl`]
module"]
pub type USICKCTL = crate::Reg<usickctl::USICKCTL_SPEC>;
#[doc = "USI Clock Control Register"]
pub mod usickctl;
#[doc = "USICNT (rw) register accessor: USI Bit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usicnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usicnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usicnt`]
module"]
pub type USICNT = crate::Reg<usicnt::USICNT_SPEC>;
#[doc = "USI Bit Counter Register"]
pub mod usicnt;
#[doc = "USISRL (rw) register accessor: USI Low Byte Shift Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usisrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usisrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usisrl`]
module"]
pub type USISRL = crate::Reg<usisrl::USISRL_SPEC>;
#[doc = "USI Low Byte Shift Register"]
pub mod usisrl;
#[doc = "USISRH (rw) register accessor: USI High Byte Shift Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usisrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usisrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usisrh`]
module"]
pub type USISRH = crate::Reg<usisrh::USISRH_SPEC>;
#[doc = "USI High Byte Shift Register"]
pub mod usisrh;
