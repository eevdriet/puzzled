use crate::{AppTypes, KeyMap, Options, Settings, Theme};

pub struct AppContext<A: AppTypes> {
    pub state: A::State,
    pub keys: KeyMap<A>,
    pub options: Options,
    pub theme: Theme,
}

impl<A: AppTypes> AppContext<A> {
    pub fn new(state: A::State, settings: Settings<A>) -> Self {
        Self {
            state,
            keys: settings.keys.action_keys(),
            options: settings.options,
            theme: settings.theme,
        }
    }
}
