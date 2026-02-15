mod parse;
pub use parse::*;

mod write;
pub use write::PuzWriter;

mod checksums;
pub use checksums::*;

pub(crate) const SECTION_SEPARATOR: &str = "---";
pub(crate) const FILE_MAGIC: &str = "ACROSS&DOWN\0";

use std::ops::Range;
pub(crate) type Span = Range<usize>;
