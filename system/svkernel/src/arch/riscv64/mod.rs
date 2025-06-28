#[allow(static_mut_refs)]
use crate::types::{BootInfo, MemBlock};

use crate::kernel_main;
use crate::{dprint, dprintln};
use core::arch::asm;
use core::{
    arch::global_asm,
    ptr,
};

global_asm! {include_str!("riscv64_start.S")}

pub struct ArchInfo<'a> {
    hart_id: u64,
    device_tree: fdt::Fdt<'a>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn riscv64_main(hart_id: u64, fdt_addr: *mut u8) -> ! {
    dprintln!("hard_id = 0x{:X}, fdt = 0x{:X}", hart_id, fdt_addr as u64);

    let device_tree = match unsafe { fdt::Fdt::from_ptr(fdt_addr) } {
        Ok(tree) => tree,
        Err(err) => panic!("failed to parse device tree: {}", err),
    };

    kernel_main(ArchInfo {
        hart_id,
        device_tree,
    });
}

unsafe extern "C" {
    static mut kernel_end: u8;

    pub static mut heap_start: u8;
    pub static mut heap_end: u8;
}

pub const HEAP_SIZE: usize = 0x10000;

pub fn init(info: &mut ArchInfo, boot: &mut BootInfo) {
    boot.initial_cpu_id = info.hart_id as usize;

    let device_tree = &info.device_tree;

    dprintln!("processing device tree\n");

    fn print_node(node: &fdt::node::FdtNode<'_, '_>, indent: i32) {
        (0..indent).for_each(|_| dprint!(" "));
        dprintln!("{}/", node.name);

        for child in node.children() {
            print_node(&child, indent + 4);
        }
    }

    dprintln!("device tree for {}:", device_tree.root().model());
    print_node(
        &device_tree.find_node("/").expect("no nodes in device tree"),
        0,
    );
    dprintln!("");

    dprintln!("finding largest memory region");
    let mut i = 0;
    for region in device_tree.memory().regions() {
        let addr = region.starting_address as usize;
        let size = region.size.unwrap_or(0);
        dprintln!("memory region {}: 0x{:X}-0x{:X}", i, addr, addr + size);
        if size > boot.memory_region.size {
            boot.memory_region.base = addr;
            boot.memory_region.size = size;
        }

        i += 1;
    }
    dprintln!("using memory region {}", boot.memory_region);

    // TODO: do something sane instead of just assuming everything after the kernel is fine
    let current_block = &raw mut kernel_end as *mut MemBlock;
    unsafe {
        (*current_block).size =
            boot.memory_region.base + boot.memory_region.size - current_block as usize;
        (*current_block).next = ptr::null_mut();
    }

    //dprintln!("finding end of last reserved region");
    //if let Some(reserved_mem) = device_tree.find_node("/reserved-memory") {
    //    for node in reserved_mem.children() {
    //        // unwrap like this because it can just be ignored if it fails
    //        if let Some(mut reg) = node.reg()
    //            && let Some(region) = reg.nth(0)
    //            && let Some(size) = region.size
    //        {
    //            let reserved_region = MemRegion {
    //                base: region.starting_address as usize,
    //                size,
    //            };
    //
    //            dprintln!("reserved memory region {}", reserved_region);
    //
    //            if reserved_region.base > current_block as usize {
    //                current_block = (reserved_region.base + reserved_region.size) as *mut MemBlock;
    //            }
    //        }
    //    }
    //}

    dprintln!("usable memory area: {}", unsafe { &(*current_block) });
    boot.memory_map = current_block;

    dprintln!("getting PCIe information");
    let pci = device_tree
        .find_node("/soc/pci")
        .expect("failed to get /soc/pci in device tree");
    let region = pci.reg().expect("pci node missing reg").nth(0).unwrap();
    boot.pcie_region.base = region.starting_address as usize;
    boot.pcie_region.size = region.size.expect("/pci reg missing size");
    dprintln!(
        "found PCIe extended configuration space at {}",
        boot.pcie_region
    );
}

// debug console extension id
const DBCN_EID: u64 = 0x4442434e;
const DBCN_WRITE_FID: u64 = 0;

pub fn debug_write(msg: &str) {
    unsafe {
        asm!(
            "ecall",
            in("a7") DBCN_EID,
            in("a6") DBCN_WRITE_FID,
            in("a0") msg.len(),
            in("a1") msg.as_ptr(),
            in("a2") 0,
        );
    }
}
