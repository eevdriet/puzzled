use puzzled_core::{GridState, Position, Puzzle, SquareGridState};

use crate::{EntryAction, EntryChange, Operator};

pub trait HandleOperator<P, S> {
    fn handle_operator<I>(&mut self, op: Operator, positions: I) -> S
    where
        Self: Sized,
        I: IntoIterator<Item = P>;
}

impl<P> HandleOperator<Position, Box<EntryAction<P>>> for GridState<P>
where
    P: Puzzle<Position = Position> + 'static,
{
    fn handle_operator<I>(&mut self, op: Operator, positions: I) -> Box<EntryAction<P>>
    where
        I: IntoIterator<Item = Position>,
    {
        tracing::info!("Applying {op:?} to");
        let changes = positions
            .into_iter()
            .filter_map(|pos| {
                let entry = self.entries.get(pos)?;
                let before = entry.entry().cloned();
                let after = match op {
                    Operator::Reveal => self.solutions.get(pos).and_then(|sol| sol.clone()),
                    _ => None,
                };

                tracing::info!("\t {pos}");
                Some(EntryChange { pos, before, after })
            })
            .collect();

        Box::new(EntryAction::<P>::new(op, changes))
    }
}

impl<P> HandleOperator<Position, Box<EntryAction<P>>> for SquareGridState<P>
where
    P: Puzzle<Position = Position> + 'static,
{
    fn handle_operator<I>(&mut self, op: Operator, positions: I) -> Box<EntryAction<P>>
    where
        I: IntoIterator<Item = Position>,
    {
        tracing::info!("Applying {op:?} to");
        let changes = positions
            .into_iter()
            .filter_map(|pos| {
                let entry = self.entries.get_fill(pos)?;
                let before = entry.entry().cloned();
                let after = match op {
                    Operator::Delete | Operator::Change => None,
                    Operator::Reveal => self.solutions.get_fill(pos).and_then(|sol| sol.clone()),
                    _ => None,
                };

                tracing::info!("\t {pos}");
                Some(EntryChange { pos, before, after })
            })
            .collect();

        Box::new(EntryAction::new(op, changes))
    }
}
