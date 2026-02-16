use crate::io::{Extras, Grids, NON_PLAYABLE_CELL, PuzReader, ReadResult, windows_1252_to_char};
use crate::{Cell, Grid, Solution, Square, Squares};

impl PuzReader {
    pub(crate) fn read_squares(&self, grids: &Grids, extras: &Extras) -> ReadResult<Squares> {
        let mut cells = Vec::new();

        for ((&solution, &state), pos) in grids
            .solution
            .iter()
            .zip(grids.state.iter())
            .zip(grids.solution.positions())
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

        let grid = Grid::new(cells, grids.solution.cols()).expect("Read correct length region");
        Ok(grid)
    }
}
