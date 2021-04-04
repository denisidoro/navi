use std::hash::{Hash, Hasher};

const MAGIC_INIT: u64 = 0x811C_9DC5;

pub fn fnv<T: Hash>(x: &T) -> u64 {
    let mut hasher = FnvHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

struct FnvHasher(u64);

impl FnvHasher {
    fn new() -> Self {
        FnvHasher(MAGIC_INIT)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
    }
}
