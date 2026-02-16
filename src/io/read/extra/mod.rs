mod error;
mod ltim;
mod section;

pub use error::*;
pub use section::*;

use std::{collections::HashMap, str::FromStr};

use crate::io::{Error, PuzReader, PuzState, Result, Span};
use crate::{CellStyle, Grid, Position, Timer};

/// In some .puz files, extra sections are used to indicate additional properties on the solving state.
#[derive(Debug, Default)]
pub struct Extras {
    /// The GRBS section contains one byte per square of the board.
    /// Each byte indicates whether or not the corresponding square is a rebus.
    /// Possible values are
    /// - `0` to indicate a non-rebus square
    /// - `1+` to indicate a rebus square, the solution for which is given by the entry with key `n` in the [RTBL] section
    pub grbs: Option<Grid<u8>>,

    pub grbs_span: Option<Span>,

    /// The RTBL section contains a string that represents the solutions for any rebus squares
    /// The solutions are separated by semi-colons and contain the square number and actual rebus
    /// For example, "0:HEART;1:DIAMOND;17:CLUB;23:SPADE" represents 4 rebuses at squares 0, 1, 17 and 23
    pub rtbl: Option<HashMap<u8, String>>,

    /// The LTIM section contains information on
    /// Specifically, two strings are stored which are separated by a comma.
    /// The former represents how much time the solver has used and the latter whether the timer is running or stopped.
    /// A value of `0` represents the timer running and `1` that the timer is stopped.
    pub ltim: Option<Timer>,

    /// The GEXT section contains one byte per square of the board
    /// Each byte represents a bitmask indicating that some style attributes are set
    /// The meaning for the following four bits are known:
    /// - `0x10` means that the square was previously marked incorrect
    /// - `0x20` means that the square is currently marked incorrect
    /// - `0x40` means that the contents of the square were given
    /// - `0x80` means that the square is circled
    pub gext: Option<Grid<CellStyle>>,
}

impl Extras {
    pub fn get_rebus(&self, pos: Position) -> Option<&String> {
        let (Some(grbs), Some(rtbl)) = (&self.grbs, &self.rtbl) else {
            return None;
        };

        let rebus = grbs.get(pos)?;
        rtbl.get(rebus)
    }

    pub fn get_style(&self, pos: Position) -> CellStyle {
        match &self.gext {
            None => CellStyle::default(),
            Some(gext) => *gext.get(pos).unwrap_or(&CellStyle::default()),
        }
    }
}

