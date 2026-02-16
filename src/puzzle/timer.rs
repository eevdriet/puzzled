use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    elapsed: Duration,
    start: Instant,
    state: TimerState,
}

impl Timer {
    pub fn new(elapsed: Duration, state: TimerState) -> Self {
        Self {
            elapsed,
            state,
            start: Instant::now(),
        }
    }

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn start(&mut self) {
        if matches!(self.state, TimerState::Stopped) {
            self.state = TimerState::Running;
            self.start = Instant::now()
        }
    }

    pub fn pause(&mut self) {
        if matches!(self.state, TimerState::Running) {
            self.elapsed += self.start.elapsed();
            self.state = TimerState::Stopped;
        }
    }

    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            TimerState::Stopped => self.start(),
        }
    }

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

#[derive(Debug, Default, Clone, Copy)]
pub enum TimerState {
    #[default]
    Running = 0,

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
