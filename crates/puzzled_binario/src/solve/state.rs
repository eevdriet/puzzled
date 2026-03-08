use delegate::delegate;
use derive_more::{Deref, DerefMut, Display};
use puzzled_core::{Entry, Grid, GridState, Line, Position, Timer, impl_solve_for_grid_state};

use crate::{Binario, Bit, Bits};

#[derive(Deref, DerefMut, Display)]
pub struct BinarioState(pub GridState<Bit>);

impl BinarioState {
    pub fn new(solutions: Grid<Option<Bit>>, entries: Grid<Entry<Bit>>, timer: Timer) -> Self {
        let state = GridState::new(solutions, entries, timer);
        Self(state)
    }
}

impl_solve_for_grid_state!(BinarioState, 0, Binario, Bit);

impl From<&Binario> for BinarioState {
    fn from(binario: &Binario) -> Self {
        let bits = binario.cells();

        let solutions = bits.map_ref(|cell| cell.solution);
        let entries = bits.map_ref(|cell| Entry::new_with_style(cell.solution, cell.style));
        let timer = Timer::default();

        BinarioState::new(solutions, entries, timer)
    }
}

impl Bits for BinarioState {
    delegate! {
        to self.solutions {
            fn middle_bit(&self, pos: Position) -> Option<Bit>;
            fn outer_bits(&self, pos: Position) -> Vec<(Position, Bit)>;
            fn remaining_line_bits(&self, line: Line) -> Vec<(Position, Bit)>;
        }
    }
}
