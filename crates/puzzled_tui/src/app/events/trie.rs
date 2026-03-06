use std::{collections::HashMap, fmt::Debug, hash::Hash, io};

use puzzled_core::Puzzle;
use puzzled_io::puzzle_config_dir;
use serde::de::DeserializeOwned;

use crate::{Action, AppEvent, RawActionKeys, app::events::parse_key, parse_action_events};

#[derive(Debug, Clone)]
pub struct EventTrieNode<A> {
    action: Option<Action<A>>,
    children: HashMap<AppEvent, EventTrieNode<A>>,
}

impl<A> Default for EventTrieNode<A> {
    fn default() -> Self {
        Self {
            action: None,
            children: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub enum EventSearchResult<A> {
    /// Event search does not lead to an action
    None,

    /// Events require an operand to constitute a full action
    RequireOperand(Action<A>),

    /// Events constitute a prefix for an action
    Prefix,

    /// Events trigger an action
    Exact(Action<A>),

    /// Events trigger an action and and prefix
    ExactPrefix(Action<A>),
}

impl<A> EventSearchResult<A> {
    pub fn action(self) -> Option<Action<A>> {
        match self {
            EventSearchResult::Exact(action)
            | EventSearchResult::ExactPrefix(action)
            | EventSearchResult::RequireOperand(action) => Some(action),
            _ => None,
        }
    }

    pub fn is_partial(&self) -> bool {
        matches!(
            self,
            EventSearchResult::Prefix | EventSearchResult::RequireOperand(_)
        )
    }
}

#[derive(Debug, Clone)]
pub struct EventTrie<A> {
    root: EventTrieNode<A>,
}

impl<A> Default for EventTrie<A> {
    fn default() -> Self {
        Self {
            root: EventTrieNode::<A>::default(),
        }
    }
}

impl<A> EventTrie<A>
where
    A: Hash + Clone + Eq + DeserializeOwned,
{
    pub fn from_config<P>() -> io::Result<Self>
    where
        P: Puzzle,
    {
        let config = puzzle_config_dir::<P>()?.join("actions.toml");
        let Ok(contents) = std::fs::read_to_string(config) else {
            return Ok(EventTrie::default());
        };

        let action_keys: RawActionKeys<A> = toml::from_str(&contents).map_err(io::Error::other)?;

        let trie = parse_action_events(action_keys).map_err(io::Error::other)?;
        Ok(trie)
    }
}

impl<A> EventTrie<A> {
    pub fn insert_key(&mut self, key: &str, action: Action<A>) -> bool {
        let Ok(events) = parse_key(key, &action) else {
            return false;
        };

        self.insert(&events, action);
        true
    }

    pub fn insert(&mut self, events: &[AppEvent], action: Action<A>) {
        let mut node = &mut self.root;

        for event in events {
            node = node.children.entry(event.clone()).or_default();
        }

        node.action = Some(action);
    }
}

impl<A> EventTrie<A>
where
    A: Clone,
{
    pub fn search(&self, events: &[AppEvent]) -> EventSearchResult<A> {
        if events.is_empty() {
            return EventSearchResult::None;
        }

        let mut node = &self.root;

        for event in events {
            tracing::debug!("\tSearch node for {event:?}");

            match node.children.get(event) {
                Some(next) => {
                    node = next;
                }
                None => {
                    tracing::debug!("\tEvent {event:?} not found, search failed");
                    return EventSearchResult::None;
                }
            };
        }

        let has_children = !node.children.is_empty();

        match node.action {
            None => {
                if has_children {
                    EventSearchResult::Prefix
                } else {
                    EventSearchResult::None
                }
            }
            Some(ref action) => {
                // if action.requires_operand() {
                //     return EventSearchResult::RequireOperand(action);
                // }

                if has_children {
                    EventSearchResult::ExactPrefix(action.clone())
                } else {
                    EventSearchResult::Exact(action.clone())
                }
            }
        }
    }
}
