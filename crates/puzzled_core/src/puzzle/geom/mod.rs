mod direction;
mod grid;
mod lattice;
mod line;
mod offset;
mod order;
mod position;

pub use direction::*;
pub use grid::*;
pub use lattice::*;
pub use line::*;
pub use offset::*;
pub use order::*;
pub use position::*;

pub(crate) fn clamped_add(lhs: usize, rhs: isize) -> usize {
    (lhs as isize).saturating_add(rhs).clamp(0, isize::MAX) as usize
}
