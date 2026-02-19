.global _RESET
_RESET:                                     // starts in el2
    b el1_start

.global el1_start
el1_start:
    bl enable_fpu
    bl setup_vtable

    // enter rust
    bl _kernel_entry                         // go to rust entry point
    //b go_to_init_process                     // go to init
    b .                                      // hang forever
enable_fpu:
    mrs x0, CPACR_EL1
    orr x0, x0, #(3 << 20)   // FPEN = 0b11 (EL0 + EL1 allowed)
    msr CPACR_EL1, x0
    isb
    ret
    
setup_vtable:
    ldr x0, =_vector_table                  // load vtable into r0
    msr VBAR_EL1, x0
    isb                                     // move r0 to base vector table register
    ret