use std::time::{Duration, Instant};

/// Timer to keep track of how much time is spent playing a puzzle
///
/// The timer can be interacted with in the following way
/// - [`start`](Timer::start) starts a timer that is currently [stopped](TimerState::Stopped)
/// - [`pause`](Timer::pause) pauses a timer that is currently [running](TimerState::Running)
/// - [`toggle`](Timer::toggle) toggles between the stopped and running [state](TimerState)
/// ```
/// use puzzled_core::{Timer, TimerState};
/// use std::time::Duration;
///
/// let mut timer = Timer::new(Duration::ZERO, TimerState::Running);
/// assert_eq!(timer.state(), TimerState::Running);
///
/// timer.pause();
/// assert_eq!(timer.state(), TimerState::Stopped);
///
/// timer.toggle();
/// assert_eq!(timer.state(), TimerState::Running);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Timer {
    elapsed: Duration,
    start: Instant,
    state: TimerState,
}

impl Timer {
    /// Create a new timer in a given [state](TimerState) with an elapsed [duration](Duration)
    pub fn new(elapsed: Duration, state: TimerState) -> Self {
        Self {
            elapsed,
            state,
            start: Instant::now(),
        }
    }

    /// Retrieve the current [state](TimerState) of the timer
    pub fn state(&self) -> TimerState {
        self.state
    }

    /// Start the timer if it is currently [stopped](TimerState::Stopped)
    pub fn start(&mut self) {
        if matches!(self.state, TimerState::Stopped) {
            self.state = TimerState::Running;
            self.start = Instant::now()
        }
    }

    /// Pause the timer if it is currently [running](TimerState::Running)
    pub fn pause(&mut self) {
        if matches!(self.state, TimerState::Running) {
            self.elapsed += self.start.elapsed();
            self.state = TimerState::Stopped;
        }
    }

    /// Toggle the timer [state](TimerState)
    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            TimerState::Stopped => self.start(),
        }
    }

    /// Determine the [duration](Duration) that has elapsed since the timer creation
    pub fn elapsed(&self) -> Duration {
        match self.state {
            TimerState::Stopped => self.elapsed,
            TimerState::Running => self.elapsed + self.start.elapsed(),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(Duration::ZERO, TimerState::default())
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.elapsed() == other.elapsed()
    }
}

impl Eq for Timer {}

/// State variants that a [timer](Timer) can be in
/// ```
/// use puzzled_core::{Timer, TimerState};
///
/// let mut timer = Timer::default();
/// assert_eq!(timer.state(), TimerState::Stopped);
///
/// timer.pause();
/// assert_eq!(timer.state(), TimerState::Stopped);
///
/// timer.start();
/// assert_eq!(timer.state(), TimerState::Running);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// State where the [timer](Timer) is running and [elapsed](Timer::elapsed) is counted
    Running = 0,

    /// State where the [timer](Timer) is stopped and [elapses](Timer::elapsed) time is ignored
    #[default]
    Stopped = 1,
}

impl From<TimerState> for u8 {
    fn from(state: TimerState) -> Self {
        match state {
            TimerState::Running => 0,
            TimerState::Stopped => 1,
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use std::time::{Duration, Instant};

    use serde::{Deserialize, Serialize, de};

    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct TimerData {
        elapsed: u64,
        state: TimerState,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for TimerState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                TimerState::Running => 0,
                TimerState::Stopped => 0,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Timer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            TimerData {
                elapsed: self.elapsed().as_secs(),
                state: self.state,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for TimerState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let inner = u8::deserialize(deserializer)?;
            let state = match inner {
                0 => TimerState::Running,
                1 => TimerState::Stopped,
                _ => {
                    return Err(de::Error::custom(
                        "invalid timer state, expected 0 (running) or 1 (stopped)",
                    ));
                }
            };

            Ok(state)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Timer {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let TimerData { elapsed, state } = TimerData::deserialize(deserializer)?;
            let elapsed = Duration::from_secs(elapsed);

            Ok(Timer {
                start: match state {
                    TimerState::Running => Instant::now() - elapsed,
                    TimerState::Stopped => Instant::now(),
                },
                elapsed,
                state,
            })
        }
    }
}
