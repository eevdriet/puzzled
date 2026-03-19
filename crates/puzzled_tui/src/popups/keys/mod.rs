mod commands;
mod render;

use ratatui::widgets::TableState;

use crate::{
    Action, ActionBehavior, Description, KeyMap, Motion, MotionBehavior, TextObject,
    TextObjectBehavior,
};

pub struct KeysPopup<A, T, M> {
    actions: Vec<(String, String, Action<A>)>,
    text_objects: Vec<(String, String, TextObject<T>)>,
    motions: Vec<(String, String, Motion<M>)>,
    map: KeyMap<A, T, M>,
}

#[derive(Debug, Default)]
pub struct KeysPopupState {
    pub tab: usize,
    pub table: TableState,
}

impl<A, T, M> KeysPopup<A, T, M> {
    pub fn new(map: KeyMap<A, T, M>) -> Self {
        Self {
            map,
            actions: Vec::default(),
            text_objects: Vec::default(),
            motions: Vec::default(),
        }
    }

    pub fn action<D>(mut self, name: D, desc: D, action: Action<A>) -> Self
    where
        D: Into<String>,
    {
        self.actions.push((name.into(), desc.into(), action));
        self
    }

    pub fn actions<D>(mut self, actions: Vec<(D, D, Action<A>)>) -> Self
    where
        D: Into<String>,
    {
        self.actions.extend(
            actions
                .into_iter()
                .map(|(name, desc, action)| (name.into(), desc.into(), action)),
        );
        self
    }

    pub fn all_actions<S>(mut self, state: &S) -> Self
    where
        A: ActionBehavior,
        Action<A>: Description<S>,
    {
        self.actions = Action::<A>::variants()
            .into_iter()
            .filter_map(|action| Some((format!("{action:?}"), action.description(state)?, action)))
            .collect();
        self
    }

    pub fn motion<D>(mut self, name: D, desc: D, motion: Motion<M>) -> Self
    where
        D: Into<String>,
    {
        self.motions.push((name.into(), desc.into(), motion));
        self
    }

    pub fn motions<D>(mut self, motions: Vec<(D, D, Motion<M>)>) -> Self
    where
        D: Into<String>,
    {
        self.motions.extend(
            motions
                .into_iter()
                .map(|(name, desc, motion)| (name.into(), desc.into(), motion)),
        );
        self
    }

    pub fn all_motions<S>(mut self, state: &S) -> Self
    where
        M: MotionBehavior,
        Motion<M>: Description<S>,
    {
        self.motions = Motion::<M>::variants()
            .into_iter()
            .filter_map(|motion| Some((format!("{motion:?}"), motion.description(state)?, motion)))
            .collect();
        self
    }

    pub fn text_object<D>(mut self, name: D, desc: D, text_object: TextObject<T>) -> Self
    where
        D: Into<String>,
    {
        self.text_objects
            .push((name.into(), desc.into(), text_object));
        self
    }

    pub fn text_objects<D>(mut self, text_objects: Vec<(D, D, TextObject<T>)>) -> Self
    where
        D: Into<String>,
    {
        self.text_objects.extend(
            text_objects
                .into_iter()
                .map(|(name, desc, text_object)| (name.into(), desc.into(), text_object)),
        );
        self
    }

    pub fn all_text_objects<S>(mut self, state: &S) -> Self
    where
        T: TextObjectBehavior,
        TextObject<T>: Description<S>,
    {
        self.text_objects = TextObject::<T>::variants()
            .into_iter()
            .filter_map(|text_obj| {
                Some((
                    format!("{text_obj:?}"),
                    text_obj.description(state)?,
                    text_obj,
                ))
            })
            .collect();
        self
    }
}
