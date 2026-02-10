use alloc::{string::String, vec::Vec};

pub struct StringsBlock {
    strings: Vec<String>,
}

impl StringsBlock {
    pub fn new(base: *mut u64, size: u32) {
        let byte: u32 = 0;
        let mut accumulator_string = String::new();
        while byte <= size {
            let char;
            unsafe { char = *(base as *const char) }
        }
    }
}
