use std::collections::VecDeque;

use puzzled_core::{Entry, Grid, GridState, Position, Timer};

use crate::{Skyscraper, Skyscrapers};

#[derive(Debug)]
pub struct SkyscraperState {
    pub state: GridState<Skyscrapers>,
    pub timer: Timer,

    pub(crate) _frontier: VecDeque<(Position, Skyscraper)>,
}

impl SkyscraperState {
    pub fn new(
        solutions: Grid<Option<Skyscraper>>,
        entries: Grid<Entry<Skyscraper>>,
        timer: Timer,
    ) -> Self {
        Self {
            state: GridState {
                solutions,
                entries,
                timer,
            },
            timer,
            _frontier: VecDeque::default(),
        }
    }

    pub fn solutions(&self) -> &Grid<Option<Skyscraper>> {
        &self.state.solutions
    }
    pub fn entries(&self) -> &Grid<Entry<Skyscraper>> {
        &self.state.entries
    }
}

impl From<&Skyscrapers> for SkyscraperState {
    fn from(skyscrapers: &Skyscrapers) -> Self {
        let cells = skyscrapers.cells();

        let solutions = cells.map_ref(|cell| cell.solution);
        let entries = cells.map_ref(|cell| Entry::new_with_style(cell.solution, cell.style));
        let timer = Timer::default();

        SkyscraperState::new(solutions, entries, timer)
    }
}
