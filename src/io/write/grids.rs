use crate::{
    Puzzle, Square,
    io::{MISSING_ENTRY_CELL, NON_PLAYABLE_CELL, PuzWriter},
};

#[derive(Debug)]
pub(crate) struct Grids {
    pub solution: Vec<u8>,
    pub state: Vec<u8>,
}

impl PuzWriter {
    pub(crate) fn write_grids(&self, puzzle: &Puzzle) -> Grids {
        let solution = {
            let mut bytes = Vec::new();

            for square in puzzle.iter() {
                let byte = match square {
                    Square::Black => NON_PLAYABLE_CELL,
                    Square::White(cell) => {
                        cell.solution();
                        4
                    }
                };

                bytes.push(byte);
            }

            bytes
        };

        let state = {
            let mut bytes = Vec::new();

            for square in puzzle.iter() {
                let byte = match square {
                    Square::Black => NON_PLAYABLE_CELL,
                    Square::White(cell) => match cell.entry() {
                        Some(v) => v.chars().next().unwrap_or(MISSING_ENTRY_CELL as char) as u8,
                        None => MISSING_ENTRY_CELL,
                    },
                };

                bytes.push(byte);
            }

            bytes
        };

        Grids { solution, state }
    }
}
