#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
#[cfg_attr(windows, path = "windows/mod.rs")]
mod platform;

pub use platform::scheme;

use core::ptr::NonNull;

pub struct Platform;

///
///
///
pub trait Sys {
    fn get_shared_object(_: Option<u64>) -> Result<Option<NonNull<u64>>, i32>;
    fn get_procedure_addr(_: Option<NonNull<u64>>, _: Option<u64>) -> Result<Option<NonNull<u64>>, i32>;
}
