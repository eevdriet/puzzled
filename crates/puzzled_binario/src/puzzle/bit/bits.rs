use puzzled_core::{Direction, Grid, Line, Position, Value};

use crate::Bit;

pub trait Bits {
    fn middle_bit(&self, pos: Position) -> Option<Bit>;
    fn outer_bits(&self, pos: Position) -> Vec<(Position, Bit)>;
    fn remaining_line_bits(&self, line: Line) -> Vec<(Position, Bit)>;
}

impl<T> Bits for Grid<T>
where
    T: Value<Bit>,
{
    fn middle_bit(&self, pos: Position) -> Option<Bit> {
        // Candidate if U/D or L/R are the same bit
        let [up, right, down, left] = self.adjacent4(pos).map(|adj| adj.and_then(|b| b.value()));

        if up.is_some() && down.is_some() && up == down {
            let bit = up.cloned().unwrap();
            return Some(!bit);
        }

        if left.is_some() && right.is_some() && left == right {
            let bit = left.cloned().unwrap();
            return Some(!bit);
        }

        None
    }

    fn outer_bits(&self, pos: Position) -> Vec<(Position, Bit)> {
        // tracing::debug!("Outer bits");
        let mut bits = Vec::with_capacity(1 + 4);
        let mut center = None;

        // Candidate if D/D2 are the same bit for any direction D
        let dir_n = |direction: Direction, n: isize| -> Option<&Bit> {
            match pos + n * direction {
                Some(pos_n) => self.get(pos_n).and_then(|p| p.value()),
                _ => None,
            }
        };

        for direction in Direction::ALL {
            let adj = dir_n(direction, 1);
            let adj2 = dir_n(direction, 2);

            if let Some(neg_bit) = adj
                && adj2.is_some()
                && adj == adj2
            {
                let bit = !*neg_bit;
                center = Some(bit);

                if let Some(adj3) = pos + 3 * direction {
                    bits.push((adj3, bit));
                }
            }
        }

        if let Some(bit) = center {
            bits.push((pos, bit));
        }

        bits
    }

    fn remaining_line_bits(&self, line: Line) -> Vec<(Position, Bit)> {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut remaining = vec![];

        for (pos, bit) in self.iter_indexed_line(line) {
            match bit.value() {
                // Only check for one remaining position, otherwise terminate search
                None => remaining.push(pos),

                // Keep check of how many zero bits are set
                Some(bit) => {
                    if bit.is_zero() {
                        zero_count += 1
                    } else {
                        one_count += 1
                    }
                }
            }
        }

        // Determine which bit to give the remaining positions if any
        let half_count = self.line_len(line) / 2;

        let bit = if zero_count == half_count {
            Bit::One
        } else if one_count == half_count {
            Bit::Zero
        } else {
            return vec![];
        };

        remaining.into_iter().map(|pos| (pos, bit)).collect()
    }
}

#[cfg(test)]
mod tests {
    use puzzled_core::Grid;
    use rstest::fixture;

    use crate::{Bit, binario};

    #[fixture]
    fn grid() -> Grid<Option<Bit>> {
        // Puzzle ID: 959,443
        //       0 1 2 3 4 5 6 7 8 9
        let puzzle = binario!(
                [- - - - - 1 - - 1 0]
                [- - 1 - - - - - - -]
                [- - - 1 - 1 - - 0 0]
                [1 - - 1 1 - - 1 0 -]
                [- 0 - - - - - - - -]
                [- - - - - - - - - -]
                [- 0 0 - 1 - - - - 1]
                [- 0 - - - - 1 0 - -]
                [- - - 0 - - 1 - - 1]
                [0 0 - - - - - - 1 -]
        );

        puzzle.cells().map_ref(|c| c.solution)
    }

    // #[rstest]
    // fn candidates() {
    //     let grid = grid();
    //     let candidates: BTreeMap<Position, Bit> = grid
    //         .positions()
    //         .filter_map(|pos| {
    //             let bit = grid.is_candidate(pos)?;
    //             Some((pos, bit))
    //         })
    //         .collect();
    //
    //     eprintln!("{grid:?}");
    //     eprintln!("{candidates:#?}");
    //
    //     assert_eq!(BTreeMap::default(), candidates);
    // }
    //
    // #[rstest]
    // #[case((0, 0), None)]
    // #[case((2, 4), None)]
    // fn is_candidate(#[case] (row, col): (usize, usize), #[case] candidate: Option<Bit>) {
    //     let pos = Position::new(row, col);
    //     let grid = grid();
    //
    //     assert_eq!(candidate, grid.is_candidate(pos));
    // }
}
