use crate::{AppTypes, KeyMap};

pub struct AppContext<A: AppTypes> {
    pub state: A::State,
    pub keys: KeyMap<A>,
}

impl<A: AppTypes> AppContext<A> {
    pub fn new(state: A::State, keys: KeyMap<A>) -> Self {
        Self { state, keys }
    }
}
