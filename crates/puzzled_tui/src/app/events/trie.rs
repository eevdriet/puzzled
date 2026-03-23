use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    io,
};

use puzzled_core::Puzzle;
use puzzled_io::{config_dir, puzzle_config_dir};
use serde::Deserialize;

use crate::{
    Action, ActionBehavior, AppEvent, AppTypes, Motion, MotionBehavior, Operator, RawKeys,
    TextModifier, TextObject, TextObjectBehavior, app::events::parse_key, insert_action_keys,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum TrieEntry<A, T, M> {
    Action(Action<A>),
    TextObject(TextObject<T>),
    Motion(Motion<M>),
    Operator(Operator),
    TextModifier(TextModifier),
}
pub type AppTrieEntry<A> =
    TrieEntry<<A as AppTypes>::Action, <A as AppTypes>::TextObject, <A as AppTypes>::Motion>;

impl<A, T, M> Ord for TrieEntry<A, T, M>
where
    A: ActionBehavior,
    T: TextObjectBehavior,
    M: MotionBehavior,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = |entry: &TrieEntry<A, T, M>| match entry {
            TrieEntry::Action(_) => 0,
            TrieEntry::TextObject(_) => 1,
            TrieEntry::Motion(_) => 2,
            TrieEntry::Operator(_) => 3,
            TrieEntry::TextModifier(_) => 4,
        };

        match (self, other) {
            (TrieEntry::Action(lhs), TrieEntry::Action(rhs)) => lhs.cmp(rhs),
            (TrieEntry::TextObject(lhs), TrieEntry::TextObject(rhs)) => lhs.cmp(rhs),
            (TrieEntry::Motion(lhs), TrieEntry::Motion(rhs)) => lhs.cmp(rhs),
            (TrieEntry::Operator(lhs), TrieEntry::Operator(rhs)) => lhs.cmp(rhs),
            (TrieEntry::TextModifier(lhs), TrieEntry::TextModifier(rhs)) => lhs.cmp(rhs),
            (lhs, rhs) => order(lhs).cmp(&order(rhs)),
        }
    }
}

impl<A, T, M> PartialOrd for TrieEntry<A, T, M>
where
    A: ActionBehavior,
    T: TextObjectBehavior,
    M: MotionBehavior,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct EventTrieNode<A: AppTypes> {
    entry: Option<AppTrieEntry<A>>,
    children: HashMap<AppEvent, EventTrieNode<A>>,
}

impl<A: AppTypes> Default for EventTrieNode<A> {
    fn default() -> Self {
        Self {
            entry: None,
            children: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub enum EventSearchResult<A: AppTypes> {
    /// Event search does not lead to an action
    None,

    /// Events constitute a prefix for an action
    Prefix,

    /// Events trigger an action
    Exact(AppTrieEntry<A>),

    /// Events trigger an action and and prefix
    ExactPrefix(AppTrieEntry<A>),
}

impl<A: AppTypes> EventSearchResult<A> {
    pub fn is_partial(&self) -> bool {
        matches!(self, EventSearchResult::Prefix)
    }
}

#[derive(Debug, Clone)]
pub struct EventTrie<A: AppTypes> {
    root: EventTrieNode<A>,
}

impl<A: AppTypes> Default for EventTrie<A> {
    fn default() -> Self {
        Self {
            root: EventTrieNode::<A>::default(),
        }
    }
}

impl<A: AppTypes> EventTrie<A> {
    pub fn from_config<P>() -> io::Result<Self>
    where
        P: Puzzle,
    {
        let mut trie = EventTrie::default();

        // --- Global config (optional) ---
        let global_path = config_dir()?.join("keys.toml");

        if let Ok(contents) = std::fs::read_to_string(&global_path) {
            let keys: RawKeys<A::Action, A::TextObject, A::Motion> =
                toml::from_str(&contents).map_err(io::Error::other)?;

            insert_action_keys(keys, &mut trie).map_err(io::Error::other)?;
        }

        // --- Puzzle config (optional, overrides global) ---
        let puzzle_path = puzzle_config_dir::<P>()?.join("keys.toml");

        if let Ok(contents) = std::fs::read_to_string(&puzzle_path) {
            let keys: RawKeys<A::Action, A::TextObject, A::Motion> =
                toml::from_str(&contents).map_err(io::Error::other)?;

            insert_action_keys(keys, &mut trie).map_err(io::Error::other)?;
        }

        Ok(trie)
    }
}

impl<A: AppTypes> EventTrie<A> {
    pub fn insert_key(
        &mut self,
        key: &str,
        entry: TrieEntry<A::Action, A::TextObject, A::Motion>,
    ) -> bool {
        let Ok(events) = parse_key(key) else {
            return false;
        };

        self.insert(&events, entry);
        true
    }

    pub fn insert(
        &mut self,
        events: &[AppEvent],
        entry: TrieEntry<A::Action, A::TextObject, A::Motion>,
    ) {
        let mut node = &mut self.root;

        for event in events {
            node = node.children.entry(event.clone()).or_default();
        }

        node.entry = Some(entry);
    }
}

impl<A: AppTypes> EventTrie<A> {
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

pub struct KeyMap<A: AppTypes> {
    keys: BTreeMap<AppTrieEntry<A>, Vec<String>>,
}

impl<A: AppTypes> KeyMap<A> {
    pub fn new(keys: BTreeMap<AppTrieEntry<A>, Vec<String>>) -> Self {
        Self { keys }
    }

    pub fn get_merged(&self, entry: &AppTrieEntry<A>) -> Option<String> {
        let keys = self.keys.get(entry)?;
        let mut iter = keys.iter();

        let first = iter.next()?.clone();
        Some(iter.fold(first, |acc, item| format!("{acc} / {item}")))
    }

    pub fn keys(&self) -> &BTreeMap<AppTrieEntry<A>, Vec<String>> {
        &self.keys
    }
}

impl<A: AppTypes> Clone for KeyMap<A> {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
        }
    }
}

impl<A: AppTypes> EventTrie<A> {
    pub fn action_keys(&self) -> KeyMap<A> {
        // Initialize keys for all action variants
        let mut keys = BTreeMap::default();

        // Perform a DFS to find all actions for which keys are defined
        dfs(&self.root, &mut keys, vec![]);

        // Sort the keys lexographically for a consistent ordering
        for values in keys.values_mut() {
            values.sort();
        }

        KeyMap { keys }
    }
}

fn dfs<A: AppTypes>(
    node: &EventTrieNode<A>,
    result: &mut BTreeMap<AppTrieEntry<A>, Vec<String>>,
    current_events: Vec<AppEvent>,
) {
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
