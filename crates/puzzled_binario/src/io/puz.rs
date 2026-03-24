use puzzled_core::{Grid, Metadata};
use puzzled_io::{
    Context, format,
    puz::{
        BinaryPuzzle, Extras, Grids, Header, PuzSizeCheck, Strings, WriteStyleGrid,
        read::{self, read_cell_entries, read_metadata},
        write::{self, WriteStateGrid},
    },
};

use crate::{Binario, BinarioState, Bit};

impl PuzSizeCheck for Binario {
    fn check_puz_size(&self) -> write::Result<()> {
        Ok(())
    }
}

impl BinaryPuzzle<BinarioState> for Binario {
    fn width(&self) -> usize {
        self.cells().cols()
    }

    fn height(&self) -> usize {
        self.cells().rows()
    }

    fn grids(&self, state: &BinarioState) -> write::Result<(Grid<u8>, Grid<u8>)> {
        let solution = state
            .state
            .solutions
            .write_state_grid(|bit| b'0' + u8::from(*bit));
        let state = state
            .state
            .entries
            .write_state_grid(|bit| b'0' + u8::from(*bit));

        Ok((solution, state))
    }

    fn metadata(&self) -> Option<&Metadata> {
        Some(self.meta())
    }

    fn extras(&self, state: &BinarioState) -> write::Result<Extras> {
        let mut extras = Extras::default();

        // LTIM
        // TODO: timer extras.ltim = Some(state.inner.timer());

        // GEXT
        let cells = self.cells();
        let entries = &state.state.entries;

        let gext = cells.write_combined_style(entries);
        extras.gext = Some(gext);

        Ok(extras)
    }

    fn read_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<(Self, BinarioState)> {
        let mut read_bit = |char: char| {
            Bit::try_from(char as u8)
                .map_err(|err| format::Error::PuzzleSpecific(Box::new(err)))
                .context("Reading byte")
        };

        let (cells, entries) = read_cell_entries(&grids, &extras, &mut read_bit)?;
        let solutions = cells.map_ref(|cell| cell.solution);
        let meta = read_metadata(&header, &strings);

        let binario = Binario::new(cells, meta);

        let timer = extras.ltim.unwrap_or_default();
        let state = BinarioState::new(solutions, entries, timer);

        Ok((binario, state))
    }
}
