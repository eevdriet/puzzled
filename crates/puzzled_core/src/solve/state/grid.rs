use crate::{Entry, Grid, Square};

#[derive(Debug)]
pub struct GridState<T> {
    pub solutions: Grid<Option<T>>,
    pub entries: Grid<Entry<T>>,
}

impl<T> GridState<T> {
    pub fn new(solutions: Grid<Option<T>>, entries: Grid<Entry<T>>) -> Self {
        Self { solutions, entries }
    }
}

#[macro_export]
macro_rules! impl_solve_for_grid_state {
    ($puzzle:ty, $ty:ty) => {
        impl Solve<$puzzle> for GridState<$ty> {
            type Value = $ty;
            type Position = $crate::Position;

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

            fn try_finalize(&self) -> Result<Grid<$ty>, Box<dyn std::error::Error>> {
                let values: Vec<_> = self
                    .solutions
                    .iter()
                    .filter_map(|s| s.as_ref())
                    .cloned()
                    .collect();

                let grid = Grid::from_vec(values, self.solutions.cols()).map_err(Box::new)?;
                Ok(grid)
            }
        }
    };
}

#[derive(Debug)]
pub struct SquareGridState<T> {
    pub solutions: Grid<Square<Option<T>>>,
    pub entries: Grid<Square<Entry<T>>>,
}

#[macro_export]
macro_rules! impl_solve_for_square_grid_state {
    ($puzzle:ty, $ty:ty) => {
        impl $crate::Solve<$puzzle> for SquareGridState<$ty> {
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

            fn try_finalize(&self) -> Result<Grid<Square<$ty>>, Box<dyn std::error::Error>> {
                let values: Vec<_> = self
                    .solutions
                    .iter_fills()
                    .filter_map(|s| s.as_ref())
                    .cloned()
                    .map(|s| Square::new(s))
                    .collect();

                let a = Grid::from_vec(values, 2).expect("Yeet");
                Ok(a)
            }
        }
    };
}
