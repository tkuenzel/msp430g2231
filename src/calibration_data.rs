#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 1MHz"]
    pub caldco_1mhz: CALDCO_1MHZ,
    #[doc = "0x01 - BCSCTL1 Calibration Data for 1MHz"]
    pub calbc1_1mhz: CALBC1_1MHZ,
}
#[doc = "CALDCO_1MHZ (rw) register accessor: DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caldco_1mhz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caldco_1mhz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_1mhz`]
module"]
pub type CALDCO_1MHZ = crate::Reg<caldco_1mhz::CALDCO_1MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "CALBC1_1MHZ (rw) register accessor: BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calbc1_1mhz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calbc1_1mhz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_1mhz`]
module"]
pub type CALBC1_1MHZ = crate::Reg<calbc1_1mhz::CALBC1_1MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
