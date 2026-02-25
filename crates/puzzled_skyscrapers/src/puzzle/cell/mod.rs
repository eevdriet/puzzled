mod cells;
mod direction;

pub use cells::*;
use derive_more::{Deref, DerefMut};
pub use direction::*;

use puzzled_core::Entry;

#[derive(Debug, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct SkyscraperCell(pub(crate) Entry<u8>);
