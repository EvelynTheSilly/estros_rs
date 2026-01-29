.macro DUMP_REGISTERS
    stp     x19, x20, [sp, #-16]!
    stp     x21, x22, [sp, #-16]!
    stp     x23, x24, [sp, #-16]!
    stp     x25, x26, [sp, #-16]!
    stp     x27, x28, [sp, #-16]!
    stp     x29, x30, [sp, #-16]!
.endm

.macro LOAD_REGISTERS
    ldp     x29, x30, [sp], #16
    ldp     x27, x28, [sp], #16
    ldp     x25, x26, [sp], #16
    ldp     x23, x24, [sp], #16
    ldp     x21, x22, [sp], #16
    ldp     x19, x20, [sp], #16
.endm

.section .vectors, "ax"
.align 11                                   // 2KB alignment required
.global _vector_table
_vector_table:

// ========== Exceptions from EL1 ==========
// ================== sp0 ==================
el1_sp0_sync:
    DUMP_REGISTERS

    bl el1_sp0_sync_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_sp0_sync)

el1_sp0_irq:
    DUMP_REGISTERS

    bl el1_sp0_irq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_sp0_irq)

el1_sp0_fiq:
    DUMP_REGISTERS

    bl el1_sp0_fiq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_sp0_fiq)

el1_sp0_serror:
    DUMP_REGISTERS

    bl el1_sp0_serror_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_sp0_serror)

// ================== spx ==================
el1_spx_sync:
    DUMP_REGISTERS

    bl el1_spx_sync_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_spx_sync)

el1_spx_irq:
    DUMP_REGISTERS

    bl el1_spx_irq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_spx_irq)

el1_spx_fiq:
    DUMP_REGISTERS

    bl el1_spx_fiq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_spx_fiq)

el1_spx_serror:
    DUMP_REGISTERS

    bl el1_spx_serror_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el1_spx_serror)

// ========== Exceptions from EL0 ==========
// ================ AArch64 ================
el0_aarch64_sync:
    DUMP_REGISTERS

    bl el0_aarch64_sync_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch64_sync)

el0_aarch64_irq:
    DUMP_REGISTERS

    bl el0_aarch64_irq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch64_irq)

el0_aarch64_fiq:
    DUMP_REGISTERS

    bl el0_aarch64_fiq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch64_fiq)

el0_aarch64_serror:
    DUMP_REGISTERS

    bl el0_aarch64_serror_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch64_serror)

// ================ AArch32 ================
el0_aarch32_sync:
    DUMP_REGISTERS

    bl el0_aarch32_sync_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch32_sync)

el0_aarch32_irq:
    DUMP_REGISTERS

    bl el0_aarch32_irq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch32_irq)

el0_aarch32_fiq:
    DUMP_REGISTERS

    bl el0_aarch32_fiq_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch32_fiq)

el0_aarch32_serror:
    DUMP_REGISTERS

    bl el0_aarch32_serror_handler

    LOAD_REGISTERS

    eret
    .space 128 - (. - el0_aarch32_serror)