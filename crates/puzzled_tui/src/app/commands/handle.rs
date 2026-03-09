use crate::{_Command, ActionResolver, Command, StatefulScreen};

pub enum CommandOutcome<A, T> {
    // Handled externally
    Command(Command<A>),
    UndoCommand(Box<dyn _Command<T>>),

    // Screen management
    Quit,
    PreviousScreen,
    NextScreen(Box<dyn StatefulScreen<A, T>>),
    ReplaceScreen(Box<dyn StatefulScreen<A, T>>),
}

pub trait HandleCommand<A, T> {
    type State;

    fn on_command(
        &mut self,
        _command: Command<A>,
        _resolver: ActionResolver<A, T>,
        _state: &mut Self::State,
    ) -> bool;
}
