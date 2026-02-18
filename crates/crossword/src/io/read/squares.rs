use puzzled_core::Grid;

use crate::io::{
    Extras, Grids, MISSING_ENTRY_CELL, NON_PLAYABLE_CELL, PuzReader, read, windows_1252_to_char,
};
use crate::puzzle::{Cell, Solution, Square, Squares};

impl PuzReader {
    pub(crate) fn read_squares(&self, grids: &Grids, extras: &Extras) -> read::Result<Squares> {
        let mut cells = Vec::new();
        eprintln!("Extras: {extras:?}");

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
                    let mut cell = Cell::new_styled(solution, style);

                    // Set the given user state for a playable cell
                    if state != MISSING_ENTRY_CELL {
                        let contents = windows_1252_to_char(state).to_string();
                        cell.enter(contents);
                    }

                    Square::White(cell)
                }
            };

            cells.push(cell);
        }

        let grid = Grid::new(cells, grids.solution.cols()).expect("Read correct length region");
        Ok(grid)
    }
}
