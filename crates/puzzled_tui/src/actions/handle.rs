use crate::{Action, Command};

pub enum ActionOutcome<T> {
    // Handled interally
    Ignored,
    Consumed,

    // Handled externally
    Command(Box<dyn Command<T>>),

    // Screen management
    Exit,
    PreviousScreen,
}

impl<T> Default for ActionOutcome<T> {
    fn default() -> Self {
        Self::Consumed
    }
}

pub trait HandleAction<A> {
    type State;
    type Error;

    fn handle_action(
        &self,
        _action: Action<A>,
        _repeat: usize,
        _state: &mut Self::State,
    ) -> Result<ActionOutcome<Self::State>, Self::Error>;
}
