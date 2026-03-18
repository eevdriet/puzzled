use std::fmt::Debug;

use puzzled_core::{GridState, Position, SquareGridState};

use crate::{ActionHistory, EntryAction, EntryChange, Operator};

pub trait HandleOperator<S> {
    type Position;

    fn handle_operator<I>(&mut self, op: Operator, positions: I, state: &mut S)
    where
        I: IntoIterator<Item = Self::Position>;
}

impl<T> HandleOperator<ActionHistory<Self>> for GridState<T>
where
    T: Clone + Eq + 'static,
{
    type Position = Position;

    fn handle_operator<I>(&mut self, op: Operator, positions: I, state: &mut ActionHistory<Self>)
    where
        I: IntoIterator<Item = Self::Position>,
    {
        let changes = positions
            .into_iter()
            .filter_map(|pos| {
                let entry = self.entries.get(pos)?;
                let before = entry.entry().cloned();
                let after = match op {
                    Operator::Delete | Operator::Change => None,
                    Operator::Reveal => self.solutions.get(pos).and_then(|sol| sol.clone()),
                    _ => None,
                };

                Some(EntryChange { pos, before, after })
            })
            .collect();

        let action = Box::new(EntryAction::new(op, changes));
        state.execute(action, self);
    }
}

impl<T> HandleOperator<ActionHistory<Self>> for SquareGridState<T>
where
    T: Clone + Eq + 'static + Debug,
{
    type Position = Position;

    fn handle_operator<I>(&mut self, op: Operator, positions: I, state: &mut ActionHistory<Self>)
    where
        I: IntoIterator<Item = Self::Position>,
    {
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

                Some(EntryChange { pos, before, after })
            })
            .collect();

        let action = Box::new(EntryAction::new(op, changes));
        state.execute(action, self);
    }
}
