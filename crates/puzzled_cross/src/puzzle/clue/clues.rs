use std::{collections::BTreeMap, ops};

#[cfg(feature = "serde")]
use crate::ClueData;
use crate::{Clue, ClueId, Direction};

/// Collection type of all [clues](Clue) in a [puzzle](crate::Crossword)
///
/// By using [`BTreeMap`] with a [`ClueId`] as key type, clues are easily traversed in order by number, then [`Direction`].
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Clues(BTreeMap<ClueId, Clue>);

impl Clues {
    pub fn new(entries: BTreeMap<ClueId, Clue>) -> Self {
        Self(entries)
    }

    /// Returns an iterator over just the across entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled_crossword::{crossword, clue, Direction::*};
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
        self.0
            .values()
            .filter(|clue| matches!(clue.direction(), Direction::Across))
    }

    /// Returns a mutable iterator over just the across entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled_crossword::{crossword, clue, Direction::*};
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
        self.0
            .values_mut()
            .filter(|clue| matches!(clue.direction(), Direction::Across))
    }

    /// Returns an iterator over just the down entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled_crossword::{crossword, clue, Direction::*};
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
        self.0
            .values()
            .filter(|clue| matches!(clue.direction(), Direction::Down))
    }

    /// Returns a mutable iterator over just the down entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled_crossword::{crossword, clue, Direction::*};
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
        self.0
            .values_mut()
            .filter(|clue| matches!(clue.direction(), Direction::Down))
    }
}

#[cfg(feature = "serde")]
impl Clues {
    pub(crate) fn from_data(data: CluesData) -> Result<Self, String> {
        let mut clues = BTreeMap::new();

        for (key, val) in data {
            use std::str::FromStr;

            use puzzled_core::Position;

            // Try to parse the clue number and direction from the key
            let (num_str, dir_str) = key.split_once('-').ok_or(format!(
                "Key '{key}' must be formatted as '<num> : <dir>' where <dir> ::= A | D"
            ))?;
            let num: u8 = num_str
                .parse()
                .map_err(|_| format!("Expected number, found '{num_str}'"))?;
            let direction = Direction::from_str(dir_str)?;

            // Then construct the clue and insert it into the clues
            let id: ClueId = (num, direction);
            let clue = Clue {
                num,
                direction,
                text: val.text,
                start: val.start,
                len: val.len,
            };

            clues.insert(id, clue);
        }

        Ok(Self(clues))
    }

    pub(crate) fn to_data(&self) -> CluesData {
        self.iter()
            .map(|((num, dir), clue)| {
                let key = format!("{num}-{dir}");
                let val = ClueData {
                    start: clue.start,
                    len: clue.len,
                    text: clue.text().clone(),
                };

                (key, val)
            })
            .collect()
    }
}

impl ops::Deref for Clues {
    type Target = BTreeMap<ClueId, Clue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Clues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "serde")]
pub(crate) type CluesData = BTreeMap<String, crate::ClueData>;
