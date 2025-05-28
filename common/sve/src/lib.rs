#![cfg_attr(not(feature = "std"), no_std)]

use bincode::{Decode, Encode};

// SVE\0 (little endian)
pub const SVE_MAGIC: u32 = 0x00455653;

// Format version
pub const SVE_REVISION: u16 = 1;

// Index into the table descriptors after the header
#[repr(u16)]
#[derive(Clone, Debug, Decode, Encode)]
pub enum SVETableType {
    Export,        // SVEExportDescriptor, contains names of exported symbols
    Import,        // SVEImportDescriptor, contains names of imported symbols
    ImportPointer, // *void, contains imported symbol pointers
    MaxTableCount,
}

#[repr(C)]
#[derive(Clone, Debug, Default, Decode, Encode)]
pub struct SVEBlobDescriptor {
    pub offset: u32,
    pub size: u32,
}

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
    pub magic: u32,                     // must be SVE_MAGIC
    pub revision: u16,                  // must be SVE_REVISION
    pub machine: SVEMachine,            // what architecture this image is for
    pub code_blob: SVEBlobDescriptor,   // executable code (.text)
    pub data_blob: SVEBlobDescriptor,   // read/write initialized data (.data)
    pub rodata_blob: SVEBlobDescriptor, // read-only initialized data (.rodata)
    pub zdata_size: u32,                // zeroed data size (.bss)
    pub table_count: u16,               // how many table descriptors are present
                                        // variable number of SVEBlobDescriptor's, indexed by SVETableType
}

impl Default for SVEHeader {
    fn default() -> Self {
        Self {
            magic: SVE_MAGIC,
            revision: SVE_REVISION,
            machine: SVEMachine::RISCV,
            code_blob: SVEBlobDescriptor::default(),
            data_blob: SVEBlobDescriptor::default(),
            rodata_blob: SVEBlobDescriptor::default(),
            zdata_size: 0,
            table_count: 0,
        }
    }
}
