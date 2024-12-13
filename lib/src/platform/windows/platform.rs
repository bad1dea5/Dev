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
    fn get_process_environment() -> Result<&'static scheme::PEB, i32> {
        extern "C" {
            static process_environment: *const scheme::PEB;
        }

        unsafe { Ok(&*process_environment) }
    }
}

impl Sys for Platform {
    ///
    ///
    ///
    fn get_shared_object(id: Option<u64>) -> Result<Option<NonNull<u64>>, i32> {
        let process_environment = Self::get_process_environment()?;

        let mut list_entry = unsafe {
            (*(process_environment.ldr as *const scheme::PEBLdrData))
                .in_memory_order_module_list
                .flink as *const scheme::ListEntry
        };

        let mut table_entry = unsafe {
            (*(process_environment.ldr as *const scheme::PEBLdrData))
                .in_memory_order_module_list
                .flink as *const scheme::LDRDataTableEntry
        };

        let addr: Result<Option<*const u64>, i32> = match id {
            Some(arg) => loop {
                unsafe {
                    if (*table_entry).full_dll_name.buffer.is_null() {
                        break Err(0);
                    }

                    if (*table_entry).full_dll_name.length > 0 {
                        let name = core::slice::from_raw_parts(
                            (*table_entry).full_dll_name.buffer,
                            (*table_entry).full_dll_name.length.wrapping_div(2) as usize,
                        );

                        if !name.is_empty() && CRC64::new(name) == arg {
                            break Ok(Some(
                                (*table_entry).in_initialization_order_links.flink as *const u64,
                            ));
                        }
                    }

                    list_entry = (*list_entry).flink;
                    table_entry = list_entry as *const scheme::LDRDataTableEntry;
                }
            },
            None => unsafe {
                Ok(Some(
                    (*table_entry).in_initialization_order_links.flink as *const u64,
                ))
            },
        };

        if let Ok(Some(arg)) = addr {
            let arg = NonNull::new(arg as *mut _);

            if let Some(ptr) = arg {
                return Ok(Some(ptr));
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

        let dos_header = unsafe { &*(addr.as_ptr() as *const scheme::ImageDosHeader) };

        if dos_header.e_magic != scheme::IMAGE_DOS_SIGNATURE {
            return Err(0);
        }

        let nt_header = unsafe {
            &*(addr
                .as_ptr()
                .wrapping_byte_add(dos_header.e_lfanew as usize)
                as *const scheme::ImageNtHeader64)
        };

        if nt_header.signature != scheme::IMAGE_NT_SIGNATURE {
            return Err(0);
        }

        if nt_header.optional_header.magic != scheme::IMAGE_NT_OPTIONAL_HDR64_MAGIC {
            return Err(0);
        }

        if nt_header.file_header.machine != scheme::IMAGE_FILE_MACHINE_AMD64 {
            return Err(0);
        }

        let optional_header = &nt_header.optional_header;

        let data_directory = &optional_header.data_directory[scheme::IMAGE_DIRECTORY_ENTRY_EXPORT];

        let export_directory = unsafe {
            &*((addr.as_ptr() as usize + data_directory.virtual_address as usize)
                as *mut scheme::ImageExportDirectory)
        };

        let export_address_table: *const u32 =
            (addr.as_ptr() as usize + export_directory.address_of_functions as usize) as *const u32;

        let name_ordinals_pointer: *const u16 = (addr.as_ptr() as usize
            + export_directory.address_of_name_ordinals as usize)
            as *const u16;

        let export_name_pointer_table: *const u32 =
            (addr.as_ptr() as usize + export_directory.address_of_names as usize) as *const u32;

        for i in 0..export_directory.number_of_names {
            let buffer = unsafe {
                (addr.as_ptr() as usize + *export_name_pointer_table.offset(i as isize) as usize)
                    as *const u8
            };

            let name =
                unsafe { core::slice::from_raw_parts(buffer, crate::externs::strlen(buffer)) };

            if let Some(arg) = id {
                if !name.is_empty() && CRC64::new(name) == arg {
                    let ordinal: u16 = unsafe { *name_ordinals_pointer.offset(i as isize) };

                    let function_address: *const u32 = unsafe {
                        (addr.as_ptr() as usize
                            + *export_address_table.offset(ordinal as isize) as usize)
                            as *const u32
                    };

                    return NonNull::new(function_address as *mut _).map(Some).ok_or(0);
                }
            }
        }

        Err(0)
    }
}
