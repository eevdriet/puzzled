mod builder;
mod clues;
mod geom;
mod grid;
mod macros;
mod solve;
mod square;
mod timer;

use std::fmt;

pub use builder::*;
pub use clues::*;
pub use geom::*;
pub use grid::*;
pub use square::*;
pub use timer::*;

macro_rules! string_prop {
    ($field:ident, $field_doc:literal, $setter:ident, $setter_doc:literal) => {
        #[doc = $field_doc]
        pub fn $field(&self) -> Option<&str> {
            self.$field.as_deref()
        }

        #[doc = $setter_doc]
        pub fn $setter<S: Into<String>>(mut self, value: S) -> Self {
            self.$field = Some(value.into());
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
/// Puzzles can be constructed dynamically from a collection of [squares](Square) and [clues](Clue).
/// Based on whether you already have the entire clue or just is [specification](ClueSpec), you can do the following:
/// - Use [`Puzzle::new`] when both squares and clues are available
/// - Use [`Puzzle::from_squares`] to first initialize the puzzle [grid](Grid) and thereafter [`Puzzle::insert_clues`] to add the clues in the right position.
///
/// # Properties
/// Currently the puzzle defines all properties that can be set in a [*.puz][PUZ google spec] file, which include:
/// - Author
/// - Version string (specified as `"x.y"` where `x,y: u8`)
/// - Copyright
/// - Notes
/// - Title
///
/// Each property `prop` can be set with `with_prop()` and retrieved with `prop()`, e.g. see [`Puzzle::author()`] and [`Puzzle::with_author`].
///
/// Puzzles keep track of the time spent solving with a [`Timer`].
/// Users can access the timer with [`timer`](Self::timer) and [`timer_mut`](Self::timer_mut) to [start](Timer::start) and [stop](Timer::stop) playing.
///
/// Finally, properties that are derived from the puzzle [squares](Squares) are [clues](Clues) can indirectly be accessed.
/// These include
/// - Dimensionality of the puzzle, such as the number of [rows](Self::rows), [columns](Self::cols)
/// - Iterators:
///     * [`iter`](Self::iter), [`iter_mut`](Self::iter_mut) for squares
///     * [`iter_clues`](Self::iter_clues), [`iter_across`](Self::iter_across), [`iter_down`](Self::iter_down) and `*_mut` variants for clues
///
/// # Mutation and solving
///
/// [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
/// [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki

#[derive(Debug, Default)]
pub struct Puzzle {
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
impl Puzzle {
    /// Constructs a new puzzle from its [squares](Square) and [clues](Clue)
    pub fn new(squares: Squares, clues: Clues) -> Self {
        Self {
            squares,
            clues,
            ..Default::default()
        }
    }

    /// Constructs a new puzzle from just its [squares](Square)
    /// Use [`Puzzle::insert_clues`] to add [clues](Clue) from their [specification](ClueSpec)
    pub fn from_squares(squares: Squares) -> Self {
        Self {
            squares,
            ..Default::default()
        }
    }
}

/// # Properties
impl Puzzle {
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

impl PartialEq for Puzzle {
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

impl Eq for Puzzle {}

impl fmt::Display for Puzzle {
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

        for entry in self.iter_across() {
            writeln!(f, "{entry}")?
        }

        writeln!(f)?;

        for entry in self.iter_down() {
            writeln!(f, "{entry}")?
        }

        Ok(())
    }
}
