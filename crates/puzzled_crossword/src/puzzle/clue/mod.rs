mod clues;
mod direction;
mod id;
mod specification;

pub use clues::*;
pub use direction::*;
pub use id::*;
pub use specification::*;

use puzzled_core::Position;
use std::{cmp::Ordering, fmt};

use crate::Crossword;

/// Clue
///
/// The [`clue!`](crate::clue) macro provides a shorthand for creating clues
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Clue {
    // Specification
    text: String,
    direction: Direction,

    // Placement
    num: u8,
    start: Position,
    len: u8,
}

impl Clue {
    /// Construct a new clue from its [specification](ClueSpec) and placement within in the [puzzle](crate::Crossword) [grid](crate::Squares)
    ///
    /// # Panics
    /// Panics if `len == 0`, i.e. the clue should always occupy at least one [square](crate::Square)
    pub fn new<S: Into<String>>(
        num: u8,
        direction: Direction,
        text: S,
        start: Position,
        len: u8,
    ) -> Self {
        assert!(len > 0, "Clue should always occupy at least one square");

        Self {
            text: text.into(),
            num,
            direction,
            start,
            len,
        }
    }

    /// Returns an iterator over every [position](Position) that the clue covers in the [puzzle grid](crate::Squares)
    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.len).map(move |offset| match self.direction {
            Direction::Across => Position {
                row: self.start.row,
                col: self.start.col + offset as usize,
            },
            Direction::Down => Position {
                row: self.start.row + offset as usize,
                col: self.start.col,
            },
        })
    }

    /// Clue text
    pub fn text(&self) -> &String {
        &self.text
    }

    /// [Direction] of the clue within the puzzle
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Number of the clue within its associated [puzzle](Crossword)
    pub fn num(&self) -> u8 {
        self.num
    }

    /// Starting [position](Position) of the clue within a [puzzle](Crossword)
    pub fn start(&self) -> Position {
        self.start
    }

    /// Number of [cells](crate::Cell) the clue occupies within a [puzzle](Crossword)
    pub fn len(&self) -> u8 {
        self.len
    }

    /// Clue [specification](ClueSpec)
    pub fn spec(&self) -> ClueSpec {
        ClueSpec::new(self.direction, &self.text)
    }

    /// Clue [identifier](ClueId)
    pub fn id(&self) -> ClueId {
        (self.num, self.direction)
    }

    /// Verify whether the clue occupies any [cells](crate::Cell) within a [puzzle](Crossword)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl fmt::Display for Clue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}: \"{}\"", self.num, self.direction, self.text)
    }
}

impl Ord for Clue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num
            .cmp(&other.num)
            .then(self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for Clue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Crossword {
    pub fn insert_clues(&mut self, clues: impl IntoIterator<Item = ClueSpec>) -> Vec<ClueSpec> {
        let (positioned, unpositioned) = self.place_clues(clues);

        for clue in positioned {
            let id = (clue.num, clue.direction);
            self.clues.insert(id, clue);
        }

        unpositioned
    }

    pub fn place_clues(
        &self,
        clues: impl IntoIterator<Item = ClueSpec>,
    ) -> (Vec<Clue>, Vec<ClueSpec>) {
        // Split the clues into across and down
        let (across, down): (Vec<_>, Vec<_>) = clues
            .into_iter()
            .partition(|clue| clue.direction() == Direction::Across);

        let mut across_iter = across.into_iter();
        let mut down_iter = down.into_iter();

        // Determine all positions past the last entry
        let last = match self.clues.values().last() {
            Some(clue) => clue.clone(),
            None => Clue::default(),
        };
        let pos_iter: Vec<_> = self
            .squares
            .positions()
            .skip_while(|pos| *pos != last.start)
            .collect();

        // Keep track of positioned clues and their number
        let mut positioned = Vec::new();
        let mut num = last.num() + 1;

        for start in pos_iter {
            let mut started = false;

            // Try to position the clue directed across
            if let Some(clue) =
                self.try_clue_position(num, start, Direction::Across, &last, &mut across_iter)
            {
                positioned.push(clue);
                started = true;
            }

            // Try to position the clue directed down
            if let Some(clue) =
                self.try_clue_position(num, start, Direction::Down, &last, &mut down_iter)
            {
                positioned.push(clue);
                started = true;
            }

            // If successful, move to the next clue
            if started {
                num += 1;
            }
        }

        // Collect the remaining clues that could not be positioned within the puzzle
        let mut unpositioned: Vec<_> = across_iter.collect();
        unpositioned.extend(down_iter);

        (positioned, unpositioned)
    }

    fn try_clue_position(
        &self,
        num: u8,
        start: Position,
        direction: Direction,
        last: &Clue,
        iter: &mut impl Iterator<Item = ClueSpec>,
    ) -> Option<Clue> {
        use crate::SquaresExtension;

        // Cannot position clue at the same start as the last clue in the same direction
        if num > 1 && last.start == start && last.direction() == direction {
            return None;
        }

        // Cannot start the clue in the given direction from the given start
        if !self.squares.starts_in_dir(start, direction) {
            return None;
        }

        // Make sure there is a next clue to position
        let clue = iter.next()?;

        // Position the clue from the given start
        Some(Clue {
            num,
            direction,
            start,
            text: clue.text().clone(),
            len: self.squares.find_playable_len(start, direction),
        })
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct ClueData {
    text: String,
    start: Position,
    len: u8,
}
