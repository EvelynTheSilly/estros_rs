use crate::{
    rng::implementations::xorshift::XORShiftRng,
    syncronisation::{GlobalSharedLock, NullLock},
};
use no_panic::no_panic;

mod implementations;

pub trait Rng {
    fn rand_u64(&mut self) -> u64;
    #[no_panic]
    fn rand_i64(&mut self) -> i64 {
        u64::cast_signed(self.rand_u64())
    }
    #[no_panic]
    fn rand_u32(&mut self) -> u32 {
        u32::from_ne_bytes(self.rand_u64().to_ne_bytes()[..4].try_into().expect("
            SAFETY: shouldnt panic because the bytes are clamped to the size of the appropriate type known at compile time, shouldnt link to the panic handler in optimised code
        "))
    }
    #[no_panic]
    fn rand_i32(&mut self) -> i32 {
        u32::cast_signed(self.rand_u32())
    }
    #[no_panic]
    fn rand_u16(&mut self) -> u16 {
        u16::from_ne_bytes(self.rand_u32().to_ne_bytes()[..2].try_into().expect("
            SAFETY: shouldnt panic because the bytes are clamped to the size of the appropriate type known at compile time, shouldnt link to the panic handler in optimised code
        "))
    }
    #[no_panic]
    fn rand_i16(&mut self) -> i16 {
        u16::cast_signed(self.rand_u16())
    }
    #[no_panic]
    fn rand_u8(&mut self) -> u8 {
        u8::from_ne_bytes(self.rand_u16().to_ne_bytes()[..1].try_into().expect("
                SAFETY: shouldnt panic because the bytes are clamped to the size of the appropriate type known at compile time, shouldnt link to the panic handler in optimised code
            "))
    }
    #[no_panic]
    fn rand_i8(&mut self) -> i8 {
        u8::cast_signed(self.rand_u8())
    }
}

pub static RNG: GlobalSharedLock<XORShiftRng> = NullLock::new(XORShiftRng::new(69420));
