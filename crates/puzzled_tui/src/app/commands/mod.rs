mod action;
mod description;
mod entry;
mod handle;
mod history;
mod motion;
mod operator;
mod resolver;
mod text_object;

pub use action::*;
pub use description::*;
pub use entry::*;
pub use handle::*;
pub use history::*;
pub use motion::*;
pub use operator::*;
pub use resolver::*;
pub use text_object::*;

use crate::AppTypes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command<A, T, M> {
    Action {
        count: usize,
        action: Action<A>,
    },
    TextObj {
        count: usize,
        obj: TextObject<T>,
        op: Operator,
    },
    Motion {
        count: usize,
        motion: Motion<M>,
        op: Option<Operator>,
    },
    Operator(Operator),
}

pub type AppCommand<A> =
    Command<<A as AppTypes>::Action, <A as AppTypes>::TextObject, <A as AppTypes>::Motion>;

impl<A, T, M> Command<A, T, M> {
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
            Self::Operator(_) => 1,
        }
    }

    pub fn is_mode_changing(&self) -> bool {
        match self {
            Self::Operator(op) => op.is_mode_changing(),
            _ => false,
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
