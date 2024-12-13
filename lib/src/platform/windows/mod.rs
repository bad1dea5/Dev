mod platform;

pub mod scheme;

use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("asm/x86_64/start.s"));

#[no_mangle]
extern "C" fn __CxxFrameHandler3() {}

#[export_name = "_fltused"]
static _FLTUSED: i32 = 0x9875;
