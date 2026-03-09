use std::{collections::HashMap, fmt::Debug, hash::Hash, io};

use puzzled_core::Puzzle;
use puzzled_io::puzzle_config_dir;
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    Action, ActionBehavior, AppEvent, Motion, Operator, RawActionKeys, app::events::parse_key,
    parse_action_events,
};

use super::EventMode;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum TrieEntry<A> {
    Motion(Motion),
    Operator(Operator),
    Action(Action<A>),
}

#[derive(Debug, Clone)]
pub struct EventTrieNode<A> {
    entry: Option<TrieEntry<A>>,
    children: HashMap<AppEvent, EventTrieNode<A>>,
}

impl<A> Default for EventTrieNode<A> {
    fn default() -> Self {
        Self {
            entry: None,
            children: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub enum EventSearchResult<A> {
    /// Event search does not lead to an action
    None,

    /// Events require an operand to constitute a full action
    RequireOperand(Operator),

    /// Events constitute a prefix for an action
    Prefix,

    /// Events trigger an action
    Exact(TrieEntry<A>),

    /// Events trigger an action and and prefix
    ExactPrefix(TrieEntry<A>),
}

impl<A> EventSearchResult<A> {
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
    pub fn insert_key(&mut self, key: &str, entry: TrieEntry<A>) -> bool {
        let Ok(events) = parse_key(key, &entry) else {
            return false;
        };

        self.insert(&events, entry);
        true
    }

    pub fn insert(&mut self, events: &[AppEvent], entry: TrieEntry<A>) {
        let mut node = &mut self.root;

        for event in events {
            node = node.children.entry(event.clone()).or_default();
        }

        node.entry = Some(entry);
    }

    pub fn insert_mode_switches(&mut self) {
        self.insert_key("i", TrieEntry::Action(Action::NextMode(EventMode::Insert)));
        self.insert_key("a", TrieEntry::Action(Action::NextMode(EventMode::Insert)));
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

        match node.entry {
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

impl<A> EventTrie<A>
where
    A: ActionBehavior + Eq + Hash + Clone,
{
    pub fn action_keys(&self) -> HashMap<TrieEntry<A>, Vec<String>> {
        // Initialize keys for all action variants
        let mut keys = HashMap::default();

        // TODO: add back all variants for motions/operators etc.
        // for action in Action::<A>::variants() {
        //     keys.entry(action).or_default();
        // }
        //

        // Perform a DFS to find all actions for which keys are defined
        dfs(&self.root, &mut keys, vec![]);

        keys
    }
}

fn dfs<A>(
    node: &EventTrieNode<A>,
    result: &mut HashMap<TrieEntry<A>, Vec<String>>,
    current_events: Vec<AppEvent>,
) where
    A: Eq + Hash + Clone,
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
