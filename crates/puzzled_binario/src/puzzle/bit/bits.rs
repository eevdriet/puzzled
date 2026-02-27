use puzzled_core::{Direction, Grid, Line, Position, Value};

use crate::Bit;

pub trait Bits {
    fn is_candidate(&self, pos: Position) -> Option<Bit>;
}

impl<T> Bits for Grid<T>
where
    T: Value<Bit>,
{
    fn is_candidate(&self, pos: Position) -> Option<Bit> {
        // Already filled bits are never a candidate
        if self.get(pos).and_then(|bit| bit.value()).is_some() {
            return None;
        }

        is_dir_candidate(self, pos)
            .or(is_dir2_candidate(self, pos))
            .or(is_line_candidate(self, pos))
    }
}

fn is_dir_candidate<T>(grid: &Grid<T>, pos: Position) -> Option<Bit>
where
    T: Value<Bit>,
{
    // Candidate if U/D or L/R are the same bit
    let [up, right, down, left] = grid.adjacent4(pos).map(|adj| adj.and_then(|b| b.value()));

    if up.is_some() && down.is_some() && up == down {
        return up.cloned();
    }
    if left.is_some() && right.is_some() && left == right {
        return left.cloned();
    }

    None
}

fn is_dir2_candidate<T>(grid: &Grid<T>, pos: Position) -> Option<Bit>
where
    T: Value<Bit>,
{
    // Candidate if D/D2 are the same bit for any direction D
    let dir_dir2 = |direction: Direction| {
        [
            grid.get(pos + 1 * direction).and_then(|b| b.value()),
            grid.get(pos + 2 * direction).and_then(|b| b.value()),
        ]
    };

    for direction in Direction::ALL {
        let [dir, dir2] = dir_dir2(direction);

        if dir.is_some() && dir2.is_some() && dir == dir2 {
            return dir.cloned();
        }
    }

    None
}

fn is_line_candidate<T>(grid: &Grid<T>, pos: Position) -> Option<Bit>
where
    T: Value<Bit>,
{
    // Count how many bits are set on the line and how many of those are zero
    let count_line = |line: Line| {
        let mut zero_count = 0;
        let mut cell_count = 0;

        for bit in grid.iter_line(line) {
            let value = bit.value();

            if value.is_some() {
                cell_count += 1;
            }
            if value.is_some_and(|b| b.is_zero()) {
                zero_count += 1;
            }
        }

        (zero_count, cell_count)
    };

    // Determine which bet to set based on the zero count
    let bit = |zero_count: usize, total: usize| {
        if zero_count < total / 2 {
            Bit::Zero
        } else {
            Bit::One
        }
    };

    let (row, col) = pos.lines();

    // Set remaining row bit if only one is left
    let (zero_count, col_count) = count_line(row);
    if col_count + 1 == grid.cols() {
        return Some(bit(zero_count, grid.cols()));
    }

    // Set remaining column bit if only one is left
    let (zero_count, row_count) = count_line(col);
    if row_count + 1 == grid.rows() {
        return Some(bit(zero_count, grid.rows()));
    }

    None
}
