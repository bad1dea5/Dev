#[repr(C)]
pub struct CurDir {
    pub dos_path: UnicodeString,
    pub handle: *const u64,
}

#[repr(C)]
pub struct LargeInteger {
    pub low_part: u32,
    pub high_part: i32,
    pub quad_part: u64,
}

#[repr(C)]
pub struct ListEntry {
    pub flink: *const ListEntry,
    pub blink: *const ListEntry,
}

#[repr(C)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *const u16,
}
