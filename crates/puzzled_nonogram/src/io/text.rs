use puzzled_core::{Cell, Color};
use puzzled_io::{
    format,
    text::{
        TxtPuzzle,
        read::{self, TxtState},
    },
};

use crate::{Colors, Fill, Fills, Nonogram};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Fill colors should be specified as `<fill_id> : \"<text>\"`")]
    InvalidColorSpec,
}

impl TxtPuzzle for Nonogram {
    fn read_text(reader: &mut TxtState) -> read::Result<Self> {
        // Read in the fills and corresponding rules
        let mut read_fill = |line: &str| {
            line.split_whitespace()
                .map(|token| {
                    let fill = match token {
                        "." => Fill::Blank,
                        _ => Fill::Cross,
                    };

                    Cell::new(fill)
                })
                .collect()
        };

        let fills = reader.read_grid(&mut read_fill)?;
        let fills = Fills::new(fills);

        // Read the colors and metadata
        let colors = read_colors(reader)?;
        let metadata = reader.read_metadata(None)?;

        Ok(Nonogram::new(fills, colors, metadata))
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

        let fill = Fill::try_from(fill_str).map_err(wrap_err)?;

        let color = Color::hex(color_str).map_err(format::Error::Color)?;
        colors.insert(fill, color);
    }

    Ok(colors)
}
