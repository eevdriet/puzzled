use puzzled_core::Color;
use puzzled_io::{
    format,
    text::{
        TxtPuzzle,
        read::{self, TxtState},
    },
};

use crate::{Colors, Fill, Fills, Nonogram, Rules};

impl TxtPuzzle for Nonogram {
    fn from_text(reader: &mut TxtState) -> read::Result<Self> {
        // Read in the fills and corresponding rules
        let mut read_fill = |token: &str| -> Fill {
            match token {
                "." => Fill::Blank,
                _ => Fill::Cross,
            }
        };

        let fills = reader.read_grid(&mut read_fill)?;
        let fills = Fills::new(fills);

        let rules = Rules::from_fills(&fills);

        // Read the colors and metadata
        let colors = read_colors(reader)?;
        let metadata = reader.read_metadata(None)?;

        Ok(Nonogram::new(fills, rules, colors, metadata))
    }
}

fn read_colors(reader: &mut TxtState) -> read::Result<Colors> {
    let mut colors = Colors::default();

    let err = |reason: &str| -> read::Error { format::Error::Custom(reason.to_string()).into() };

    while let Some(line) = reader.next_prefixed("-") {
        let (fill_str, color_str) = line.split_once(':').ok_or(err(
            "Fill colors should be specified as `<fill_id> : \"<text>\"`",
        ))?;

        let fill = Fill::try_from(fill_str).map_err(|err| read::Error::Custom(err.to_string()))?;

        let color = Color::hex(color_str).map_err(format::Error::Color)?;
        colors.insert(fill, color);
    }

    Ok(colors)
}
