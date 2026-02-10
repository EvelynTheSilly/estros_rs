use crate::dtb::header::DtbHeader;
use crate::dtb::memory_reservation_block::MemoryReservationBlock;
use crate::println;
use anyhow::Result;

mod header;
mod memory_reservation_block;
mod strings_block;

pub struct Dtb {}

impl Dtb {
    pub fn new(base: *mut u64) -> Result<Dtb> {
        let header = unsafe { DtbHeader::new(base) };
        println!("header {:?}", header);
        // header can be concidered sane after this
        header.is_sane()?;
        let mem_base;
        let struct_base;
        let strings_base;
        unsafe {
            mem_base = base.byte_add(header.off_mem_rsvmap as usize);
            struct_base = base.byte_add(header.off_dt_struct as usize);
            strings_base = base.byte_add(header.off_dt_strings as usize);
        }
        let mem_res_block = MemoryReservationBlock::new(mem_base);
        println!("mem res block {:X?}", mem_res_block);
        Ok(Dtb {})
    }
}
