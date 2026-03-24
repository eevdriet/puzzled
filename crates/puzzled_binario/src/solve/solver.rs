use std::collections::{HashSet, VecDeque};

use crate::{Binario, BinarioState, Bit, Bits};
use puzzled_core::{Grid, Position, Puzzle, Solve, Solver, SolverError};
use puzzled_io::TxtPuzzle;

#[derive(Debug, Default)]
pub struct BinarioSolver {
    frontier: VecDeque<Position>,
    seen: HashSet<Position>,
}

impl Solver<BinarioState> for BinarioSolver {
    type Puzzle = Binario;
    type Error = SolverError<String>;

    fn solve(
        &mut self,
        puzzle: &Binario,
        state: &mut BinarioState,
    ) -> Result<Grid<Bit>, Self::Error> {
        tracing::info!(
            "START: {}",
            state.solutions.iter().filter(|v| v.is_some()).count()
        );
        let _ = puzzle.save_text("start");

        self.init(state);

        while self.propagate(state) {
            self.init(state);
        }

        tracing::info!(
            "END: {}",
            state.solutions.iter().filter(|v| v.is_some()).count()
        );
        let _ = puzzle.save_text("end3");

        self.try_finalize(state)
    }

    fn try_finalize(
        &self,
        _state: &BinarioState,
    ) -> Result<<Binario as Puzzle>::Solution, Self::Error> {
        Err(SolverError::Stuck)
    }
}

impl BinarioSolver {
    fn init(&mut self, state: &mut BinarioState) {
        self.frontier.clear();
        self.seen.clear();

        for pos in state
            .solutions
            .iter_indexed()
            .filter_map(|(pos, bit)| bit.is_none().then_some(pos))
        {
            self.frontier.push_back(pos);
        }
    }

    fn propagate(&mut self, state: &mut BinarioState) -> bool {
        let mut has_solve = false;

        while let Some(pos) = self.frontier.pop_front() {
            tracing::info!(
                "Position {pos} ({} items left in frontier)",
                self.frontier.len()
            );

            // Avoid duplicates
            if self.seen.contains(&pos) {
                tracing::debug!("\tAlready seen, skipping...");
                continue;
            }

            // Avoid overriding set bit
            if state.solutions.get(pos).is_some_and(|bit| bit.is_some()) {
                tracing::debug!("\tAlready solved, skipping...");
                continue;
            }

            // Check whether a bit can be the middle bit at the current position
            if let Some(bit) = state.middle_bit(pos) {
                if self.seen.contains(&pos) {
                    tracing::error!("\tMiddle bit was already seen");
                }

                tracing::debug!("\tSetting middle bit {bit} at {pos}");
                state.solve(&pos, bit);
                has_solve = true;
                self.seen.insert(pos);
            }

            // Check whether a bit can be the center of some outer bits
            for (next_pos, bit) in state.outer_bits(pos) {
                if self.seen.contains(&next_pos) {
                    tracing::error!("\tOuter bit at {next_pos} (from {pos}) was already seen");
                }

                tracing::debug!("\tSetting outer bit {bit} at {next_pos} (from {pos})");
                state.solve(&next_pos, bit);
                has_solve = true;
                self.seen.insert(next_pos);
            }

            // Check whether one bit remains on the lines of the current position
            let (row, col) = pos.relative();

            for (pos, bit) in state.remaining_line_bits(row.line) {
                if self.seen.contains(&pos) {
                    tracing::error!(
                        "\tLast remaining bit {bit} at {pos} (from {}) was already seen",
                        row.line
                    );
                }

                tracing::debug!("\tSetting last remaining bit {bit} at {pos} in {row}");

                state.solve(&pos, bit);
                has_solve = true;
                self.seen.insert(pos);
            }

            for (pos, bit) in state.remaining_line_bits(col.line) {
                if self.seen.contains(&pos) {
                    tracing::error!(
                        "\tLast remaining bit at {pos} (from {}) was already seen",
                        col.line
                    );
                }

                tracing::debug!("\tSetting last remaining bit {bit} at {pos} in {col}");
                state.solve(&pos, bit);
                has_solve = true;
                self.seen.insert(pos);
            }

            self.seen.insert(pos);
        }

        has_solve
    }
}

#[cfg(test)]
mod tests {
    use puzzled_core::Puzzle;
    use tracing_test::traced_test;

    use crate::{BinarioSolver, binario};

    // #[rstest]
    // #[ignore = "Just for now"]
    // fn solve_txt(#[files("puzzles/ok/*.txt")] path: PathBuf) {
    //     let mut solver = BinarioSolver::default();
    //     let reader = ChumskyReader::new(false);
    //     let (puzzle, _) = reader
    //         .read_from_path::<_, Binario, BinarioState>(path)
    //         .expect("Puzzled is parsed correctly");
    //
    //     let solution = puzzle.solve_with(&mut solver).expect("To solve");
    // }

    #[traced_test]
    #[test]
    fn solve() {
        let puzzle = binario!(
            [ - - - - - 1 - - 1 0 ]
            [ - - 1 - - - - - - - ]
            [ - - - 1 - 1 - - 0 0 ]
            [ 1 - - 1 1 - - 1 0 - ]
            [ - 0 - - - - - - - - ]
            [ - - - - - - - - - - ]
            [ - 0 0 - 1 - - - - 1 ]
            [ - 0 - - - - 1 0 - - ]
            [ - - - 0 - - 1 - - 1 ]
            [ 0 0 - - - - - - 1 - ]
        );

        let mut solver = BinarioSolver::default();

        let solution = puzzle.solve_with(&mut solver).expect("To solve");
        eprintln!("Solution: {solution}");
    }
}
