use crate::prelude::*;

pub trait Component: Any + AsAny + Send + Sync {}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
