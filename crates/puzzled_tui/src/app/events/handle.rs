use crate::{ActionResolver, AppEvent};

pub trait HandleEvent<A, T> {
    type State;

    fn on_event(
        &mut self,
        _event: AppEvent,
        _resolver: ActionResolver<A, T>,
        _state: &mut Self::State,
    ) -> bool;
}
