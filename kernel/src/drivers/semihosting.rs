#[cfg(feature = "qemu")]
pub fn shutdown(code: u32) {
    use crate::println;
    use qemu_exit::QEMUExit;
    println!("shutting down");
    qemu_exit::AArch64::new().exit(code);
}
