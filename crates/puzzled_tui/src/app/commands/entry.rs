use puzzled_core::Solve;

use crate::{_Command, UndoCommand};

pub struct EntryCommand<T, P> {
    changes: Vec<EntryChange<T, P>>,
}

pub struct EntryChange<T, P> {
    pos: P,
    before: T,
    after: T,
}

impl<T, P, S> _Command<S> for EntryCommand<T, P>
where
    T: Clone,
    S: Solve<Value = T, Position = P>,
{
    fn execute(&mut self, state: &mut S) {
        for change in &self.changes {
            state.enter(&change.pos, change.after.clone());
        }
    }
}

impl<T, P, S> UndoCommand<S> for EntryCommand<T, P>
where
    T: Clone,
    S: Solve<Value = T, Position = P>,
{
    fn undo(&mut self, state: &mut S) {
        for change in &self.changes {
            state.enter(&change.pos, change.before.clone());
        }
    }
}
