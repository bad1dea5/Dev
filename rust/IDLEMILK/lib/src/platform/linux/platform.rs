use core::ptr::NonNull;

use crate::{
    platform::{Platform, Sys},
    utilities::hash::CRC64,
};

use super::scheme;

///
///
///
impl Platform {
    fn get_process_environment() -> Result<&'static [scheme::ELF64Dyn], i32> {
        extern "C" {
            static _DYNAMIC: scheme::ELF64Dyn;
        }

        unsafe {
            let addr = &_DYNAMIC as *const scheme::ELF64Dyn;

            let count = (0..)
                .position(|i| (*addr.wrapping_add(i)).d_tag == scheme::DT_NULL)
                .unwrap_or(0);

            let array = core::slice::from_raw_parts(addr, count);

            if array.is_empty() {
                return Err(0);
            }

            Ok(array)
        }
    }
}

impl Sys for Platform {
    ///
    ///
    ///
    fn get_shared_object(id: Option<u64>) -> Result<Option<NonNull<u64>>, i32> {
        let dynamic = Self::get_process_environment()?;

        for section in dynamic.iter() {
            if section.d_tag == scheme::DT_DEBUG {
                let debug = NonNull::<scheme::RDebug>::new(section.d_ptr as *mut scheme::RDebug);

                if debug.is_none() {
                    return Err(1);
                }

                let mut linkmap = unsafe { &*(debug.unwrap().as_mut().r_map) };

                let addr: Result<Option<*const u64>, i32> = match id {
                    Some(arg) => loop {
                        if linkmap.l_name.is_null() {
                            break Err(0);
                        }

                        if !linkmap.l_name.is_null() {
                            let name = unsafe {
                                core::slice::from_raw_parts(
                                    linkmap.l_name,
                                    crate::externs::strlen(linkmap.l_name),
                                )
                            };

                            if !name.is_empty() && CRC64::new(name) == arg {
                                break Ok(Some(linkmap.l_addr as *const u64));
                            }
                        }

                        linkmap = unsafe { &*(linkmap.l_next) };
                    },
                    None => match NonNull::<u64>::new(linkmap.l_addr as *mut u64) {
                        Some(arg) => Ok(Some(arg.as_ptr())),
                        None => Err(0),
                    },
                };

                if let Ok(Some(arg)) = addr {
                    let arg = NonNull::new(arg as *mut _);

                    if let Some(ptr) = arg {
                        return Ok(Some(ptr));
                    }
                }
            }
        }

        Err(0)
    }

    ///
    ///
    ///
    fn get_procedure_addr(
        addr: Option<NonNull<u64>>,
        id: Option<u64>,
    ) -> Result<Option<NonNull<u64>>, i32> {
        let addr = match addr {
            None => return Err(0),
            Some(arg) => arg,
        };

        let ehdr = unsafe { &*(addr.as_ptr() as *const scheme::ELF64Ehdr) };

        if ehdr.e_ident[scheme::EI_MAG0] != scheme::ELFMAG0
            || ehdr.e_ident[scheme::EI_MAG1] != scheme::ELFMAG1
            || ehdr.e_ident[scheme::EI_MAG2] != scheme::ELFMAG2
            || ehdr.e_ident[scheme::EI_MAG3] != scheme::ELFMAG3
        {
            return Err(0);
        }

        if ehdr.e_ident[scheme::EI_CLASS] != scheme::ELFCLASS64 {
            return Err(0);
        }

        if ehdr.e_machine != scheme::EM_X86_64 {
            return Err(0);
        }

        let phdr = unsafe {
            core::slice::from_raw_parts(
                addr.as_ptr().wrapping_byte_add(ehdr.e_phoff as usize) as *const scheme::ELF64Phdr,
                ehdr.e_phnum as usize,
            )
        };

        if phdr.is_empty() {
            return Err(0);
        }

        for program_header in phdr.iter() {
            if program_header.p_type == scheme::PT_DYNAMIC {
                if let Some(arg) = NonNull::<scheme::ELF64Dyn>::new(
                    addr.as_ptr()
                        .wrapping_byte_add(program_header.p_vaddr as usize)
                        as *mut scheme::ELF64Dyn,
                ) {
                    let count = (0..)
                        .take_while(|i| unsafe {
                            (*arg.offset(*i).as_ptr()).d_tag != scheme::DT_NULL
                        })
                        .count();

                    let segments = unsafe { core::slice::from_raw_parts(arg.as_ptr(), count) };

                    if segments.is_empty() {
                        return Err(0);
                    }

                    let mut strtab = NonNull::<u8>::new(core::ptr::null_mut());
                    let mut symtab = NonNull::<scheme::ELF64Sym>::new(core::ptr::null_mut());

                    for segment in segments {
                        match segment.d_tag {
                            scheme::DT_STRTAB => {
                                strtab = NonNull::<u8>::new(segment.d_ptr as *mut _)
                            }
                            scheme::DT_SYMTAB => {
                                symtab = NonNull::<scheme::ELF64Sym>::new(segment.d_ptr as *mut _)
                            }
                            _ => {}
                        }
                    }

                    let symbol_table = match strtab {
                        Some(str) => match symtab {
                            Some(sym) => unsafe {
                                core::slice::from_raw_parts(
                                    sym.as_ptr(),
                                    (str.as_ptr() as usize)
                                        .wrapping_sub(sym.as_ptr() as usize)
                                        .wrapping_div(core::mem::size_of::<scheme::ELF64Sym>()),
                                )
                            },
                            None => return Err(0),
                        },
                        None => return Err(0),
                    };

                    for symbol in symbol_table {
                        if let Some(arg) = strtab {
                            let buffer = arg.as_ptr().wrapping_byte_add(symbol.st_name as usize);
                            let name = unsafe {
                                core::slice::from_raw_parts(buffer, crate::externs::strlen(buffer))
                            };

                            if let Some(arg) = id {
                                if !name.is_empty() && CRC64::new(name) == arg {
                                    return NonNull::new(
                                        addr.as_ptr().wrapping_byte_add(symbol.st_value as usize)
                                            as *mut _,
                                    )
                                    .map(Some)
                                    .ok_or(0);
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(0)
    }
}
