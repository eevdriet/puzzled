use delegate::delegate;
use puzzled_core::{
    Entry, Grid, GridLinearIter, GridState, Line, Order, Position, SidedGrid, Solve, Timer,
};

use crate::{Binario, Bit, Bits};

#[derive(Debug)]
pub struct BinarioState {
    pub state: GridState<Binario>,

    pub valid: SidedGrid<u8, bool, (isize, isize), (isize, isize), bool>,
}

impl BinarioState {
    pub fn new(solutions: Grid<Option<Bit>>, entries: Grid<Entry<Bit>>, timer: Timer) -> Self {
        let right: Vec<_> = solutions
            .iter_rows()
            .map(|row| BinarioState::count_bits(row, entries.cols() / 2))
            .collect();
        let bottom: Vec<_> = solutions
            .iter_cols()
            .map(|col| BinarioState::count_bits(col, entries.rows() / 2))
            .collect();

        let state = GridState::new(solutions, entries, timer);
        let possible = state.solutions.map_ref(|sol| match sol {
            Some(bit) => u8::from(*bit),
            None => 2,
        });

        Self {
            state,
            valid: SidedGrid::new(possible)
                .with_top_value(true)
                .with_right(right)
                .expect("Correct number of counts")
                .with_bottom(bottom)
                .expect("Correct number of counts")
                .with_left_value(true),
        }
    }

    pub fn validate_cell(&mut self, pos: Position) {
        // Retrieve the bit, which is valid (not incorrect) when not filled
        let Some(bit) = self.state.entries[pos].entry().cloned() else {
            return;
        };

        tracing::info!("Validating {pos}");
        tracing::info!("\tEntry {bit}");

        // Compare with the adjacent neighbors for no repeating fills
        let [up, right, down, left] = self
            .state
            .entries
            .adjacent4(pos)
            .map(|bit| bit.and_then(|b| b.entry().cloned()));

        // Mark the entry as incorrect if it shares its bit with orthogonal neighbors
        let entry = &mut self.state.entries[pos];

        if up.is_some_and(|u| u == bit) && down.is_some_and(|d| d == bit) {
            tracing::info!("\tIncorrect up/down");
            entry.mark_incorrect();
        } else if left.is_some_and(|l| l == bit) && right.is_some_and(|r| r == bit) {
            tracing::info!("\tIncorrect left/right");
            entry.mark_incorrect();
        } else {
            entry.reset_correctness();
        }
    }

    pub fn validate_line(&mut self, line: Line) {
        let entries = &mut self.state.entries;

        // Mark the line as incorrect if it is equal to another
        let order = Order::from(line);
        let iter = entries.iter_line(line);
        let pos = line.line();

        for (idx, other_iter) in entries.iter_lines(order).enumerate() {
            if idx != pos && iter.eq(other_iter) {
                return;
            }
        }

        // Mark the line as incorrect if it is full
        // but doesn't have the same number of zeroes and ones
    }

    pub fn update_count(&mut self, line: Line, bit: Bit, is_added: bool) {
        let counts = if line.is_row() {
            self.valid.right.as_mut().expect("Should be defined")
        } else {
            self.valid.bottom.as_mut().expect("Should be defined")
        };

        let Some((zeroes, ones)) = counts.get_mut(line.line()) else {
            return;
        };

        match (bit, is_added) {
            (Bit::Zero, false) => *zeroes += 1,
            (Bit::Zero, true) => *zeroes -= 1,
            (Bit::One, false) => *ones += 1,
            (Bit::One, true) => *ones -= 1,
        }
    }

    fn count_bits<'a>(iter: GridLinearIter<'a, Option<Bit>>, count: usize) -> (isize, isize) {
        let mut zeroes = count as isize;
        let mut ones = count as isize;

        for entry in iter.flatten() {
            match entry {
                Bit::Zero => zeroes -= 1,
                Bit::One => ones -= 1,
            }
        }

        (zeroes, ones)
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
    fn enter(&mut self, pos: &Position, curr: Bit) -> bool {
        let (row, col) = pos.lines();

        // Update the remaining bits counts
        if self.state.entry(pos).is_none_or(|prev| *prev != curr) {
            self.update_count(row, curr, true);
            self.update_count(col, curr, true);
        }

        tracing::info!("Setting {pos} -> {curr}");
        let result = self.state.enter(pos, curr);

        // Validate the cell and its neighbors
        self.validate_cell(*pos);

        for neighbor in self.state.entries.neighbor_indices(*pos) {
            self.validate_cell(neighbor);
        }

        // Validate the lines the position is on
        self.validate_line(row);
        self.validate_line(col);

        result
    }

    fn clear(&mut self, pos: &Position) -> bool {
        let (row, col) = pos.lines();

        // Update the remaining bits counts
        if let Some(entry) = self.state.entry(pos).cloned() {
            self.update_count(row, entry, false);
            self.update_count(col, entry, false);
        }

        // Validate the neighboring cells
        for neighbor in self.state.entries.neighbor_indices(*pos) {
            self.validate_cell(neighbor);
        }

        // Validate the lines the position is on
        self.validate_line(row);
        self.validate_line(col);

        tracing::info!("Clearing {pos}");

        self.state.clear(pos)
    }

    delegate! {
        to self.state {
            fn solution(&self, pos: &Position) -> Option<&Bit>;
            fn entry(&self, pos: &Position) -> Option<&Bit>;

            fn solve(&mut self, pos: &Position, solution: Bit) -> bool;
            fn reveal(&mut self, pos: &Position) -> bool;
            fn check(&mut self, pos: &Position) -> Option<bool>;

            fn guess(&mut self, pos: &Position, guess: Bit) -> bool;
        }
    }
}
