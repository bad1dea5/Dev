#![no_main]
#![no_std]

use idlemilk;

#[no_mangle]
extern "Rust" fn main(_argc: usize) -> Result<(), i32> {
    Ok(())
}
