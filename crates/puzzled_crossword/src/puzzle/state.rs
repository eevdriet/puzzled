use derive_more::{Deref, DerefMut};
use puzzled_core::{Entry, Grid, Solve, Square, State, Timer};

use crate::{ClueId, Crossword, Solution};

#[derive(Debug, Deref, DerefMut)]
pub struct CrosswordState(
    pub(crate) State<Grid<Square<Option<Solution>>>, Grid<Square<Entry<Solution>>>>,
);

/// # Mutation and solving
impl CrosswordState {
    pub fn new(
        solution: Grid<Square<Option<Solution>>>,
        entries: Grid<Square<Entry<Solution>>>,
        timer: Timer,
    ) -> Self {
        let state = State::new_timed(solution, entries, timer);
        Self(state)
    }

    /// Try to reveal a [clue](crate::Clue) from a given [identifier](ClueId).
    /// Returns whether the clue exists in the puzzle and all its [positions](Position) could be revealed
    /// ```
    /// ```
    pub fn reveal_clue(&mut self, crossword: &Crossword, id: ClueId) -> bool {
        // Try to get the clue to reveal squares for
        let Some(clue) = crossword.clues().get(&id) else {
            return false;
        };

        // Try reveal all squares that the is positioned in
        clue.positions().all(|pos| self.reveal(&pos))
    }
}
