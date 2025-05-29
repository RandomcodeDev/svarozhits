#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "riscv64")]
mod riscv;

pub fn boot_main() -> ! {
    
    loop {}
}
