use delegate::delegate;
use puzzled_core::{Entry, Grid, GridState, Position, Solve, Timer, impl_solve_for_grid_state};

use crate::{Binario, Bit};

#[derive(Debug)]
pub struct BinarioState {
    pub state: GridState<Bit>,
    pub timer: Timer,
}

impl_solve_for_grid_state!(Binario, Bit);

impl BinarioState {
    pub fn new(solutions: Grid<Option<Bit>>, entries: Grid<Entry<Bit>>, timer: Timer) -> Self {
        Self {
            state: GridState { solutions, entries },
            timer,
        }
    }

    pub fn solutions(&self) -> &Grid<Option<Bit>> {
        &self.state.solutions
    }
    pub fn entries(&self) -> &Grid<Entry<Bit>> {
        &self.state.entries
    }
}

impl From<&Binario> for BinarioState {
    fn from(binario: &Binario) -> Self {
        let bits = binario.cells();

        let solutions = bits.map_ref(|cell| cell.solution);
        let entries = bits.map_ref(|cell| Entry::new_with_style(cell.solution, cell.style));
        let timer = Timer::default();

        BinarioState {
            state: GridState { solutions, entries },
            timer,
        }
    }
}

impl Solve<Binario> for BinarioState {
    type Value = Bit;
    type Position = Position;

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

            fn try_finalize(&self) -> Result<Grid<Bit>, Box<dyn std::error::Error>>;
        }
    }
}
