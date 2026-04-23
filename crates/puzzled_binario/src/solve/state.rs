use delegate::delegate;
use puzzled_core::{Entry, Grid, GridState, Line, Order, Position, SidedGrid, Solve, Timer};

use crate::{Binario, Bit, Bits};

#[derive(Debug)]
pub struct BinarioState {
    pub state: GridState<Binario>,

    pub valid: SidedGrid<u8, bool, LineBits, LineBits, bool>,
}

#[derive(Debug, Clone, Copy)]
pub struct LineBits {
    pub zeroes: isize,
    pub ones: isize,
}

impl LineBits {
    pub fn new(line_len: usize) -> Self {
        let count = (line_len / 2) as isize;

        Self {
            zeroes: count,
            ones: count,
        }
    }

    pub fn update(&mut self, bit: Bit, is_added: bool) {
        match (bit, is_added) {
            (Bit::Zero, false) => self.zeroes -= 1,
            (Bit::Zero, true) => self.zeroes += 1,
            (Bit::One, false) => self.ones -= 1,
            (Bit::One, true) => self.ones += 1,
        }
    }

    pub fn from_bits<I>(iter: I) -> Self
    where
        I: Iterator<Item = Option<Bit>>,
    {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut total = 0;

        for opt_bit in iter {
            total += 1;

            if let Some(bit) = opt_bit {
                match bit {
                    Bit::Zero => zeroes += 1,
                    Bit::One => ones += 1,
                }
            }
        }

        Self {
            zeroes: (total / 2) - zeroes,
            ones: (total / 2) - ones,
        }
    }
}

impl BinarioState {
    pub fn new(solutions: Grid<Option<Bit>>, entries: Grid<Entry<Bit>>, timer: Timer) -> Self {
        let right: Vec<_> = solutions
            .iter_rows()
            .map(|row| LineBits::from_bits(row.cloned()))
            .collect();
        let bottom: Vec<_> = solutions
            .iter_cols()
            .map(|col| LineBits::from_bits(col.cloned()))
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
        // Retrieve the line validity and counts
        let valid = if line.is_row() {
            self.valid.left.as_mut()
        } else {
            self.valid.top.as_mut()
        }
        .expect("Should be defined")
        .get_mut(line.line())
        .expect("Should be valid line");

        let counts = if line.is_row() {
            self.valid.right.as_ref()
        } else {
            self.valid.bottom.as_ref()
        }
        .expect("Should be defined")
        .get(line.line())
        .expect("Should be valid line");

        *valid = true;

        // Mark the line as incorrect if any of its entries are incorrect
        if self
            .state
            .entries
            .iter_line(line)
            .any(|entry| entry.is_incorrect())
        {
            *valid = false;
            return;
        }

        // Mark the line as incorrect if either count is incorrect
        let limit = (self.valid.grid.line_len(line) / 2) as isize;
        let valid_range = 0..=limit;

        if !valid_range.contains(&counts.zeroes) || !valid_range.contains(&counts.ones) {
            *valid = false;
            return;
        }

        // Mark the line as incorrect if it is equal to another
        let entries = &mut self.state.entries;

        let order = Order::from(line);
        let iter = entries.iter_line(line);
        let pos = line.line();

        for (idx, other_iter) in entries.iter_lines(order).enumerate() {
            if idx != pos && iter.eq(other_iter) {
                *valid = false;
                return;
            }
        }
    }

    pub fn update_count(&mut self, line: Line, curr: Option<Bit>, prev: Option<Bit>) {
        let counts = if line.is_row() {
            self.valid.right.as_mut()
        } else {
            self.valid.bottom.as_mut()
        }
        .expect("Should be defined")
        .get_mut(line.line())
        .expect("Should be defined");

        if let Some(curr) = curr {
            counts.update(curr, true);
        }
        if let Some(prev) = prev {
            counts.update(prev, false);
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
    fn enter(&mut self, pos: &Position, curr: Bit) -> bool {
        let (row, col) = pos.lines();

        // Update the remaining bits counts
        if self.state.solution(pos).is_none() {
            let prev = self.state.entry(pos).cloned();
            self.update_count(row, Some(curr), prev);
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
        if self.state.solution(pos).is_none() {
            let prev = self.state.entry(pos).cloned();
            self.update_count(row, None, prev);
        }

        let result = self.state.clear(pos);

        // Validate the neighboring cells
        for neighbor in self.state.entries.neighbor_indices(*pos) {
            self.validate_cell(neighbor);
        }

        // Validate the lines the position is on
        self.validate_line(row);
        self.validate_line(col);

        tracing::info!("Clearing {pos}");

        result
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
