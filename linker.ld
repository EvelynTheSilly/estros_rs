ENTRY(_RESET)

SECTIONS
{
	. = ALIGN(16);
	. = 0x40000000;
	_start = .;
	.startup . : { build/kernel(.text) } 
	.text : { *(.text) } 
	.data : { *(.data) } 
	.bss : { *(.bss COMMON) } 
    .vectors ALIGN(11): {
        KEEP(*(.vectors))
    }
    .rodata : {
        *(.rodata)
    }
	. = . + 0x100000; /* 4kB of stack memory */
	. = ALIGN(16);
	el1_stack_top = .;
	. = . + 0x100000; /* 4kB of stack memory */
	. = ALIGN(16);
	_end = .;
	el0_stack_top = .;
}