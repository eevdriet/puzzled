pub trait Description<S> {
    fn description(&self, state: &S) -> Option<String>;
}

impl<S> Description<S> for () {
    fn description(&self, _state: &S) -> Option<String> {
        None
    }
}
