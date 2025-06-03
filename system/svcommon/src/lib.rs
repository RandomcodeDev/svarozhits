#![no_std]

use core::fmt::Display;

#[allow(non_camel_case_types)]
pub type uptr = usize;
#[allow(non_camel_case_types)]
pub type iptr = isize;

#[derive(Clone, Debug, Default)]
pub struct MemRegion {
    pub base: uptr,
    pub size: usize,
}

impl MemRegion {
    pub fn end(&self) -> uptr {
        self.base + self.size
    }
}

impl Display for MemRegion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "0x{:X}-0x{:X} (0x{:X} bytes)",
            self.base,
            self.end(),
            self.size
        )
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct MemBlock {
    pub next: *mut Self,
    pub size: usize,
}

impl MemBlock {
    pub fn base(&self) -> uptr {
        &raw const *self as uptr
    }

    pub fn end(&self) -> uptr {
        self.base() + self.size
    }

    pub fn append(&mut self, new_next: *mut Self) {
        let prev_next = if !self.next.is_null() {
            Some(self.next)
        } else {
            None
        };

        self.next = new_next;
        if let Some(prev_next) = prev_next {
            unsafe { (*self.next).next = prev_next };
        }
    }
}

impl Display for MemBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "0x{:X}-0x{:X} (0x{:X} bytes) next = 0x{:X}",
            self.base(),
            self.end(),
            self.size,
            self.next as uptr
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct BootInfo {
    pub initial_cpu_id: usize,
    pub memory_region: MemRegion,
    pub memory_map: *mut MemBlock,
    pub pcie_region: MemRegion,
}
