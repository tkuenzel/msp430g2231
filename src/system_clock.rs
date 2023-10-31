#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Basic Clock System Control 3"]
    pub bcsctl3: BCSCTL3,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - DCO Clock Frequency Control"]
    pub dcoctl: DCOCTL,
    #[doc = "0x05 - Basic Clock System Control 1"]
    pub bcsctl1: BCSCTL1,
    #[doc = "0x06 - Basic Clock System Control 2"]
    pub bcsctl2: BCSCTL2,
}
#[doc = "BCSCTL3 (rw) register accessor: Basic Clock System Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsctl3`]
module"]
pub type BCSCTL3 = crate::Reg<bcsctl3::BCSCTL3_SPEC>;
#[doc = "Basic Clock System Control 3"]
pub mod bcsctl3;
#[doc = "DCOCTL (rw) register accessor: DCO Clock Frequency Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcoctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcoctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoctl`]
module"]
pub type DCOCTL = crate::Reg<dcoctl::DCOCTL_SPEC>;
#[doc = "DCO Clock Frequency Control"]
pub mod dcoctl;
#[doc = "BCSCTL1 (rw) register accessor: Basic Clock System Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsctl1`]
module"]
pub type BCSCTL1 = crate::Reg<bcsctl1::BCSCTL1_SPEC>;
#[doc = "Basic Clock System Control 1"]
pub mod bcsctl1;
#[doc = "BCSCTL2 (rw) register accessor: Basic Clock System Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcsctl2`]
module"]
pub type BCSCTL2 = crate::Reg<bcsctl2::BCSCTL2_SPEC>;
#[doc = "Basic Clock System Control 2"]
pub mod bcsctl2;
