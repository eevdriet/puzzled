use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    io,
};

use puzzled_core::Puzzle;
use puzzled_io::puzzle_config_dir;
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    Action, ActionBehavior, AppEvent, Motion, MotionBehavior, Operator, RawActionKeys, TextObject,
    TextObjectBehavior, app::events::parse_key, parse_action_events,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum TrieEntry<A, T, M> {
    Action(Action<A>),
    TextObject(TextObject<T>),
    Motion(Motion<M>),
    Operator(Operator),
}

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
        };

        match (self, other) {
            (TrieEntry::Action(lhs), TrieEntry::Action(rhs)) => lhs.cmp(rhs),
            (TrieEntry::TextObject(lhs), TrieEntry::TextObject(rhs)) => lhs.cmp(rhs),
            (TrieEntry::Motion(lhs), TrieEntry::Motion(rhs)) => lhs.cmp(rhs),
            (TrieEntry::Operator(lhs), TrieEntry::Operator(rhs)) => lhs.cmp(rhs),
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

pub struct Keys<A, T, M> {
    keys: BTreeMap<TrieEntry<A, T, M>, Vec<String>>,
}

impl<A, T, M> Keys<A, T, M> {
    pub fn new(keys: BTreeMap<TrieEntry<A, T, M>, Vec<String>>) -> Self {
        Self { keys }
    }

    pub fn get_merged(&self, entry: &TrieEntry<A, T, M>) -> Option<String>
    where
        A: ActionBehavior,
        T: TextObjectBehavior,
        M: MotionBehavior,
    {
        let keys = self.keys.get(entry)?;
        let mut iter = keys.iter();

        let first = iter.next()?.clone();
        Some(iter.fold(first, |acc, item| format!("{acc} / {item}")))
    }

    pub fn keys(&self) -> &BTreeMap<TrieEntry<A, T, M>, Vec<String>> {
        &self.keys
    }
}

impl<A, T, M> EventTrie<A, T, M>
where
    A: ActionBehavior + Hash,
    M: MotionBehavior + Hash,
    T: TextObjectBehavior + Hash,
{
    pub fn action_keys(&self) -> Keys<A, T, M> {
        // Initialize keys for all action variants
        let mut keys = BTreeMap::default();

        // TODO: add back all variants for motions/operators etc.
        // for action in Action::<A, T, M>::variants() {
        //     keys.entry(action).or_default();
        // }
        //

        // Perform a DFS to find all actions for which keys are defined
        dfs(&self.root, &mut keys, vec![]);

        Keys { keys }
    }
}

fn dfs<A, T, M>(
    node: &EventTrieNode<A, T, M>,
    result: &mut BTreeMap<TrieEntry<A, T, M>, Vec<String>>,
    current_events: Vec<AppEvent>,
) where
    A: ActionBehavior,
    M: MotionBehavior,
    T: TextObjectBehavior,
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
