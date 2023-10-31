#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC10 Data Transfer Control 0"]
    pub adc10dtc0: ADC10DTC0,
    #[doc = "0x01 - ADC10 Data Transfer Control 1"]
    pub adc10dtc1: ADC10DTC1,
    #[doc = "0x02 - ADC10 Analog Enable 0"]
    pub adc10ae0: ADC10AE0,
    _reserved3: [u8; 0x0165],
    #[doc = "0x168 - ADC10 Control 0"]
    pub adc10ctl0: ADC10CTL0,
    #[doc = "0x16a - ADC10 Control 1"]
    pub adc10ctl1: ADC10CTL1,
    #[doc = "0x16c - ADC10 Memory"]
    pub adc10mem: ADC10MEM,
    _reserved6: [u8; 0x06],
    #[doc = "0x174 - ADC10 Data Transfer Start Address"]
    pub adc10sa: ADC10SA,
}
#[doc = "ADC10DTC0 (rw) register accessor: ADC10 Data Transfer Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10dtc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10dtc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10dtc0`]
module"]
pub type ADC10DTC0 = crate::Reg<adc10dtc0::ADC10DTC0_SPEC>;
#[doc = "ADC10 Data Transfer Control 0"]
pub mod adc10dtc0;
#[doc = "ADC10DTC1 (rw) register accessor: ADC10 Data Transfer Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10dtc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10dtc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10dtc1`]
module"]
pub type ADC10DTC1 = crate::Reg<adc10dtc1::ADC10DTC1_SPEC>;
#[doc = "ADC10 Data Transfer Control 1"]
pub mod adc10dtc1;
#[doc = "ADC10AE0 (rw) register accessor: ADC10 Analog Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10ae0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10ae0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ae0`]
module"]
pub type ADC10AE0 = crate::Reg<adc10ae0::ADC10AE0_SPEC>;
#[doc = "ADC10 Analog Enable 0"]
pub mod adc10ae0;
#[doc = "ADC10CTL0 (rw) register accessor: ADC10 Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ctl0`]
module"]
pub type ADC10CTL0 = crate::Reg<adc10ctl0::ADC10CTL0_SPEC>;
#[doc = "ADC10 Control 0"]
pub mod adc10ctl0;
#[doc = "ADC10CTL1 (rw) register accessor: ADC10 Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ctl1`]
module"]
pub type ADC10CTL1 = crate::Reg<adc10ctl1::ADC10CTL1_SPEC>;
#[doc = "ADC10 Control 1"]
pub mod adc10ctl1;
#[doc = "ADC10MEM (rw) register accessor: ADC10 Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10mem`]
module"]
pub type ADC10MEM = crate::Reg<adc10mem::ADC10MEM_SPEC>;
#[doc = "ADC10 Memory"]
pub mod adc10mem;
#[doc = "ADC10SA (rw) register accessor: ADC10 Data Transfer Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc10sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc10sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10sa`]
module"]
pub type ADC10SA = crate::Reg<adc10sa::ADC10SA_SPEC>;
#[doc = "ADC10 Data Transfer Start Address"]
pub mod adc10sa;
