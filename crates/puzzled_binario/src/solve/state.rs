use delegate::delegate;
use puzzled_core::{
    Entry, Grid, GridError, GridState, Line, Position, Solve, Timer, impl_solve_for_grid_state,
};

use crate::{Binario, Bit, Bits};

#[derive(Debug)]
pub struct BinarioState {
    pub state: GridState<Bit>,
    pub timer: Timer,
}

impl_solve_for_grid_state!(Binario, Bit);

impl BinarioState {
    pub fn new(solutions: Grid<Option<Bit>>, entries: Grid<Entry<Bit>>, timer: Timer) -> Self {
        Self {
            timer,
            state: GridState { solutions, entries },
        }
    }

    pub fn solutions(&self) -> &Grid<Option<Bit>> {
        &self.state.solutions
    }
    pub fn entries(&self) -> &Grid<Entry<Bit>> {
        &self.state.entries
    }

    pub fn check_dir(&self, pos: Position) -> Option<Bit> {
        let [up, right, down, left] = self
            .solutions()
            .adjacent4(pos)
            .map(|adj| adj.cloned().expect("At least one layer of indirection"));

        if up.is_some() && down.is_some() && up == down {
            return up;
        }

        if left.is_some() && right.is_some() && left == right {
            return left;
        }

        None
    }

    pub fn check_dir2(&self, pos: Position) -> Option<(Bit, Vec<Position>)> {
        None
    }

    pub fn check_line(&self, pos: Position) -> Option<(Bit, Vec<Position>)> {
        None
    }
}

impl From<&Binario> for BinarioState {
    fn from(binario: &Binario) -> Self {
        let bits = binario.cells();

        let solutions = bits.map_ref(|cell| cell.solution);
        let entries = bits.map_ref(|cell| Entry::new_with_style(cell.solution, cell.style));
        let timer = Timer::default();

        BinarioState::new(solutions, entries, timer)
    }
}

impl Solve<Binario> for BinarioState {
    type Value = Bit;
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

            fn try_finalize(&self) -> Result<Grid<Bit>, Self::Error>;
        }
    }
}

impl Bits for BinarioState {
    delegate! {
        to self.state.solutions {
            fn middle_bit(&self, pos: Position) -> Option<Bit>;
            fn outer_bits(&self, pos: Position) -> Vec<(Position, Bit)>;
            fn remaining_line_bit(&self, line: Line) -> Option<(Position, Bit)>;
        }
    }
}
