use crate::{AppTypes, KeyMap, Options, Settings};

pub struct AppContext<A: AppTypes> {
    pub state: A::State,
    pub keys: KeyMap<A>,
    pub options: Options,
}

impl<A: AppTypes> AppContext<A> {
    pub fn new(state: A::State, settings: Settings<A>) -> Self {
        Self {
            state,
            keys: settings.keys.action_keys(),
            options: settings.options,
        }
    }
}
