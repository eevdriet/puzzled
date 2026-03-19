mod commands;
mod render;

use std::borrow::Cow;

use ratatui::widgets::TableState;

use crate::{
    Action, ActionBehavior, Description, KeyMap, Motion, MotionBehavior, TextObject,
    TextObjectBehavior,
};

pub struct KeysPopup<'a, A, T, M, S> {
    state: &'a S,
    actions: Vec<(Cow<'a, str>, Action<A>)>,
    text_objects: Vec<(Cow<'a, str>, TextObject<T>)>,
    motions: Vec<(Cow<'a, str>, Motion<M>)>,

    map: &'a KeyMap<A, T, M>,
}

#[derive(Debug, Default)]
pub struct KeysPopupState {
    pub tab: usize,
    pub table: TableState,
}

impl<'a, A, T, M, S> KeysPopup<'a, A, T, M, S> {
    pub fn new(state: &'a S, map: &'a KeyMap<A, T, M>) -> Self {
        Self {
            state,
            map,
            actions: Vec::default(),
            text_objects: Vec::default(),
            motions: Vec::default(),
        }
    }

    pub fn action<D>(mut self, desc: D, action: Action<A>) -> Self
    where
        D: Into<Cow<'a, str>>,
    {
        self.actions.push((desc.into(), action));
        self
    }

    pub fn actions<D>(mut self, actions: Vec<(D, Action<A>)>) -> Self
    where
        D: Into<Cow<'a, str>>,
    {
        self.actions.extend(
            actions
                .into_iter()
                .map(|(key, action)| (key.into(), action)),
        );
        self
    }

    pub fn all_actions(mut self) -> Self
    where
        A: ActionBehavior,
        Action<A>: Description<S>,
    {
        self.actions = Action::<A>::variants()
            .into_iter()
            .filter_map(|action| Some((action.description(self.state)?.into(), action)))
            .collect();
        self
    }

    pub fn motion<D>(mut self, desc: D, motion: Motion<M>) -> Self
    where
        D: Into<Cow<'a, str>>,
    {
        self.motions.push((desc.into(), motion));
        self
    }

    pub fn motions<D>(mut self, motions: Vec<(D, Motion<M>)>) -> Self
    where
        D: Into<Cow<'a, str>>,
    {
        self.motions.extend(
            motions
                .into_iter()
                .map(|(key, motion)| (key.into(), motion)),
        );
        self
    }

    pub fn all_motions(mut self) -> Self
    where
        M: MotionBehavior,
        Motion<M>: Description<S>,
    {
        self.motions = Motion::<M>::variants()
            .into_iter()
            .filter_map(|motion| Some((motion.description(self.state)?.into(), motion)))
            .collect();
        self
    }

    pub fn text_object<D>(mut self, desc: D, text_object: TextObject<T>) -> Self
    where
        D: Into<Cow<'a, str>>,
    {
        self.text_objects.push((desc.into(), text_object));
        self
    }

    pub fn text_objects<D>(mut self, text_objects: Vec<(D, TextObject<T>)>) -> Self
    where
        D: Into<Cow<'a, str>>,
    {
        self.text_objects.extend(
            text_objects
                .into_iter()
                .map(|(key, text_object)| (key.into(), text_object)),
        );
        self
    }

    pub fn all_text_objects(mut self) -> Self
    where
        T: TextObjectBehavior,
        TextObject<T>: Description<S>,
    {
        self.text_objects = TextObject::<T>::variants()
            .into_iter()
            .filter_map(|text_obj| Some((text_obj.description(self.state)?.into(), text_obj)))
            .collect();
        self
    }
}
