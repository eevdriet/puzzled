pub trait Describe {
    fn describe(&self) -> Option<String> {
        None
    }
}

pub trait Description<S> {
    fn description(&self, state: &S) -> Option<String>;
}

impl Describe for () {
    fn describe(&self) -> Option<String> {
        None
    }
}

impl<T> Description<()> for T
where
    T: Describe,
{
    fn description(&self, _state: &()) -> Option<String> {
        self.describe()
    }
}
