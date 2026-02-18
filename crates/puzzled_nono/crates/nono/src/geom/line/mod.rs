mod mask;
mod pos;

pub use mask::*;
pub use pos::*;

use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Line {
    Row(u16),
    Col(u16),
}

pub type LineMap<T> = HashMap<Line, T>;

impl Line {
    pub fn line(&self) -> u16 {
        match self {
            Self::Row(row) => *row,
            Self::Col(col) => *col,
        }
    }
}
