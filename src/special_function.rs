#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub ie1: IE1,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Interrupt Flag 1"]
    pub ifg1: IFG1,
}
#[doc = "IE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie1`]
module"]
pub type IE1 = crate::Reg<ie1::IE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg1`]
module"]
pub type IFG1 = crate::Reg<ifg1::IFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
