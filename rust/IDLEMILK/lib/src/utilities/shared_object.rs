use core::ptr::NonNull;

use crate::platform::{Platform, Sys};

///
///
///
pub struct SharedObject {
    addr: Option<NonNull<u64>>,
}

impl SharedObject {
    ///
    ///
    ///
    pub fn new(id: Option<u64>) -> Result<Self, i32> {
        let addr = Platform::get_shared_object(id)?;

        if let Some(arg) = addr {
            return Ok(Self { addr: Some(arg) });
        }

        Err(0)
    }

    ///
    ///
    ///
    pub fn get<T>(&self, id: Option<u64>) -> Result<T, i32>
    where
        T: Sized
    {
        if let Some(arg) = Platform::get_procedure_addr(self.addr, id)? {
            unsafe {
                let addr: T = core::mem::transmute_copy(&arg.as_ptr());
                return Ok(addr)
            }
        }

        Err(0)
    }
}
