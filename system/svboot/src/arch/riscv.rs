use svbootlib::BootInfo;

use crate::{boot_main, dprint, dprintln};
use core::arch::{asm, global_asm};

global_asm! {include_str!("riscv_start.S")}

pub struct ArchInfo<'a> {
    hart_id: u64,
    device_tree: fdt::Fdt<'a>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn riscv_main(hart_id: u64, fdt_addr: *mut u8) -> ! {
    dprintln!("hard_id = 0x{:X}, fdt = 0x{:X}", hart_id, fdt_addr as u64);

    let device_tree = match unsafe { fdt::Fdt::from_ptr(fdt_addr) } {
        Ok(tree) => tree,
        Err(err) => panic!("failed to parse device tree: {}", err),
    };

    boot_main(ArchInfo {
        hart_id,
        device_tree,
    });
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

pub fn init(info: &mut ArchInfo, boot: &mut BootInfo) {
    let device_tree = &info.device_tree;

    dprintln!("discovering devices\n");

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

    dprintln!(
        "using memory region 0x{:X}-0x{:X}",
        boot.memory_region.base,
        boot.memory_region.base + boot.memory_region.size
    );

    dprintln!("getting PCIe information");
}
