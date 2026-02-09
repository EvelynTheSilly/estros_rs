use anyhow::Result;

pub struct Dtb {}

#[repr(C, packed)]
#[derive(Clone)]
struct DtbHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_structs: u32,
}

impl DtbHeader {
    pub unsafe fn new(base: *mut u8) -> DtbHeader {
        unsafe { (*(base as *const DtbHeader)).clone() }
    }
    pub fn is_sane() -> bool {
        false
    }
}

impl Dtb {
    pub fn new(base: *mut u8) -> Result<Dtb> {
        Ok(Dtb {})
    }
}
