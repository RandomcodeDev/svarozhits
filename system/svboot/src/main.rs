#![no_std]
#![no_main]

use svcommon::BootInfo;

mod arch;
mod util;

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

    let device = svpci::check_device(boot_info.pcie_region.base, 0, 0)
        .expect("failed to get device 0 on bus 0");
    dprintln!("{:#X?}", device);

    loop {}
}
