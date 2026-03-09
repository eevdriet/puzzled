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
    pub count: usize,
    pub operator: Option<Operator>,
    pub action: Option<Action<A>>,
    pub motion: Option<Motion>,
}

impl<A> Default for Command<A> {
    fn default() -> Self {
        Self {
            count: 1,
            operator: None,
            action: None,
            motion: None,
        }
    }
}

impl<A> Command<A> {
    pub fn action(action: Action<A>) -> Self {
        Self {
            action: Some(action),
            ..Default::default()
        }
    }

    pub fn motion(motion: Motion) -> Self {
        Self {
            motion: Some(motion),
            ..Default::default()
        }
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
