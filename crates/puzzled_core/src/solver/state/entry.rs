use crate::CellStyle;
use std::fmt::{self, Debug};

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
/// use puzzled::core::{cell, Cell, CellStyle, Reveal};
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
pub struct Entry<E> {
    entry: Option<E>,
    guesses: Vec<E>,
    style: CellStyle,
}

pub type CellGrid<T> = Grid<Entry<T>>;

impl<E> Entry<E> {
    // Current styles
    check_style!(CellStyle::REVEALED, style, is_revealed());
    check_style!(CellStyle::INCORRECT, style, is_incorrect());
    check_style!(CellStyle::PREVIOUSLY_INCORRECT, style, was_incorrect());

    // Initial styles
    check_style!(CellStyle::CIRCLED, style, is_circled());

    pub fn new<T>(entry: T) -> Self
    where
        T: Into<E>,
    {
        Self {
            entry: Some(entry.into()),
            ..Default::default()
        }
    }

    pub fn new_styled(style: CellStyle) -> Self {
        Self {
            style,
            ..Default::default()
        }
    }

    /// Retrieve the current entry in the cell
    pub fn entry(&self) -> Option<&E> {
        self.entry.as_ref()
    }

    /// Retrieve the current style of the cell
    pub fn style(&self) -> CellStyle {
        self.style
    }

    /// Enter a new guess to solve the cell
    /// This updates the cell [style](CellStyle) based on the [current](CellStyle::INCORRECT) and [previous](CellStyle::PREVIOUSLY_INCORRECT) correctness.
    pub fn enter<T: Into<E>>(&mut self, entry: T) -> bool {
        // Never overwrite revealed solution
        if self.is_revealed() {
            return false;
        }

        // // Check whether the cell was previously incorrect
        // if self.entry.is_some() && !self.is_correct() {
        //     self.style |= CellStyle::PREVIOUSLY_INCORRECT;
        // }

        // Enter the new guess and set its correctness style
        self.entry = Some(entry.into());
        self.guesses.clear();

        true

        // self.style = match self.is_correct() {
        //     true => self.style - CellStyle::INCORRECT,
        //     false => self.style | CellStyle::INCORRECT,
        // };
    }

    pub fn reveal<T: Into<E>>(&mut self, solution: T) -> bool {
        let result = self.enter(solution);
        self.style |= CellStyle::REVEALED;

        result
    }

    /// Clear the current entry.
    ///
    /// Note that this does not apply to revealed solutions
    pub fn clear(&mut self) {
        if !self.is_revealed() {
            self.entry = None;
            self.guesses.clear();

            // NOTE: correctness is guaranteed as `init` only allows correct state
            // Therefore the style will never be incorrect when set to its initial state
            self.style -= CellStyle::INCORRECT;
        }
    }
}

impl<E> Default for Entry<E> {
    fn default() -> Self {
        Self {
            entry: None,
            guesses: Vec::new(),
            style: CellStyle::empty(),
        }
    }
}

impl<E> Clone for Entry<E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            entry: self.entry.clone(),
            style: self.style,
            guesses: self.guesses.clone(),
        }
    }
}

impl<E> PartialEq for Entry<E>
where
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.entry == other.entry
    }
}

impl<E> Eq for Entry<E> where E: Eq {}

impl<E> Entry<E>
where
    E: Eq,
{
    pub fn check(&mut self, solution: &E) -> Option<bool> {
        // Try to compare the current entry to the solution
        let state = self.entry().map(|e| e == solution);

        if let Some(is_correct) = state {
            // Set previous correctness style
            if self.style.contains(CellStyle::INCORRECT) {
                self.style |= CellStyle::PREVIOUSLY_INCORRECT;
            }

            // Set current correctness style
            if is_correct {
                self.style -= CellStyle::INCORRECT;
            } else {
                self.style |= CellStyle::INCORRECT;
            }
        }

        state
    }
}

impl<E> fmt::Display for Entry<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.entry, self.style)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::{CellStyle, Entry};

    #[doc(hidden)]
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SerdeEntry<E> {
        Simple(E),
        Full {
            #[serde(skip_serializing_if = "Option::is_none")]
            entry: Option<E>,

            #[serde(skip_serializing_if = "Vec::is_empty")]
            guesses: Vec<E>,

            #[serde(skip_serializing_if = "CellStyle::is_empty")]
            style: CellStyle,
        },
    }

    impl<E> Entry<E>
    where
        E: Clone,
    {
        #[doc(hidden)]
        pub fn to_serde(&self) -> SerdeEntry<E> {
            if let Some(ref entry) = self.entry
                && self.style.is_empty()
                && self.guesses.is_empty()
            {
                SerdeEntry::Simple(entry.clone())
            } else {
                SerdeEntry::Full {
                    entry: self.entry.to_owned(),
                    guesses: self.guesses.clone(),
                    style: self.style,
                }
            }
        }

        #[doc(hidden)]
        pub fn from_serde(cell: SerdeEntry<E>) -> Self {
            match cell {
                SerdeEntry::Simple(solution) => {
                    let mut entry = Entry::default();
                    entry.enter(solution);

                    entry
                }
                SerdeEntry::Full {
                    entry,
                    guesses,
                    style,
                } => Self {
                    entry,
                    guesses,
                    style,
                },
            }
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<Sol> Serialize for Entry<Sol>
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
    impl<'de, Sol> Deserialize<'de> for Entry<Sol>
    where
        Sol: Deserialize<'de> + Clone,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let cell = SerdeEntry::deserialize(deserializer)?;

            Ok(Self::from_serde(cell))
        }
    }
}

#[cfg(feature = "serde")]
pub use serde_impl::SerdeEntry;

use crate::{Grid, check_style};
