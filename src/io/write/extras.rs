use std::{collections::BTreeMap, io::Write};

use crate::{
    Puzzle, Square,
    io::{PuzWriter, write::PuzWrite},
};

impl PuzWriter {
    pub(crate) fn write_extras(&self, puzzle: &Puzzle) -> std::io::Result<Vec<u8>> {
        let mut extras = Vec::new();

        self.write_rebuses(&mut extras, puzzle)?;
        self.write_ltim(&mut extras, puzzle)?;
        self.write_gext(&mut extras, puzzle)?;

        Ok(extras)
    }

    fn write_rebuses(&self, extras: &mut Vec<u8>, puzzle: &Puzzle) -> std::io::Result<()> {
        // Only write GRBS/LTBR if any rebuses are used
        if !puzzle.iter_cells().any(|cell| cell.is_rebus()) {
            return Ok(());
        }

        // First write the GRBS, indicating which squares hold a rebus
        extras.write_all(b"GRBS")?;

        let mut rebuses: BTreeMap<u8, String> = BTreeMap::new();
        let mut num = 0;

        for square in puzzle.iter() {
            let byte: u8 = match square {
                Square::White(cell) if cell.is_rebus() => {
                    num += 1;
                    rebuses.insert(num, cell.solution().to_string());

                    num
                }
                _ => 0,
            };

            extras.write_u8(byte)?;
        }

        // Then write the RTBL, which holds the actual rebus definitions
        extras.write_all(b"RTBL")?;

        for (num, rebus) in rebuses {
            let key = format!("{num:02}:{rebus};");
            extras.write_all(key.as_bytes())?;
        }
        extras.write_u8(0)?;

        Ok(())
    }

    fn write_ltim(&self, extras: &mut Vec<u8>, puzzle: &Puzzle) -> std::io::Result<()> {
        extras.write_all(b"LTIM")?;

        let timer = puzzle.timer();
        let secs = timer.elapsed().as_secs();
        let state: u8 = timer.state().into();

        let ltim = format!("{secs},{state}");
        extras.write_str0(&ltim)
    }

    fn write_gext(&self, extras: &mut Vec<u8>, puzzle: &Puzzle) -> std::io::Result<()> {
        // Only write GEXT if any styles are set
        if puzzle.iter_cells().all(|cell| cell.style().is_empty()) {
            return Ok(());
        }

        // Then write the style of each square
        extras.write_all(b"GEXT")?;

        for square in puzzle.iter() {
            let byte: u8 = match square {
                Square::Black => 0,
                Square::White(cell) => cell.style().bits(),
            };

            extras.write_u8(byte)?;
        }

        Ok(())
    }
}
