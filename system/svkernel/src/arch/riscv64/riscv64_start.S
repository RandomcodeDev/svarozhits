.option norvc # no compression

.section .init

.type _start, @function
.global _start
_start:
	.cfi_startproc

.option push
.option norelax # can't relax loading the global pointer
	la gp, global_pointer
.option pop

	# no address translation yet
	csrw satp, zero

	# load sp and s0
	la sp, stack_base
	andi sp, sp, -16
	add s0, sp, zero

	# clear bss
	la t5, bss_start
	la t6, bss_end
bss_clear:
	sd zero, (t5)
	addi t5, t5, 8
	bltu t5, t6, bss_clear

	# la t0, riscv64_main
	# csrw mepc, t0 # exception location? not sure why entry point should be there

	tail riscv64_main

	.cfi_endproc

.end
