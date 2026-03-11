use crate::{ActionResolver, Command, EventMode, StatefulScreen};

pub enum CommandOutcome<M, A, T> {
    // Handled externally
    Command(Command<M, A>),
    Mode(EventMode),

    // Screen management
    Quit,
    PreviousScreen,
    NextScreen(Box<dyn StatefulScreen<M, A, T>>),
}

pub trait HandleCommand<M, A, T> {
    type State;

    fn on_command(
        &mut self,
        _command: Command<M, A>,
        _resolver: ActionResolver<M, A, T>,
        _state: &mut Self::State,
    ) -> bool;
}
