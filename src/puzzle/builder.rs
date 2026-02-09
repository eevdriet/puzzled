use std::marker::PhantomData;

use crate::Puzzle;

/// Used to indicate that a partially-constructed [`Puzzle`] is missing the width of its grid
pub struct MissingWidth;
/// Used to indicate that a partially-constructed [`Puzzle`] has set the width of its grid
pub struct HasWidth;

/// Used to indicate that a partially-constructed [`Puzzle`] is missing the height of its grid
pub struct MissingField;
/// Used to indicate that a partially-constructed [`Puzzle`] has set the height of its grid
pub struct HasField;

pub struct Builder<W, H> {
    under_construction: Puzzle,
    has: (PhantomData<W>, PhantomData<H>),
}

impl<W, H> Builder<W, H> {
    pub fn width(mut self, width: impl Into<u8>) -> Builder<HasWidth, H> {
        self.under_construction.width = width.into();

        Builder {
            under_construction: self.under_construction,
            has: (PhantomData::<HasWidth>, self.has.1),
        }
    }

    pub fn height(mut self, height: impl Into<u8>) -> Builder<W, HasField> {
        self.under_construction.height = height.into();

        Builder {
            under_construction: self.under_construction,
            has: (self.has.0, PhantomData::<HasField>),
        }
    }
}
