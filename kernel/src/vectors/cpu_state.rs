use core::arch::global_asm;

use crate::println;

#[repr(C)]
#[derive(Debug)]
struct State {
    x: [u64; 30],
}

unsafe extern "C" {
    pub unsafe fn dump_cpu_state_test() -> !;
}

global_asm!(
    r"
    .global dump_cpu_state_test
    dump_cpu_state_test:
        mov x0, #0xFFFFFFFF
        mov x1, #0xFFFFFFFF
        mov x2, #0xFFFFFFFF
        mov x3, #0xFFFFFFFF
        mov x29, #0xFFFFFFFF
        stp x28, x29, [sp, #-16]!
        stp x26, x27, [sp, #-16]!
        stp x24, x25, [sp, #-16]!
        stp x22, x23, [sp, #-16]!
        stp x20, x21, [sp, #-16]!
        stp x18, x19, [sp, #-16]!
        stp x16, x17, [sp, #-16]!
        stp x14, x15, [sp, #-16]!
        stp x12, x13, [sp, #-16]!
        stp x10, x11, [sp, #-16]!
        stp x8, x9, [sp, #-16]!
        stp x6, x7, [sp, #-16]!
        stp x4, x5, [sp, #-16]!
        stp x2, x3, [sp, #-16]!
        stp x0, x1, [sp, #-16]!
        mov x0, sp
        b print_cpu_state
"
);

#[unsafe(no_mangle)]
extern "C" fn print_cpu_state(state: State) -> ! {
    println!("state dump {:?}", state);
    panic!("wahhhh");
}
