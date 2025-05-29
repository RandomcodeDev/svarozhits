#![no_std]

#[derive(Clone, Debug, Default)]
pub struct MemRegion {
    pub base: usize,
    pub size: usize
}

#[derive(Clone, Debug, Default)]
pub struct MemBlock {
    pub base: usize,
    pub size: usize,
    pub region: u8,
    pub free: bool,
    pub next: *mut Self
}

#[derive(Clone, Debug, Default)]
pub struct BootInfo {
    pub memory_region: MemRegion,
    pub memory_map: *mut MemBlock,
    pub pci_config_base: usize
}
