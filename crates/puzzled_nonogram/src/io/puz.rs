use puzzled_core::{Color, Entry, Grid};
use puzzled_io::{
    Context,
    format::{self, StringError},
    puz::{
        BinaryPuzzle, Extras, Grids, Header, MISSING_ENTRY_CHAR, PuzSizeCheck, PuzWrite, Strings,
        check_puz_size,
        read::{self, read_metadata},
        write,
    },
};

use crate::{Colors, Fill, Fills, Nonogram, NonogramCell};

impl PuzSizeCheck for Fills {
    fn check_puz_size(&self) -> write::Result<()> {
        // Make sure the grid is of valid size
        self.0.check_puz_size()?;

        // Make sure every color fits in a single byte
        let max_size = u8::MAX as usize;

        let color_ids = self.iter_colors().filter_map(|fill| match fill {
            Fill::Color(id) => Some(id),
            _ => None,
        });

        for &id in color_ids {
            check_puz_size("Fill color", id as usize, max_size)?;
        }

        Ok(())
    }
}

impl PuzSizeCheck for Colors {
    fn check_puz_size(&self) -> write::Result<()> {
        let max_color_id = u8::MAX as usize;
        let color_ids = self.keys().filter_map(|fill| match fill {
            Fill::Color(id) => Some(id),
            _ => None,
        });

        for &color_id in color_ids {
            check_puz_size("Fill colors", color_id as usize, max_color_id)?;
        }

        Ok(())
    }
}

impl BinaryPuzzle for Nonogram {
    fn write_header(&self, state: &Self::State) -> write::Result<Header> {
        let mut header = Header::default();

        // Grids
        let fills = self.fills();
        fills.check_puz_size()?;
        header.width = fills.cols() as u8;
        header.height = fills.rows() as u8;

        // Clues
        let colors = self.colors();
        colors.check_puz_size()?;
        header.clue_count = colors.len() as u16;

        // Metadata
        header.write_cib();

        Ok(header)
    }

    fn write_grids(&self, state: &Self::State) -> write::Result<Grids> {
        // Get the squares and check for overflow of their size
        let fills = self.fills();
        fills.check_puz_size()?;

        let width = fills.rows() as u8;
        let height = fills.cols() as u8;

        // Write the individual grids from the squares
        let solution = fills.map_ref(|cell| {
            let fill = *cell.solution();
            let fill_char = char::try_from(fill).expect("Checked fills");

            fill_char as u8
        });

        let state = fills.map_ref(|cell| {
            let fill = *cell
                .entry()
                .unwrap_or(&Fill::Color(MISSING_ENTRY_CHAR as u32));
            let fill_char = char::try_from(fill).expect("Checked fills");

            fill_char as u8
        });

        // Construct the result and validate
        let grids = Grids {
            solution,
            state,
            width,
            height,
        };

        Ok(grids)
    }

    fn write_strings(&self, state: &Self::State) -> write::Result<Strings> {
        let colors = self.colors();
        colors.check_puz_size()?;

        let mut strings = Strings::from_metadata(self.)

        let mut strings = Strings {
            clues: Vec::with_capacity(colors.len()),
            ..Default::default()
        };

        for (idx, &color) in colors.values().enumerate() {
            let color_str = format!("{color:?}");

            strings.clues[idx].write_str0(&color_str).expect("Clue");
        }

        Ok(strings)
    }

    fn write_extras(&self, state: &Self::State) -> write::Result<Extras> {
        let extras = Extras::default();

        Ok(extras)
    }

    fn read_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<Self> {
        let fills = read_fills(&grids)?;
        let colors = read_colors(&fills, &strings)?;
        let meta = read_metadata(&header, &strings);

        let nonogram = Nonogram::new(fills, colors, meta);
        Ok(nonogram)
    }
}

fn read_fills(grids: &Grids) -> read::Result<Fills> {
    grids.validate().context("Fills grids")?;

    let size = usize::from(grids.width) * usize::from(grids.height);
    let mut data = Vec::with_capacity(size);

    let byte_fill = |byte: u8| -> format::Result<Fill> {
        Fill::decode_char(byte as char).map_err(|err| {
            let boxed_err = Box::new(err);
            format::Error::PuzzleSpecific(boxed_err)
        })
    };

    for (&solution_byte, &state_byte) in grids.solution.iter().zip(grids.state.iter()) {
        // Create the inner cell with its solution
        let solution = byte_fill(solution_byte).context("Solution fill")?;
        let mut cell = Entry::new(solution);

        // Optionally enter a state
        let state = byte_fill(state_byte).context("State fill")?;
        if char::try_from(state).is_ok_and(|ch| ch != MISSING_ENTRY_CHAR) {
            cell.enter(state);
        }

        // Create cell
        let cell = NonogramCell::new(cell);
        data.push(cell);
    }

    let fills = Grid::from_vec(data, grids.width as usize).expect("Valid grids");
    let fills = Fills::new(fills);

    Ok(fills)
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
    fn get_colors(ids: Vec<u32>, clues: &[Vec<u8>]) -> format::Result<Colors> {
        let mut colors = Colors::default();

        for (id, color_bytes) in ids.into_iter().zip(clues.iter()) {
            let color_str = str::from_utf8(color_bytes)
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
