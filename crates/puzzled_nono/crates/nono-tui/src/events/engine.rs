use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode};

use crate::{Action, EventSearchResult, EventTrie, events::AppEvent};

#[derive(Debug, Clone)]
pub struct ActionInput {
    pub action: Action,
    pub event: AppEvent,
    pub repeat: Option<u16>,
}

impl ActionInput {
    pub fn with_action(&self, action: Action) -> Self {
        Self {
            action,
            event: self.event.clone(),
            repeat: self.repeat,
        }
    }
    pub fn with_event(&self, event: AppEvent) -> Self {
        Self {
            event,
            repeat: self.repeat,
            action: self.action,
        }
    }
}

#[derive(Debug)]
pub struct EventEngine {
    buffer: Vec<AppEvent>,
    actions: EventTrie,
    pending_operand: Option<Action>,

    repeat: RepeatState,

    last_insert: Instant,
    timeout: Duration,
}

impl EventEngine {
    pub fn new(actions: EventTrie, timeout: Duration) -> Self {
        Self {
            timeout,
            actions,
            repeat: RepeatState::default(),
            pending_operand: None,
            buffer: Vec::new(),
            last_insert: Instant::now(),
        }
    }

    pub fn push(&mut self, event: AppEvent) -> Option<ActionInput> {
        tracing::debug!("[EVENT] {event:?} (with buffer {:?})", self.buffer);
        self.last_insert = Instant::now();

        // If we are waiting for an operand, consume this event directly
        if let Some(action) = self.pending_operand.take() {
            let count = self.repeat.count();
            let input = ActionInput {
                action,
                event: event.clone(), // operand lives here
                repeat: count,
            };

            tracing::debug!("\tFound pending action {action:?}, sending with {event:?} as operand");
            self.reset(); // clear buffer + repeat
            return Some(input);
        }

        // Intercept leading digits (note: 0 is not a command if following another digit)
        if let Event::Key(key) = *event
            && let KeyCode::Char(ch) = key.code
            && ch.is_ascii_digit()
            && self.buffer.is_empty()
            && (ch != '0' || !self.repeat.is_empty())
        {
            tracing::debug!("\tLeading digit {ch} found, ignoring event");
            let digit = ch as u8;
            self.repeat.push_digit(digit);

            return None;
        }

        tracing::debug!("\tPush {event:?}");
        self.buffer.push(event.clone());

        let result = self.actions.search(&self.buffer);
        tracing::debug!("\tSearch with events {:?} -> {result:?}", &self.buffer);

        match result {
            // Perform action for known sequence
            EventSearchResult::Exact(action) | EventSearchResult::ExactPrefix(action) => {
                let count = self.repeat.count();
                self.reset();

                tracing::debug!("\tFound action {action:?} with repeat {count:?}");

                Some(ActionInput {
                    event,
                    action,
                    repeat: count,
                })
            }

            // Clear previous keys for unknown sequence but keep repeat
            EventSearchResult::None => {
                tracing::debug!("\tFound no action, clearing buffer");

                self.buffer.clear();
                None
            }

            // Wait for additional input for prefix sequence
            EventSearchResult::Prefix => {
                tracing::debug!("\tFound prefix, waiting...");
                None
            }

            // Wait for additional input for prefix sequence
            EventSearchResult::RequireOperand(action) => {
                tracing::debug!("\tFound action {action:?} that requires operand, waiting...");
                self.pending_operand = Some(action);
                None
            }
        }
    }

    pub fn tick(&mut self) -> Option<ActionInput> {
        if self.buffer.is_empty() {
            return None;
        }

        // Before time out, wait for additional events to handle
        if self.last_insert.elapsed() < self.timeout {
            return None;
        }

        // After time out, perform the action w.r.t. longest valid action
        let mut result = None;

        for idx in (1..=self.buffer.len()).rev() {
            let events = &self.buffer[..idx];
            let search = self.actions.search(events);

            // Do not reset on partial results that require more input
            if search.is_partial() {
                break;
            }

            if let Some(action) = search.action() {
                let count = self.repeat.count();
                let events = events.to_vec();

                result = Some(ActionInput {
                    event: events.last().unwrap().clone(),
                    action,
                    repeat: count,
                });
                break;
            }
        }

        self.reset();
        result
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.repeat.clear();
    }
}

#[derive(Debug, Default)]
struct RepeatState {
    digits: Vec<u8>,
}

impl RepeatState {
    fn push_digit(&mut self, digit: u8) {
        self.digits.push(digit);
    }
    fn value(&self) -> u16 {
        self.digits.iter().fold(0, |acc, d| {
            acc.saturating_mul(10).saturating_add((d - b'0') as u16)
        })
    }
    fn count(&self) -> Option<u16> {
        (!self.is_empty()).then(|| self.value())
    }

    fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    fn clear(&mut self) {
        self.digits.clear();
    }
}
