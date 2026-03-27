use std::fmt::{self, Display};

use crate::{Entry, Grid, Position, Puzzle, SolutionEntry, Solve, Square, Timer};

#[derive(Debug)]
pub struct GridState<P>
where
    P: Puzzle<Position = Position>,
{
    pub solutions: Grid<Option<P::Value>>,
    pub entries: Grid<Entry<P::Value>>,
    pub timer: Timer,
}

impl<P> GridState<P>
where
    P: Puzzle<Position = Position>,
{
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
    P: Puzzle<Position = Position>,
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
    fn solution(&self, pos: &Position) -> Option<&P::Value> {
        let result = self.solutions.get(*pos)?;

        result.as_ref()
    }

    fn entry(&self, pos: &Position) -> Option<&P::Value> {
        let result = self.entries.get(*pos)?;

        result.entry()
    }

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
        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.reveal();
        true
    }

    fn check(&mut self, pos: &Position) -> Option<bool> {
        let Some(Some(solution)) = self.solutions.get(*pos) else {
            return None;
        };

        let entry = self.entries.get_mut(*pos)?;
        let is_correct = entry.entry().map(|s| s == solution)?;

        if is_correct {
            entry.mark_correct();
        } else {
            entry.mark_incorrect();
        }

        Some(is_correct)
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
pub struct SquareGridState<P>
where
    P: Puzzle<Position = Position>,
{
    pub solutions: Grid<Square<Option<P::Value>>>,
    pub entries: Grid<Square<Entry<P::Value>>>,
    pub timer: Timer,
}

impl<P> SquareGridState<P>
where
    P: Puzzle<Position = Position>,
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
    P: Puzzle<Position = Position>,
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
    fn solution(&self, pos: &Position) -> Option<&P::Value> {
        let result = self.solutions.get_fill(*pos)?;

        result.as_ref()
    }

    fn entry(&self, pos: &Position) -> Option<&P::Value> {
        let result = self.entries.get_fill(*pos)?;

        result.entry()
    }

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
        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.reveal();
        true
    }

    fn check(&mut self, pos: &Position) -> Option<bool> {
        let Some(Some(solution)) = self.solutions.get_fill(*pos) else {
            return None;
        };

        let entry = self.entries.get_fill_mut(*pos)?;
        let is_correct = entry.entry().map(|s| s == solution)?;

        if is_correct {
            entry.mark_correct();
        } else {
            entry.mark_incorrect();
        }

        Some(is_correct)
    }
}
