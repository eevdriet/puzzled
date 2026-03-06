use std::collections::VecDeque;

use delegate::delegate;
use puzzled_core::{Entry, Grid, GridState, Position, Solve, Timer, impl_solve_for_grid_state};

use crate::{Skyscraper, Skyscrapers};

#[derive(Debug)]
pub struct SkyscraperState {
    pub state: GridState<Skyscraper>,
    pub timer: Timer,

    pub(crate) frontier: VecDeque<(Position, Skyscraper)>,
}

impl_solve_for_grid_state!(Skyscrapers, Skyscraper);

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
            frontier: VecDeque::default(),
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

impl Solve<Skyscrapers> for SkyscraperState {
    type Value = Skyscraper;
    type Position = Position;
    type Error = String;

    delegate! {
        to self.state {
            fn solve(&mut self, pos: &Self::Position, solution: Self::Value) -> bool;
            fn enter(&mut self, pos: &Self::Position, entry: Self::Value) -> bool;
            fn reveal(&mut self, pos: &Self::Position) -> bool;
            fn check(&mut self, pos: &Self::Position) -> Option<bool>;

            fn reveal_all(&mut self);
            fn check_all(&mut self);

            fn enter_checked(&mut self, pos: &Self::Position, entry: Self::Value) -> Option<bool>;

            fn guess(&mut self, pos: &Self::Position, guess: Self::Value) -> bool;

            fn guess_checked(&mut self, pos: &Self::Position, guess: Self::Value) -> Option<bool>;

            fn try_finalize(&self) -> Result<Grid<Skyscraper>, Self::Error>;
        }
    }
}
