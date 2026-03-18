pub struct AppContext<S> {
    pub state: S,
}

impl<S> AppContext<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }
}
