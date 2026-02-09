mod error;
mod ltim;
mod section;

pub use error::*;
pub use ltim::*;
pub use section::*;

use std::{collections::HashMap, str::FromStr};

use crate::{Parser, Result, Timer, parse_string};

const ALLOWED_GEXT_BITS: u8 = 0x10 | 0x20 | 0x40 | 0x80; // 0xF0

/// In some .puz files, extra sections are used to indicate additional properties on the solving state.
#[derive(Debug, Default)]
pub struct Extras<'a> {
    /// The GRBS section contains one byte per square of the board.
    /// Each byte indicates whether or not the corresponding square is a rebus.
    /// Possible values are
    /// - `0` to indicate a non-rebus square
    /// - `1+` to indicate a rebus square, the solution for which is given by the entry with key `n` in the [RTBL] section
    pub grbs: Option<&'a [u8]>,

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
    pub gext: Option<&'a [u8]>,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_extras(&mut self, width: u8, height: u8) -> Result<Extras<'a>> {
        let size = u16::from(width) * u16::from(height);
        let mut extras = Extras::default();
        let mut parsed = true;

        while parsed {
            parsed = false;

            // Parse the header of the next extra section
            let header = self.read_fixed_len_str(4, "Extras section")?;
            let header = parse_string(header);

            // Short-circuit if the section doesn't have the correct header
            let section = self.ok_or_warn(ExtraSection::from_str(&header).map_err(Into::into))?;
            let Some(section) = section else {
                return Ok(extras);
            };

            // Otherwise, parse the next section
            match section {
                ExtraSection::Grbs => {
                    let grbs = self.parse_grbs(size);
                    extras.grbs = self.ok_or_warn(grbs)?;

                    parsed = true;
                }
                ExtraSection::Rtbl => {
                    let rtbl = self.parse_rtbl();
                    extras.rtbl = self.ok_or_warn(rtbl)?;

                    parsed = true;
                }
                ExtraSection::Ltim => {
                    let ltim = self.parse_ltim();
                    extras.ltim = self.ok_or_warn(ltim)?;

                    parsed = true;
                }
                ExtraSection::Gext => {
                    let gext = self.parse_gext(size);
                    extras.gext = self.ok_or_warn(gext)?;

                    parsed = true;
                }
            }
        }

        self.validate_extras(extras)
    }

    fn parse_grbs(&mut self, size: u16) -> Result<&'a [u8]> {
        self.read(size as usize, "GRBS")
    }

    fn parse_rtbl(&mut self) -> Result<HashMap<u8, String>> {
        let mut rtbl = HashMap::default();
        let rebuses_str = self.read_str("RTBL")?;
        let rebuses_str = parse_string(rebuses_str);

        for (idx, entry) in rebuses_str.split(';').enumerate() {
            let cell = idx as u16 + 1;
            let entry = entry.trim();

            // Skip empty trailing part (common because string ends with ;)
            if entry.is_empty() {
                continue;
            }

            // Split rebus into identifying number and text
            let (num_str, value) =
                entry
                    .split_once(':')
                    .ok_or_else(|| ExtrasError::InvalidRebus {
                        rebus: cell,
                        reason: format!("'{entry}' should be formatted as '<num>:<rebus>'"),
                    })?;

            // Make sure the number is specified as a 2-digit ASCII string
            if num_str.len() != 2 {
                return Err(ExtrasError::InvalidRebus {
                    rebus: cell,
                    reason: format!(
                        "Number should be formated as a 2-digit ASCII string, with an optional leading digit (found '{num_str}')"
                    ),
                }.into());
            }

            // Try to parse the rebus number
            let num: u8 = num_str
                .trim()
                .parse()
                .map_err(|_| ExtrasError::InvalidRebus {
                    rebus: cell,
                    reason: format!("Could not parse '{num_str}' into a valid rebus number"),
                })?;

            // Make sure the rebus number is in-bounds
            if num >= 100 {
                return Err(ExtrasError::InvalidRebus {
                    rebus: num as u16,
                    reason: format!("expected number to be in 0..100, found {num}"),
                }
                .into());
            }

            rtbl.insert(num, value.trim().to_string());
        }

        Ok(rtbl)
    }

    fn parse_gext(&mut self, size: u16) -> Result<&'a [u8]> {
        self.read(size as usize, "GEXT")
    }

    fn validate_extras(&mut self, extras: Extras<'a>) -> Result<Extras<'a>> {
        // Iterate over all rebuses specified in GRBS
        if let (Some(grbs), Some(rtbl)) = (&extras.grbs, &extras.rtbl) {
            for (idx, _num) in grbs.iter().enumerate().filter(|&(_, num)| *num != 0) {
                let cell = idx as u8 + 1;

                // Make sure each rebus in GRBS has a definition in RTBL
                if !rtbl.contains_key(&cell) {
                    let err: Result<()> = Err(ExtrasError::MissingRebus { rebus: cell }.into());
                    self.ok_or_warn(err)?;
                }
            }
        }

        // Make sure each bitmask in GEXT is valid
        if let Some(gext) = &extras.gext {
            for (idx, &mask) in gext.iter().enumerate() {
                let cell = idx as u16 + 1;

                if mask & !ALLOWED_GEXT_BITS != 0 {
                    let err: Result<()> = Err(ExtrasError::InvalidBitmask { cell, mask }.into());
                    self.ok_or_warn(err)?;
                }
            }
        }

        Ok(extras)
    }
}
