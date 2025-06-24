#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use svcommon::BootInfo;
use talc::{ClaimOnOom, Span, Talc, Talck};

mod arch;
mod util;

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
    ClaimOnOom::new(Span::from_base_size(
        &raw mut arch::heap_start,
        arch::HEAP_SIZE,
    ))
})
.lock();

pub fn boot_main(mut arch_info: arch::ArchInfo<'_>) -> ! {
    dprintln!(
        r#"
--<[{{**********************************************************************}}]*--
       _____   _____   _____   _____   _____  __  _  __  _     _   _   _
      |  ___| |  _  \ |  _  | |  _  | |  _  | \ \| |/ / | |   / | | | | |
      | |     | |_| / | |_| | | |_| | | | | |  \     /  | |  // | | | | |
      | |     |  _ |  |  _  | |  ___| | | | |   |   |   | | //| | | |_| |
      | |___  | |_| \ | | | | | |     | |_| |  /     \  | |// | | |____ |
      |_____| |_____/ |_| |_| |_|     |_____| /_/|_|\_\ |_ /  |_|      \_\

--<[{{**********************************************************************}}]>--
"#
    );

    dprintln!("Сварожиц bootloader starting");

    let mut boot_info = BootInfo::default();
    arch::init(&mut arch_info, &mut boot_info);

    ctor_bare::call_ctors();

    dprintln!("heap goes from 0x{:X}-0x{:X}", &raw const arch::heap_start as usize, &raw const arch::heap_end as usize);

    dprintln!("getting PCIe devices");
    let devices = svpci::check_bus(boot_info.pcie_region.base, 0);
    devices.iter().for_each(|device| dprintln!("{device:#?}"));

    loop {}
}
