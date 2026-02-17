mod cell;
mod solution;
mod style;

use std::fmt;

pub use {cell::*, solution::*, style::*};

use crate::Grid;

/// Square that is placed in the [puzzle](crate::Puzzle) [grid](crate::Grid).
///
/// Squares come in two forms: [black](Square::Black) squares that do not contain user entries and [white](Square::White) ones that do.
/// Note that the latter are also called [cells](Cell) to make it clear that the square is playable.
///
/// To easily construct a cell with a given [solution](Solution), you can make use of [`Square::letter`] and [`Square::rebus`].
/// These constructors create new white squares that have no [style](CellStyle) and no initial user entry, which is usually desired.
/// The [`square!`](crate::square) macro achieves the same.
///
/// Another common task is [revealing](Cell::reveal) a square without having to check whether it is white square -- black squares have no notion of being revealed.
/// The [`reveal`](Square::reveal) and [`is_revealed`](Square::is_revealed) methods provide an indirection for squares to do this.
/// ```
/// use puzzled::{cell, Cell, square, Square, Solution::*};
///
/// // Creating cells from squares
/// let letter = square!('L');
/// assert_eq!(letter, Square::White(cell!('L')));
///
/// let mut rebus = square!("Rebus");
/// assert_eq!(rebus, Square::White(cell!("Rebus")));
///
/// // Revealing squares
/// assert!(!rebus.is_revealed());
///
/// rebus.reveal();
/// assert!(rebus.is_revealed());
/// ```
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Square {
    /// Non-playable square that defines the borders of the [puzzle](crate::Puzzle) [grid](crate::Grid)
    #[default]
    Black,

    /// Playable [cell](Cell) that can be entered and holds a [solution](Solution) to verify a [puzzle](crate::Puzzle) with
    White(Cell),
}

impl Square {
    /// Construct a filled square that has a [one-letter solution](Solution::Letter)
    pub fn letter(letter: char) -> Self {
        let fill = Cell::new(Solution::Letter(letter));
        Self::White(fill)
    }

    /// Construct a filled square that has a [rebus solution](Solution::Rebus)
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

/// Collection type of all [squares](Square) in a [puzzle](crate::Puzzle)
///
/// A [grid](Grid) is used to represent the squares.
pub type Squares = Grid<Square>;
