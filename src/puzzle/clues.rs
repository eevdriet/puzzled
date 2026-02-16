use std::{cmp::Ordering, collections::BTreeMap, fmt, ops};

use crate::{Direction, Position, Puzzle};

/// Type that identifies where a [clue](Clue) is placed within a [puzzle](Puzzle)
///
/// The identifier mimics the way clues are commonly identified in real crosswords.
/// For example, "4 across" can be specified as `(4, Direction::Across)`.
pub type ClueId = (u8, Direction);

/// Collection type of all [clues](Clue) in a [puzzle](Puzzle)
///
/// By using [`BTreeMap`] with a [`ClueId`] as key type, clues are easily traversed in order by number, then [`Direction`].
pub type Clues = BTreeMap<ClueId, Clue>;

/// Specification for how to add a [clue](Clue) to a [puzzle](Puzzle).
///
/// This struct can be used when the user is unsure what [squares](crate::Square) the clue should correspond to.
/// By calling [`Puzzle::position_clues`], the specs are turned into [clues](Clue) by positioning them from the next available square in the puzzle.
/// Furthermore, [`Puzzle::insert_clues`] can be used to add the clues to the puzzle after positioning them.
#[derive(Debug, Clone)]
pub struct ClueSpec {
    text: String,
    direction: Direction,
}

impl ClueSpec {
    /// Specify a clue from its [direction](Direction) and text
    pub fn new<S: Into<String>>(direction: Direction, text: S) -> Self {
        Self {
            direction,
            text: text.into(),
        }
    }

    /// Specify a [across](Direction::Across) clue
    pub fn across<S: Into<String>>(text: S) -> Self {
        Self::new(Direction::Across, text.into())
    }

    /// Specify a [down](Direction::Down) clue
    pub fn down<S: Into<String>>(text: S) -> Self {
        Self::new(Direction::Down, text.into())
    }

    /// Clue text
    pub fn text(&self) -> &String {
        &self.text
    }

    /// [Direction] in which the clue should be placed in a [puzzle](Puzzle)
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Construct a [clue](Clue) from its specification and placement
    pub fn place(self, num: u8, start: Position, len: u8) -> Clue {
        Clue {
            num,
            start,
            len,
            text: self.text,
            direction: self.direction,
        }
    }
}

