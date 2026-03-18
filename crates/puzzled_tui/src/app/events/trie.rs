use std::{collections::HashMap, fmt::Debug, hash::Hash, io};

use puzzled_core::Puzzle;
use puzzled_io::puzzle_config_dir;
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    Action, ActionBehavior, AppEvent, Motion, MotionBehavior, Operator, RawActionKeys,
    SelectionKind, TextObject, TextObjectBehavior, app::events::parse_key, parse_action_events,
};

use super::EventMode;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum TrieEntry<A, T, M> {
    Action(Action<A>),
    TextObject(TextObject<T>),
    Motion(Motion<M>),
    Operator(Operator),
}

#[derive(Debug, Clone)]
pub struct EventTrieNode<A, T, M> {
    entry: Option<TrieEntry<A, T, M>>,
    children: HashMap<AppEvent, EventTrieNode<A, T, M>>,
}

impl<A, T, M> Default for EventTrieNode<A, T, M> {
    fn default() -> Self {
        Self {
            entry: None,
            children: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub enum EventSearchResult<A, T, M> {
    /// Event search does not lead to an action
    None,

    /// Events constitute a prefix for an action
    Prefix,

    /// Events trigger an action
    Exact(TrieEntry<A, T, M>),

    /// Events trigger an action and and prefix
    ExactPrefix(TrieEntry<A, T, M>),
}

impl<A, T, M> EventSearchResult<A, T, M> {
    pub fn is_partial(&self) -> bool {
        matches!(self, EventSearchResult::Prefix)
    }
}

#[derive(Debug, Clone)]
pub struct EventTrie<A, T, M> {
    root: EventTrieNode<A, T, M>,
}

impl<A, T, M> Default for EventTrie<A, T, M> {
    fn default() -> Self {
        Self {
            root: EventTrieNode::<A, T, M>::default(),
        }
    }
}

impl<A, T, M> EventTrie<A, T, M>
where
    A: Hash + Clone + Eq + DeserializeOwned,
    T: Hash + Clone + Eq + DeserializeOwned,
    M: Hash + Clone + Eq + DeserializeOwned,
{
    pub fn from_config<P>() -> io::Result<Self>
    where
        P: Puzzle,
    {
        let config = puzzle_config_dir::<P>()?.join("actions.toml");
        let Ok(contents) = std::fs::read_to_string(config) else {
            return Ok(EventTrie::default());
        };

        let action_keys: RawActionKeys<A, T, M> =
            toml::from_str(&contents).map_err(io::Error::other)?;

        let trie = parse_action_events(action_keys).map_err(io::Error::other)?;
        Ok(trie)
    }
}

impl<A, T, M> EventTrie<A, T, M> {
    pub fn insert_key(&mut self, key: &str, entry: TrieEntry<A, T, M>) -> bool {
        let Ok(events) = parse_key(key) else {
            return false;
        };

        self.insert(&events, entry);
        true
    }

    pub fn insert(&mut self, events: &[AppEvent], entry: TrieEntry<A, T, M>) {
        let mut node = &mut self.root;

        for event in events {
            node = node.children.entry(event.clone()).or_default();
        }

        node.entry = Some(entry);
    }
}

impl<A, T, M> EventTrie<A, T, M>
where
    A: Clone,
    T: Clone,
    M: Clone,
{
    pub fn search(&self, events: &[AppEvent]) -> EventSearchResult<A, T, M> {
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

        match node.entry {
            None => {
                if has_children {
                    EventSearchResult::Prefix
                } else {
                    EventSearchResult::None
                }
            }
            Some(ref command) => {
                if has_children {
                    EventSearchResult::ExactPrefix(command.clone())
                } else {
                    EventSearchResult::Exact(command.clone())
                }
            }
        }
    }
}

impl<A, T, M> EventTrie<A, T, M>
where
    A: ActionBehavior + Hash,
    M: MotionBehavior + Hash,
    T: TextObjectBehavior + Hash,
{
    pub fn action_keys(&self) -> HashMap<TrieEntry<A, T, M>, Vec<String>> {
        // Initialize keys for all action variants
        let mut keys = HashMap::default();

        // TODO: add back all variants for motions/operators etc.
        // for action in Action::<A, T, M>::variants() {
        //     keys.entry(action).or_default();
        // }
        //

        // Perform a DFS to find all actions for which keys are defined
        dfs(&self.root, &mut keys, vec![]);

        keys
    }
}

fn dfs<A, T, M>(
    node: &EventTrieNode<A, T, M>,
    result: &mut HashMap<TrieEntry<A, T, M>, Vec<String>>,
    current_events: Vec<AppEvent>,
) where
    A: Eq + Hash + Clone,
    M: Eq + Hash + Clone,
    T: Eq + Hash + Clone,
{
    // If the node has an action, add the current path of events to the result
    if let Some(entry) = &node.entry {
        let keys = current_events
            .clone()
            .into_iter()
            .map(|ev| ev.to_string())
            .fold(String::new(), |acc, item| acc + &item);

        result.entry(entry.clone()).or_default().push(keys);
    }

    // Traverse children nodes, accumulating events
    for (event, child) in &node.children {
        let mut new_events = current_events.clone();
        new_events.push(event.clone());
        dfs(child, result, new_events);
    }
}
