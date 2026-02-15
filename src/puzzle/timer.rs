use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    elapsed: Duration,
    state: TimerState,
}

impl Timer {
    pub(crate) fn new(elapsed: Duration, state: TimerState) -> Self {
        Self { elapsed, state }
    }

    pub fn new_stopped(elapsed: Duration) -> Self {
        Self {
            elapsed,
            state: TimerState::Stopped,
        }
    }

    pub fn new_running(elapsed: Duration) -> Self {
        Self {
            elapsed,
            state: TimerState::Running {
                start: Instant::now(),
            },
        }
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

impl Default for Timer {
    fn default() -> Self {
        Self::new_running(Duration::ZERO)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TimerState {
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
