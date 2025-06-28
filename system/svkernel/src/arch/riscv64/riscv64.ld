ENTRY(_start);

MEMORY {
	init (rwx) : ORIGIN = 0x80200000, LENGTH = 0x20000
	data (rw) : ORIGIN = 0x80220000, LENGTH = 0x8000
	rodata (r) : ORIGIN = 0x80228000, LENGTH = 0x8000
    heap (rw) : ORIGIN = 0x80230000, LENGTH = 0x10000
}

SECTIONS {
	/DISCARD/ : {
		*(.eh_frame);
	}

	.text : ALIGN(4K) {
        PROVIDE_HIDDEN(__init_array_start = .);
        *(.init_array .init_array.*)
        PROVIDE_HIDDEN(__init_array_end = .);
		*(.init);
		*(.text);
	} > init

	.bss : ALIGN(4K) {
		PROVIDE_HIDDEN(bss_start = .);
		*(.bss);
		. += 0x1000;
		PROVIDE_HIDDEN(stack_base = .);
		. += 0x1000;
		PROVIDE_HIDDEN(global_pointer = .);
		PROVIDE_HIDDEN(bss_end = .);
	} > data

    .heap : ALIGN(4K) {
        PROVIDE_HIDDEN(heap_start = .);
        . += 0x10000;
        PROVIDE_HIDDEN(heap_end = .);
    } > heap

	.data : ALIGN(4K) {
		*(.data);
	} > data

	.rodata : ALIGN(4K) {
		*(.rodata);
	} > rodata

    PROVIDE_HIDDEN(kernel_end = .);
}
