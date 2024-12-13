#![no_std]

mod allocator;
mod externs;
mod macros;
mod panic;

pub mod platform;
pub mod utilities;

///
///
///
#[no_mangle]
unsafe extern "C" fn startup(sp: *const u64) -> i32 {
    extern "Rust" {
        fn main(_: usize) -> Result<(), i32>;
    }

    let argc = *sp as usize;
    let _argv = sp.wrapping_add(1);

    match main(argc) {
        Ok(_) => return 0,
        Err(err) => return err,
    }
}
