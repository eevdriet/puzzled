use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Position;

use crate::{
    Action, AppCommand, AppEvent, AppTrieEntry, AppTypes, Command, EventMode, EventSearchResult,
    EventTrie, Motion, Operator, SelectionKind, TrieEntry,
};

#[derive(Debug)]
pub struct EventEngine<A: AppTypes> {
    mode: EventMode,

    buffer: Vec<AppEvent>,
    actions: EventTrie<A>,
    pending_operator: Option<Operator>,

    repeat: RepeatState,

    last_insert: Instant,
    is_dragging: bool,
    timeout: Duration,
}

#[derive(Debug)]
pub enum EventEffect<A: AppTypes> {
    Command(AppCommand<A>),
    Mode(EventMode),
}

#[derive(Debug)]
pub struct EventResult<A: AppTypes> {
    pub effects: Vec<EventEffect<A>>,
}

impl<A: AppTypes> EventResult<A> {
    pub fn next_mode(&self) -> Option<EventMode> {
        for effect in self.effects.iter().rev() {
            if let EventEffect::Mode(mode) = effect {
                return Some(*mode);
            }
        }

        None
    }
}

impl<A: AppTypes> Default for EventResult<A> {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

impl<A: AppTypes> EventEngine<A> {
    pub fn new(actions: EventTrie<A>, timeout: Duration) -> Self {
        Self {
            mode: EventMode::default(),
            timeout,
            actions,
            repeat: RepeatState::default(),
            pending_operator: None,
            buffer: Vec::new(),
            is_dragging: false,
            last_insert: Instant::now(),
        }
    }

    pub fn set_mode(&mut self, mode: EventMode) {
        self.mode = mode;
    }
}

impl<A: AppTypes> EventEngine<A> {
    pub fn push(&mut self, event: AppEvent, override_mode: Option<EventMode>) -> EventResult<A> {
        let mode = override_mode.unwrap_or(self.mode);

        tracing::debug!("[EVENT] {event}");

        if let Some(mouse) = event.mouse() {
            return self.mouse_event(mouse);
        }

        let result = match mode {
            EventMode::Normal => self.normal_event(event, override_mode),
            EventMode::Visual(_) => self.visual_event(event, override_mode),
            EventMode::Insert => self.insert_event(event),
            EventMode::Replace => self.replace_event(event),
        };

        if let Some(mode) = result.next_mode() {
            self.mode = mode;
        }

        result
    }

    pub fn tick(&mut self, override_mode: Option<EventMode>) -> EventResult<A> {
        let mode = override_mode.unwrap_or(self.mode);

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
                result = self.fallback_literal();

                if result.is_none() {
                    self.reset();
                }
            }

            result
        };

