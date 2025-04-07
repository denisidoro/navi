use crate::prelude::*;

pub trait HasDeps {
    fn deps(&self) -> HashSet<TypeId> {
        HashSet::new()
    }
}
