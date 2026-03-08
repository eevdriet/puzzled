use std::{collections::BTreeMap, fmt};

use derive_more::{Deref, DerefMut};
use puzzled_core::{Offset, Position};

#[cfg(feature = "serde")]
use crate::SerdeClue;
use crate::{Clue, ClueDirection, ClueId};

/// Collection type of all [clues](Clue) in a [puzzle](crate::Crossword)
///
/// By using [`BTreeMap`] with a [`ClueId`] as key type, clues are easily traversed in order by number, then [`ClueDirection`].
#[derive(Debug, Default, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct Clues {
    #[deref]
    #[deref_mut]
    entries: BTreeMap<ClueId, Clue>,

    numbers: BTreeMap<Position, u8>,
    across: BTreeMap<Position, ClueId>,
    down: BTreeMap<Position, ClueId>,
}

impl Clues {
    pub fn new(entries: BTreeMap<ClueId, Clue>) -> Self {
        dbg!(&entries);
        let mut clues = Clues::default();

        for (id, clue) in entries {
            clues.insert_clue_positions(&id, &clue);
            clues.insert(id, clue);
        }

        clues
    }

    pub fn insert(&mut self, id: ClueId, clue: Clue) -> Option<Clue> {
        self.insert_clue_positions(&id, &clue);
        self.entries.insert(id, clue)
    }

    fn insert_clue_positions(&mut self, id: &ClueId, clue: &Clue) {
        // Insert the clue number at its start
        let mut pos = clue.start;
        self.numbers.insert(pos, id.num);

        // Determine which direction to insert the clue positions for
        let (clues, offset) = match id.direction {
            ClueDirection::Across => (&mut self.across, Offset::RIGHT),
            ClueDirection::Down => (&mut self.down, Offset::DOWN),
        };

        // Insert the clue identifier for each position of the clue
        for _ in 0..clue.len {
            clues.insert(pos, *id);
            pos += offset;
        }
    }

    pub fn get_clues(&self, pos: Position) -> Option<(&Clue, &Clue)> {
        let across = self.entries.get(self.across.get(&pos)?)?;
        let down = self.entries.get(self.down.get(&pos)?)?;

        Some((across, down))
    }

    pub fn get_num(&self, pos: Position) -> Option<u8> {
        self.numbers.get(&pos).cloned()
    }

    /// Returns an iterator over just the across entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::crossword::{crossword, clue, ClueDirection::*};
    ///
    /// let puzzle = crossword! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     - A: "To be able to"
    ///     - A: "The length of life"
    ///     - A: "Some stuff arranged in a line"
    ///     - D: "An automobile"
    ///     - D: "Past, gone, before now"
    ///     - D: "Not existing before"
    /// );
    /// let mut iter = puzzle.clues().iter_across();
    ///
    /// assert_eq!(iter.next(), Some(&clue!(1 A: "To be able to" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue!(4 A: "The length of life" @ (1, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue!(5 A: "Some stuff arranged in a line" @ (2, 0) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_across(&self) -> impl Iterator<Item = &Clue> {
        self.entries
            .values()
            .filter(|clue| matches!(clue.direction(), ClueDirection::Across))
    }

    /// Returns a mutable iterator over just the across entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::crossword::{crossword, clue, ClueDirection::*};
    ///
    /// let mut puzzle = crossword! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     - A: "To be able to"
    ///     - A: "The length of life"
    ///     - A: "Some stuff arranged in a line"
    ///     - D: "An automobile"
    ///     - D: "Past, gone, before now"
    ///     - D: "Not existing before"
    /// );
    /// let mut iter = puzzle.clues_mut().iter_across_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut clue!(1 A: "To be able to" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue!(4 A: "The length of life" @ (1, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue!(5 A: "Some stuff arranged in a line" @ (2, 0) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_across_mut(&mut self) -> impl Iterator<Item = &mut Clue> {
        self.entries
            .values_mut()
            .filter(|clue| matches!(clue.direction(), ClueDirection::Across))
    }

    /// Returns an iterator over just the down entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::crossword::{crossword, clue, ClueDirection::*};
    ///
    /// let puzzle = crossword! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     - D: "An automobile"
    ///     - D: "Past, gone, before now"
    ///     - D: "Not existing before"
    ///     - A: "To be able to"
    ///     - A: "The length of life"
    ///     - A: "Some stuff arranged in a line"
    /// );
    /// let mut iter = puzzle.clues().iter_down();
    ///
    /// assert_eq!(iter.next(), Some(&clue!(1 D: "An automobile" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue!(2 D: "Past, gone, before now" @ (0, 1) + 3)));
    /// assert_eq!(iter.next(), Some(&clue!(3 D: "Not existing before" @ (0, 2) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_down(&self) -> impl Iterator<Item = &Clue> {
        self.entries
            .values()
            .filter(|clue| matches!(clue.direction(), ClueDirection::Down))
    }

    /// Returns a mutable iterator over just the down entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::crossword::{crossword, clue, ClueDirection::*};
    ///
    /// let mut puzzle = crossword! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     - D: "An automobile"
    ///     - D: "Past, gone, before now"
    ///     - D: "Not existing before"
    ///     - A: "To be able to"
    ///     - A: "The length of life"
    ///     - A: "Some stuff arranged in a line"
    /// );
    /// let mut iter = puzzle.clues_mut().iter_down_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut clue!(1 D: "An automobile" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue!(2 D: "Past, gone, before now" @ (0, 1) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue!(3 D: "Not existing before" @ (0, 2) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_down_mut(&mut self) -> impl Iterator<Item = &mut Clue> {
        self.entries
            .values_mut()
            .filter(|clue| matches!(clue.direction(), ClueDirection::Down))
    }
}

impl fmt::Display for Clues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id, clue) in self.iter() {
            writeln!(f, "{id}: {}", clue.text())?;
        }

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl Clues {
    pub(crate) fn from_serde(data: SerdeClues) -> Result<Self, String> {
        let mut clues = BTreeMap::new();

        for (key, val) in data {
            use std::str::FromStr;

            // Try to parse the clue number and direction from the key
            let (num_str, dir_str) = key.split_once('-').ok_or(format!(
                "Key '{key}' must be formatted as '<num> : <dir>' where <dir> ::= A | D"
            ))?;
            let num: u8 = num_str
                .parse()
                .map_err(|_| format!("Expected number, found '{num_str}'"))?;
            let direction = ClueDirection::from_str(dir_str)?;

            // Then construct the clue and insert it into the clues
            let id: ClueId = (num, direction).into();
            let clue = Clue {
                num,
                direction,
                text: val.text,
                start: val.start,
                len: val.len,
            };

            clues.insert(id, clue);
        }

        Ok(Self::new(clues))
    }

    pub(crate) fn to_serde(&self) -> SerdeClues {
        self.iter()
            .map(|(id, clue)| {
                let val = SerdeClue {
                    text: clue.text().clone(),
                    start: clue.start,
                    len: clue.len,
                };

                (id.to_string(), val)
            })
            .collect()
    }
}

#[cfg(feature = "serde")]
pub(crate) type SerdeClues = BTreeMap<String, crate::SerdeClue>;
