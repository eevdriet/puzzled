use std::str::FromStr;

use puzzled_core::Cell;

use crate::text::{TxtState, read};

impl<'a> TxtState<'a> {
    pub fn read_cell<T>(&mut self) -> read::Result<Cell<T>>
    where
        T: FromStr,
    {
        let sol_end = 
    }
}
