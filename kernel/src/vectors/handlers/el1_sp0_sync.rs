use crate::{println, vectors::cpu_state};
use core::arch::asm;

#[unsafe(no_mangle)]
extern "C" fn el1_sp0_sync_handler(state: &mut cpu_state::State) {
    let esr_el1: u64;
    unsafe {
        asm!(
            "
            mrs x0, sctlr_el1
            ",
            out("x0") esr_el1
        );
    }
    let ec = (esr_el1 >> 26) & 0x3f;
    let iss = esr_el1 & 0x1FFFFFF;
    println!("esr: {:b} ec: {:b} iss: {:b}", esr_el1, ec, iss);
    panic!("el1_sp0_sync_handler triggered\n state dump \n{:x?}", state);
}
