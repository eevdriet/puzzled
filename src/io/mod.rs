mod checksums;
mod parse;
mod strings;
mod write;

pub(crate) use checksums::*;
pub(crate) use strings::*;

pub use parse::*;
pub use write::PuzWriter;

pub(crate) const SECTION_SEPARATOR: &str = "---";
pub(crate) const FILE_MAGIC: &str = "ACROSS&DOWN\0";

use std::ops::Range;

use crate::{Puzzle, io::write::PuzWrite};
pub(crate) type Span = Range<usize>;