/// Clue
///
///
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
    /// Construct a new clue from its [specification](ClueSpec) and placement within in the [puzzle](crate::Puzzle) [grid](crate::Squares)
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
                col: self.start.col + offset,
            },
            Direction::Down => Position {
                row: self.start.row + offset,
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

    /// Number of the clue within its associated [puzzle](Puzzle)
    pub fn num(&self) -> u8 {
        self.num
    }

    /// Starting [position](Position) of the clue within a [puzzle](Puzzle)
    pub fn start(&self) -> Position {
        self.start
    }

    /// Number of [cells](crate::Cell) the clue occupies within a [puzzle](Puzzle)
    pub fn len(&self) -> u8 {
        self.len
    }

    /// Verify whether the clue occupies any [cells](crate::Cell) within a [puzzle](Puzzle)
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

impl Puzzle {
    pub fn clues(&self) -> &Clues {
        &self.clues
    }

    pub fn insert_clues(&mut self, clues: impl IntoIterator<Item = ClueSpec>) -> Vec<ClueSpec> {
        let (positioned, unpositioned) = self.position_clues(clues);

        for clue in positioned {
            let id = (clue.num, clue.direction);
            self.clues.insert(id, clue);
        }

        unpositioned
    }

    pub fn position_clues(
        &self,
        clues: impl IntoIterator<Item = ClueSpec>,
    ) -> (Vec<Clue>, Vec<ClueSpec>) {
        // Split the clues into across and down
        let (across, down): (Vec<_>, Vec<_>) = clues
            .into_iter()
            .partition(|clue| clue.direction == Direction::Across);

        let mut across_iter = across.iter();
        let mut down_iter = down.iter();

        // Determine all positions past the last entry
        let last = match self.iter_clues().last() {
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
                self.try_clue_at_position(num, start, Direction::Across, &last, &mut across_iter)
            {
                positioned.push(clue);
                started = true;
            }

            // Try to position the clue directed down
            if let Some(clue) =
                self.try_clue_at_position(num, start, Direction::Down, &last, &mut down_iter)
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
        let mut unpositioned: Vec<_> = across_iter.cloned().collect();
        unpositioned.extend(down_iter.cloned());

        (positioned, unpositioned)
    }

    fn try_clue_at_position<'a>(
        &self,
        num: u8,
        start: Position,
        direction: Direction,
        last: &Clue,
        iter: &mut impl Iterator<Item = &'a ClueSpec>,
    ) -> Option<Clue> {
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
        let direction = clue.direction;
        let text = clue.text.clone();
        let len = self.squares.find_playable_len(start, direction);

        Some(Clue {
            num,
            direction,
            start,
            text,
            len,
        })
    }
}

/// # Puzzle clues
impl Puzzle {
    /// Get a reference to the [clue](Clue) that is identified
    /// [`Some(Clue)`](Option::Some) is returned if the identifier is valid, otherwise [`None`].
    pub fn get_clue(&self, id: ClueId) -> Option<&Clue> {
        self.clues.get(&id)
    }

    /// Get a mutable reference to the [clue](Clue) that is identified
    /// [`Some(Clue)`](Option::Some) is returned if the identifier is valid, otherwise [`None`].
    pub fn get_clue_mut(&mut self, id: ClueId) -> Option<&mut Clue> {
        self.clues.get_mut(&id)
    }

    /// Returns an iterator over the entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::{puzzle, clue_at};
    ///
    /// let puzzle = puzzle! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     ---
    ///     A: "To be able to",
    ///     D: "An automobile",
    ///     D: "Past, gone, before now",
    ///     D: "Not existing before",
    ///     A: "The length of life",
    ///     A: "Some stuff arranged in a line",
    /// );
    /// let mut iter = puzzle.iter_clues();
    ///
    /// assert_eq!(iter.next(), Some(&clue_at!(1 A: "To be able to" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(1 D: "An automobile" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(2 D: "Past, gone, before now" @ (0, 1) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(3 D: "Not existing before" @ (0, 2) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(4 A: "The length of life" @ (1, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(5 A: "Some stuff arranged in a line" @ (2, 0) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_clues(&self) -> impl Iterator<Item = &Clue> {
        self.clues.values()
    }

    /// Returns a mutable iterator over the entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::{puzzle, clue_at};
    ///
    /// let mut puzzle = puzzle! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     ---
    ///     A: "To be able to",
    ///     D: "An automobile",
    ///     D: "Past, gone, before now",
    ///     D: "Not existing before",
    ///     A: "The length of life",
    ///     A: "Some stuff arranged in a line",
    /// );
    /// let mut iter = puzzle.iter_clues_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut clue_at!(1 A: "To be able to" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(1 D: "An automobile" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(2 D: "Past, gone, before now" @ (0, 1) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(3 D: "Not existing before" @ (0, 2) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(4 A: "The length of life" @ (1, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(5 A: "Some stuff arranged in a line" @ (2, 0) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_clues_mut(&mut self) -> impl Iterator<Item = &mut Clue> {
        self.clues.values_mut()
    }

    /// Returns an iterator over just the across entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::{puzzle, clue_at, Direction::*};
    ///
    /// let puzzle = puzzle! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     ---
    ///     A: "To be able to",
    ///     A: "The length of life",
    ///     A: "Some stuff arranged in a line",
    ///     D: "An automobile",
    ///     D: "Past, gone, before now",
    ///     D: "Not existing before",
    /// );
    /// let mut iter = puzzle.iter_across();
    ///
    /// assert_eq!(iter.next(), Some(&clue_at!(1 A: "To be able to" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(4 A: "The length of life" @ (1, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(5 A: "Some stuff arranged in a line" @ (2, 0) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_across(&self) -> impl Iterator<Item = &Clue> {
        self.clues
            .values()
            .filter(|clue| matches!(clue.direction(), Direction::Across))
    }

    /// Returns a mutable iterator over just the across entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::{puzzle, clue_at, Direction::*};
    ///
    /// let mut puzzle = puzzle! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     ---
    ///     A: "To be able to",
    ///     A: "The length of life",
    ///     A: "Some stuff arranged in a line",
    ///     D: "An automobile",
    ///     D: "Past, gone, before now",
    ///     D: "Not existing before",
    /// );
    /// let mut iter = puzzle.iter_across_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut clue_at!(1 A: "To be able to" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(4 A: "The length of life" @ (1, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(5 A: "Some stuff arranged in a line" @ (2, 0) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_across_mut(&mut self) -> impl Iterator<Item = &mut Clue> {
        self.clues
            .values_mut()
            .filter(|clue| matches!(clue.direction(), Direction::Across))
    }

    /// Returns an iterator over just the down entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::{puzzle, clue_at, Direction::*};
    ///
    /// let puzzle = puzzle! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     ---
    ///     D: "An automobile",
    ///     D: "Past, gone, before now",
    ///     D: "Not existing before",
    ///     A: "To be able to",
    ///     A: "The length of life",
    ///     A: "Some stuff arranged in a line",
    /// );
    /// let mut iter = puzzle.iter_down();
    ///
    /// assert_eq!(iter.next(), Some(&clue_at!(1 D: "An automobile" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(2 D: "Past, gone, before now" @ (0, 1) + 3)));
    /// assert_eq!(iter.next(), Some(&clue_at!(3 D: "Not existing before" @ (0, 2) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_down(&self) -> impl Iterator<Item = &Clue> {
        self.clues
            .values()
            .filter(|clue| matches!(clue.direction(), Direction::Down))
    }

    /// Returns a mutable iterator over just the down entries of the puzzle.
    /// The order is defined by the [`Ord`] implementation on [`Clue`].
    /// ```
    /// use puzzled::{puzzle, clue_at, Direction::*};
    ///
    /// let mut puzzle = puzzle! (
    ///     [C A N]
    ///     [A G E]
    ///     [R O W]
    ///     ---
    ///     D: "An automobile",
    ///     D: "Past, gone, before now",
    ///     D: "Not existing before",
    ///     A: "To be able to",
    ///     A: "The length of life",
    ///     A: "Some stuff arranged in a line",
    /// );
    /// let mut iter = puzzle.iter_down_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut clue_at!(1 D: "An automobile" @ (0, 0) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(2 D: "Past, gone, before now" @ (0, 1) + 3)));
    /// assert_eq!(iter.next(), Some(&mut clue_at!(3 D: "Not existing before" @ (0, 2) + 3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_down_mut(&mut self) -> impl Iterator<Item = &mut Clue> {
        self.clues
            .values_mut()
            .filter(|clue| matches!(clue.direction(), Direction::Down))
    }
}

