use crate::parse::{
    Extras, NON_PLAYABLE_CELL, PuzParser, PuzzleGrid, Result, windows_1252_to_char,
};
use crate::{Cell, Grid, Solution, Square, Squares};

impl PuzParser {
    pub(crate) fn read_squares(&self, grid: &PuzzleGrid, extras: &Extras) -> Result<Squares> {
        let mut cells = Vec::new();

        for ((&solution, &state), pos) in grid
            .solution
            .iter()
            .zip(grid.state.iter())
            .zip(grid.solution.positions())
        {
            let cell = match solution {
                // Non-playable cells are always black
                NON_PLAYABLE_CELL => Square::Black,

                byte => {
                    // Derive the solution based on the rebus information in the extras
                    let solution = match extras.get_rebus(pos) {
                        Some(rebus) => Solution::Rebus(rebus.clone()),
                        None => Solution::Letter(windows_1252_to_char(byte)),
                    };

                    let style = extras.get_style(pos);
                    let mut fill = Cell::new_styled(solution, style);

                    // Set the given user state for a playable cell
                    if state != NON_PLAYABLE_CELL {
                        let contents = windows_1252_to_char(state).to_string();
                        fill.enter(contents);
                    }

                    Square::White(fill)
                }
            };

            cells.push(cell);
        }

        let grid = Grid::new(cells, grid.solution.cols()).expect("Read correct length region");
        Ok(grid)
    }
}
