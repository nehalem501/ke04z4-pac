#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pulse Width Timer Register 1"]
    pub r1: crate::Reg<r1::R1_SPEC>,
    #[doc = "0x04 - Pulse Width Timer Register 2"]
    pub r2: crate::Reg<r2::R2_SPEC>,
}
#[doc = "R1 register accessor: an alias for `Reg<R1_SPEC>`"]
pub type R1 = crate::Reg<r1::R1_SPEC>;
#[doc = "Pulse Width Timer Register 1"]
pub mod r1;
#[doc = "R2 register accessor: an alias for `Reg<R2_SPEC>`"]
pub type R2 = crate::Reg<r2::R2_SPEC>;
#[doc = "Pulse Width Timer Register 2"]
pub mod r2;
