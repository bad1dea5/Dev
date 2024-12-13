mod platform;

pub mod scheme;

use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("asm/x86_64/start.s"), options(att_syntax));

#[no_mangle]
extern "C" fn rust_eh_personality() {}
