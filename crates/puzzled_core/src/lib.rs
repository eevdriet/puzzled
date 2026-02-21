#![cfg_attr(docsrs, feature(doc_cfg))]

mod grid;
mod line;
mod macros;
mod offset;
mod order;
mod position;
mod style;
mod timer;
mod version;

pub use grid::*;
pub use line::*;
pub use offset::*;
pub use order::*;
pub use position::*;
pub use style::*;
pub use timer::*;
pub use version::*;

pub(crate) fn clamped_add(lhs: usize, rhs: isize) -> usize {
    (lhs as isize).saturating_add(rhs).clamp(0, isize::MAX) as usize
}
