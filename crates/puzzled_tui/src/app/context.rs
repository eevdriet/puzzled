use crate::Keys;

pub struct AppContext<A, T, M, S> {
    pub state: S,
    pub keys: Keys<A, T, M>,
}

impl<A, T, M, S> AppContext<A, T, M, S> {
    pub fn new(state: S, keys: Keys<A, T, M>) -> Self {
        Self { state, keys }
    }
}
