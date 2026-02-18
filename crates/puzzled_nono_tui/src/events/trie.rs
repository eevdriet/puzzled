use std::{collections::HashMap, fmt::Debug};

use crate::{Action, AppEvent};

#[derive(Debug, Default, Clone)]
pub struct EventTrieNode {
    action: Option<Action>,
    children: HashMap<AppEvent, EventTrieNode>,
}

#[derive(Debug)]
pub enum EventSearchResult {
    /// Event search does not lead to an action
    None,

    /// Events require an operand to constitute a full action
    RequireOperand(Action),

    /// Events constitute a prefix for an action
    Prefix,

    /// Events trigger an action
    Exact(Action),

    /// Events trigger an action and and prefix
    ExactPrefix(Action),
}

impl EventSearchResult {
    pub fn action(&self) -> Option<Action> {
        match self {
            EventSearchResult::Exact(action)
            | EventSearchResult::ExactPrefix(action)
            | EventSearchResult::RequireOperand(action) => Some(*action),
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

#[derive(Debug, Default, Clone)]
pub struct EventTrie {
    root: EventTrieNode,
}

impl EventTrie {
    pub fn new() -> Self {
        Self {
            root: EventTrieNode::default(),
        }
    }

    pub fn insert(&mut self, events: &[AppEvent], action: Action) {
        let mut node = &mut self.root;

        for event in events {
            node = node.children.entry(event.clone()).or_default();
        }

        node.action = Some(action);
    }

    pub fn search(&self, events: &[AppEvent]) -> EventSearchResult {
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

        tracing::debug!(
            "\tAll events {events:?} matched, stopping at ({:?}, {:?} children)",
            node.action,
            node.children.len()
        );

        let has_children = !node.children.is_empty();

        match node.action {
            None => {
                if has_children {
                    EventSearchResult::Prefix
                } else {
                    EventSearchResult::None
                }
            }
            Some(action) => {
                if action.requires_operand() {
                    return EventSearchResult::RequireOperand(action);
                }

                if has_children {
                    EventSearchResult::ExactPrefix(action)
                } else {
                    EventSearchResult::Exact(action)
                }
            }
        }
    }
}
