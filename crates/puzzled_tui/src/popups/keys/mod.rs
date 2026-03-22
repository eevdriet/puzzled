mod list;
mod table;

pub use list::*;
pub use table::*;

use crate::{
    Action, ActionBehavior, AppTypes, Description, KeyMap, Motion, MotionBehavior, TextObject,
    TextObjectBehavior,
};

pub struct Keys<A: AppTypes> {
    pub actions: Vec<(String, String, Action<A::Action>)>,
    pub text_objects: Vec<(String, String, TextObject<A::TextObject>)>,
    pub motions: Vec<(String, String, Motion<A::Motion>)>,
    map: KeyMap<A>,
}

impl<A: AppTypes> Keys<A> {
    pub fn new(map: KeyMap<A>) -> Self {
        Self {
            map,
            actions: Vec::default(),
            text_objects: Vec::default(),
            motions: Vec::default(),
        }
    }

    pub fn custom_action<D>(mut self, name: D, desc: D, action: Action<A::Action>) -> Self
    where
        D: Into<String>,
    {
        self.actions.push((name.into(), desc.into(), action));
        self
    }

    pub fn actions<D>(mut self, actions: Vec<(D, D, Action<A::Action>)>) -> Self
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

    pub fn action<S>(mut self, action: Action<A::Action>, state: &S) -> Self
    where
        Action<A::Action>: Description<S>,
    {
        let name = format!("{action:?}");

        if let Some(desc) = action.description(state) {
            self.actions.push((name, desc, action));
        }

        self
    }

    pub fn all_actions<S>(mut self, state: &S) -> Self
    where
        Action<A::Action>: Description<S>,
    {
        self.actions = Action::<A::Action>::variants()
            .into_iter()
            .filter_map(|action| Some((format!("{action:?}"), action.description(state)?, action)))
            .collect();
        self
    }

    pub fn motion<D>(mut self, name: D, desc: D, motion: Motion<A::Motion>) -> Self
    where
        D: Into<String>,
    {
        self.motions.push((name.into(), desc.into(), motion));
        self
    }

    pub fn motions<D>(mut self, motions: Vec<(D, D, Motion<A::Motion>)>) -> Self
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
        Motion<A::Motion>: Description<S>,
    {
        self.motions = Motion::<A::Motion>::variants()
            .into_iter()
            .filter_map(|motion| Some((format!("{motion:?}"), motion.description(state)?, motion)))
            .collect();
        self
    }

    pub fn text_object<D>(
        mut self,
        name: D,
        desc: D,
        text_object: TextObject<A::TextObject>,
    ) -> Self
    where
        D: Into<String>,
    {
        self.text_objects
            .push((name.into(), desc.into(), text_object));
        self
    }

    pub fn text_objects<D>(mut self, text_objects: Vec<(D, D, TextObject<A::TextObject>)>) -> Self
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
        TextObject<A::TextObject>: Description<S>,
    {
        self.text_objects = TextObject::<A::TextObject>::variants()
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

    pub fn all<S>(self, state: &S) -> Self
    where
        Action<A::Action>: Description<S>,
        Motion<A::Motion>: Description<S>,
        TextObject<A::TextObject>: Description<S>,
    {
        self.all_actions(state)
            .all_motions(state)
            .all_text_objects(state)
    }
}

impl<A: AppTypes> Clone for Keys<A> {
    fn clone(&self) -> Self {
        Self {
            actions: self.actions.clone(),
            text_objects: self.text_objects.clone(),
            motions: self.motions.clone(),
            map: self.map.clone(),
        }
    }
}
