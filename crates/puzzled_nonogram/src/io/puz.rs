use puzzled_core::{Color, Grid, Metadata};
use puzzled_io::{
    Context,
    format::{self, StringError},
    puz::{
        BinaryPuzzle, ByteStr, Extras, Grids, Header, PuzSizeCheck, Strings, WriteStyleGrid,
        check_puz_size,
        read::{self, read_cell_entries, read_metadata},
        write::{self, WriteStateGrid},
    },
};

use crate::{Colors, Fill, Fills, Nonogram, NonogramState};

impl PuzSizeCheck for Nonogram {
    fn check_puz_size(&self) -> write::Result<()> {
        let fills = self.fills();
        let colors = self.colors();
        let rules = self.rules();

        // Fills grid is of valid size
        fills.check_puz_size()?;

        // Every fill color fits in a single byte
        let max_size = u8::MAX as usize;

        let color_ids = fills.iter_colors().filter_map(|fill| match fill {
            Fill::Color(id) => Some(id),
            _ => None,
        });

        for &id in color_ids {
            check_puz_size("Fill color", id as usize, max_size)?;
        }

        // Clue count fits into a u16
        let clue_count = colors.len() + rules.len();
        check_puz_size("Clue count", clue_count, u16::MAX as usize)?;

        Ok(())
    }
}

impl BinaryPuzzle<NonogramState> for Nonogram {
    fn width(&self) -> usize {
        self.fills().cols()
    }

    fn height(&self) -> usize {
        self.fills().rows()
    }

    fn clues(&self) -> Vec<ByteStr> {
        self.colors()
            .values()
            .map(|color| format!("{color:?}"))
            .chain(std::iter::empty())
            .map(|str| ByteStr::new(str.as_bytes()))
            .collect()
    }

    fn grids(&self, state: &NonogramState) -> write::Result<(Grid<u8>, Grid<u8>)> {
        let solution = state.state.solutions.write_state_grid(|fill| {
            char::try_from(*fill).expect("Solution fill to be valid") as u8
        });

        let state = state
            .state
            .entries
            .write_state_grid(|fill| char::try_from(*fill).expect("State fill to be valid") as u8);

        Ok((solution, state))
    }

    fn metadata(&self) -> Option<&Metadata> {
        Some(self.meta())
    }

    fn extras(&self, state: &NonogramState) -> write::Result<Extras> {
        let mut extras = Extras::default();

        // LTIM
        // TODO: timer extras.ltim = Some(state.inner.timer());

        // GEXT
        let fills = self.fills();
        let entries = &state.state.entries;

        let gext = fills.write_combined_style(entries);
        extras.gext = Some(gext);

        Ok(extras)
    }

    fn read_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<(Self, NonogramState)> {
        let mut read_fill = |char: char| {
            Fill::decode_char(char)
                .map_err(|err| {
                    let boxed_err = Box::new(err);
                    format::Error::PuzzleSpecific(boxed_err)
                })
                .context("Reading fill")
        };

        let (cells, entries) = read_cell_entries(&grids, &extras, &mut read_fill)?;
        let solutions = cells.map_ref(|cell| cell.solution);

        let colors = read_colors(&cells, &strings)?;
        let meta = read_metadata(&header, &strings);

        let nonogram = Nonogram::new(cells, colors, meta);

        let timer = extras.ltim.unwrap_or_default();
        let state = NonogramState::new(solutions, entries, timer);

        Ok((nonogram, state))
    }
}

fn read_colors(fills: &impl Fills, strings: &Strings) -> read::Result<Colors> {
    // Make sure enough colors are defined to color the fills grid
    let ids = fills.colors_ids();
    let clues = &strings.clues;

    if ids.len() < clues.len() {
        let kind = read::ErrorKind::InvalidClueCount {
            found: ids.len(),
            expected: clues.len(),
        };

        return Err(read::Error::new("Color clues", kind));
    }

    // Then construct the colors
    fn get_colors(ids: Vec<u32>, clues: &[ByteStr]) -> format::Result<Colors> {
        let mut colors = Colors::default();

        for (id, color_byte_str) in ids.into_iter().zip(clues.iter()) {
            let color_str = str::from_utf8(color_byte_str.bytes(false))
                .map_err(|err| format::Error::String(StringError::Utf8Error(err)))?;

            let fill = Fill::Color(id);
            let color = Color::hex(color_str)?;

            colors.insert(fill, color);
        }

        Ok(colors)
    }

    let colors = get_colors(ids, clues).context("Color clues")?;

    Ok(colors)
}
