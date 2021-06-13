#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Flash CCOB Index Register"]
    pub fccobix: crate::Reg<fccobix::FCCOBIX_SPEC>,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: crate::Reg<fsec::FSEC_SPEC>,
    #[doc = "0x03 - Flash Clock Divider Register"]
    pub fclkdiv: crate::Reg<fclkdiv::FCLKDIV_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x05 - Flash Status Register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x07 - Flash Configuration Register"]
    pub fcnfg: crate::Reg<fcnfg::FCNFG_SPEC>,
    #[doc = "0x08 - Flash Common Command Object Register: Low"]
    pub fccoblo: crate::Reg<fccoblo::FCCOBLO_SPEC>,
    #[doc = "0x09 - Flash Common Command Object Register:High"]
    pub fccobhi: crate::Reg<fccobhi::FCCOBHI_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0b - Flash Protection Register"]
    pub fprot: crate::Reg<fprot::FPROT_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x0f - Flash Option Register"]
    pub fopt: crate::Reg<fopt::FOPT_SPEC>,
}
#[doc = "FCCOBIX register accessor: an alias for `Reg<FCCOBIX_SPEC>`"]
pub type FCCOBIX = crate::Reg<fccobix::FCCOBIX_SPEC>;
#[doc = "Flash CCOB Index Register"]
pub mod fccobix;
#[doc = "FSEC register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FCLKDIV register accessor: an alias for `Reg<FCLKDIV_SPEC>`"]
pub type FCLKDIV = crate::Reg<fclkdiv::FCLKDIV_SPEC>;
#[doc = "Flash Clock Divider Register"]
pub mod fclkdiv;
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FCCOBLO register accessor: an alias for `Reg<FCCOBLO_SPEC>`"]
pub type FCCOBLO = crate::Reg<fccoblo::FCCOBLO_SPEC>;
#[doc = "Flash Common Command Object Register: Low"]
pub mod fccoblo;
#[doc = "FCCOBHI register accessor: an alias for `Reg<FCCOBHI_SPEC>`"]
pub type FCCOBHI = crate::Reg<fccobhi::FCCOBHI_SPEC>;
#[doc = "Flash Common Command Object Register:High"]
pub mod fccobhi;
#[doc = "FPROT register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Flash Protection Register"]
pub mod fprot;
#[doc = "FOPT register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
