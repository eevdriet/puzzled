use crate::KeyMap;

pub struct AppContext<A, T, M, S> {
    pub state: S,
    pub keys: KeyMap<A, T, M>,
}

impl<A, T, M, S> AppContext<A, T, M, S> {
    pub fn new(state: S, keys: KeyMap<A, T, M>) -> Self {
        Self { state, keys }
    }
}
