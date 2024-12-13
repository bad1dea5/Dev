pub const DT_NULL: u64 = 0;
pub const DT_STRTAB: u64 = 5;
pub const DT_SYMTAB: u64 = 6;
pub const DT_DEBUG: u64 = 21;

pub const ELFMAG0: u8 = 0x7f;
pub const ELFMAG1: u8 = 0x45;
pub const ELFMAG2: u8 = 0x4c;
pub const ELFMAG3: u8 = 0x46;

pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_NIDENT: usize = 16;

pub const ELFCLASS64: u8 = 2;
pub const EM_X86_64: u16 = 62;

pub const PT_DYNAMIC: u32 = 2;

#[repr(C)]
pub struct ELF64Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct ELF64Dyn {
    pub d_tag: u64,
    pub d_ptr: u64,
}

#[repr(C)]
pub struct ELF64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[repr(C)]
pub struct ELF64Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}
