ENTRY(_start);

MEMORY {
	init (rwx) : ORIGIN = 0x80200000, LENGTH = 0x10000
	data (rw) : ORIGIN = 0x80210000, LENGTH = 0x4000
	rodata (r) : ORIGIN = 0x80214000, LENGTH = 0x1000
}

SECTIONS {
	.text : ALIGN(4K) {
		*(.init);
		*(.text);
	} > init

	.bss : ALIGN(4K) {
		PROVIDE(bss_start = .);
		*(.bss);
		. += 4096;
		PROVIDE(stack_base = .);
		. += 4096;
		PROVIDE(global_pointer = .);
		PROVIDE(bss_end = .);
	} > data

	.data : ALIGN(4K) {
		*(.data);
	}

	.rodata : ALIGN(4K) {
		*(.rodata);
	} > rodata
}
