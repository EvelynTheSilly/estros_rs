fn main() {
    println!("cargo:rustc-link-arg=-Tuser/minimal_init/linker.ld");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=linker.ld");
}
