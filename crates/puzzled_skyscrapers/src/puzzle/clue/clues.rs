use std::collections::BTreeMap;

use derive_more::{Deref, DerefMut};
use puzzled_core::Direction;

use crate::{Clue, ClueId};

#[derive(Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Clues {
    #[deref]
    #[deref_mut]
    clues: BTreeMap<ClueId, Clue>,

    rows: usize,
    cols: usize,
}

impl Clues {
    pub fn new(clues: BTreeMap<ClueId, Clue>, rows: usize, cols: usize) -> Self {
        Self { clues, rows, cols }
    }

    pub fn iter_left(&self) -> impl Iterator<Item = (&ClueId, &Clue)> {
        self.iter_direction(&Direction::Left)
    }

    pub fn iter_right(&self) -> impl Iterator<Item = (&ClueId, &Clue)> {
        self.iter_direction(&Direction::Left)
    }

    pub fn iter_up(&self) -> impl Iterator<Item = (&ClueId, &Clue)> {
        self.iter_direction(&Direction::Up)
    }

    pub fn iter_down(&self) -> impl Iterator<Item = (&ClueId, &Clue)> {
        self.iter_direction(&Direction::Down)
    }

    fn iter_direction(&self, direction: &Direction) -> impl Iterator<Item = (&ClueId, &Clue)> {
        self.iter().filter(|(id, _)| id.direction == *direction)
    }
}
