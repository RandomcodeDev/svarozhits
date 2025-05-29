use core::arch::naked_asm;

#[unsafe(naked)]
pub unsafe extern "C" fn _start() {
    naked_asm!(
        ""
    );
}
