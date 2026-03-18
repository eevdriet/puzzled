use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::layout::Position as AppPosition;

use crate::{
    Action, ActionBehavior, AppEvent, Command, EventMode, EventSearchResult, EventTrie, Motion,
    MotionBehavior, Operator, SelectionKind, TextObjectBehavior, TrieEntry,
};

#[derive(Debug)]
pub struct EventEngine<A, T, M> {
    buffer: Vec<AppEvent>,
    actions: EventTrie<A, T, M>,
    pending_operator: Option<Operator>,

    repeat: RepeatState,

    last_insert: Instant,
    timeout: Duration,
}

#[derive(Debug)]
pub struct EventResult<A, T, M> {
    pub command: Option<Command<A, T, M>>,
    pub next_mode: Option<EventMode>,
}

impl<A, T, M> Default for EventResult<A, T, M> {
    fn default() -> Self {
        Self {
            command: None,
            next_mode: None,
        }
    }
}

impl<A, T, M> EventEngine<A, T, M> {
    pub fn new(actions: EventTrie<A, T, M>, timeout: Duration) -> Self {
        Self {
            timeout,
            actions,
            repeat: RepeatState::default(),
            pending_operator: None,
            buffer: Vec::new(),
            last_insert: Instant::now(),
        }
    }
}

