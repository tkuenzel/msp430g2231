#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer A Interrupt Vector Word"]
    pub taiv: TAIV,
    _reserved1: [u8; 0x30],
    #[doc = "0x32 - Timer A Control"]
    pub tactl: TACTL,
    #[doc = "0x34 - Timer A Capture/Compare Control 0"]
    pub tacctl0: TACCTL0,
    #[doc = "0x36 - Timer A Capture/Compare Control 1"]
    pub tacctl1: TACCTL1,
    _reserved4: [u8; 0x0a],
    #[doc = "0x42 - Timer A Counter Register"]
    pub tar: TAR,
    #[doc = "0x44 - Timer A Capture/Compare 0"]
    pub taccr0: TACCR0,
    #[doc = "0x46 - Timer A Capture/Compare 1"]
    pub taccr1: TACCR1,
}
#[doc = "TAIV (rw) register accessor: Timer A Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taiv`]
module"]
pub type TAIV = crate::Reg<taiv::TAIV_SPEC>;
#[doc = "Timer A Interrupt Vector Word"]
pub mod taiv;
#[doc = "TACTL (rw) register accessor: Timer A Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tactl`]
module"]
pub type TACTL = crate::Reg<tactl::TACTL_SPEC>;
#[doc = "Timer A Control"]
pub mod tactl;
#[doc = "TACCTL0 (rw) register accessor: Timer A Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl0`]
module"]
pub type TACCTL0 = crate::Reg<tacctl0::TACCTL0_SPEC>;
#[doc = "Timer A Capture/Compare Control 0"]
pub mod tacctl0;
#[doc = "TACCTL1 (rw) register accessor: Timer A Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl1`]
module"]
pub type TACCTL1 = crate::Reg<tacctl1::TACCTL1_SPEC>;
#[doc = "Timer A Capture/Compare Control 1"]
pub mod tacctl1;
#[doc = "TAR (rw) register accessor: Timer A Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Timer A Counter Register"]
pub mod tar;
#[doc = "TACCR0 (rw) register accessor: Timer A Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr0`]
module"]
pub type TACCR0 = crate::Reg<taccr0::TACCR0_SPEC>;
#[doc = "Timer A Capture/Compare 0"]
pub mod taccr0;
#[doc = "TACCR1 (rw) register accessor: Timer A Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr1`]
module"]
pub type TACCR1 = crate::Reg<taccr1::TACCR1_SPEC>;
#[doc = "Timer A Capture/Compare 1"]
pub mod taccr1;
