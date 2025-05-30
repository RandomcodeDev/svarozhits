#![no_std]
#![no_main]

use core::{cell::UnsafeCell, mem, panic::PanicInfo};
use svbootlib::{BootInfo, MemBlock};

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    dprintln!("{:#?}", info);
    loop {}
}

mod arch;

pub fn boot_main(mut arch_info: arch::ArchInfo<'_>) -> ! {
    dprintln!(
        r#"
 _____   _____   _____   _____   _____   __  _  __  _     _   _   _
|  ___| |  _  \ |  _  | |  _  | |  _  |  \ \| |/ / | |   / | | | | |
| |     | |_| / | |_| | | |_| | | | | |   \     /  | |  // | | | | |
| |     |  _ |  |  _  | |  ___| | | | |    |   |   | | //| | | |_| |
| |___  | |_| \ | | | | | |     | |_| |   /     \  | |// | | |____ |
|_____| |_____/ |_| |_| |_|     |_____|  /_/|_|\_\ |_ /  |_|      \_\
"#
    );

    dprintln!("Сварожиц bootloader starting");

    let mut boot_info = BootInfo::default();
    arch::init(&mut arch_info, &mut boot_info);

    loop {}
}
