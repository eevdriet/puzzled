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
pub struct Command<A> {
    count: usize,
    operator: Option<Operator>,
    motion: Option<Motion>,
    action: Option<Action<A>>,
}

impl<A> Default for Command<A> {
    fn default() -> Self {
        Self {
            count: 1,
            operator: None,
            motion: None,
            action: None,
        }
    }
}

impl<A> Command<A> {
    pub fn new(
        count: usize,
        operator: Option<Operator>,
        motion: Option<Motion>,
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

    pub fn new_motion(motion: Motion) -> Self {
        Self {
            motion: Some(motion),
            ..Default::default()
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn operator(&self) -> Option<&Operator> {
        self.operator.as_ref()
    }

    pub fn motion(&self) -> Option<&Motion> {
        self.motion.as_ref()
    }

    pub fn action(&self) -> Option<&Action<A>> {
        self.action.as_ref()
    }
}

pub trait _Command<T> {
    fn execute(&mut self, state: &mut T);
}

pub trait UndoCommand<T>: _Command<T> {
    fn undo(&mut self, state: &mut T);

    fn redo(&mut self, state: &mut T) {
        self.execute(state);
    }
}
