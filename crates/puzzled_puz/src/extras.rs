use std::{collections::BTreeMap, time::Duration};

// use crate::Context;
use crate::{Context, PuzRead, PuzState, PuzWrite, Span, build_string, format, read, write};
use puzzled_core::{CellStyle, Grid, Position, Timer, TimerState};

pub(crate) type Grbs = Grid<u8>;
pub(crate) type Rtbl = BTreeMap<u8, String>;
pub(crate) type Ltim = Timer;
pub(crate) type Gext = Grid<CellStyle>;

#[derive(Debug, Default)]
pub struct Extras {
    /// The GRBS section contains one byte per square of the board.
    /// Each byte indicates whether or not the corresponding square is a rebus.
    /// Possible values are
    /// - `0` to indicate a non-rebus square
    /// - `1+` to indicate a rebus square, the solution for which is given by the entry with key `n` in the [RTBL] section
    pub grbs: Option<Grbs>,

    /// The RTBL section contains a string that represents the solutions for any rebus squares
    /// The solutions are separated by semi-colons and contain the square number and actual rebus
    /// For example, "0:HEART;1:DIAMOND;17:CLUB;23:SPADE" represents 4 rebuses at squares 0, 1, 17 and 23
    pub rtbl: Option<Rtbl>,

    /// The LTIM section contains information on
    /// Specifically, two strings are stored which are separated by a comma.
    /// The former represents how much time the solver has used and the latter whether the timer is running or stopped.
    /// A value of `0` represents the timer running and `1` that the timer is stopped.
    pub ltim: Option<Ltim>,

    /// The GEXT section contains one byte per square of the board
    /// Each byte represents a bitmask indicating that some style attributes are set
    /// The meaning for the following four bits are known:
    /// - `0x10` means that the square was previously marked incorrect
    /// - `0x20` means that the square is currently marked incorrect
    /// - `0x40` means that the contents of the square were given
    /// - `0x80` means that the square is circled
    pub gext: Option<Gext>,
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

/// # Read
impl Extras {
    pub(crate) fn read_from<R: PuzRead>(
        reader: &mut R,
        width: u8,
        height: u8,
        state: &mut PuzState,
    ) -> read::Result<Self> {
        let context = "Extra sections";
        let size = usize::from(width) * usize::from(height);
        let mut extras = Extras::default();

        eprintln!("Extras START");

        loop {
            // Try to read a section header
            let result = reader.read_slice::<4>().context("Extras section header");
            let Some(header) = state.ok_or_warn(result)? else {
                break;
            };

            eprintln!("Found header '{}'", build_string(&header));

            match &header {
                // Try to read valid sections
                b"GRBS" => extras.grbs = state.ok_or_warn(Self::read_grbs(reader, size, width))?,
                b"RTBL" => extras.rtbl = state.ok_or_warn(Self::read_rtbl(reader))?,
                b"LTIM" => extras.ltim = state.ok_or_warn(Self::read_ltim(reader))?,
                b"GEXT" => extras.gext = state.ok_or_warn(Self::read_gext(reader, size, width))?,

                // Warn against invalid section headers
                header => {
                    let result: read::Result<()> = Err(read::Error {
                        span: Span::default(),
                        kind: read::ErrorKind::InvalidSection {
                            found: build_string(header),
                        },
                        context: context.into(),
                    });
                    state.ok_or_warn(result)?;
                }
            }
        }

        eprintln!("Extras END");
        Ok(extras)
    }

    fn read_grbs<R: PuzRead>(reader: &mut R, size: usize, width: u8) -> read::Result<Grbs> {
        let grbs = reader.read_vec(size).context("GRBS")?;
        let grbs = Grid::from_vec(grbs, width as usize).expect("Read correct length");

        Ok(grbs)
    }

