use crate::EventMode;

pub struct AppContext<S> {
    pub state: S,
    pub mode: EventMode,
}

impl<S> AppContext<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            mode: EventMode::Normal,
        }
    }
}
