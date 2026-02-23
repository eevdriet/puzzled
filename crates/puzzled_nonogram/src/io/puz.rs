use puzzled_core::{Color, Grid};
use puzzled_io::{
    Context,
    format::{self, StringError},
    puz::{
        Extras, Grids, Header, Puz, PuzSizeCheck, PuzWrite, Strings, check_puz_size,
        read::{self, read_metadata},
        write,
    },
};

use crate::{Colors, Fill, Fills, Nonogram, Rules};

impl PuzSizeCheck for Colors {
    fn check_puz_size(&self) -> write::Result<()> {
        let max_color_id = u8::MAX as usize;
        let color_ids = self.keys().filter_map(|fill| match fill {
            Fill::Color(id) => Some(id),
            _ => None,
        });

        for &color_id in color_ids {
            check_puz_size("Fill colors", color_id, max_color_id)?;
        }

        Ok(())
    }
}

impl Puz for Nonogram {
    fn to_header(&self) -> write::Result<Header> {
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

    fn to_grids(&self) -> write::Result<Grids> {
        // Get the squares and check for overflow of their size
        let fills = self.fills();
        fills.check_puz_size()?;

        let width = fills.rows() as u8;
        let height = fills.cols() as u8;

        // Write the individual grids from the squares
        let solution = fills.map_ref(|_| 0);

        let state = fills.map_ref(|fill| fill.byte() as u8);

        // Construct the result and validate
        let grids = Grids {
            solution,
            state,
            width,
            height,
        };

        Ok(grids)
    }

    fn to_strings(&self) -> write::Result<Strings> {
        let colors = self.colors();
        colors.check_puz_size()?;

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

    fn to_extras(&self) -> write::Result<Extras> {
        let extras = Extras::default();

        Ok(extras)
    }

    fn from_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<Self> {
        let (fills, rules) = read_fills_and_rules(&grids)?;
        let colors = read_colors(&fills, &strings)?;
        let meta = read_metadata(&header, &strings, &extras);

        let nonogram = Nonogram::new(fills, rules, colors, meta);
        Ok(nonogram)
    }
}

fn read_fills_and_rules(grids: &Grids) -> read::Result<(Fills, Rules)> {
    // Map the solution and state grids into fills
    let get_fills = |grid: &Grid<u8>| -> format::Result<Fills> {
        let mut fills = Vec::with_capacity(grid.size());

        for &byte in grid.iter() {
            let fill_char = byte as char;
            let fill = Fill::decode_char(fill_char).map_err(|err| {
                let boxed_err = Box::new(err);
                format::Error::PuzzleSpecific(boxed_err)
            })?;

            fills.push(fill);
        }

        // let grid = grid.map_ref(|&byte| Fill::decode_byte(byte));
        let fills = Grid::from_vec(fills, grid.cols()).expect("Valid grid");

        Ok(Fills::new(fills))
    };

    let solution_fills = get_fills(&grids.solution).context("Solution fills")?;
    let fills = get_fills(&grids.state).context("State fills")?;

    // Construct rules from the solution fills
    let rules = Rules::from_fills(&solution_fills);

    Ok((fills, rules))
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
    fn get_colors(ids: Vec<usize>, clues: &[Vec<u8>]) -> format::Result<Colors> {
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
