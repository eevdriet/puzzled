use puzzled_core::{Grid, Value};

use crate::Bit;

pub trait Bits {
    fn iter_ones(&self) -> impl Iterator<Item = &Bit>;
}

impl<T> Bits for Grid<T>
where
    T: Value<Bit>,
{
    fn iter_ones(&self) -> impl Iterator<Item = &Bit> {
        std::iter::empty()
    }
}
