use puzzled_core::{Cell, Color, Entry, Grid, Metadata};
use puzzled_io::{
    Context,
    format::{self, StringError},
    puz::{
        BinaryPuzzle, ByteStr, Extras, Grids, Header, MISSING_ENTRY_CHAR, PuzSizeCheck, Strings,
        WriteStyleGrid, check_puz_size,
        read::{self, read_metadata},
        windows_1252_to_char,
        write::{self, WriteStateGrid},
    },
};

use crate::{ {{ puzzle | pascal_case }}, {{ puzzle | pascal_case }}State};

impl PuzSizeCheck for {{ puzzle | pascal_case }} {
    fn check_puz_size(&self) -> write::Result<()> {
        Ok(())
    }
}

impl BinaryPuzzle<{{ puzzle | pascal_case }}State> for {{ puzzle | pascal_case }} {
    fn width(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        0
    }

    fn clues(&self) -> Vec<ByteStr> {
        Vec::New()
    }

    fn grids(&self, state: &{{ puzzle | pascal_case }}State) -> write::Result<(Grid<u8>, Grid<u8>)> {
        Ok((solution, state))
    }

    fn metadata(&self) -> Option<&Metadata> {
        Some(self.meta())
    }

    fn extras(&self, state: &{{ puzzle | pascal_case }}State) -> write::Result<Extras> {
        let mut extras = Extras::default();

        // LTIM
        // TODO: timer extras.ltim = Some(state.inner.timer());

        // GEXT
        let fills = self.fills();
        let entries = &state.inner.entries;

        let gext = fills.write_combined_style(entries);
        extras.gext = Some(gext);

        Ok(extras)
    }

    fn read_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<(Self, {{ puzzle | pascal_case }}State)> {
        let {{ puzzle }} = {{ puzzle | pascal_case }}::new(fills, colors, meta);
        Ok(({{ puzzle }}, state))
    }
}
