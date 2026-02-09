use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Timer {
    elapsed: Duration,
    state: TimerState,
}

impl Timer {
    pub fn new(elapsed: Duration, state: TimerState) -> Self {
        Self { elapsed, state }
    }

    pub fn start(&mut self) {
        if matches!(self.state, TimerState::Stopped) {
            self.state = TimerState::Running {
                start: Instant::now(),
            }
        }
    }

    pub fn stop(&mut self) {
        if let TimerState::Running { start } = self.state {
            self.elapsed += start.elapsed();
            self.state = TimerState::Stopped;
        }
    }

    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Running { .. } => self.stop(),
            TimerState::Stopped => self.start(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.state {
            TimerState::Stopped => self.elapsed,
            TimerState::Running { start } => self.elapsed + start.elapsed(),
        }
    }
}

#[derive(Debug)]
pub enum TimerState {
    Running { start: Instant },

    Stopped,
}

impl Default for TimerState {
    fn default() -> Self {
        Self::Running {
            start: Instant::now(),
        }
    }
}
