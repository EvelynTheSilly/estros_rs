ENTRY(_start)
PHDRS
{
    text PT_LOAD FLAGS(5); /* 5 is Read (4) + Execute (1) */
    data PT_LOAD FLAGS(6); /* 6 is Read (4) + Write (2) */
}
SECTIONS
{
	. = ALIGN(16);
	. = 0x40000000;
    .text : {
        *(.text*)
    } : text
	.data : { *(.data) } : data
	.bss : { *(.bss COMMON) } : data
    .rodata : {
        *(.rodata)
    } : text
}