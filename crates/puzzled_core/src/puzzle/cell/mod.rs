use std::fmt::{self, Display};

mod style;

pub use style::CellStyle;

/// Playable square that the user can enter their solution into
///
/// This is the main structure for interacting with a puzzle after it has been constructed.
/// User can interact with a cell in the following ways:
/// - [`enter`](Self::enter) a new guess for the solution
/// - [`clear`](Self::clear) the current guess and put back the initial entry
/// - [`reveal`](Self::reveal) what the solution is by automatically entering it
///
/// When calling these methods, the square [style](CellStyle) is updated to match the current correctness.
/// The correctness of the entry can be checked with [`is_correct`](Self::is_correct)
/// ```
/// use puzzled::core::{cell, Cell, CellStyle};
///
/// // Cell creation
/// let mut number = cell!(100@);
/// let number2 = Cell::new_styled(100, CellStyle::CIRCLED);
/// assert_eq!(number, number2);
///
/// let char = cell!('A' ('B'));
/// let mut char2 = Cell::new('A');
/// char2.enter('B');
/// assert_eq!(char, char2);
///
/// // Solving
/// assert!(!number.was_incorrect());
///
/// number.enter(50);
/// assert!(!number.was_incorrect());
/// assert!(!number.is_correct());
/// assert!(!number.is_revealed());
///
/// number.enter(100);
/// assert!(number.was_incorrect());
///
/// number.reveal();
/// assert!(number.is_correct());
/// assert!(number.is_revealed());
///
/// // Avoid entering again after reveals
/// number.enter(80);
/// assert!(number.is_correct());
/// assert!(number.is_revealed());
///
/// // Style
/// assert!(number.is_circled());
/// ```
#[derive(Debug)]
pub struct Cell<S> {
    solution: S,

    // Initial state
    is_initially_revealed: bool,
    initial_style: CellStyle,

    // Current state
    entry: Option<S>,
    style: CellStyle,
}

impl<S> Cell<S> {
    // Current styles
    check_style!(CellStyle::REVEALED, style, is_revealed());
    check_style!(CellStyle::INCORRECT, style, is_incorrect());
    check_style!(CellStyle::PREVIOUSLY_INCORRECT, style, was_incorrect());

    // Initial styles
    check_style!(CellStyle::CIRCLED, initial_style, is_circled());
    check_style!(CellStyle::REVEALED, initial_style, is_initially_revealed());

    /// Create a "simple cell", i.e. one without entries or styles
    pub fn new(solution: S) -> Self {
        Self::new_styled(solution, CellStyle::empty())
    }

    pub fn new_styled(solution: S, style: CellStyle) -> Self {
        let is_initially_revealed = style.contains(CellStyle::INITIALLY_REVEALED);

        Self {
            solution,
            initial_style: style.initial(),
            entry: None,
            is_initially_revealed,
            style,
        }
    }

    /// Retrieve the solution of the cell
    pub fn solution(&self) -> &S {
        &self.solution
    }

    /// Retrieve the current entry in the cell
    pub fn entry(&self) -> Option<&S> {
        self.entry.as_ref()
    }

    /// Retrieve the initial entry in the cell
    pub fn initial_entry(&self) -> Option<&S> {
        self.is_initially_revealed.then_some(&self.solution)
    }

    /// Retrieve the current style of the cell
    pub fn style(&self) -> CellStyle {
        self.style
    }

    /// Retrieve the initial style of the cell
    pub fn initial_style(&self) -> CellStyle {
        self.initial_style
    }
}

impl<S> Cell<S>
where
    S: Eq,
{
    /// Verify whether the current entry solves the square
    pub fn is_correct(&self) -> bool {
        match (&self.solution, &self.entry) {
            // Empty entries are always false
            (_, None) => false,
            (solution, Some(entry)) => solution == entry,
        }
    }

    /// Enter a new guess to solve the cell
    /// This updates the cell [style](CellStyle) based on the [current](CellStyle::INCORRECT) and [previous](CellStyle::PREVIOUSLY_INCORRECT) correctness.
    pub fn enter<E: Into<S>>(&mut self, guess: E) -> bool {
        // Never overwrite revealed solution
        if self.is_revealed() {
            return false;
        }

        // Check whether the cell was previously incorrect
        if self.entry.is_some() && !self.is_correct() {
            self.style |= CellStyle::PREVIOUSLY_INCORRECT;
        }

        // Enter the new guess and set its correctness style
        self.entry = Some(guess.into());

        self.style = match self.is_correct() {
            true => self.style - CellStyle::INCORRECT,
            false => self.style | CellStyle::INCORRECT,
        };

        true
    }
}

impl<S> Cell<S>
where
    S: Clone,
{
    /// Reveal the square by manually entering its solution.
    /// This sets its [style](CellStyle) to be [revealed](CellStyle::REVEALED)
    pub fn reveal(&mut self) {
        self.style |= CellStyle::REVEALED;
        self.entry = Some(self.solution.clone())
    }

    /// Clear the current entry.
    /// Note that this does not apply to revealed solutions
    pub fn clear(&mut self) {
        if !self.is_revealed() {
            self.entry = None;

            // NOTE: correctness is guaranteed as `init` only allows correct state
            // Therefore the style will never be incorrect when set to its initial state
            self.style -= CellStyle::INCORRECT;
        }
    }
}

impl<S> Clone for Cell<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            solution: self.solution.clone(),
            entry: self.entry.clone(),
            style: self.style,
            is_initially_revealed: self.is_initially_revealed,
            initial_style: self.style,
        }
    }
}

impl<S> Copy for Cell<S> where S: Copy {}

impl<S> PartialEq for Cell<S>
where
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.solution == other.solution
    }
}

impl<S> Eq for Cell<S> where S: Eq {}

impl<S> fmt::Display for Cell<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.entry {
            Some(ref entry) => write!(f, "{}{} ({entry})", self.solution, self.style),
            None => write!(f, "{}{}", self.solution, self.style),
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::{Cell, CellStyle};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SerdeCell<S> {
        Simple(S),
        Full {
            solution: S,

            #[serde(skip_serializing_if = "Option::is_none")]
            entry: Option<S>,

            #[serde(skip_serializing_if = "CellStyle::is_empty")]
            style: CellStyle,
        },
    }

    impl<S> Cell<S>
    where
        S: Clone,
    {
        pub fn to_serde(&self) -> SerdeCell<S> {
            if self.entry.is_none() && self.style.is_empty() {
                SerdeCell::Simple(self.solution.clone())
            } else {
                SerdeCell::Full {
                    solution: self.solution.clone(),
                    entry: self.entry.to_owned(),
                    style: self.style,
                }
            }
        }

        pub fn from_serde(cell: SerdeCell<S>) -> Self {
            match cell {
                SerdeCell::Simple(solution) => Cell::new(solution),
                SerdeCell::Full {
                    solution,
                    entry,
                    style,
                } => {
                    let is_initially_revealed = style.contains(CellStyle::INITIALLY_REVEALED);

                    Self {
                        solution,
                        initial_style: style.initial(),
                        entry,
                        is_initially_revealed,
                        style,
                    }
                }
            }
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<Sol> Serialize for Cell<Sol>
    where
        Sol: Serialize + Clone,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.to_serde().serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, Sol> Deserialize<'de> for Cell<Sol>
    where
        Sol: Deserialize<'de> + Clone,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let cell = SerdeCell::deserialize(deserializer)?;

            Ok(Self::from_serde(cell))
        }
    }
}

#[cfg(feature = "serde")]
pub use serde_impl::SerdeCell;

use crate::check_style;
