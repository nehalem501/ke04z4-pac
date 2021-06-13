#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register 1"]
    pub cs1: crate::Reg<cs1::CS1_SPEC>,
    #[doc = "0x01 - Watchdog Control and Status Register 2"]
    pub cs2: crate::Reg<cs2::CS2_SPEC>,
    #[doc = "0x02 - Watchdog Counter Register: High"]
    pub cnth: crate::Reg<cnth::CNTH_SPEC>,
    #[doc = "0x03 - Watchdog Counter Register: Low"]
    pub cntl: crate::Reg<cntl::CNTL_SPEC>,
    #[doc = "0x04 - Watchdog Timeout Value Register: High"]
    pub tovalh: crate::Reg<tovalh::TOVALH_SPEC>,
    #[doc = "0x05 - Watchdog Timeout Value Register: Low"]
    pub tovall: crate::Reg<tovall::TOVALL_SPEC>,
    #[doc = "0x06 - Watchdog Window Register: High"]
    pub winh: crate::Reg<winh::WINH_SPEC>,
    #[doc = "0x07 - Watchdog Window Register: Low"]
    pub winl: crate::Reg<winl::WINL_SPEC>,
}
#[doc = "CS1 register accessor: an alias for `Reg<CS1_SPEC>`"]
pub type CS1 = crate::Reg<cs1::CS1_SPEC>;
#[doc = "Watchdog Control and Status Register 1"]
pub mod cs1;
#[doc = "CS2 register accessor: an alias for `Reg<CS2_SPEC>`"]
pub type CS2 = crate::Reg<cs2::CS2_SPEC>;
#[doc = "Watchdog Control and Status Register 2"]
pub mod cs2;
#[doc = "CNTH register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "Watchdog Counter Register: High"]
pub mod cnth;
#[doc = "CNTL register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Watchdog Counter Register: Low"]
pub mod cntl;
#[doc = "TOVALH register accessor: an alias for `Reg<TOVALH_SPEC>`"]
pub type TOVALH = crate::Reg<tovalh::TOVALH_SPEC>;
#[doc = "Watchdog Timeout Value Register: High"]
pub mod tovalh;
#[doc = "TOVALL register accessor: an alias for `Reg<TOVALL_SPEC>`"]
pub type TOVALL = crate::Reg<tovall::TOVALL_SPEC>;
#[doc = "Watchdog Timeout Value Register: Low"]
pub mod tovall;
#[doc = "WINH register accessor: an alias for `Reg<WINH_SPEC>`"]
pub type WINH = crate::Reg<winh::WINH_SPEC>;
#[doc = "Watchdog Window Register: High"]
pub mod winh;
#[doc = "WINL register accessor: an alias for `Reg<WINL_SPEC>`"]
pub type WINL = crate::Reg<winl::WINL_SPEC>;
#[doc = "Watchdog Window Register: Low"]
pub mod winl;