    fn read_rtbl<R: PuzRead>(reader: &mut R) -> read::Result<Rtbl> {
        let context = "RTBL";
        let mut rtbl = Rtbl::default();

        let rebuses_str = reader.read_str0().context("RTBL")?;
        let rebuses_str = build_string(&rebuses_str);

        let err = |square: u16, reason: String| read::Error {
            span: Span::default(),
            kind: read::ErrorKind::InvalidRebus { square, reason },
            context: context.into(),
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

    fn read_ltim<R: PuzRead>(reader: &mut R) -> read::Result<Ltim> {
        let context = "LTIM";
        let ltim = reader.read_str0().context(context)?;
        let ltim = build_string(&ltim);

        let err = |reason: String| read::Error {
            span: Span::default(),
            kind: format::Error::InvalidTimer { reason }.into(),
            context: context.into(),
        };

        let (elapsed_str, state_str) = ltim.split_once(',').ok_or(err(format!("Timer needs to be specified as '<elapsed>,<state>' where <elapsed> is a non-negative number and state = 0|1 (found '{ltim}')")))?;

        // Make sure the elapsed time is valid
        let secs: u64 = elapsed_str.parse().map_err(|_| {
            err(format!(
                "Could not parse '{elapsed_str}' into a non-negative number"
            ))
        })?;
        let secs = Duration::from_secs(secs);

        // Make sure the state is valid
        let state_num: u64 = state_str
            .parse()
            .map_err(|_| err(format!("Could not parse '{state_str}' into a 0 or 1")))?;

        // Create the resulting timer
        let state: TimerState = match state_num {
            0 => Ok(TimerState::Running {}),
            1 => Ok(TimerState::Stopped),

            num => Err(err(format!(
                "Number {num} does not represent a valid timer state (use 0 for running, 1 for stopped)"
            ))),
        }?;

        let timer = Ltim::new(secs, state);
        Ok(timer)
    }

    fn read_gext<R: PuzRead>(reader: &mut R, size: usize, width: u8) -> read::Result<Gext> {
        let context = "GEXT";

        let bytes = reader.read_vec(size).context(context.to_string())?;
        let bytes = Grid::from_vec(bytes, width as usize).expect("Read correct length");
        let mut styles = Vec::with_capacity(size);

        for (pos, &mask) in bytes.iter_indexed() {
            let Some(style) = CellStyle::from_mask(mask) else {
                return Err(read::Error {
                    span: Span::default(),
                    kind: read::ErrorKind::InvalidCellStyle { pos, mask },
                    context: context.to_string(),
                });
            };

            styles.push(style);
        }

        let gext = Grid::from_vec(styles, width as usize).expect("Read correct length");
        Ok(gext)
    }
}

/// # Write
impl Extras {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        if let Some(grbs) = &self.grbs {
            writer.write_all(b"GRBS").context("GRBS header")?;

            for (pos, &byte) in grbs.iter_indexed() {
                let context = format!("Square {pos}");
                writer.write_u8(byte).context(context)?;
            }
        }

        if let Some(rtbl) = &self.rtbl {
            writer.write_all(b"RTBL").context("RTBL header")?;

            for (num, rebus) in rtbl {
                let key = format!("{num:02}:{rebus};");
                let context = format!("Rebus #{num}");

                writer.write_all(key.as_bytes()).context(context)?;
            }

            writer.write_u8(0).context("RTBL EOF bit")?;
        }

        if let Some(ltim) = &self.ltim {
            writer.write_all(b"LTIM").context("LTIM header")?;

            let secs = ltim.elapsed().as_secs();
            let state: u8 = ltim.state().into();

            let format = format!("{secs},{state}");
            writer.write_str0(&format).context("LTIM")?;
        }

        if let Some(gext) = &self.gext {
            writer.write_all(b"GEXT").context("GEXT header")?;

            for (pos, &style) in gext.iter_indexed() {
                let context = format!("Cell {pos} style");
                writer.write_u8(style.bits()).context(context)?;
            }
        }

        Ok(())
    }
}