        EventResult {
            effects: match command {
                Some(command) => vec![EventEffect::Command(command)],
                _ => vec![],
            },
        }
    }

    fn mouse_event(&mut self, mouse: MouseEvent) -> EventResult<A> {
        let mut effects = Vec::new();

        match mouse.kind {
            // Start visual selection on mouse drag
            MouseEventKind::Drag(MouseButton::Left) if !self.is_dragging => {
                tracing::info!("[MOUSE] Start selection (left start drag)");
                self.is_dragging = true;

                let additive = mouse.modifiers.contains(KeyModifiers::CONTROL);
                let pos = Position {
                    x: mouse.column,
                    y: mouse.row,
                };
                let action = Action::StartSelection { pos, additive };

                effects.push(EventEffect::Mode(EventMode::Visual(SelectionKind::Cells)));
                effects.push(EventEffect::Command(Command::new_action(action)));
            }

            // Update visual selection on additional mouse drag
            MouseEventKind::Drag(MouseButton::Left) if self.is_dragging => {
                tracing::info!("[MOUSE] Update selection (left continue drag)");
                effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                    mouse,
                ))));
            }

            // Stop dragging when mouse released
            MouseEventKind::Up(MouseButton::Left) if self.is_dragging => {
                tracing::info!("[MOUSE] Finish selection (up end drag)");
                self.is_dragging = false;

                let action = Action::EndSelection;
                effects.push(EventEffect::Command(Command::new_action(action)));
            }

            // Return to normal mode on another mouse click
            MouseEventKind::Up(MouseButton::Left) if !self.is_dragging => {
                tracing::info!("[MOUSE] Back to normal (up no drag)");
                effects.push(EventEffect::Mode(EventMode::Normal));
                effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                    mouse,
                ))));
            }

            // Pass through other mouse events normally
            kind if !matches!(kind, MouseEventKind::Moved) => {
                tracing::info!("[MOUSE] Other: {mouse:?}");
                effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                    mouse,
                ))));
            }
            _ => {}
        }

        EventResult { effects }
    }

    fn search_command(&mut self, events: &[AppEvent], mode: &EventMode) -> Option<AppCommand<A>> {
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
            EventSearchResult::None => self.fallback_literal(),

            // Wait for additional input for prefix sequence
            EventSearchResult::Prefix => {
                tracing::debug!("\tFound prefix, waiting...");
                None
            }
        }
    }

    fn entry_command(&mut self, entry: AppTrieEntry<A>) -> Option<AppCommand<A>> {
        tracing::info!("Entry {entry:?}");
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

    fn key_event(&mut self, event: AppEvent, override_mode: Option<EventMode>) -> EventResult<A> {
        tracing::debug!("[EVENT] {event} (with buffer {:?})", self.buffer);
        self.last_insert = Instant::now();

        // Check for mode switching keys
        if override_mode.is_none()
            && let Event::Key(key) = *event
        {
            tracing::info!("Key event: {key:?}");

            let next_mode = match (self.mode, key.code, key.modifiers) {
                // -> Normal
                (mode, KeyCode::Esc, _) if !matches!(mode, EventMode::Normal) => {
                    Some(EventMode::Normal)
                }

                // -> Insert
                (
                    EventMode::Normal,
                    KeyCode::Char('i')
                    | KeyCode::Char('a')
                    | KeyCode::Char('I')
                    | KeyCode::Char('A'),
                    _,
                ) => Some(EventMode::Insert),

                // -> Visual
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::NONE) => {
                    Some(EventMode::Visual(SelectionKind::Cells))
                }
                (EventMode::Normal, KeyCode::Char('V'), KeyModifiers::SHIFT) => {
                    Some(EventMode::Visual(SelectionKind::Rows))
                }
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    Some(EventMode::Visual(SelectionKind::Cols))
                }
                _ => None,
            };

            if let Some(mode) = next_mode {
                return EventResult {
                    effects: vec![EventEffect::Mode(mode)],
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

        let curr_mode = self.mode;
        let mut effects = Vec::new();

        if let Some(command) = self.search_command(&self.buffer.to_vec(), &curr_mode) {
            let next_mode = self.handle_mode_switch(&command);
            effects.push(EventEffect::Command(command));

            if let Some(mode) = next_mode {
                effects.push(EventEffect::Mode(mode));
            }
        }

        EventResult { effects }
    }

    fn fallback_literal(&mut self) -> Option<AppCommand<A>> {
        // Try to find a prefix of the buffer that is a valid command
        for split in 1..=self.buffer.len() {
            let prefix = &self.buffer[..self.buffer.len() - split];

            match self.actions.search(prefix) {
                EventSearchResult::Exact(_)
                | EventSearchResult::ExactPrefix(_)
                | EventSearchResult::Prefix => {
                    // Return the first event as a literal action if it is a key
                    let first = self.buffer.remove(0);

                    return first.key().and_then(|key| {
                        if let KeyCode::Char(ch) = key.code {
                            Some(Command::Action {
                                count: 1,
                                action: Action::Literal(ch),
                            })
                        } else {
                            None
                        }
                    });
                }
                EventSearchResult::None => continue,
            }
        }

        // Still emit the first action if it is a key
        let first = self.buffer.remove(0);
        first.key().and_then(|key| {
            if let KeyCode::Char(ch) = key.code {
                Some(Command::Action {
                    count: 1,
                    action: Action::Literal(ch),
                })
            } else {
                None
            }
        })
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.repeat.clear();
        self.pending_operator = None;
    }

    fn normal_event(
        &mut self,
        event: AppEvent,
        override_mode: Option<EventMode>,
    ) -> EventResult<A> {
        self.key_event(event, override_mode)
    }

    fn visual_event(
        &mut self,
        event: AppEvent,
        override_mode: Option<EventMode>,
    ) -> EventResult<A> {
        self.key_event(event, override_mode)
    }

    fn insert_event(&mut self, event: AppEvent) -> EventResult<A> {
        let Some(key) = event.key() else {
            return EventResult::default();
        };

        let effect = match key.code {
            // Insert
            KeyCode::Char(char) => EventEffect::Command(Command::new_action(Action::Literal(char))),

            // Delete
            KeyCode::Backspace => EventEffect::Command(Command::new_action(Action::DeleteLeft)),
            KeyCode::Delete => EventEffect::Command(Command::new_action(Action::DeleteRight)),

            // Movements
            KeyCode::Down => EventEffect::Command(Command::new_motion(Motion::Down)),
            KeyCode::End => EventEffect::Command(Command::new_motion(Motion::RowEnd)),
            KeyCode::Home => EventEffect::Command(Command::new_motion(Motion::RowStart)),
            KeyCode::Left => EventEffect::Command(Command::new_motion(Motion::Left)),
            KeyCode::PageDown => EventEffect::Command(Command::new_motion(Motion::ColEnd)),
            KeyCode::PageUp => EventEffect::Command(Command::new_motion(Motion::ColStart)),
            KeyCode::Right => EventEffect::Command(Command::new_motion(Motion::Right)),
            KeyCode::Up => EventEffect::Command(Command::new_motion(Motion::Up)),

            // Modes
            KeyCode::Esc => EventEffect::Mode(EventMode::Normal),

            _ => return EventResult::default(),
        };

        EventResult {
            effects: vec![effect],
        }
    }

    fn replace_event(&mut self, event: AppEvent) -> EventResult<A> {
        let Some(key) = event.key() else {
            return EventResult::default();
        };

        let effect = match key.code {
            // Insert
            KeyCode::Char(char) => EventEffect::Command(Command::new_action(Action::Literal(char))),

            // Delete
            KeyCode::Delete => EventEffect::Command(Command::new_action(Action::DeleteRight)),

            // Movements
            KeyCode::Down => EventEffect::Command(Command::new_motion(Motion::Down)),
            KeyCode::End => EventEffect::Command(Command::new_motion(Motion::RowEnd)),
            KeyCode::Home => EventEffect::Command(Command::new_motion(Motion::RowStart)),
            KeyCode::Left | KeyCode::Backspace => {
                EventEffect::Command(Command::new_motion(Motion::Left))
            }
            KeyCode::PageDown => EventEffect::Command(Command::new_motion(Motion::ColEnd)),
            KeyCode::PageUp => EventEffect::Command(Command::new_motion(Motion::ColStart)),
            KeyCode::Right => EventEffect::Command(Command::new_motion(Motion::Right)),
            KeyCode::Up => EventEffect::Command(Command::new_motion(Motion::Up)),

            // Modes
            KeyCode::Esc => EventEffect::Mode(EventMode::Normal),
            KeyCode::Insert => EventEffect::Mode(EventMode::Insert),

            _ => return EventResult::default(),
        };

        EventResult {
            effects: vec![effect],
        }
    }

    fn handle_mode_switch(&mut self, command: &AppCommand<A>) -> Option<EventMode> {
        tracing::info!("\tHandling mode switch for result {command:?}");

        let mode = match command {
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
        };

        tracing::info!("\tNext mode: {mode:?}");
        mode
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
