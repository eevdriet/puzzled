use crate::{ClueId, Position, Puzzle, Square};

/// # Mutation and solving
impl Puzzle {
    /// Try to reveal a [cell](crate::Cell) at a given [position](Position).
    /// Returns whether successful, i.e. the [square](Square) at the position is in-bounds and a cell
    /// ```
    /// use puzzled::{puzzle, Position};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A B C]
    ///    [D E F]
    ///    [G H I]
    /// );
    ///
    /// let pos = Position::new(1, 1);
    /// assert!(puzzle.reveal_cell(pos));
    /// assert!(puzzle[pos].is_revealed());
    ///
    /// let pos2 = Position::new(10, 10);
    /// assert!(!puzzle.reveal_cell(pos2));
    /// ```
    pub fn reveal_cell(&mut self, pos: Position) -> bool {
        // Try to get the square to reveal
        let Some(square) = self.get_mut(pos) else {
            return false;
        };

        square.reveal();
        true
    }

    /// Try to reveal a [clue](crate::Clue) from a given [identifier](ClueId).
    /// Returns whether the clue exists in the puzzle and all its [positions](Position) could be revealed
    /// ```
    /// use puzzled::{puzzle, Position};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A B C]
    ///    [D E F]
    ///    [G H I]
    /// );
    ///
    /// let pos = Position::new(1, 1);
    /// assert!(puzzle.reveal_cell(pos));
    /// assert!(puzzle[pos].is_revealed());
    ///
    /// let pos2 = Position::new(10, 10);
    /// assert!(!puzzle.reveal_cell(pos2));
    /// ```
    pub fn reveal_clue(&mut self, id: ClueId) -> bool {
        // Try to get the clue to reveal squares for
        let Some(clue) = self.get_clue(id) else {
            return false;
        };

        // Try reveal all squares that the is positioned in
        clue.clone().positions().all(|pos| self.reveal_cell(pos))
    }

    pub fn reveal(&mut self) {
        for square in self.iter_mut() {
            square.reveal();
        }
    }

    pub fn is_revealed(&self) -> bool {
        self.iter().all(|square| square.is_revealed())
    }

    /// Checks whether the puzzle is solved
    pub fn is_solved(&self) -> bool {
        self.squares
            .iter()
            .filter_map(|square| match square {
                Square::Black => None,
                Square::White(fill) => Some(fill),
            })
            .all(|square| square.is_correct())
    }
}
