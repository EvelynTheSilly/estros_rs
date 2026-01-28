.global _RESET
_RESET:                                     // starts in el2
    b el1_start

.global el1_start
el1_start:
    bl load_stack

    //bl setup_vtable

    // enter rust
    bl _kernel_entry                         // go to rust entry point
    //b go_to_init_process                     // go to init
    b .                                      // hang forever

load_stack:
    ldr x0, =el1_stack_top
    mov sp, x0
    ret
/*
setup_vtable:
    ldr x0, =_vector_table                  // load vtable into r0
    msr VBAR_EL1, x0
    isb                                     // move r0 to base vector table register
    ret

go_to_init_process:
                            //set up stack pointer for EL0
    ldr x0, =el0_stack_top
    msr sp_el0, x0          // SP_EL0 = top of user stack
                            // set up SPSR_EL1 to enter EL0, using AArch64, interrupts masked
                            // M[4:2]=0b000 for EL0t
                            // D/I/A/F bits mask exceptions if needed
    mov x1, #0              // SPSR_EL1 value
    msr spsr_el1, x1        // SPSR_EL1 = EL0 flags
                            // set ELR_EL1 = address of first instruction in EL0
    ldr x2, =_INIT
    msr elr_el1, x2
    eret                    // jumps into EL0
*/