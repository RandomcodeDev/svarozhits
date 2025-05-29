#[cfg(target_arch = "riscv64")]
mod riscv;

use core::fmt::{self, Write};

#[cfg(target_arch = "riscv64")]
pub use riscv::*;

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
    ($($arg:tt)*) => ($crate::arch::debug_print_args(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => ($crate::arch::debug_println_args(format_args!($($arg)*)))
}
