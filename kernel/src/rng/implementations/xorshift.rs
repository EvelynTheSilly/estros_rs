use crate::rng::Rng;

pub struct XORShiftRng {
    state: u64,
}

impl XORShiftRng {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng for XORShiftRng {
    fn rand_u64(&mut self) -> u64 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.state = x;
        x
    }
}
