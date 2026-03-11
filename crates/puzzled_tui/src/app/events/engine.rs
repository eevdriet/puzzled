use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, MouseEvent, MouseEventKind};
use derive_more::Debug;
use ratatui::layout::Position as AppPosition;

use crate::{
    Action, ActionBehavior, AppEvent, Command, EventMode, EventSearchResult, EventTrie, Motion,
    Operator, TrieEntry,
};

#[derive(Debug)]
pub struct EventEngine<M, A> {
    buffer: Vec<AppEvent>,
    actions: EventTrie<M, A>,
    pending_operator: Option<Operator>,

    repeat: RepeatState,

    last_insert: Instant,
    timeout: Duration,
    mode: EventMode,
}

impl<M, A> EventEngine<M, A> {
    pub fn new(mut actions: EventTrie<M, A>, timeout: Duration) -> Self {
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

impl<M, A> EventEngine<M, A>
where
    A: Clone + ActionBehavior,
    M: Clone,
{
    pub fn push(&mut self, event: AppEvent, mode: &mut EventMode) -> Option<Command<M, A>> {
        let result = match mode {
            EventMode::Normal => self.normal_event(event),
            EventMode::Insert => self.insert_event(event),
            EventMode::Replace => self.replace_event(event),
        };

        self.handle_mode_switch(&result, mode);
        result
    }

    fn normal_event(&mut self, event: AppEvent) -> Option<Command<M, A>> {
        if let Some(mouse) = event.mouse() {
            self.mouse_event(mouse)
        } else if event.key().is_some() {
            self.key_event(event)
        } else {
            None
        }
    }

    fn mouse_event(&self, mouse: MouseEvent) -> Option<Command<M, A>> {
        let pos = AppPosition {
            x: mouse.column,
            y: mouse.row,
        };

        let command = match mouse.kind {
            MouseEventKind::Up(button) => {
                let action = Action::Click { pos, button };
                Command::new_action(action)
            }
            MouseEventKind::Drag(button) => {
                let action = Action::Drag { pos, button };
                Command::new_action(action)
            }
            MouseEventKind::ScrollDown => Command::new_motion(Motion::Down),
            MouseEventKind::ScrollLeft => Command::new_motion(Motion::Left),
            MouseEventKind::ScrollUp => Command::new_motion(Motion::Up),
            MouseEventKind::ScrollRight => Command::new_motion(Motion::Right),
            _ => return None,
        };

        Some(command)
    }

    fn search_command(&mut self, events: &[AppEvent]) -> Option<Command<M, A>> {
        match self.actions.search(events) {
            // Perform action for known sequence
            EventSearchResult::Exact(entry) | EventSearchResult::ExactPrefix(entry) => {
                let count = self.repeat.count().unwrap_or(1);
                // let events: Vec<_> = self.buffer.drain(..).collect();
                self.reset();

                match entry {
                    TrieEntry::Motion(motion) => {
                        let operator = self.pending_operator.take();

                        Some(Command::new(count, motion, operator, None))
                    }
                    TrieEntry::Operator(op) => {
                        if op.requires_motion() {
                            self.pending_operator = Some(op);
                            None
                        } else {
                            Some(Command::new(count, Motion::None, Some(op), None))
                        }
                    }
                    TrieEntry::Action(action) => {
                        Some(Command::new(count, Motion::None, None, Some(action)))
                    }
                }
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

    fn key_event(&mut self, event: AppEvent) -> Option<Command<M, A>> {
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

    pub fn tick(&mut self) -> Option<Command<M, A>> {
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

    fn insert_event(&mut self, event: AppEvent) -> Option<Command<M, A>> {
        let key = event.key()?;

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

    fn replace_event(&mut self, event: AppEvent) -> Option<Command<M, A>> {
        let key = event.key()?;

        let command = match key.code {
            // Insert
            KeyCode::Char(char) => Command::new_action(Action::Insert(char)),

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

    fn handle_mode_switch(&mut self, result: &Option<Command<M, A>>, mode: &mut EventMode) {
        let Some(command) = result else {
            return;
        };

        if let Some(Action::NextMode(next_mode)) = command.action() {
            *mode = *next_mode;
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
