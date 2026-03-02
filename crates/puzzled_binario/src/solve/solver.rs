use std::collections::{HashSet, VecDeque};

use puzzled_core::{Grid, Position, Solve, Solver, SolverError};

use crate::{Binario, BinarioState, Bit, Bits};

#[derive(Debug, Default)]
pub struct BinarioSolver {
    frontier: VecDeque<Position>,
    seen: HashSet<Position>,
}

impl Solver<Binario, BinarioState> for BinarioSolver {
    type Error = SolverError<String>;

    fn solve(
        &mut self,
        _puzzle: &Binario,
        state: &mut BinarioState,
    ) -> Result<Grid<Bit>, Self::Error> {
        self.init(state);
        self.propagate(state);

        state.try_finalize().map_err(SolverError::CannotFinalize)
    }
}

impl BinarioSolver {
    fn init(&mut self, state: &mut BinarioState) {
        self.frontier.clear();
        self.seen.clear();

        for pos in state
            .solutions()
            .iter_indexed()
            .filter_map(|(pos, bit)| bit.is_none().then_some(pos))
        {
            self.frontier.push_back(pos);
        }
    }

    fn propagate(&mut self, state: &mut BinarioState) {
        while let Some(pos) = self.frontier.pop_front() {
            // Avoid duplicates
            if self.seen.contains(&pos) {
                continue;
            }

            self.seen.insert(pos);

            println!("Frontier has {} items left", self.frontier.len());

            // Check whether a bit can be the middle bit at the current position
            if let Some(bit) = state.middle_bit(pos) {
                state.solve(&pos, bit);
                self.seen.insert(pos);
            }

            // Check whether a bit can be the center of some outer bits
            for (pos, bit) in state.outer_bits(pos) {
                state.solve(&pos, bit);
                self.seen.insert(pos);
            }

            // Check whether one bit remains on the lines of the current position
            let (row, col) = pos.relative();

            if let Some((pos, bit)) = state.remaining_line_bit(row.line) {
                state.solve(&pos, bit);
                self.seen.insert(pos);
            }
            if let Some((pos, bit)) = state.remaining_line_bit(col.line) {
                state.solve(&pos, bit);
                self.seen.insert(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use puzzled_core::Puzzle;
    use puzzled_io::ChumskyReader;
    use rstest::rstest;

    use crate::{Binario, BinarioSolver, BinarioState, binario};

    #[rstest]
    #[ignore = "Just for now"]
    fn solve_txt(#[files("puzzles/ok/*.txt")] path: PathBuf) {
        let mut solver = BinarioSolver::default();
        let reader = ChumskyReader::new(false);
        let (puzzle, _) = reader
            .read_from_path::<_, Binario, BinarioState>(path)
            .expect("Puzzled is parsed correctly");

        let solution = puzzle.solve_with(&mut solver).expect("To solve");
    }

    #[test]
    fn solve() {
        let puzzle = binario!(
            [1 -]
            [- 1]
        );

        let mut solver = BinarioSolver::default();

        let solution = puzzle.solve_with(&mut solver).expect("To solve");
        // eprintln!("Solution: {solution}");
    }
}
