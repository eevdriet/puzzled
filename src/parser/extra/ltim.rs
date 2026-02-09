use std::time::Duration;

use crate::{Error, ExtrasError, Parser, Result, parse_string};

#[derive(Debug)]
pub enum TimerState {
    Running,
    Stopped,
}

impl TryFrom<u64> for TimerState {
    type Error = ExtrasError;

    fn try_from(num: u64) -> std::result::Result<Self, Self::Error> {
        match num {
            0 => Ok(TimerState::Running),
            1 => Ok(TimerState::Stopped),
            num => Err(ExtrasError::InvalidTimer {
                reason: format!(
                    "Number {num} does not represent a valid timer state (use 0 for running, 1 for stopped)"
                ),
            }),
        }
    }
}

#[derive(Debug)]
pub struct Timer {
    elapsed: Duration,
    state: TimerState,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_ltim(&mut self) -> Result<Timer> {
        // Parse the LTIM string into its elapsed and state parts
        let ltim = self.read_str("LTIM")?;
        let ltim = parse_string(ltim);

        let Some((elapsed_str, state_str)) = ltim.split_once(',') else {
            return Err(ExtrasError::InvalidTimer {
                    reason: format!(
                        "Timer needs to be specified as '<elapsed>,<state>' where <elapsed> is a non-negative number and state = 0|1 (found '{ltim}')"
                    ),
                }.into());
        };

        // Make sure the elapsed time is valid
        let secs: u64 = elapsed_str.parse().map_err(|_| {
            Into::<Error>::into(ExtrasError::InvalidTimer {
                reason: format!("Could not parse '{elapsed_str}' into a non-negative number"),
            })
        })?;

        // Make sure the state is valid
        let state_num: u64 = state_str.parse().map_err(|_| {
            Into::<Error>::into(ExtrasError::InvalidTimer {
                reason: format!("Could not parse '{state_str}' into a 0 or 1"),
            })
        })?;

        // Create the resulting timer
        let state: TimerState = state_num.try_into()?;

        Ok(Timer {
            elapsed: Duration::from_secs(secs),
            state,
        })
    }
}
