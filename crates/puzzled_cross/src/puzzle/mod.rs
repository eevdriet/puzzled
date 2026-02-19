/// Defines all functionality for solving and interacting with [puzzles](Crossword)
///
///
mod clue;
mod macros;
mod solve;
mod square;

pub use clue::*;
pub use square::*;

use puzzled_core::Timer;
use std::fmt;

macro_rules! string_prop {
    ($field:ident, $field_doc:literal, $setter:ident, $setter_doc:literal) => {
        #[doc = $field_doc]
        pub fn $field(&self) -> Option<&str> {
            self.$field.as_deref()
        }

        #[doc = $setter_doc]
        pub fn $setter<S: Into<String>>(mut self, value: S) -> Self {
            let value = value.into();

            if !value.is_empty() {
                self.$field = Some(value.into());
            }

            self
        }
    };
}

/// A crossword puzzle
///
/// This is the main data structure that is delivered by the crate.
/// It contains all needed information to play a crossword puzzle, such as
///
/// # Constructors
/// Crosswords can be constructed dynamically from a collection of [squares](Square) and [clues](Clue).
/// Based on whether you already have the entire clue or just is [specification](ClueSpec), you can do the following:
/// - Use [`Crossword::new`] when both squares and clues are available
/// - Use [`Crossword::from_squares`] to first initialize the puzzle [grid](crate::Grid) and thereafter [`Crossword::insert_clues`] to add the clues in the right position.
///
/// # Properties
/// Currently the puzzle defines all properties that can be set in a [*.puz][PUZ google spec] file, which include:
/// - Author
/// - Version string (specified as `"x.y"` where `x,y: u8`)
/// - Copyright
/// - Notes
/// - Title
///
/// Each property `prop` can be set with `with_prop()` and retrieved with `prop()`, e.g. see [`Crossword::author()`] and [`Crossword::with_author`].
///
/// Crosswords keep track of the time spent solving with a [`Timer`].
/// Users can access the timer with [`timer`](Self::timer) and [`timer_mut`](Self::timer_mut) to [start](Timer::start) and [stop](Timer::pause) playing.
/// If the user does not set a timer of their own, a [running](crate::TimerState::Running) timer is attached that has no initial [elapsed](Timer::elapsed) time.
///
/// # Mutation and solving
///
/// [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
/// [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki

#[derive(Debug, Default)]
pub struct Crossword {
    // State
    squares: Squares,
    clues: Clues,
    timer: Timer,

    // Information
    author: Option<String>,
    version: Option<String>,
    copyright: Option<String>,
    notes: Option<String>,
    title: Option<String>,
}

/// # Constructors
impl Crossword {
    /// Constructs a new puzzle from its [squares](Square) and [clues](Clue)
    pub fn new(squares: Squares, clues: Clues) -> Self {
        Self {
            squares,
            clues,
            ..Default::default()
        }
    }

    /// Constructs a new puzzle from just its [squares](Square)
    /// Use [`Crossword::insert_clues`] to add [clues](Clue) from their [specification](ClueSpec)
    pub fn from_squares(squares: Squares) -> Self {
        Self {
            squares,
            ..Default::default()
        }
    }

    pub fn squares(&self) -> &Squares {
        &self.squares
    }

    pub fn squares_mut(&mut self) -> &mut Squares {
        &mut self.squares
    }

    pub fn clues(&self) -> &Clues {
        &self.clues
    }

    pub fn clues_mut(&mut self) -> &mut Clues {
        &mut self.clues
    }

    /// Number of rows (height) in the puzzle.
    ///
    /// Note that this includes blank squares
    /// ```
    /// use puzzled_crossword::crossword;
    ///
    /// let puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    /// );
    /// assert_eq!(puzzle.rows(), 2);
    /// assert_eq!(puzzle.cols(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.squares.rows()
    }

    /// Number of columns (width) in the puzzle.
    ///
    /// Note that this includes blank squares
    /// ```
    /// use puzzled_crossword::crossword;
    ///
    /// let puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    /// );
    /// assert_eq!(puzzle.rows(), 2);
    /// assert_eq!(puzzle.cols(), 3);
    /// ```
    pub fn cols(&self) -> usize {
        self.squares.cols()
    }
}

/// # Properties
impl Crossword {
    string_prop!(
        author,
        "Author of the puzzle",
        with_author,
        "Include puzzle author"
    );

    string_prop!(
        copyright,
        "Copyright information of the puzzle",
        with_copyright,
        "Include copyright information"
    );

    string_prop!(
        notes,
        "Notes on the puzzle",
        with_notes,
        "Include puzzle notes"
    );

    /// Reference to the timer that keeps track of the total playing time and whether the user is currently playing
    pub fn timer(&self) -> Timer {
        self.timer
    }

    /// Mutable reference to the timer that keeps track of the total playing time and whether the user is currently playing
    pub fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    string_prop!(
        title,
        "Title of the puzzle",
        with_title,
        "Include puzzle title"
    );
    string_prop!(
        version,
        "Version of the puzzle",
        with_version,
        "Include puzzle version"
    );
}

impl PartialEq for Crossword {
    fn eq(&self, other: &Self) -> bool {
        self.squares == other.squares
            && self.clues == other.clues
            && self.author == other.author
            && self.version == other.version
            && self.copyright == other.copyright
            && self.notes == other.notes
            && self.title == other.title
    }
}

impl Eq for Crossword {}

impl fmt::Display for Crossword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = self.squares.cols();

        for pos in self.squares.positions() {
            let square = &self.squares[pos];
            write!(f, "{square}")?;

            if pos.col == cols - 1 {
                writeln!(f)?
            } else {
                write!(f, " ")?
            }
        }

        writeln!(f)?;

        for entry in self.clues().iter_across() {
            writeln!(f, "{entry}")?
        }

        writeln!(f)?;

        for entry in self.clues().iter_down() {
            writeln!(f, "{entry}")?
        }

        Ok(())
    }
}
