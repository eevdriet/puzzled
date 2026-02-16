mod style;

use std::fmt;

pub use style::*;

use crate::Grid;

/// Square that is placed in the puzzle grid.
///
/// Squares come in two forms: [black](Square::Black) squares that do not contain user entries and [white](Square::White) ones that do.
/// Note that the latter are also called [cells](Cell) to make it clear that the square is playable.
///
/// To easily construct a [square](Square), you can make use of [`Square::Black`], [`Square::letter`] and [`Square::rebus`].
/// These constructors create new squares that have no [style](CellStyle) and no initial user entry, which is usually desired.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Square {
    /// Non-playable square that defines the borders of the puzzle grid
    #[default]
    Black,

    /// Playable square that can be entered and holds a solution to verify the puzzle with
    White(Cell),
}

impl Square {
    /// Construct a filled square that has a one-letter solution
    pub fn letter(letter: char) -> Self {
        let fill = Cell::new(Solution::Letter(letter));
        Self::White(fill)
    }

    /// Construct a filled square that has a rebus solution
    pub fn rebus(rebus: String) -> Self {
        let fill = Cell::new(Solution::Rebus(rebus));
        Self::White(fill)
    }

    /// Convenience method to [reveal](Cell::reveal) a square, regardless of whether it's playable.
    ///
    /// Note that this is a no-op for [black](Square::Black) squares
    pub fn reveal(&mut self) {
        match self {
            Square::Black => {}
            Square::White(square) => square.reveal(),
        }
    }

    /// Convenience method to verify whether a square is [revealed](Cell::is_revealed).
    ///
    /// This is trivially true for [black](Square::Black) square
    pub fn is_revealed(&self) -> bool {
        match self {
            Square::Black => true,
            Square::White(square) => square.is_revealed(),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "."),
            Self::White(fill) => write!(f, "{fill}"),
        }
    }
}

pub type Squares = Grid<Square>;

/// Playable [square](Square) that the user can enter their [solution](Solution) into
///
/// This is the main structure for interacting with the puzzle after it has been constructed.
/// User can interact with a square in the following ways:
/// - [`enter`](Self::enter) a new guess for the solution
/// - [`clear`](Self::reveal) the current guess
/// - [`reveal`](Self::reveal) what the solution is by automatically entering it
///
/// When calling these methods, the square [style](CellStyle) is updated to match the current correctness.
/// The correctness of the entry can be checked with [`is_correct`](Self::is_correct)
#[derive(Debug)]
pub struct Cell {
    solution: Solution,
    entry: Option<String>,
    style: CellStyle,
}

impl Cell {
    /// Construct a cell from its [solution](Solution)
    pub fn new(solution: Solution) -> Self {
        Self::new_styled(solution, CellStyle::default())
    }

    /// Construct a cell from its [solution](Solution) and intial [style](CellStyle).
    /// Note that the style can only be modified through the methods mentioned above
    pub fn new_styled(solution: Solution, style: CellStyle) -> Self {
        Self {
            solution,
            style,
            entry: None,
        }
    }

    /// Retrieve the solution of the cell
    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    /// Retrieve the current entry in the cell
    pub fn entry(&self) -> &Option<String> {
        &self.entry
    }

    /// Retrieve the current style of the cell
    pub fn style(&self) -> CellStyle {
        self.style
    }

    /// Verify whether the solution to the cell is a letter
    pub fn is_letter(&self) -> bool {
        matches!(self.solution, Solution::Letter(_))
    }

    /// Verify whether the solution to the cell is a rebus
    pub fn is_rebus(&self) -> bool {
        matches!(self.solution, Solution::Rebus(_))
    }

    /// Reveal the square by manually entering its solution.
    /// This sets its [style](CellStyle) to be [revealed](CellStyle::REVEALED)
    pub fn reveal(&mut self) {
        self.style |= CellStyle::REVEALED;
        self.entry = Some(self.solution.clone().to_string())
    }

    /// Enter a new guess to solve the cell
    /// This updates the cell [style](CellStyle) based on the [current](CellStyle::INCORRECT) and [previous](CellStyle::PREVIOUSLY_INCORRECT) correctness.
    pub fn enter<S: Into<String>>(&mut self, guess: S) {
        // Never overwrite revealed solution
        if self.is_revealed() {
            return;
        }

        // Update the style based on current and previous correctness
        let is_correct = self.is_correct();
        if is_correct && self.is_incorrect() {
            self.style |= CellStyle::PREVIOUSLY_INCORRECT;
        }
        if !is_correct {
            self.style |= CellStyle::INCORRECT;
        }

        // Enter the new guess
        self.entry = Some(guess.into());
    }

    /// Clear the current entry.
    /// Note that this does not apply to revealed solutions
    pub fn clear(&mut self) {
        if !self.is_revealed() {
            self.entry = None
        }
    }

    /// Verify whether the current entry solves the square
    pub fn is_correct(&self) -> bool {
        match (&self.solution, &self.entry) {
            // Empty entries are always false
            (_, None) => false,

            (Solution::Rebus(rebus), Some(word)) => rebus == word,

            (Solution::Letter(letter), Some(word)) => {
                let mut chars = word.chars();
                chars.next() == Some(*letter) && chars.next().is_none()
            }
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.solution == other.solution
    }
}

impl Eq for Cell {}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.solution)
    }
}

/// Solution to a [square](Square) that can be used to verify its correctness
///
/// In almost all cases, solutions consist of a single [letter](Self::Letter).
/// However, users may define a [rebus](Self::Rebus) to construct a multi-letter solution.
/// In `*.puz` files, rebuses are defined from the [GRBS and RTBL sections](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Solution {
    /// One-letter solution
    Letter(char),

    /// Multiple-letter solution, a.k.a. a rebus
    Rebus(String),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(letter) => write!(f, "{letter}"),
            Self::Rebus(rebus) => write!(f, "{rebus}"),
        }
    }
}
