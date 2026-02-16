use std::time::Duration;

use crate::{
    CellStyle, Grid, TimerState,
    io::{
        Context, Extras, Gext, Grbs, Ltim, PuzRead, Rtbl, Span, build_string,
        read::{self, PuzState},
    },
};

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

        loop {
            // Try to read a section header
            let result = reader.read_slice::<4>().context("Extras section header");
            let Some(header) = state.ok_or_warn(result)? else {
                break;
            };

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

        Ok(extras)
    }

    fn read_grbs<R: PuzRead>(reader: &mut R, size: usize, width: u8) -> read::Result<Grbs> {
        let grbs = reader.read_vec(size).context("GRBS")?;
        let grbs = Grid::new(grbs, width).expect("Read correct length");

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
            kind: read::ErrorKind::InvalidTimer { reason },
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
        let bytes = Grid::new(bytes, width).expect("Read correct length");
        let mut styles = Vec::with_capacity(size);

        for (&mask, pos) in bytes.iter().zip(bytes.positions()) {
            let Some(style) = CellStyle::from_bits(mask) else {
                return Err(read::Error {
                    span: Span::default(),
                    kind: read::ErrorKind::InvalidCellStyle { pos, mask },
                    context: context.to_string(),
                });
            };

            styles.push(style);
        }

        let gext = Grid::new(styles, width).expect("Read correct length");
        Ok(gext)
    }
}
