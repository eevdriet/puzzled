use puzzled_core::{Puzzle, Solve};

use crate::{ExecuteAction, Operator, UndoAction};

pub struct EntryAction<P: Puzzle> {
    op: Operator,
    changes: Vec<EntryChange<P>>,
}

impl<P> EntryAction<P>
where
    P: Puzzle,
{
    pub fn new(op: Operator, changes: Vec<EntryChange<P>>) -> Self {
        Self { op, changes }
    }
}

#[derive(Debug)]
pub struct EntryChange<P: Puzzle> {
    pub pos: P::Position,
    pub before: Option<P::Value>,
    pub after: Option<P::Value>,
}

impl<P, S> ExecuteAction<S> for EntryAction<P>
where
    P: Puzzle,
    P::Value: Clone,
    S: Solve<P>,
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

impl<P, S> UndoAction<S> for EntryAction<P>
where
    P: Puzzle,
    P::Value: Clone,
    S: Solve<P>,
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
