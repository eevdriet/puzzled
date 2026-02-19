use std::fmt;

use crate::{CellStyle, Solution};

/// Playable [square](crate::Square) that the user can enter their [solution](Solution) into
///
/// This is the main structure for interacting with the puzzle after it has been constructed.
/// User can interact with a square in the following ways:
/// - [`enter`](Self::enter) a new guess for the solution
/// - [`clear`](Self::reveal) the current guess
/// - [`reveal`](Self::reveal) what the solution is by automatically entering it
///
/// When calling these methods, the square [style](CellStyle) is updated to match the current correctness.
/// The correctness of the entry can be checked with [`is_correct`](Self::is_correct)
/// ```
/// use puzzled_crossword::{cell, Cell, Solution, CellStyle};
///
/// // Cell creation
/// let mut letter = cell!('A');
/// assert!(letter.is_letter());
///
/// let mut rebus = Cell::new_styled(Solution::Rebus("Cats".to_string()), CellStyle::CIRCLED);
/// assert!(rebus.is_rebus());
///
/// // Solving
/// letter.enter('A');
/// assert!(!letter.was_incorrect());
/// assert!(!letter.is_incorrect());
/// assert!(letter.is_correct());
///
/// letter.enter('B');
/// assert!(!letter.was_incorrect());
/// assert!(letter.is_incorrect());
/// assert!(!letter.is_correct());
///
/// // Style
/// assert!(rebus.is_circled());
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
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

        // Check whether the cell was previously incorrect
        if self.entry.is_some() && !self.is_correct() {
            self.style |= CellStyle::PREVIOUSLY_INCORRECT;
        }

        // Enter the new guess
        self.entry = Some(guess.into());

        // Check whether the cell is currently incorrect
        if !self.is_correct() {
            self.style |= CellStyle::INCORRECT;
        }
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
