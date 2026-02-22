#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod format;
pub use format::StringError;

mod color;
mod grid;
mod line;
mod macros;
mod metadata;
mod offset;
mod order;
mod position;
mod style;
mod timer;
mod version;

pub use grid::*;
pub use line::*;
pub use metadata::*;
pub use offset::*;
pub use order::*;
pub use position::*;
pub use style::*;

pub use color::{Color, ColorId, Error as ColorError};
pub use timer::{Error as TimerError, Timer, TimerState};
pub use version::{Error as VersionError, Version};

pub(crate) fn clamped_add(lhs: usize, rhs: isize) -> usize {
    (lhs as isize).saturating_add(rhs).clamp(0, isize::MAX) as usize
}
