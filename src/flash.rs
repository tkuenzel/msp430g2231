#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    pub fctl1: FCTL1,
    #[doc = "0x02 - FLASH Control 2"]
    pub fctl2: FCTL2,
    #[doc = "0x04 - FLASH Control 3"]
    pub fctl3: FCTL3,
}
#[doc = "FCTL1 (rw) register accessor: FLASH Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl1`]
module"]
pub type FCTL1 = crate::Reg<fctl1::FCTL1_SPEC>;
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FCTL2 (rw) register accessor: FLASH Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl2`]
module"]
pub type FCTL2 = crate::Reg<fctl2::FCTL2_SPEC>;
#[doc = "FLASH Control 2"]
pub mod fctl2;
#[doc = "FCTL3 (rw) register accessor: FLASH Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl3`]
module"]
pub type FCTL3 = crate::Reg<fctl3::FCTL3_SPEC>;
#[doc = "FLASH Control 3"]
pub mod fctl3;
