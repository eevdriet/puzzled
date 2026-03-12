use crate::Action;

pub trait HandleBaseAction<A, S> {
    fn handle_base_action(&mut self, _action: Action<A>, state: &mut S) -> bool;
}

pub trait HandleCustomAction<A, S> {
    fn handle_custom_action(&mut self, _action: A, _state: &mut S) -> bool;
}
