use std::str::FromStr;

use puzzled_core::Color;
use puzzled_io::{
    format,
    text::{
        TxtPuzzle,
        read::{self, TxtState},
    },
};

use crate::{Colors, Fill, Nonogram, NonogramState};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Fill colors should be specified as `<fill_id> : \"<text>\"`")]
    InvalidColorSpec,
}

impl TxtPuzzle<NonogramState> for Nonogram {
    fn read_text(reader: &mut TxtState) -> read::Result<(Nonogram, NonogramState)> {
        let (fills, entries) = reader.read_cells_and_entries()?;

        // Read the colors and metadata
        let colors = read_colors(reader)?;
        let (metadata, timer) = reader.read_metadata(None)?;

        // Create the puzzle and state
        let solutions = fills.map_ref(|cell| cell.solution);
        let timer = timer.unwrap_or_default();
        let state = NonogramState::new(solutions, entries, timer);

        let nonogram = Nonogram::new(fills, colors, metadata);

        Ok((nonogram, state))
    }
}

fn read_colors(reader: &mut TxtState) -> read::Result<Colors> {
    let mut colors = Colors::default();

    fn wrap_err<T>(err: T) -> format::Error
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        format::Error::PuzzleSpecific(Box::new(err))
    }

    while let Some(line) = reader.next_prefixed("-") {
        let (fill_str, color_str) = line
            .split_once(':')
            .ok_or(wrap_err(Error::InvalidColorSpec))?;

        let fill = Fill::from_str(fill_str).map_err(wrap_err)?;

        let color = Color::hex(color_str).map_err(format::Error::Color)?;
        colors.insert(fill, color);
    }

    Ok(colors)
}
