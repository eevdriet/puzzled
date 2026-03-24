use puzzled_core::Solve;

use crate::{ExecuteAction, Operator, UndoAction};

#[derive(Debug)]
pub struct EntryAction<T, P> {
    op: Operator,
    changes: Vec<EntryChange<T, P>>,
}

impl<T, P> EntryAction<T, P> {
    pub fn new(op: Operator, changes: Vec<EntryChange<T, P>>) -> Self {
        Self { op, changes }
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
            match self.op {
                // Enter the next entry into the state
                Operator::Change
                | Operator::ChangeSingle
                | Operator::Delete
                | Operator::DeleteLeft
                | Operator::DeleteRight => match change.after {
                    Some(ref entry) => {
                        state.enter(&change.pos, entry.clone());
                    }
                    None => {
                        state.clear(&change.pos);
                    }
                },

                // Puzzle actions
                Operator::Reveal | Operator::RevealSingle => {
                    state.reveal(&change.pos);
                }
                Operator::Check | Operator::CheckSingle => {
                    state.check(&change.pos);
                }
                _ => {
                    continue;
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
            match self.op {
                // Enter the previous entry back into the state
                Operator::Change
                | Operator::ChangeSingle
                | Operator::Delete
                | Operator::DeleteLeft
                | Operator::DeleteRight => match change.before {
                    Some(ref entry) => {
                        state.enter(&change.pos, entry.clone());
                    }
                    None => {
                        state.clear(&change.pos);
                    }
                },

                // Cannot undo reveal/check
                Operator::Reveal | Operator::Check => {
                    continue;
                }

                _ => continue,
            }
        }
    }
}
