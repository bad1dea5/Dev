#[repr(C)]
pub struct LinkMap {
    pub l_addr: u64,
    pub l_name: *const u8,
    pub l_ld: *const super::ELF64Dyn,
    pub l_next: *const LinkMap,
    pub l_prev: *const LinkMap,
}

pub enum RState {
    RtConsisent,
    RtAdd,
    RtDelete,
}

#[repr(C)]
pub struct RDebug {
    pub r_version: i32,
    pub r_map: *const LinkMap,
    pub r_brk: u64,
    pub r_state: RState,
}
