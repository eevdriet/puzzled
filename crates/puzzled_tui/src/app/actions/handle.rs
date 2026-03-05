use crate::{Action, ActionResolver, Command, StatefulScreen};

pub enum ActionOutcome<A, T> {
    // Handled interally
    Ignored,
    Consumed,

    // Handled externally
    Command(Box<dyn Command<T>>),

    // Screen management
    Exit,
    PreviousScreen,
    NextScreen(Box<dyn StatefulScreen<A, T>>),
    ReplaceScreen(Box<dyn StatefulScreen<A, T>>),
}

impl<A, T> Default for ActionOutcome<A, T> {
    fn default() -> Self {
        Self::Consumed
    }
}

pub trait HandleAction<A, T> {
    type State;

    fn on_action(
        &mut self,
        _action: Action<A>,
        _resolver: ActionResolver<A, T>,
        _state: &mut Self::State,
    );
}
