use delegate::delegate;
use puzzled_core::{Entry, Grid, GridState, Line, Position, SidedGrid, Solve, Timer};

use crate::{Binario, Bit, Bits};

#[derive(Debug)]
pub struct BinarioState {
    pub state: GridState<Binario>,
    pub possible: Grid<u8>,
    pub validity: SidedGrid<bool, bool>,
}

impl BinarioState {
    pub fn new(solutions: Grid<Option<Bit>>, entries: Grid<Entry<Bit>>, timer: Timer) -> Self {
        let state = GridState::new(solutions, entries, timer);
        let possible = state.solutions.map_ref(|sol| match sol {
            Some(_) => 0,
            None => 3,
        });
        let validity = SidedGrid::new(state.solutions.map_ref(|_| true))
            .with_top(vec![true; possible.cols()])
            .expect("Checked dimensions")
            .with_left(vec![true; possible.rows()])
            .expect("Checked dimensions");

        Self {
            state,
            possible,
            validity,
        }
    }

    pub fn validate_cell(&mut self, pos: Position) {
        // Retrieve the cell, which is always valid if not filled
        let validate = |pos: Position| {
            let entries = &self.state.entries;
            let Some(cell) = entries[pos].entry() else {
                return true;
            };

            // Compare with the adjacent neighbors for no repeating fills
            let [up, right, down, left] = entries
                .adjacent4(pos)
                .map(|bit| bit.and_then(|b| b.entry()));

            if up.is_some_and(|u| u == cell) && down.is_some_and(|d| d == cell) {
                return false;
            }
            if left.is_some_and(|l| l == cell) && right.is_some_and(|r| r == cell) {
                return false;
            }

            true
        };

        self.validity[pos] = validate(pos);

        let neighbors: Vec<_> = self
            .state
            .entries
            .indexed_adjacent4(pos)
            .into_iter()
            .flatten()
            .map(|(pos, _)| pos)
            .collect();

        for neighbor in neighbors {
            self.validity[neighbor] = validate(neighbor);
        }
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

impl Bits for BinarioState {
    delegate! {
        to self.state.solutions {
            fn middle_bit(&self, pos: Position) -> Option<Bit>;
            fn outer_bits(&self, pos: Position) -> Vec<(Position, Bit)>;
            fn remaining_line_bits(&self, line: Line) -> Vec<(Position, Bit)>;
        }
    }
}

impl Solve<Binario> for BinarioState {
    fn enter(&mut self, pos: &Position, entry: Bit) -> bool {
        let result = self.state.enter(pos, entry);
        self.validate_cell(*pos);

        result
    }

    fn clear(&mut self, pos: &Position) -> bool {
        let result = self.state.clear(pos);
        self.validate_cell(*pos);

        result
    }

    delegate! {
        to self.state {
            fn solve(&mut self, pos: &Position, solution: Bit) -> bool;
            fn reveal(&mut self, pos: &Position) -> bool;
            fn check(&mut self, pos: &Position) -> Option<bool>;

            fn reveal_all(&mut self);
            fn check_all(&mut self);
            fn clear_all(&mut self);

            fn enter_checked(&mut self, pos: &Position, entry: Bit) -> Option<bool>;

            fn guess(&mut self, pos: &Position, guess: Bit) -> bool;

            fn guess_checked(&mut self, pos: &Position, guess: Bit) -> Option<bool>;
        }
    }
}
