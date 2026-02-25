/// Defines all functionality for solving and interacting with [puzzles](Crossword)
///
///
mod clue;
mod square;
mod state;

pub use clue::*;
use puzzled_io::puz::{MISSING_ENTRY_CHAR, NON_PLAYABLE_CHAR};
pub use square::*;
pub use state::*;

use puzzled_core::{Grid, Metadata, Puzzle, Square};
use std::fmt;

/// A [crossword](https://en.wikipedia.org/wiki/Crossword) puzzle
///
/// This is the main data structure that is delivered by the crate.
/// It contains all needed information to play a crossword puzzle
///
/// # Constructors
/// Crosswords can be constructed in a number of different ways, depending on the underlying data format
/// - With the [`crossword!`](crate::crossword) macro, that allows for specifying crosswords inline
/// - Dynamically from a collection of [squares](Square) and [clues](Clues).
///   Based on whether you already have [clues](Clue) and their placement or just their [specifications](ClueSpec), you can do the following:
///     - Use [`Crossword::new`] when both squares and clues are available
///     - Use [`Crossword::from_squares`] to first initialize the puzzle [grid](crate::Grid) and thereafter [`Crossword::insert_clues`] to add the clues in the right position.
/// - By using a reader from the [`io`](crate::io) module.
///   Various readers are available, including [`PuzReader`](crate::PuzReader) which uses the [Across Lite *.puz specification](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki)
/// - By
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
#[derive(Debug, Clone)]
pub struct Crossword {
    // State
    squares: Squares,
    clues: Clues,

    // Metadata
    meta: Metadata,
}

impl Puzzle for Crossword {
    type Solution = Grid<Square<Solution>>;
}

/// # Constructors
impl Crossword {
    /// Constructs a new puzzle from its [squares](Square) and [clues](Clue)
    pub fn new(squares: Squares, clues: Clues, meta: Metadata) -> Self {
        Self {
            squares,
            clues,
            meta,
        }
    }

    /// Constructs a new puzzle from just its [squares](Square)
    /// Use [`Crossword::insert_clues`] to add [clues](Clue) from their [specification](ClueSpec)
    pub fn from_squares(squares: Squares, meta: Metadata) -> Self {
        let clues = Clues::default();

        Self::new(squares, clues, meta)
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

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }

    /// Number of rows (height) in the puzzle.
    ///
    /// Note that this includes blank squares
    /// ```
    /// use puzzled::crossword::crossword;
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
    /// use puzzled::crossword::crossword;
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

impl PartialEq for Crossword {
    fn eq(&self, other: &Self) -> bool {
        self.squares == other.squares && self.clues == other.clues && self.meta == other.meta
    }
}

impl Eq for Crossword {}

impl fmt::Display for Crossword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = self.squares.cols();

        for (pos, square) in self.squares.iter_indexed() {
            match square.inner() {
                None => write!(f, "{NON_PLAYABLE_CHAR}"),
                Some(cell) => match cell.solution {
                    Some(ref solution) => write!(f, "{solution}"),
                    None => write!(f, "{MISSING_ENTRY_CHAR}"),
                },
            }?;

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

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Metadata;
    use serde::{Deserialize, Serialize, de::Error};

    use crate::{Clues, Crossword, SerdeClues, Squares};

    #[derive(Serialize, Deserialize)]
    struct SerdeCrossword {
        rows: usize,
        cols: usize,

        squares: Squares,
        clues: Option<SerdeClues>,

        // Metadata
        #[serde(flatten)]
        meta: Metadata,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Crossword {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            // Puzzle
            let squares = self.squares().clone();

            let has_clues = !self.clues().is_empty();
            let clues = has_clues.then_some(self.clues().to_serde());

            // Metadata
            let meta = self.meta.clone();

            SerdeCrossword {
                rows: self.squares().rows(),
                cols: self.squares().cols(),
                squares,
                clues,
                meta,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Crossword {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeCrossword {
                squares,
                clues: clues_data,
                meta,
                ..
            } = SerdeCrossword::deserialize(deserializer)?;

            let clues = Clues::from_serde(clues_data.unwrap_or_default()).map_err(Error::custom)?;

            Ok(Crossword {
                squares,
                clues,
                meta,
            })
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod test {
    use crate::crossword;

    #[test]
    fn serialize_crossword() {
        use serde_json;

        let crossword = crossword!(
            [C A T]
            [A . R]
            [R A T]

            - A: "Animal"
        );

        let json = serde_json::to_string_pretty(&crossword).unwrap();

        println!("{}", json);

        assert!(json.len() == 150);
    }
}
