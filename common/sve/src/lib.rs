#![cfg_attr(not(feature = "std"), no_std)]

/// Custom 64-bit only executable/library format, inspired by PE/COFF

// SVE\0 (little endian)
pub const SVE_MAGIC: u32 = 0x00698683;

// Format version
pub const SVE_REVISION: u16 = 1;

pub enum SVETableType {
    Section, // SVESectionDescriptor, contains section information
    Export, // SVEExportDescriptor, contains names of exported symbols
    Import, // SVEImportDescriptor, contains names of imported symbols
    ImportPointer, // *void, contains imported symbol pointers
}

// The type of table this refers to is determined by its index in the 
#[repr(C)]
pub struct SVETableDescriptor {
    offset: u32,
    size: u32
}

#[repr(u16)]
pub enum SVEMachine {
    RISCV = 0x5064,
    AMD64 = 0x8664
}

#[repr(C)]
pub struct SVEHeader {
    magic: u32, // must be SVE_MAGIC
    revision: u16, // must be SVE_REVISION
    machine: u16, // what architecture this image is for
    table_count: u16, // how many table descriptors are present
    // variable number of SVETableDescriptor's, indexed by SVETableType
    // variable number of section descriptors
}
