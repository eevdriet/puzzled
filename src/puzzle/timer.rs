use std::time::{Duration, Instant};

/// Timer to keep track of how much time is spent playing a [puzzle](crate::Puzzle)
///
/// Each puzzle internally keeps a timer that can be accessed through [`timer`](crate::Puzzle::timer) and [`timer_mut`](crate::Puzzle::timer_mut).
/// If the user does not set a timer of their own, a [running](TimerState::Running) timer is attached that has no initial [elapsed](Timer::elapsed) time.
/// The timer can be interacted with in the following way
/// - [`start`](Timer::start) starts a timer that is currently [stopped](TimerState::Stopped)
/// - [`pause`](Timer::pause) pauses a timer that is currently [running](TimerState::Running)
/// - [`toggle`](Timer::toggle) toggles between the stopped and running [state](TimerState)
///
/// While reading a puzzle, the [`PuzReader`](crate::io::PuzReader) can optionally read a timer from the **GEXT** extra section.
/// For more information on how this is achieved, consult the [*.puz specification](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki).
/// ```
/// use puzzled::{Timer, TimerState};
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

/// State variants that a [timer](Timer) can be in
///
/// As per the [*.puz specification](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki), timers are *running* by default.
/// ```
/// use puzzled::{Timer, TimerState};
///
/// let mut timer = Timer::default();
/// assert_eq!(timer.state(), TimerState::Running);
///
/// timer.pause();
/// assert_eq!(timer.state(), TimerState::Stopped);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// State where the [timer](Timer) is running and [elapsed](Timer::elapsed) is counted
    #[default]
    Running = 0,

    /// State where the [timer](Timer) is stopped and [elapses](Timer::elapsed) time is ignored
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