impl<'a> PuzReader {
    pub(crate) fn parse_extras(
        &self,
        width: u8,
        height: u8,
        state: &mut PuzState<'a>,
    ) -> Result<Extras> {
        let size = u16::from(width) * u16::from(height);
        let mut extras = Extras::default();

        while !state.reached_eof() {
            // Parse the header of the next extra section
            let result = state.read_span(|p| p.read_fixed_len_str(5, "Extras section"));
            let Some((header, header_span)) = self.ok_or_warn(result, state)? else {
                continue;
            };

            let header = PuzState::build_string(header);

            // Short-circuit if the section doesn't have the correct header
            let section = self.ok_or_warn(
                ExtraSection::from_str(&header).map_err(|_| Error {
                    span: header_span,
                    kind: ExtrasError::InvalidSection { found: header }.into(),
                    context: "Extras section header".to_string(),
                }),
                state,
            )?;

            let Some(section) = section else {
                return Ok(extras);
            };

            // Otherwise, parse the next section
            match section {
                ExtraSection::Grbs => match state.read_span(|s| self.parse_grbs(size, width, s)) {
                    Ok((grbs, grbs_span)) => {
                        extras.grbs = Some(grbs);
                        extras.grbs_span = Some(grbs_span);
                    }
                    err @ Err(_) => {
                        self.ok_or_warn(err, state)?;
                    }
                },
                ExtraSection::Rtbl => {
                    let rtbl = self.parse_rtbl(state);
                    extras.rtbl = self.ok_or_warn(rtbl, state)?;
                }
                ExtraSection::Ltim => {
                    let ltim = self.parse_ltim(state);
                    extras.ltim = self.ok_or_warn(ltim, state)?;
                }
                ExtraSection::Gext => {
                    let gext = self.parse_gext(size as usize, width, state);
                    extras.gext = self.ok_or_warn(gext, state)?;
                }
            }
        }

        self.validate_extras(extras, state)
    }

    fn parse_grbs(&self, size: u16, width: u8, state: &mut PuzState<'a>) -> Result<Grid<u8>> {
        let grbs = state.read(size as usize, "GRBS")?;
        let grid = Grid::new(grbs.to_vec(), width).expect("Read correct length region");

        Ok(grid)
    }

    fn parse_rtbl(&self, state: &mut PuzState<'a>) -> Result<HashMap<u8, String>> {
        let mut rtbl = HashMap::default();
        let (rebuses_str, rtbl_span) = state.read_span(|p| p.read_str("RTBL"))?;
        let rebuses_str = PuzState::build_string(rebuses_str);

        let err = |square: u16, reason: String| Error {
            span: rtbl_span.clone(),
            context: "RTBL".to_string(),
            kind: ExtrasError::InvalidRebus { square, reason }.into(),
        };

        for (idx, entry) in rebuses_str.split(';').enumerate() {
            let square = idx as u16 + 1;
            let entry = entry.trim();

            // Skip empty trailing part (common because string ends with ;)
            if entry.is_empty() {
                continue;
            }

            // Split rebus into identifying number and text
            let (num_str, value) = entry.split_once(':').ok_or_else(|| {
                err(
                    square,
                    format!("'{entry}' should be formatted as '<num>:<rebus>'"),
                )
            })?;

            // Make sure the number is specified as a 2-digit ASCII string
            if num_str.len() != 2 {
                return Err(err(
                    square,
                    format!(
                        "Number should be formated as a 2-digit ASCII string, with an optional leading digit (found '{num_str}')"
                    ),
                ));
            }

            // Try to parse the rebus number
            let num: u8 = num_str.trim().parse().map_err(|_| {
                err(
                    square,
                    format!("Could not parse '{num_str}' into a valid rebus number"),
                )
            })?;

            // Make sure the rebus number is in-bounds
            if num >= 100 {
                return Err(err(
                    square,
                    format!("expected number to be in 0..100, found {num}"),
                ));
            }

            rtbl.insert(num, value.trim().to_string());
        }

        Ok(rtbl)
    }

    fn parse_gext(
        &self,
        size: usize,
        width: u8,
        state: &mut PuzState<'a>,
    ) -> Result<Grid<CellStyle>> {
        // Parse the underlying bits for the GEXT
        let (gext, gext_span) = state.read_span(|p| p.read(size, "GEXT"))?;
        let gext = Grid::new(gext.to_vec(), width).expect("Read correct length region");

        // Make sure each bitmask in GEXT represents a valid square style
        let mut styles = Vec::with_capacity(size);

        for (idx, (&mask, pos)) in gext.iter().zip(gext.positions()).enumerate() {
            let start = gext_span.start + idx;

            let Some(style) = CellStyle::from_bits(mask) else {
                return Err(Error {
                    span: start..start + 1,
                    kind: ExtrasError::InvalidBitmask { pos, mask }.into(),
                    context: "GEXT".to_string(),
                });
            };

            styles.push(style);
        }

        // Then transmute the GEXT to a styles grid
        let grid = Grid::new(styles, width).expect("Read correct length region");
        Ok(grid)
    }

    fn validate_extras(&self, extras: Extras, state: &mut PuzState<'a>) -> Result<Extras> {
        // Iterate over all rebuses specified in GRBS
        if let (Some(grbs), Some(rtbl)) = (&extras.grbs, &extras.rtbl) {
            for (&rebus, pos) in grbs
                .iter()
                .zip(grbs.positions())
                .filter(|&(&rebus, _)| rebus != 0)
            {
                // Make sure each rebus in GRBS has a definition in RTBL
                if !rtbl.contains_key(&rebus) {
                    let err: Result<()> = Err(Error {
                        span: 0..0,
                        kind: ExtrasError::MissingRebus { pos, rebus }.into(),
                        context: "RTBL".to_string(),
                    });

                    self.ok_or_warn(err, state)?;
                }
            }
        }

        Ok(extras)
    }
}
