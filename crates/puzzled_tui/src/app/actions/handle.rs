use crate::{Action, ActionResolver, Command, StatefulScreen};

pub enum ActionOutcome<A, T> {
    // Handled externally
    Action(Action<A>),
    Command(Box<dyn Command<T>>),

    // Screen management
    Quit,
    PreviousScreen,
    NextScreen(Box<dyn StatefulScreen<A, T>>),
    ReplaceScreen(Box<dyn StatefulScreen<A, T>>),
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
