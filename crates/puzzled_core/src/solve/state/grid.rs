use std::fmt::{self, Display};

use crate::{Entry, Grid, MISSING_ENTRY_CHAR, NON_PLAYABLE_CHAR, Square, Timer};

#[derive(Debug)]
pub struct GridState<T> {
    pub solutions: Grid<Option<T>>,
    pub entries: Grid<Entry<T>>,
    pub timer: Timer,
}

impl<T> GridState<T> {
    pub fn new(solutions: Grid<Option<T>>, entries: Grid<Entry<T>>, timer: Timer) -> Self {
        Self {
            solutions,
            entries,
            timer,
        }
    }
}

fn display_solution_entry<T>(solution: Option<&T>, entry: &Entry<T>) -> String
where
    T: Display,
{
    let style = entry.style();

    match (solution, entry.entry()) {
        (None, None) => MISSING_ENTRY_CHAR.to_string(),

        (Some(s), None) => format!("{s}{style}"),
        (None, Some(e)) => format!("({e}{style})"),
        (Some(s), Some(e)) => format!("{s}{style} ({e})"),
    }
}

impl<T> fmt::Display for GridState<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let merged: Vec<_> = self
            .solutions
            .iter()
            .zip(self.entries.iter())
            .map(|(solution, entry)| display_solution_entry(solution.as_ref(), entry))
            .collect();

        let grid = Grid::from_vec(merged, self.entries.cols())
            .expect("Merged solutions and entries grids of same dimensions");
        write!(f, "{grid}")
    }
}

#[macro_export]
macro_rules! impl_solve_for_grid_state {
    ($puzzle:ty, $val:ty) => {
        impl $crate::Solve<$puzzle> for GridState<$val> {
            type Value = $val;
            type Position = $crate::Position;
            type Error = String;

            fn solve(&mut self, pos: &Self::Position, value: Self::Value) -> bool {
                let Some(solution) = self.solutions.get_mut(*pos) else {
                    return false;
                };

                *solution = Some(value);
                true
            }

            fn enter(&mut self, pos: &Self::Position, value: Self::Value) -> bool {
                let Some(entry) = self.entries.get_mut(*pos) else {
                    return false;
                };

                entry.enter(value);
                true
            }

            fn reveal(&mut self, pos: &Self::Position) -> bool {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                let Some(Some(solution)) = solutions.get(*pos) else {
                    return false;
                };

                let Some(entry) = entries.get_mut(*pos) else {
                    return false;
                };

                entry.enter(solution.clone());
                true
            }

            fn check(&mut self, pos: &Self::Position) -> Option<bool> {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                let Some(Some(solution)) = solutions.get(*pos) else {
                    return None;
                };

                let entry = entries.get_mut(*pos)?;
                entry.check(solution)
            }

            fn reveal_all(&mut self) {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                for (pos, solution) in solutions
                    .iter_indexed()
                    .filter_map(|(pos, s)| s.as_ref().map(|sol| (pos, sol.clone())))
                {
                    if let Some(entry) = entries.get_mut(pos) {
                        entry.enter(solution);
                    }
                }
            }

            fn check_all(&mut self) {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                for pos in solutions.positions() {
                    if let (Some(Some(solution)), Some(entry)) =
                        (solutions.get(pos), entries.get_mut(pos))
                    {
                        entry.check(solution);
                    }
                }
            }

            fn try_finalize(&self) -> Result<$crate::Grid<$val>, Self::Error> {
                let solutions = &self.solutions;

                if solutions.iter().any(|bit| bit.is_none()) {
                    return Err("Expected all values to be set in the solution".to_string());
                }

                let values: Vec<_> = solutions
                    .iter()
                    .filter_map(|s| s.as_ref())
                    .cloned()
                    .collect();

                Ok($crate::Grid::from_vec(values, solutions.cols()).expect("Grid should be valid"))
            }
        }
    };
}

impl<T> fmt::Display for SquareGridState<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let merged: Vec<_> = self
            .solutions
            .iter()
            .zip(self.entries.iter())
            .map(|(solution, entry)| match (&solution.0, &entry.0) {
                (None, None) => NON_PLAYABLE_CHAR.to_string(),
                (Some(s), Some(e)) => display_solution_entry(s.as_ref(), e),
                _ => unreachable!("Solution and entry should always be same type of square"),
            })
            .collect();

        let grid = Grid::from_vec(merged, self.entries.cols())
            .expect("Merged solutions and entries grids of same dimensions");
        write!(f, "{grid}")
    }
}

#[derive(Debug)]
pub struct SquareGridState<T> {
    pub solutions: Grid<Square<Option<T>>>,
    pub entries: Grid<Square<Entry<T>>>,
    pub timer: Timer,
}

impl<T> SquareGridState<T> {
    pub fn new(
        solutions: Grid<Square<Option<T>>>,
        entries: Grid<Square<Entry<T>>>,
        timer: Timer,
    ) -> Self {
        Self {
            solutions,
            entries,
            timer,
        }
    }
}

#[macro_export]
macro_rules! impl_solve_for_square_grid_state {
    ($puzzle:ty, $ty:ty) => {
        impl $crate::Solve<$puzzle> for SquareGridState<$ty> {
            type Value = $ty;
            type Position = $crate::Position;
            type Error = $crate::GridError;

            fn solve(&mut self, pos: &Self::Position, value: Self::Value) -> bool {
                let Some(solution) = self.solutions.get_fill_mut(*pos) else {
                    return false;
                };

                *solution = Some(value);
                true
            }

            fn enter(&mut self, pos: &Self::Position, value: Self::Value) -> bool {
                let Some(entry) = self.entries.get_fill_mut(*pos) else {
                    return false;
                };

                entry.enter(value);
                true
            }

            fn reveal(&mut self, pos: &Self::Position) -> bool {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                let Some(Some(solution)) = solutions.get_fill(*pos) else {
                    return false;
                };

                let Some(entry) = entries.get_fill_mut(*pos) else {
                    return false;
                };

                entry.enter(solution.clone());
                true
            }

            fn check(&mut self, pos: &Self::Position) -> Option<bool> {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                let Some(Some(solution)) = solutions.get_fill(*pos) else {
                    return None;
                };

                let entry = entries.get_fill_mut(*pos)?;
                entry.check(solution)
            }

            fn reveal_all(&mut self) {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                for (pos, solution) in solutions
                    .iter_fills_indexed()
                    .filter_map(|(pos, s)| s.as_ref().map(|sol| (pos, sol.clone())))
                {
                    if let Some(entry) = entries.get_fill_mut(pos) {
                        entry.enter(solution);
                    }
                }
            }

            fn check_all(&mut self) {
                let (solutions, entries) = (&self.solutions, &mut self.entries);

                for pos in solutions.positions() {
                    if let (Some(Some(solution)), Some(entry)) =
                        (solutions.get_fill(pos), entries.get_fill_mut(pos))
                    {
                        entry.check(solution);
                    }
                }
            }

            fn try_finalize(&self) -> Result<$crate::Grid<$crate::Square<$ty>>, Self::Error> {
                let values: Vec<_> = self
                    .solutions
                    .iter_fills()
                    .filter_map(|s| s.as_ref())
                    .cloned()
                    .map(|s| $crate::Square::new(s))
                    .collect();

                $crate::Grid::from_vec(values, self.solutions.cols())
            }
        }
    };
}
