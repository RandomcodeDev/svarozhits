ENTRY(kernel_main);

MEMORY {
	text (rwx) : ORIGIN = 0x80200000, LENGTH = 0x10000
	data (rw) : ORIGIN = 0x80210000, LENGTH = 0x4000
	rodata (r) : ORIGIN = 0x80214000, LENGTH = 0x1000
}

SECTIONS {
	/DISCARD/ : {
		*(.eh_frame);
	}

	.text : ALIGN(4K) {
		*(.text);
        *(.text.*);
	} > text

	.bss : ALIGN(4K) {
		PROVIDE(bss_start = .);
		*(.bss);
		. += 4096;
		. += 4096;
		PROVIDE(global_pointer = .);
		PROVIDE(bss_end = .);
	} > data

	.data : ALIGN(4K) {
		*(.data);
	} > data

	.rodata : ALIGN(4K) {
		*(.rodata);
	} > rodata

    PROVIDE(kernel_end = .);
}
