use puzzled_core::{Entry, Grid, GridIter, GridPositionsIter, Square};

use crate::{Clue, Solution};

pub trait CluesSolveState {
    fn iter_clue(&self, clue: &Clue) -> impl Iterator<Item = &Entry<Solution>> + Clone;
}

impl CluesSolveState for Grid<Square<Entry<Solution>>> {
    fn iter_clue(&self, clue: &Clue) -> impl Iterator<Item = &Entry<Solution>> + Clone {
        let positions: Vec<_> = clue.positions().collect();
        let iter = GridIter::Positions(GridPositionsIter::new(self, positions));

        iter.map(|square| {
            square
                .as_ref()
                .expect("Clue squares should always be filled")
        })
    }
}
