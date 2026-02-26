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
        let clue_count = colors.len() + rules.rows.len() + rules.cols.len();
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
        let solution = state.inner.solutions.write_state_grid(|fill| {
            char::try_from(*fill).expect("Solution fill to be valid") as u8
        });

        let state = state
            .inner
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
    ) -> read::Result<(Self, NonogramState)> {
        let (fills, state) = read_state(&grids, &extras)?;
        let colors = read_colors(&fills, &strings)?;
        let meta = read_metadata(&header, &strings);

        let nonogram = Nonogram::new(fills, colors, meta);
        Ok((nonogram, state))
    }
}

fn read_state(grids: &Grids, extras: &Extras) -> read::Result<(Fills, NonogramState)> {
    grids.validate().context("Fills grids")?;
    let cols = grids.width as usize;

    let mut cells = Vec::with_capacity(cols);
    let mut entries = Vec::with_capacity(cols);

    let byte_fill = |ch: char| -> format::Result<Fill> {
        Fill::decode_char(ch).map_err(|err| {
            let boxed_err = Box::new(err);
            format::Error::PuzzleSpecific(boxed_err)
        })
    };

    for ((pos, &solution), &state) in grids.solution.iter_indexed().zip(grids.state.iter()) {
        let style = extras.get_style(pos);

        let cell = match windows_1252_to_char(solution) {
            MISSING_ENTRY_CHAR => Cell::default_with_style(style),
            char => {
                let fill = byte_fill(char).context("Solution fill")?;
                Cell::new_with_style(fill, style)
            }
        };
        cells.push(cell);

        let entry = match windows_1252_to_char(state) {
            MISSING_ENTRY_CHAR => Entry::default_with_style(style),
            char => {
                let fill = byte_fill(char).context("State fill")?;
                Entry::new_with_style(fill, style)
            }
        };
        entries.push(entry);
    }

    let cells = Grid::from_vec(cells, grids.width as usize).expect("Valid grids");
    let solutions = cells.map_ref(|cell| cell.solution);
    let fills = Fills::new(cells);

    let entries = Grid::from_vec(entries, cols).expect("Read correct length state grid");

    // TODO: add back timer let timer = extras.ltim.unwrap_or_default();
    let state = NonogramState::new(solutions, entries);

    Ok((fills, state))
}

fn read_colors(fills: &Fills, strings: &Strings) -> read::Result<Colors> {
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
