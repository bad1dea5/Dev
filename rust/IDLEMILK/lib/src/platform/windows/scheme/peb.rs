#[repr(C)]
pub struct PEB {
    pub inherited_address: u8,
    pub read_image_exec_options: u8,
    pub being_debugged: u8,
    pub reserved_1: u8,
    pub mutant: *const u64,
    pub image_base: *const u64,
    pub ldr: *const PEBLdrData,
    pub process_parameters: *const RTLUserProcessParameters,
    pub subsystem: *const u64,
    pub process_heap: *const u64,
    pub fast_peb_lock: *const RTLCriticalSection,
    pub fast_peb_lock_routine: *const u64,
    pub fast_peb_unlock_routine: *const u64,
    pub environment_update_count: u32,
    pub kernel_callback_table: *const u64,
    pub reserved2: [u32; 2],
    pub free_list: *const u64,
    pub tls_expansion_counter: u32,
    pub tls_bitmap: *const RTLBitmap,
    pub tls_bitmap_bits: [u32; 2],
    pub read_only_shared_memory_base: *const u64,
    pub read_only_shared_memory_heap: *const u64,
    pub read_only_static_server_data: *const *const u64,
    pub ansi_code_page: *const u64,
    pub oem_code_page: *const u64,
    pub unicode_case_table: *const u64,
    pub number_of_processors: u32,
    pub nt_global_flag: u32,
    pub critical_section_timeout: super::LargeInteger,
    pub heap_segment_reserve: usize,
    pub heap_segment_commit: usize,
    pub heap_decommit_total_free_threshold: usize,
    pub heap_decommit_free_block_threshold: usize,
    pub number_of_heaps: u32,
    pub maximum_number_of_heaps: u32,
    pub process_heaps: *const *const u64,
    pub gdi_shared_handle_table: *const u64,
    pub process_starter_helper: *const u64,
    pub gdi_dc_attribute_list: *const u64,
    pub loader_lock: *const u64,
    pub os_major_version: u32,
    pub os_minor_version: u32,
    pub os_build_number: u32,
    pub os_platform_id: u32,
    pub image_sub_system: u32,
    pub image_subsystem_major_version: u32,
    pub image_subsystem_minor_version: u32,
    // ...
}

#[repr(C)]
pub struct PEBLdrData {
    pub length: u32,
    pub initialized: u8,
    pub sshandle: *const u64,
    pub in_load_order_module_list: super::ListEntry,
    pub in_memory_order_module_list: super::ListEntry,
    pub in_initialization_order_module_list: super::ListEntry,
    pub entry_in_progress: *const u64,
    pub shutdown_in_progress: u8,
    pub shutdown_thread_id: *const u64,
}

#[repr(C)]
pub struct RTLUserProcessParameters {
    pub allocation_size: u32,
    pub size: u32,
    pub flags: u32,
    pub debug_flags: u32,
    pub console_handle: *const u64,
    pub console_flags: u32,
    pub std_input: *const u64,
    pub std_output: *const u64,
    pub std_error: *const u64,
    pub current_directory: super::CurDir,
    pub dll_path: super::UnicodeString,
    pub image_path_name: super::UnicodeString,
    pub command_line: super::UnicodeString,
    pub environment: *const u64,
    pub dw_x: u32,
    pub dw_y: u32,
    pub dw_x_size: u32,
    pub dw_y_size: u32,
    pub dw_x_count_chars: u32,
    pub dw_y_count_chars: u32,
    pub dw_fill_attribute: u32,
    pub dw_flags: u32,
    pub dw_show_window: u32,
    pub window_title: super::UnicodeString,
    pub desktop: super::UnicodeString,
    pub shell_info: super::UnicodeString,
    pub runtime_info: super::UnicodeString,
    pub drive_current_directory: [RTLDriveLetterCurDir; 32],
    // ...
}

#[repr(C)]
pub struct RTLCriticalSection {
    pub debug_info: *const RTLCriticalSectionDebug,
    pub lock_count: i32,
    pub recursion_count: i32,
    pub owning_thread: *const u64,
    pub lock_semaphore: *const u64,
    pub spin_count: u64,
}

#[repr(C)]
pub struct RTLCriticalSectionDebug {
    pub type_: u16,
    pub creator_backtrace_index: u16,
    pub critical_section: *const RTLCriticalSection,
    pub process_lock_lists: super::ListEntry,
    pub entry_count: u32,
    pub contention_count: u32,
    pub reserved1: [u32; 2],
}

#[repr(C)]
pub struct RTLBitmap {
    pub sizeof_bitmap: u32,
    pub buffer: *const u64,
}

#[repr(C)]
pub struct RTLDriveLetterCurDir {
    pub flags: u16,
    pub length: u16,
    pub timestamp: u32,
    pub dos_path: super::UnicodeString,
}

#[repr(C)]
pub struct LDRDataTableEntry {
    pub in_load_order_links: super::ListEntry,
    pub in_memory_order_links: super::ListEntry,
    pub in_initialization_order_links: super::ListEntry,
    pub dll_base: *const u64,
    pub entry_point: *const u64,
    pub size_of_image: u32,
    pub full_dll_name: super::UnicodeString,
    pub base_dll_name: super::UnicodeString,
    pub flags: u32,
    pub obsolete_load_count: u16,
    pub tls_index: u16,
    pub hash_links: super::ListEntry,
    pub timedate_stamp: u32,
    pub entry_point_activation_context: *const u64,
    pub lock: *const u64,
    pub ddag_none: *const LDRDdagNode,
    pub node_module_link: super::ListEntry,
    pub snap_context: *const u64,
    pub load_context: *const u64,
    pub parent_dll_base: *const u64,
    pub base_address_index_node: [*const u64; 3],
    pub mapping_info_index_node: [*const u64; 3],
    // ...
}

#[repr(C)]
pub struct LDRDdagNode {
    pub modules: super::ListEntry,
    pub service_tag_list: *const u64,
    pub load_count: u32,
    pub load_while_unloading_count: u32,
    pub lowest_link: u32,
    pub dependencies: u64,
    pub incoming_dependencies: u64,
    pub state: u32,
    pub condense_link: *const u64,
    pub pre_order_number: u32,
    // ...
}
