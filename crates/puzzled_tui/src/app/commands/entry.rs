use puzzled_core::Solve;

use crate::{ExecuteAction, UndoAction};

#[derive(Debug)]
pub struct EntryAction<T, P> {
    changes: Vec<EntryChange<T, P>>,
}

impl<T, P> EntryAction<T, P> {
    pub fn new(changes: Vec<EntryChange<T, P>>) -> Self {
        Self { changes }
    }
}

#[derive(Debug)]
pub struct EntryChange<T, P> {
    pub pos: P,
    pub before: Option<T>,
    pub after: Option<T>,
}

impl<T, P, S> ExecuteAction<S> for EntryAction<T, P>
where
    T: Clone,
    S: Solve<Value = T, Position = P>,
{
    fn execute(&mut self, state: &mut S) {
        for change in &self.changes {
            match change.after {
                Some(ref entry) => {
                    state.enter(&change.pos, entry.clone());
                }
                None => {
                    state.clear(&change.pos);
                }
            }
        }
    }
}

impl<T, P, S> UndoAction<S> for EntryAction<T, P>
where
    T: Clone,
    S: Solve<Value = T, Position = P>,
{
    fn undo(&mut self, state: &mut S) {
        for change in &self.changes {
            match change.before {
                Some(ref entry) => {
                    state.enter(&change.pos, entry.clone());
                }
                None => {
                    state.clear(&change.pos);
                }
            }
        }
    }
}
