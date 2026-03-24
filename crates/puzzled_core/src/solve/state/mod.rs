mod entry;
mod grid;
mod lattice;

use std::fmt::{self, Display};

pub use entry::*;
pub use grid::*;

use crate::MISSING_ENTRY_CHAR;

pub struct SolutionEntry<'a, T> {
    pub solution: &'a Option<T>,
    pub entry: &'a Entry<T>,
}

impl<'a, T> SolutionEntry<'a, T> {
    pub fn get(&self) -> Option<&T> {
        if self.entry.is_initially_revealed() || self.entry.is_revealed() {
            return self.solution.as_ref();
        }

        self.entry.entry()
    }
}

impl<'a, T> fmt::Display for SolutionEntry<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = self.entry.style();

        let display = match (self.solution, self.entry.entry()) {
            (None, None) => MISSING_ENTRY_CHAR.to_string(),

            (Some(s), None) => format!("{s}{style}"),
            (None, Some(e)) => format!("({e}{style})"),
            (Some(s), Some(e)) => format!("{s}{style} ({e})"),
        };

        write!(f, "{display}")
    }
}