impl ops::Index<ClueId> for Puzzle {
    type Output = Clue;

    /// Index the puzzle to retrieve a reference to the square at the given position.
    /// ```
    /// use puzzled::{clue_at, Direction::*, puzzle};
    ///
    /// let puzzle = puzzle! (
    ///     [A B]
    ///     [C .]
    ///     ---
    ///     A: "AB",
    ///     D: "AC",
    ///     D: "B",
    ///     A: "C",
    /// );
    ///
    /// assert_eq!(puzzle[(1, Across)], clue_at!(1 A: "AB" @ (0, 0) + 2));
    /// assert_eq!(puzzle[(1, Down)], clue_at!(1 D: "AC" @ (0, 0) + 2));
    /// assert_eq!(puzzle[(2, Down)], clue_at!(2 D: "B" @ (0, 1) + 1));
    /// assert_eq!(puzzle[(3, Across)], clue_at!(3 A: "C" @ (1, 0) + 1));
    /// ```
    ///
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled::{Direction::*, puzzle};
    ///
    /// let puzzle = puzzle! (
    ///    [A B]
    ///    [C D]
    /// );
    ///
    /// let clue = &puzzle[(10, Across)];
    /// ```
    fn index(&self, id: ClueId) -> &Self::Output {
        &self.clues[&id]
    }
}