impl<A, T, M> EventEngine<A, T, M>
where
    A: ActionBehavior,
    M: MotionBehavior,
    T: TextObjectBehavior,
{
    pub fn push(&mut self, event: AppEvent, mode: &mut EventMode) -> EventResult<A, T, M> {
        tracing::info!("[EVENT] {event:?}");

        match mode {
            EventMode::Normal => self.normal_event(event, mode),
            EventMode::Visual(_) => self.visual_event(event, mode),
            EventMode::Insert => self.insert_event(event),
            EventMode::Replace => self.replace_event(event),
        }
    }

    fn mouse_event(&self, mouse: MouseEvent) -> EventResult<A, T, M> {
        let pos = AppPosition {
            x: mouse.column,
            y: mouse.row,
        };

        let command = match mouse.kind {
            MouseEventKind::Up(button) => {
                let action = Action::Click { pos, button };
                Some(Command::new_action(action))
            }
            MouseEventKind::Drag(button) => {
                let action = Action::Drag { pos, button };
                Some(Command::new_action(action))
            }
            MouseEventKind::ScrollDown => Some(Command::new_motion(Motion::Down)),
            MouseEventKind::ScrollLeft => Some(Command::new_motion(Motion::Left)),
            MouseEventKind::ScrollUp => Some(Command::new_motion(Motion::Up)),
            MouseEventKind::ScrollRight => Some(Command::new_motion(Motion::Right)),
            _ => None,
        };

        EventResult {
            command,
            next_mode: None,
        }
    }

    fn search_command(
        &mut self,
        events: &[AppEvent],
        mode: &EventMode,
    ) -> Option<Command<A, T, M>> {
        match self.actions.search(events) {
            // Perform action for known sequence
            EventSearchResult::Exact(entry) => self.entry_command(entry),

            EventSearchResult::ExactPrefix(entry) => match entry {
                TrieEntry::Operator(op) => {
                    if mode.is_visual() || !op.requires_motion() {
                        self.entry_command(entry)
                    } else {
                        tracing::info!("Setting pending operator: {op:?}");
                        self.pending_operator = Some(op);
                        None
                    }
                }
                _ => None,
            },

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

    fn entry_command(&mut self, entry: TrieEntry<A, T, M>) -> Option<Command<A, T, M>> {
        tracing::info!("Entry {entry:?} with {self:?}");
        let count = self.repeat.count().unwrap_or(1);
        self.reset();

        match entry {
            TrieEntry::Action(action) => Some(Command::Action { count, action }),
            TrieEntry::TextObject(obj) => {
                let op = self.pending_operator.take()?;
                Some(Command::TextObj { count, obj, op })
            }
            TrieEntry::Motion(motion) => {
                let op = self.pending_operator.take();
                Some(Command::Motion { count, motion, op })
            }
            TrieEntry::Operator(op) => Some(Command::Operator(op)),
        }
    }

    fn key_event(&mut self, event: AppEvent, mode: &EventMode) -> EventResult<A, T, M> {
        tracing::debug!("[EVENT] {event:?} (with buffer {:?})", self.buffer);
        self.last_insert = Instant::now();

        // Check for mode switching keys
        if let Event::Key(key) = *event {
            let next_mode = match (mode, key.code, key.modifiers) {
                // -> Normal
                (mode, KeyCode::Esc, _) if !matches!(mode, EventMode::Normal) => {
                    Some(EventMode::Normal)
                }

                // -> Insert
                (
                    EventMode::Normal,
                    KeyCode::Char('i') | KeyCode::Char('a'),
                    KeyModifiers::NONE | KeyModifiers::SHIFT,
                ) => Some(EventMode::Insert),

                // -> Visual
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::NONE) => {
                    Some(EventMode::Visual(SelectionKind::Cells))
                }
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::SHIFT) => {
                    Some(EventMode::Visual(SelectionKind::Rows))
                }
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    Some(EventMode::Visual(SelectionKind::Cols))
                }
                _ => None,
            };

            if next_mode.is_some() {
                return EventResult {
                    command: None,
                    next_mode,
                };
            }
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

            return EventResult::default();
        }

        tracing::debug!("\tPush {event:?}");
        self.buffer.push(event.clone());

        let command = self.search_command(&self.buffer.to_vec(), mode);
        let next_mode = self.handle_mode_switch(&command);

        EventResult { command, next_mode }
    }

    pub fn tick(&mut self, mode: &EventMode) -> EventResult<A, T, M> {
        if self.buffer.is_empty() {
            return EventResult::default();
        }

        // Before time out, wait for additional events to handle
        if self.last_insert.elapsed() < self.timeout {
            return EventResult::default();
        }

        // After time out, perform the action w.r.t. longest valid action
        let command = {
            let mut result = None;

            if matches!(mode, EventMode::Normal) {
                for idx in (1..=self.buffer.len()).rev() {
                    let events = self.buffer[..idx].to_vec();

                    if let Some(command) = self.search_command(&events, mode) {
                        result = Some(command);
                    }
                }

                if result.is_none() {
                    self.reset();
                }
            }

            result
        };

        EventResult {
            command,
            next_mode: None,
        }
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.repeat.clear();
        self.pending_operator = None;
    }

    fn normal_event(&mut self, event: AppEvent, mode: &EventMode) -> EventResult<A, T, M> {
        if let Some(mouse) = event.mouse() {
            self.mouse_event(mouse)
        } else if event.key().is_some() {
            self.key_event(event, mode)
        } else {
            EventResult::default()
        }
    }

    fn visual_event(&mut self, event: AppEvent, mode: &EventMode) -> EventResult<A, T, M> {
        if let Some(mouse) = event.mouse() {
            self.mouse_event(mouse)
        } else if event.key().is_some() {
            self.key_event(event, mode)
        } else {
            EventResult::default()
        }
    }

    fn insert_event(&mut self, event: AppEvent) -> EventResult<A, T, M> {
        let Some(key) = event.key() else {
            return EventResult::default();
        };

        let mut next_mode = None;

        let command = match key.code {
            // Insert
            KeyCode::Char(char) => Some(Command::new_action(Action::Insert(char))),

            // Delete
            KeyCode::Backspace => Some(Command::new_action(Action::DeleteLeft)),
            KeyCode::Delete => Some(Command::new_action(Action::DeleteRight)),

            // Movements
            KeyCode::Down => Some(Command::new_motion(Motion::Down)),
            KeyCode::End => Some(Command::new_motion(Motion::RowEnd)),
            KeyCode::Home => Some(Command::new_motion(Motion::RowStart)),
            KeyCode::Left => Some(Command::new_motion(Motion::Left)),
            KeyCode::PageDown => Some(Command::new_motion(Motion::ColEnd)),
            KeyCode::PageUp => Some(Command::new_motion(Motion::ColStart)),
            KeyCode::Right => Some(Command::new_motion(Motion::Right)),
            KeyCode::Up => Some(Command::new_motion(Motion::Up)),

            // Modes
            KeyCode::Esc => {
                next_mode = Some(EventMode::Normal);
                None
            }

            _ => None,
        };

        EventResult { command, next_mode }
    }

    fn replace_event(&mut self, event: AppEvent) -> EventResult<A, T, M> {
        let Some(key) = event.key() else {
            return EventResult::default();
        };

        let mut next_mode = None;

        let command = match key.code {
            // Insert
            KeyCode::Char(char) => Some(Command::new_action(Action::Insert(char))),

            // Delete
            KeyCode::Delete => Some(Command::new_action(Action::DeleteRight)),

            // Movements
            KeyCode::Down => Some(Command::new_motion(Motion::Down)),
            KeyCode::End => Some(Command::new_motion(Motion::RowEnd)),
            KeyCode::Home => Some(Command::new_motion(Motion::RowStart)),
            KeyCode::Left | KeyCode::Backspace => Some(Command::new_motion(Motion::Left)),
            KeyCode::PageDown => Some(Command::new_motion(Motion::ColEnd)),
            KeyCode::PageUp => Some(Command::new_motion(Motion::ColStart)),
            KeyCode::Right => Some(Command::new_motion(Motion::Right)),
            KeyCode::Up => Some(Command::new_motion(Motion::Up)),

            // Modes
            KeyCode::Esc => {
                next_mode = Some(EventMode::Normal);
                None
            }
            KeyCode::Insert => {
                next_mode = Some(EventMode::Insert);
                None
            }

            _ => None,
        };

        EventResult { command, next_mode }
    }

    fn handle_mode_switch(&mut self, command: &Option<Command<A, T, M>>) -> Option<EventMode> {
        tracing::info!("\tHandling mode switch for result {command:?}");

        let Some(command) = command else {
            tracing::info!("\tNo command");
            return None;
        };

        match command {
            // Switch to the next mode explicitly
            // Switch implicitly based on e.g. the operator
            Command::Operator(op)
            | Command::Motion { op: Some(op), .. }
            | Command::TextObj { op, .. } => {
                tracing::info!("\tNext mode from operator {op:?}");
                let next_mode = match op {
                    Operator::Change | Operator::ChangeSingle => EventMode::Insert,
                    _ => EventMode::Normal,
                };

                Some(next_mode)
            }

            _ => None,
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
