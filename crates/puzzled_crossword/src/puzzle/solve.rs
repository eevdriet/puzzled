use crate::{ClueId, Crossword};
use puzzled_core::Position;

/// # Mutation and solving
impl Crossword {
    /// Try to reveal a [cell](crate::Cell) at a given [position](Position).
    /// Returns whether successful, i.e. the [square](Square) at the position is in-bounds and a cell
    /// ```
    /// use puzzled_crossword::{crossword, Position};
    ///
    /// let mut puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    ///    [G H I]
    /// );
    ///
    /// let pos = Position::new(1, 1);
    /// assert!(puzzle.reveal_cell(pos));
    /// assert!(
    ///     puzzle
    ///         .squares()
    ///         .get_fill(pos)
    ///         .is_some_and(|cell| cell.is_revealed())
    /// );
    ///
    /// let pos2 = Position::new(10, 10);
    /// assert!(!puzzle.reveal_cell(pos2));
    /// ```
    pub fn reveal_cell(&mut self, pos: Position) -> bool {
        // Try to get the square to reveal
        let Some(square) = self.squares.get_fill_mut(pos) else {
            return false;
        };

        square.reveal();
        true
    }

    /// Try to reveal a [clue](crate::Clue) from a given [identifier](ClueId).
    /// Returns whether the clue exists in the puzzle and all its [positions](Position) could be revealed
    /// ```
    /// ```
    pub fn reveal_clue(&mut self, id: ClueId) -> bool {
        // Try to get the clue to reveal squares for
        let Some(clue) = self.clues().get(&id) else {
            return false;
        };

        // Try reveal all squares that the is positioned in
        clue.clone().positions().all(|pos| self.reveal_cell(pos))
    }

    pub fn reveal(&mut self) {
        for square in self.squares.iter_fills_mut() {
            square.reveal();
        }
    }

    pub fn is_revealed(&self) -> bool {
        self.squares.iter_fills().all(|square| square.is_revealed())
    }

    /// Checks whether the puzzle is solved
    pub fn is_solved(&self) -> bool {
        self.squares.iter_fills().all(|square| square.is_correct())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::{Position, crossword};

        let mut puzzle = crossword! (
           [A B C]
           [D E F]
           [G H I]
        );

        let pos = Position::new(1, 1);
        assert!(puzzle.reveal_cell(pos));
        assert!(
            puzzle
                .squares()
                .get_fill(pos)
                .is_some_and(|cell| cell.is_revealed())
        );
    }
}
