#![cfg_attr(docsrs, feature(doc_cfg))]

// Puzzle
mod puzzle;

use std::fmt::{self, Display};

#[doc(inline)]
pub use puzzle::*;

// Solver
mod solve;

#[doc(inline)]
pub use solve::*;

// Macros
#[cfg(feature = "macros")]
mod macros;

pub trait Value<T> {
    fn value(&self) -> Option<&T>;
    fn value_mut(&mut self) -> Option<&mut T>;
}

impl<T> Value<T> for Option<T> {
    fn value(&self) -> Option<&T> {
        self.as_ref()
    }

    fn value_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }
}

impl<T> fmt::Display for dyn Value<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value() {
            None => write!(f, "{MISSING_ENTRY_CHAR}"),
            Some(val) => write!(f, "{val}"),
        }
    }
}
