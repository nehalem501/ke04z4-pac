#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status and ID Register"]
    pub srsid: crate::Reg<srsid::SRSID_SPEC>,
    #[doc = "0x04 - System Options Register"]
    pub sopt: crate::Reg<sopt::SOPT_SPEC>,
    #[doc = "0x08 - Pin Selection Register"]
    pub pinsel: crate::Reg<pinsel::PINSEL_SPEC>,
    #[doc = "0x0c - System Clock Gating Control Register"]
    pub scgc: crate::Reg<scgc::SCGC_SPEC>,
    #[doc = "0x10 - Universally Unique Identifier Low Register"]
    pub uuidl: crate::Reg<uuidl::UUIDL_SPEC>,
    #[doc = "0x14 - Universally Unique Identifier Middle Low Register"]
    pub uuidml: crate::Reg<uuidml::UUIDML_SPEC>,
    #[doc = "0x18 - Universally Unique Identifier Middle High Register"]
    pub uuidmh: crate::Reg<uuidmh::UUIDMH_SPEC>,
    #[doc = "0x1c - Clock Divider Register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
}
#[doc = "SRSID register accessor: an alias for `Reg<SRSID_SPEC>`"]
pub type SRSID = crate::Reg<srsid::SRSID_SPEC>;
#[doc = "System Reset Status and ID Register"]
pub mod srsid;
#[doc = "SOPT register accessor: an alias for `Reg<SOPT_SPEC>`"]
pub type SOPT = crate::Reg<sopt::SOPT_SPEC>;
#[doc = "System Options Register"]
pub mod sopt;
#[doc = "PINSEL register accessor: an alias for `Reg<PINSEL_SPEC>`"]
pub type PINSEL = crate::Reg<pinsel::PINSEL_SPEC>;
#[doc = "Pin Selection Register"]
pub mod pinsel;
#[doc = "SCGC register accessor: an alias for `Reg<SCGC_SPEC>`"]
pub type SCGC = crate::Reg<scgc::SCGC_SPEC>;
#[doc = "System Clock Gating Control Register"]
pub mod scgc;
#[doc = "UUIDL register accessor: an alias for `Reg<UUIDL_SPEC>`"]
pub type UUIDL = crate::Reg<uuidl::UUIDL_SPEC>;
#[doc = "Universally Unique Identifier Low Register"]
pub mod uuidl;
#[doc = "UUIDML register accessor: an alias for `Reg<UUIDML_SPEC>`"]
pub type UUIDML = crate::Reg<uuidml::UUIDML_SPEC>;
#[doc = "Universally Unique Identifier Middle Low Register"]
pub mod uuidml;
#[doc = "UUIDMH register accessor: an alias for `Reg<UUIDMH_SPEC>`"]
pub type UUIDMH = crate::Reg<uuidmh::UUIDMH_SPEC>;
#[doc = "Universally Unique Identifier Middle High Register"]
pub mod uuidmh;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider Register"]
pub mod clkdiv;
