use std::fmt::{self, Display};

use crate::{Entry, Grid, Position, SolutionEntry, Solve, Square, Timer};

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

    pub fn to_merged(&self) -> Grid<SolutionEntry<'_, T>> {
        let data: Vec<_> = self
            .solutions
            .iter()
            .zip(self.entries.iter())
            .map(|(solution, entry)| SolutionEntry { solution, entry })
            .collect();

        Grid::from_vec(data, self.solutions.cols())
            .expect("Solutions and entries have the same size")
    }
}

impl<T> fmt::Display for GridState<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_merged())
    }
}

impl<T> Solve for GridState<T>
where
    T: Clone + Eq,
{
    type Value = T;
    type Position = Position;

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

    fn clear(&mut self, pos: &Self::Position) -> bool {
        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.clear();
        true
    }

    fn reveal(&mut self, pos: &Self::Position) -> bool {
        let Some(Some(solution)) = self.solutions.get(*pos) else {
            return false;
        };

        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.enter(solution.clone());
        true
    }

    fn check(&mut self, pos: &Self::Position) -> Option<bool> {
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

#[macro_export]
macro_rules! impl_solve_for_grid_state {
    ($state:ty, $field:tt, $puzzle:ty, $val:ty) => {
        impl $crate::Solve for $state {
            type Value = $val;
            type Position = $crate::Position;

            fn solve(&mut self, pos: &Self::Position, value: Self::Value) -> bool {
                let $crate::GridState { solutions, .. } = &mut self.$field;

                let Some(solution) = solutions.get_mut(*pos) else {
                    return false;
                };

                *solution = Some(value);
                true
            }

            fn enter(&mut self, pos: &Self::Position, value: Self::Value) -> bool {
                let $crate::GridState { entries, .. } = &mut self.$field;

                let Some(entry) = entries.get_mut(*pos) else {
                    return false;
                };

                entry.enter(value);
                true
            }

            fn clear(&mut self, pos: &Self::Position) -> bool {
                let $crate::GridState { entries, .. } = &mut self.$field;

                let Some(entry) = entries.get_mut(*pos) else {
                    return false;
                };

                entry.clear();
                true
            }

            fn reveal(&mut self, pos: &Self::Position) -> bool {
                let $crate::GridState {
                    solutions, entries, ..
                } = &mut self.$field;

                let solutions = &*solutions;

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
                let $crate::GridState {
                    solutions, entries, ..
                } = &mut self.$field;

                let solutions = &*solutions;

                let Some(Some(solution)) = solutions.get(*pos) else {
                    return None;
                };

                let entry = entries.get_mut(*pos)?;
                entry.check(solution)
            }

            fn reveal_all(&mut self) {
                let $crate::GridState {
                    solutions, entries, ..
                } = &mut self.$field;

                let solutions = &*solutions;

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
                let $crate::GridState {
                    solutions, entries, ..
                } = &mut self.$field;

                for pos in solutions.positions() {
                    if let (Some(Some(solution)), Some(entry)) =
                        (solutions.get(pos), entries.get_mut(pos))
                    {
                        entry.check(solution);
                    }
                }
            }

            fn clear_all(&mut self) {
                let $crate::GridState {
                    solutions, entries, ..
                } = &mut self.$field;

                for pos in solutions.positions() {
                    if let Some(entry) = entries.get_mut(pos) {
                        entry.clear();
                    }
                }
            }
        }
    };
}

impl<T> fmt::Display for SquareGridState<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_merged())
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

    pub fn to_merged(&self) -> Grid<Square<SolutionEntry<'_, T>>> {
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

impl<T> Solve for SquareGridState<T>
where
    T: Clone + Eq,
{
    type Value = T;
    type Position = Position;

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

    fn clear(&mut self, pos: &Self::Position) -> bool {
        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.clear();
        true
    }

    fn reveal(&mut self, pos: &Self::Position) -> bool {
        let Some(Some(solution)) = self.solutions.get_fill(*pos) else {
            return false;
        };

        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.enter(solution.clone());
        true
    }

    fn check(&mut self, pos: &Self::Position) -> Option<bool> {
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

#[macro_export]
macro_rules! impl_solve_for_square_grid_state {
    ($state:ty, $puzzle:ty, $ty:ty) => {
        impl $crate::Solve for $state {
            type Value = $ty;
            type Position = $crate::Position;

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
                let (solutions, entries) = (&self.0.solutions, &mut self.0.entries);

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
                let (solutions, entries) = (&self.0.solutions, &mut self.0.entries);

                let Some(Some(solution)) = solutions.get_fill(*pos) else {
                    return None;
                };

                let entry = entries.get_fill_mut(*pos)?;
                entry.check(solution)
            }

            fn clear(&mut self, pos: &Self::Position) -> bool {
                let (solutions, entries) = (&self.0.solutions, &mut self.0.entries);

                let Some(entry) = entries.get_fill_mut(*pos) else {
                    return false;
                };

                entry.clear();
                true
            }

            fn reveal_all(&mut self) {
                let (solutions, entries) = (&self.0.solutions, &mut self.0.entries);

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
                let (solutions, entries) = (&self.0.solutions, &mut self.0.entries);

                for pos in solutions.positions() {
                    if let (Some(Some(solution)), Some(entry)) =
                        (solutions.get_fill(pos), entries.get_fill_mut(pos))
                    {
                        entry.check(solution);
                    }
                }
            }

            fn clear_all(&mut self) {
                let (solutions, entries) = (&self.0.solutions, &mut self.0.entries);

                for pos in solutions.positions() {
                    if let Some(entry) = entries.get_fill_mut(pos) {
                        entry.clear();
                    }
                }
            }
        }
    };
}
