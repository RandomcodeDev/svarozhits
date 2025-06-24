use core::{fmt::{self, Write}, hint::spin_loop, panic::PanicInfo, sync::atomic::{AtomicBool, Ordering}};

use crate::arch::debug_write;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    crate::dprintln!("{:#?}", info);
    loop {}
}

pub struct DebugConsole;

impl fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        debug_write(s);
        Ok(())
    }
}

pub fn debug_print_args(args: fmt::Arguments) {
    let _ = write!(DebugConsole, "{args}");
}

pub fn debug_println_args(args: fmt::Arguments) {
    let _ = writeln!(DebugConsole, "{args}");
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => ($crate::util::debug_print_args(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => ($crate::util::debug_println_args(format_args!($($arg)*)))
}
