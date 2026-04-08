use std::hash::{DefaultHasher, Hash, Hasher};

pub fn fnv<T: Hash>(x: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}
