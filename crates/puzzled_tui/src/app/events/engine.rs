use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode};
use derive_more::Debug;

use crate::{
    Action, ActionBehavior, AppEvent, Command, EventMode, EventSearchResult, EventTrie, Motion,
    Operator, TrieEntry,
};

#[derive(Debug)]
pub enum ActionOrEvent<A> {
    Action(Action<A>),
    Event(AppEvent),
}

#[derive(Debug)]
pub struct EventEngine<A> {
    buffer: Vec<AppEvent>,
    actions: EventTrie<A>,
    pending_operator: Option<Operator>,

    repeat: RepeatState,

    last_insert: Instant,
    timeout: Duration,
    mode: EventMode,
}

impl<A> EventEngine<A> {
    pub fn new(mut actions: EventTrie<A>, timeout: Duration) -> Self {
        actions.insert_mode_switches();

        Self {
            timeout,
            actions,
            repeat: RepeatState::default(),
            pending_operator: None,
            buffer: Vec::new(),
            last_insert: Instant::now(),
            mode: EventMode::Normal,
        }
    }
}

impl<A> EventEngine<A>
where
    A: Clone + ActionBehavior,
{
    pub fn push(&mut self, event: AppEvent) -> Option<Command<A>> {
        let result = match self.mode {
            EventMode::Normal => self.normal_event(event),
            EventMode::Insert => self.insert_event(event),
            EventMode::Replace => self.replace_event(event),
        };

        self.handle_mode_switch(&result);
        result
    }

    fn search_command(&mut self, events: &[AppEvent]) -> Option<Command<A>> {
        match self.actions.search(events) {
            // Perform action for known sequence
            EventSearchResult::Exact(entry) | EventSearchResult::ExactPrefix(entry) => {
                let count = self.repeat.count().unwrap_or(1);
                // let events: Vec<_> = self.buffer.drain(..).collect();
                self.reset();

                match entry {
                    TrieEntry::Motion(motion) => {
                        let operator = self.pending_operator.take();

                        Some(Command::new(count, operator, Some(motion), None))
                    }
                    TrieEntry::Operator(op) => {
                        self.pending_operator = Some(op);
                        None
                    }
                    TrieEntry::Action(action) => {
                        Some(Command::new(count, None, None, Some(action)))
                    }
                }
            }

            // Wait for additional input for prefix sequence
            EventSearchResult::RequireOperand(operator) => {
                // tracing::debug!("\tFound action {action:?} that requires operand, waiting...");
                self.pending_operator = Some(operator);
                None
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
        }
    }

    fn push_action(&mut self, event: AppEvent) -> Option<Command<A>> {
        tracing::debug!("[EVENT] {event:?} (with buffer {:?})", self.buffer);
        self.last_insert = Instant::now();

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

        self.search_command(&self.buffer.to_vec())
    }

    pub fn tick(&mut self) -> Option<Command<A>> {
        if self.buffer.is_empty() {
            return None;
        }

        // Before time out, wait for additional events to handle
        if self.last_insert.elapsed() < self.timeout {
            return None;
        }

        // After time out, perform the action w.r.t. longest valid action
        match self.mode {
            EventMode::Normal => {
                for idx in (1..=self.buffer.len()).rev() {
                    let events = self.buffer[..idx].to_vec();

                    if let Some(command) = self.search_command(&events) {
                        return Some(command);
                    }
                }

                self.reset();
            }
            _ => {} // _ => self.buffer.pop().map(ActionOrEvent::Event),
        }

        None
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.repeat.clear();
    }

    fn normal_event(&mut self, event: AppEvent) -> Option<Command<A>> {
        let action = self.push_action(event)?;

        Some(action)
    }

    fn insert_event(&mut self, event: AppEvent) -> Option<Command<A>> {
        let Some(key) = event.key() else {
            return None;
        };

        let command = match key.code {
            // Insert
            KeyCode::Char(char) => Command::new_action(Action::Insert(char)),

            // Delete
            KeyCode::Backspace => Command::new_action(Action::DeleteLeft),
            KeyCode::Delete => Command::new_action(Action::DeleteRight),

            // Movements
            KeyCode::Down => Command::new_motion(Motion::Down),
            KeyCode::End => Command::new_motion(Motion::RowEnd),
            KeyCode::Home => Command::new_motion(Motion::RowStart),
            KeyCode::Left => Command::new_motion(Motion::Left),
            KeyCode::PageDown => Command::new_motion(Motion::ColEnd),
            KeyCode::PageUp => Command::new_motion(Motion::ColStart),
            KeyCode::Right => Command::new_motion(Motion::Right),
            KeyCode::Up => Command::new_motion(Motion::Up),

            // Modes
            KeyCode::Esc => Command::new_action(Action::NextMode(EventMode::Normal)),

            _ => return None,
        };

        Some(command)
    }

    fn replace_event(&mut self, event: AppEvent) -> Option<Command<A>> {
        let Some(key) = event.key() else {
            return None;
        };

        let command = match key.code {
            // Insert
            KeyCode::Char(char) => Command::new_action(Action::Replace(char)),

            // Delete
            KeyCode::Delete => Command::new_action(Action::DeleteRight),

            // Movements
            KeyCode::Down => Command::new_motion(Motion::Down),
            KeyCode::End => Command::new_motion(Motion::RowEnd),
            KeyCode::Home => Command::new_motion(Motion::RowStart),
            KeyCode::Left | KeyCode::Backspace => Command::new_motion(Motion::Left),
            KeyCode::PageDown => Command::new_motion(Motion::ColEnd),
            KeyCode::PageUp => Command::new_motion(Motion::ColStart),
            KeyCode::Right => Command::new_motion(Motion::Right),
            KeyCode::Up => Command::new_motion(Motion::Up),

            // Modes
            KeyCode::Esc => Command::new_action(Action::NextMode(EventMode::Normal)),

            _ => return None,
        };

        Some(command)
    }

    fn handle_mode_switch(&mut self, result: &Option<Command<A>>) {
        let Some(command) = result else {
            return;
        };

        if let Some(Action::NextMode(mode)) = command.action() {
            self.mode = *mode;
        }
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
    fn value(&self) -> usize {
        self.digits.iter().fold(0, |acc, d| {
            acc.saturating_mul(10).saturating_add((d - b'0') as usize)
        })
    }
    fn count(&self) -> Option<usize> {
        (!self.is_empty()).then(|| self.value())
    }

    fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    fn clear(&mut self) {
        self.digits.clear();
    }
}
