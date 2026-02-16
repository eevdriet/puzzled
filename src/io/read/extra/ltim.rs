use std::time::Duration;

use crate::io::{Error, ExtrasError, PuzReader, PuzState, Result};
use crate::{Timer, TimerState};

impl<'a> PuzReader {
    pub(crate) fn parse_ltim(&self, state: &mut PuzState<'a>) -> Result<Timer> {
        // Parse the LTIM string into its elapsed and state parts
        let (ltim, ltim_span) = state.read_span(|s| s.read_str("LTIM"))?;
        let ltim = PuzState::build_string(ltim);

        let err = |reason: String| Error {
            span: ltim_span.clone(),
            context: "LTIM".to_string(),
            kind: ExtrasError::InvalidTimer { reason }.into(),
        };

        let (elapsed_str, state_str) = ltim.split_once(',').ok_or(
            err(format!("Timer needs to be specified as '<elapsed>,<state>' where <elapsed> is a non-negative number and state = 0|1 (found '{ltim}')"))
        )?;

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

        let timer = Timer::new(secs, state);
        Ok(timer)
    }
}
