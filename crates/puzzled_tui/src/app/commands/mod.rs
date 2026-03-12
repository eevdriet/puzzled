mod action;
mod entry;
mod handle;
mod history;
mod motion;
mod operator;
mod resolver;

pub use action::*;
pub use entry::*;
pub use handle::*;
pub use history::*;
pub use motion::*;
pub use operator::*;
pub use resolver::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command<M, A> {
    Action {
        count: usize,
        action: Action<A>,
    },
    Motion {
        count: usize,
        motion: Motion<M>,
        op: Option<Operator>,
    },
    TextObj {
        count: usize,
        a: usize,
        op: Operator,
    },
}

impl<M, A> Command<M, A> {
    pub fn new_action(action: Action<A>) -> Self {
        Self::Action { count: 1, action }
    }

    pub fn new_motion(motion: Motion<M>) -> Self {
        Self::Motion {
            count: 1,
            motion,
            op: None,
        }
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Action { count, .. } => *count,
            Self::Motion { count, .. } => *count,
            Self::TextObj { count, .. } => *count,
        }
    }
}

pub trait ExecuteAction<T> {
    fn execute(&mut self, state: &mut T);
}

pub trait UndoAction<T>: ExecuteAction<T> {
    fn undo(&mut self, state: &mut T);

    fn redo(&mut self, state: &mut T) {
        self.execute(state);
    }
}
