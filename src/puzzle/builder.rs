use std::marker::PhantomData;

use crate::{Cell, Entries, Grid, Puzzle, Timer};

/// Used to indicate that a partially-constructed [`Puzzle`] is missing the height of its grid
pub struct MissingField;
/// Used to indicate that a partially-constructed [`Puzzle`] has set the height of its grid
pub struct HasField;

macro_rules! string_setter {
    ($field:ident) => {
        pub fn $field<S: Into<String>>(mut self, value: S) -> Self {
            self.puzzle.$field = Some(value.into());
            self
        }
    };
}

pub struct Builder<W, H> {
    puzzle: Puzzle,
    has: (PhantomData<W>, PhantomData<H>),
}

impl<C, E> Builder<C, E> {
    pub fn cells(mut self, cells: Grid<Cell>) -> Builder<HasField, E> {
        self.puzzle.cells = cells;

        Builder {
            puzzle: self.puzzle,
            has: (PhantomData::<HasField>, self.has.1),
        }
    }

    pub fn entries(mut self, entries: Entries) -> Builder<C, HasField> {
        self.puzzle.entries = entries;

        Builder {
            puzzle: self.puzzle,
            has: (self.has.0, PhantomData::<HasField>),
        }
    }

    pub fn timer(mut self, timer: Timer) -> Self {
        self.puzzle.timer = timer;
        self
    }

    string_setter!(author);
    string_setter!(copyright);
    string_setter!(notes);
    string_setter!(title);
    string_setter!(version);
}

impl<C, E> Default for Builder<C, E> {
    fn default() -> Self {
        Self {
            puzzle: Puzzle::default(),
            has: (PhantomData, PhantomData),
        }
    }
}

impl Builder<HasField, HasField> {
    pub fn build(self) -> Puzzle {
        self.puzzle
    }
}

impl Puzzle {
    pub fn builder() -> Builder<MissingField, MissingField> {
        Builder::default()
    }
}
