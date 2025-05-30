#![cfg_attr(not(feature = "std"), no_std)]

use bincode::{Decode, Encode};

// custom 64-bit executable format for Svarozhits system software

// SVE\0 (little endian)
pub const SVE_MAGIC: u32 = 0x00455653;

// Format version
pub const SVE_REVISION: u16 = 1;

#[repr(u16)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub enum SVEMachine {
    RISCV = 0x5064,
    AMD64 = 0x8664,
    #[default]
    Unknown = 0xFFFF,
}

#[repr(C)]
#[derive(Clone, Debug, Decode, Encode)]
pub struct SVEHeader {
    pub magic: u32,    // must be SVE_MAGIC
    pub revision: u16, // must be SVE_REVISION

    pub machine: SVEMachine, // what architecture this image is for

    pub entry: u32, // offset of the entry point

    pub code: SVESectionDescriptor,   // executable code (.text)
    pub data: SVESectionDescriptor,   // read/write initialized data (.data)
    pub rodata: SVESectionDescriptor, // read-only initialized data (.rodata)
    pub zdata: SVESectionDescriptor,  // zeroed data size (.bss)

    pub strings: SVEBlobDescriptor, // string table, null terminated strings
    pub relocs: SVEBlobDescriptor, // relocation table, SVERelocDescriptor
    pub imports: SVEBlobDescriptor, // import table, SVEImportDescriptor
    pub import_ptrs: SVEBlobDescriptor, // import pointer table, void (*)()/void*
    pub exports: SVEBlobDescriptor, // export table, SVEExportDescriptor
}

impl Default for SVEHeader {
    fn default() -> Self {
        Self {
            magic: SVE_MAGIC,
            revision: SVE_REVISION,

            machine: SVEMachine::RISCV,

            entry: 0,

            code: SVESectionDescriptor::default(),
            data: SVESectionDescriptor::default(),
            rodata: SVESectionDescriptor::default(),
            zdata: SVESectionDescriptor::default(),

            strings: SVEBlobDescriptor::default(),
            relocs: SVEBlobDescriptor::default(),
            imports: SVEBlobDescriptor::default(),
            import_ptrs: SVEBlobDescriptor::default(),
            exports: SVEBlobDescriptor::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub struct SVESectionDescriptor {
    pub base: u32,     // offset from the base address to where this section is in memory
    pub size: u32,     // size of the section in memory
    pub offset: u32,   // offset in the file where the data is (can be zero)
    pub raw_size: u32, // size of the section's data in the file (can be zero)
}

#[repr(C)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub struct SVEBlobDescriptor {
    pub offset: u32,
    pub size: u32,
}

#[repr(u8)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub enum SVERelocType {
    #[default]
    Unknown,

    // sym + constant
    Absolute32,
    // sym + constant
    Absolute64,
    // base + sym
    Relative,
    // ipt + sym
    Imported,

    // TODO: TLS stuff? it's defined in ELF but maybe I'll just do it some other way
}

#[repr(C)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub struct SVERelocDescriptor {
    pub addr: u32, // virtual address of the thing to be relocated
    pub reloc_type: SVERelocType, // type of relocation

}

#[repr(C)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub struct SVEImportDescriptor {
    pub lib_name: u32, // library name string table offset
    pub sym_name: u32, // symbol name string table offset
    pub ptr_offset: u32, // import pointer offset
}

#[repr(C)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub struct SVEExportDescriptor {
    pub sym_name: u32, // symbol name string table offset
    pub addr: u32, // symbol address
}
