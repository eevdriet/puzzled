mod entry;
mod grid;
mod lattice;

pub use entry::*;

use crate::Timer;

#[derive(Debug)]
pub struct State<S, E> {
    solutions: S,
    entries: E,
    timer: Timer,
}

impl<S, E> State<S, E> {
    pub fn new(solution: S, entries: E) -> Self {
        let timer = Timer::default();
        Self::new_timed(solution, entries, timer)
    }

    pub fn new_timed(solution: S, entries: E, timer: Timer) -> Self {
        Self {
            solutions: solution,
            entries,
            timer,
        }
    }

    pub fn solution(&self) -> &S {
        &self.solutions
    }

    pub fn solution_mut(&mut self) -> &mut S {
        &mut self.solutions
    }

    pub fn entries(&self) -> &E {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut E {
        &mut self.entries
    }

    /// State of the puzzle's internal timer
    pub fn timer(&self) -> Timer {
        self.timer
    }

    /// Mutable reference to the puzzle's internal timer
    ///
    /// This can be used to start/stop playing the puzzle
    pub fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }
}
