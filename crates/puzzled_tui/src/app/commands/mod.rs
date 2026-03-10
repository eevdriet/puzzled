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
pub struct Command<M, A> {
    count: usize,
    motion: Motion<M>,

    operator: Option<Operator>,
    action: Option<Action<A>>,
}

impl<M, A> Default for Command<M, A> {
    fn default() -> Self {
        Self {
            count: 1,
            motion: Motion::None,
            operator: None,
            action: None,
        }
    }
}

impl<M, A> Command<M, A> {
    pub fn new(
        count: usize,
        motion: Motion<M>,
        operator: Option<Operator>,
        action: Option<Action<A>>,
    ) -> Self {
        Self {
            count,
            operator,
            action,
            motion,
        }
    }

    pub fn new_action(action: Action<A>) -> Self {
        Self {
            action: Some(action),
            ..Default::default()
        }
    }

    pub fn new_motion(motion: Motion<M>) -> Self {
        Self {
            motion,
            ..Default::default()
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn operator(&self) -> Option<&Operator> {
        self.operator.as_ref()
    }

    pub fn motion(&self) -> &Motion<M> {
        &self.motion
    }

    pub fn action(&self) -> Option<&Action<A>> {
        self.action.as_ref()
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
