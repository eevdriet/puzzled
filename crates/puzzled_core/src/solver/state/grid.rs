use crate::{Entry, Grid, Position, Solve, Square, State};

impl<T> Solve for State<Grid<Option<T>>, Grid<Entry<T>>>
where
    T: Clone + Eq,
{
    type Value = T;
    type Position = Position;

    fn solve(&mut self, pos: &Position, value: T) -> bool {
        let Some(solution) = self.solutions.get_mut(*pos) else {
            return false;
        };

        *solution = Some(value);
        true
    }

    fn enter(&mut self, pos: &Position, value: T) -> bool {
        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        entry.enter(value);
        true
    }

    fn reveal(&mut self, pos: &Position) -> bool {
        // Make sure a solution is set and a corresponding entry exists
        let Some(Some(solution)) = self.solutions.get(*pos) else {
            return false;
        };
        let Some(entry) = self.entries.get_mut(*pos) else {
            return false;
        };

        // Set the solution to reveal it in the user entries
        entry.enter(solution.clone());
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
            .filter_map(|(pos, solution)| solution.as_ref().map(|sol| (pos, sol)))
        {
            let Some(entry) = self.entries.get_mut(pos) else {
                continue;
            };

            entry.enter(solution.clone());
        }
    }

    fn check_all(&mut self) {
        let solutions = &self.solutions;

        for pos in solutions.positions() {
            if let (Some(Some(solution)), Some(entry)) =
                (solutions.get(pos), self.entries.get_mut(pos))
            {
                entry.check(solution);
            }
        }
    }
}

impl<T> Solve for State<Grid<Square<Option<T>>>, Grid<Square<Entry<T>>>>
where
    T: Clone + Eq,
{
    type Value = T;
    type Position = Position;

    fn solve(&mut self, pos: &Position, value: T) -> bool {
        let Some(solution) = self.solutions.get_fill_mut(*pos) else {
            return false;
        };

        *solution = Some(value);
        true
    }

    fn enter(&mut self, pos: &Position, value: T) -> bool {
        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        entry.enter(value);
        true
    }

    fn reveal(&mut self, pos: &Position) -> bool {
        // Make sure a solution is set and a corresponding entry exists
        let Some(Some(solution)) = self.solutions.get_fill(*pos) else {
            return false;
        };
        let Some(entry) = self.entries.get_fill_mut(*pos) else {
            return false;
        };

        // Set the solution to reveal it in the user entries
        entry.enter(solution.clone());
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
            .iter_fills_indexed_mut()
            .filter_map(|(pos, solution)| solution.as_ref().map(|sol| (pos, sol)))
        {
            let Some(entry) = self.entries.get_fill_mut(pos) else {
                continue;
            };

            entry.enter(solution.clone());
        }
    }

    fn check_all(&mut self) {
        let solutions = &self.solutions;

        for pos in solutions.positions() {
            if let (Some(Some(solution)), Some(entry)) =
                (solutions.get_fill(pos), self.entries.get_fill_mut(pos))
            {
                entry.check(solution);
            }
        }
    }
}
