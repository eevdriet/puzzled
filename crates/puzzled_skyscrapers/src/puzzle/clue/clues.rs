use std::collections::BTreeMap;

use derive_more::{Deref, DerefMut};
use puzzled_core::Line;

use crate::ClueId;

#[derive(Debug, Default, Deref, DerefMut, PartialEq, Eq)]
pub struct Clues(pub(crate) BTreeMap<ClueId, usize>);

impl Clues {
    pub fn new(clues: BTreeMap<ClueId, usize>) -> Self {
        Self(clues)
    }
}
