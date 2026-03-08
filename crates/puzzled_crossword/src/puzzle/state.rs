use derive_more::{Deref, DerefMut, Display};
use puzzled_core::{
    Entry, Grid, Solve, Square, SquareGridState, Timer, impl_solve_for_square_grid_state,
};

use crate::{ClueId, Crossword, Solution};

#[derive(Debug, Deref, DerefMut, Display)]
pub struct CrosswordState(pub(crate) SquareGridState<Solution>);

impl CrosswordState {
    pub fn new(
        solutions: Grid<Square<Option<Solution>>>,
        entries: Grid<Square<Entry<Solution>>>,
        timer: Timer,
    ) -> Self {
        let state = SquareGridState::new(solutions, entries, timer);
        Self(state)
    }

    pub fn reveal_clue(&mut self, crossword: &Crossword, id: ClueId) -> bool {
        // Try to get the clue to reveal squares for
        let Some(clue) = crossword.clues().get(&id) else {
            return false;
        };

        // Try reveal all squares that the is positioned in
        clue.positions().all(|pos| self.reveal(&pos))
    }
}

impl_solve_for_square_grid_state!(CrosswordState, Crossword, Solution);

pub trait CrosswordSolve {
    /// Try to reveal a [clue](crate::Clue) from a given [identifier](ClueId).
    /// Returns whether the clue exists in the puzzle and all its [positions](Position) could be revealed
    /// ```
    /// ```
    fn reveal_clue(&mut self, crossword: &Crossword, id: ClueId) -> bool;
}

impl From<&Crossword> for CrosswordState {
    fn from(crossword: &Crossword) -> Self {
        let squares = crossword.squares();

        let solutions =
            squares.map_ref(|square| square.map_ref(|cell| Some(cell.solution.clone())));

        let entries = squares.map_ref(|square| {
            square.map_ref(|cell| {
                let mut entry = Entry::default_with_style(cell.style);

                if let Some(ref solution) = cell.solution {
                    entry.enter(solution.clone());
                }

                Some(entry)
            })
        });

        let timer = Timer::default();

        CrosswordState::new(solutions, entries, timer)
    }
}
