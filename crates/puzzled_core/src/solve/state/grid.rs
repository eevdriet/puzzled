use std::fmt::{self, Display};

use crate::{Entry, Grid, Position, Puzzle, SolutionEntry, Solve, Square, Timer};

#[derive(Debug)]
pub struct GridState<P: Puzzle> {
    pub solutions: Grid<Option<P::Value>>,
    pub entries: Grid<Entry<P::Value>>,
    pub timer: Timer,
}

impl<P: Puzzle> GridState<P> {
    pub fn new(
        solutions: Grid<Option<P::Value>>,
        entries: Grid<Entry<P::Value>>,
        timer: Timer,
    ) -> Self {
        Self {
            solutions,
            entries,
            timer,
        }
    }

    pub fn to_merged(&self) -> Grid<SolutionEntry<'_, P::Value>> {
        self.solutions
            .join_ref(&self.entries, |solution, entry| SolutionEntry {
                solution,
                entry,
            })
            .expect("Solutions and entries have the same size")
    }
}

impl<P> fmt::Display for GridState<P>
where
    P: Puzzle,
    P::Value: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_merged())
    }
}

impl<P> Solve<P> for GridState<P>
where
    P: Puzzle<Position = Position>,
{
    fn solve(&mut self, pos: &Position, value: P::Value) -> bool {
        let Some(solution) = self.solutions.get_mut(*pos) else {
            return false;
        };

        *solution = Some(value);
        true
    }

    fn enter(&mut self, pos: &Position, value: P::Value) -> bool {
        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.enter(value);
        true
    }

    fn clear(&mut self, pos: &Position) -> bool {
        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.clear();
        true
    }

    fn reveal(&mut self, pos: &Position) -> bool {
        let Some(Some(solution)) = self.solutions.get(*pos) else {
            return false;
        };

        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.reveal(solution.clone());
        true
    }

    fn check(&mut self, pos: &Position) -> Option<bool> {
        let Some(Some(solution)) = self.solutions.get(*pos) else {
            return None;
        };

        let entry = self.entries.get_mut(*pos)?;
        entry.check(solution)
    }

    fn reveal_all(&mut self) {
        for (pos, solution) in self
            .solutions
            .iter_indexed()
            .filter_map(|(pos, sol)| sol.as_ref().map(|sol| (pos, sol.clone())))
        {
            if let Some(entry) = self.entries.get_mut(pos) {
                entry.enter(solution);
            }
        }
    }

    fn check_all(&mut self) {
        for pos in self.solutions.positions() {
            if let (Some(Some(solution)), Some(entry)) =
                (self.solutions.get(pos), self.entries.get_mut(pos))
            {
                entry.check(solution);
            }
        }
    }

    fn clear_all(&mut self) {
        for pos in self.solutions.positions() {
            if let Some(entry) = self.entries.get_mut(pos) {
                entry.clear();
            }
        }
    }

    // fn try_finalize(&self) -> Result<<P as Puzzle>::Solution, Self::Error> {
    //     if self.solutions.iter().any(|cell| cell.is_none()) {
    //         return Err("Expected all values to be set in the solution".to_string());
    //     }
    //
    //     let values: Vec<_> = self
    //         .solutions
    //         .iter()
    //         .filter_map(|s| s.as_ref())
    //         .cloned()
    //         .collect();
    //
    //     Ok(Grid::from_vec(values, self.solutions.cols()).expect("Grid should be valid"))
    // }
}

#[derive(Debug)]
pub struct SquareGridState<P: Puzzle> {
    pub solutions: Grid<Square<Option<P::Value>>>,
    pub entries: Grid<Square<Entry<P::Value>>>,
    pub timer: Timer,
}

impl<P> SquareGridState<P>
where
    P: Puzzle,
{
    pub fn new(
        solutions: Grid<Square<Option<P::Value>>>,
        entries: Grid<Square<Entry<P::Value>>>,
        timer: Timer,
    ) -> Self {
        Self {
            solutions,
            entries,
            timer,
        }
    }

    pub fn to_merged(&self) -> Grid<Square<SolutionEntry<'_, P::Value>>> {
        let data: Vec<_> = self
            .solutions
            .iter()
            .zip(self.entries.iter())
            .map(
                |(solution, entry)| match (solution.0.as_ref(), entry.0.as_ref()) {
                    (Some(solution), Some(entry)) => Square::new(SolutionEntry { solution, entry }),
                    _ => Square::new_empty(),
                },
            )
            .collect();

        Grid::from_vec(data, self.solutions.cols())
            .expect("Solutions and entries have the same size")
    }
}

impl<P> fmt::Display for SquareGridState<P>
where
    P: Puzzle,
    P::Value: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_merged())
    }
}

impl<P> Solve<P> for SquareGridState<P>
where
    P: Puzzle<Position = Position>,
{
    fn solve(&mut self, pos: &Position, value: P::Value) -> bool {
        let Some(solution) = self.solutions.get_fill_mut(*pos) else {
            return false;
        };

        *solution = Some(value);
        true
    }

    fn enter(&mut self, pos: &Position, value: P::Value) -> bool {
        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.enter(value);
        true
    }

    fn clear(&mut self, pos: &Position) -> bool {
        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.clear();
        true
    }

    fn reveal(&mut self, pos: &Position) -> bool {
        let Some(Some(solution)) = self.solutions.get_fill(*pos) else {
            return false;
        };

        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.reveal(solution.clone());
        true
    }

    fn check(&mut self, pos: &Position) -> Option<bool> {
        let Some(Some(solution)) = self.solutions.get_fill(*pos) else {
            return None;
        };

        let entry = self.entries.get_fill_mut(*pos)?;
        entry.check(solution)
    }

    fn reveal_all(&mut self) {
        for (pos, solution) in self
            .solutions
            .iter_fills_indexed()
            .filter_map(|(pos, sol)| sol.as_ref().map(|sol| (pos, sol.clone())))
        {
            if let Some(entry) = self.entries.get_fill_mut(pos) {
                entry.enter(solution);
            }
        }
    }

    fn check_all(&mut self) {
        for pos in self.solutions.positions() {
            if let (Some(Some(solution)), Some(entry)) =
                (self.solutions.get_fill(pos), self.entries.get_fill_mut(pos))
            {
                entry.check(solution);
            }
        }
    }

    fn clear_all(&mut self) {
        for pos in self.solutions.positions() {
            if let Some(entry) = self.entries.get_fill_mut(pos) {
                entry.clear();
            }
        }
    }
}
