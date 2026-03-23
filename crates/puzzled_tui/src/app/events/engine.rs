use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Position;

use crate::{
    Action, AppCommand, AppEvent, AppTrieEntry, AppTypes, Command, EventMode, EventSearchResult,
    EventTrie, Motion, Operator, SelectionKind, TextModifier, TrieEntry,
};

#[derive(Debug)]
pub struct EventEngine<A: AppTypes> {
    mode: EventMode,

    buffer: Vec<AppEvent>,
    actions: EventTrie<A>,
    pending_operator: Option<Operator>,
    pending_modifier: Option<TextModifier>,

    repeat: RepeatState,

    last_insert: Instant,
    is_selecting: bool,
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
    pub fn new(mut events: EventTrie<A>, timeout: Duration) -> Self {
        // Add text modifier events
        events.insert(
            &[AppEvent::new_key(KeyCode::Char('i'), KeyModifiers::empty())],
            TrieEntry::TextModifier(TextModifier::Inside),
        );
        events.insert(
            &[AppEvent::new_key(KeyCode::Char('a'), KeyModifiers::empty())],
            TrieEntry::TextModifier(TextModifier::Around),
        );

        Self {
            mode: EventMode::default(),
            timeout,
            actions: events,
            repeat: RepeatState::default(),
            pending_operator: None,
            pending_modifier: None,
            buffer: Vec::new(),
            is_selecting: false,
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
        use MouseButton::*;
        use MouseEventKind::*;

        let mut effects = Vec::new();
        let has_ctrl = mouse.modifiers.contains(KeyModifiers::CONTROL);

        match (
            mouse.kind,
            has_ctrl,
            self.is_selecting,
            self.mode.is_visual(),
        ) {
            // Finish active visual selection before starting a new one
            (Drag(Left), _, false, is_visual) | (Down(Left), true, false, is_visual) => {
                if is_visual {
                    tracing::debug!("[MOUSE] Finish old selection (e.g. manual visual)");
                    self.is_selecting = false;

                    let action = Action::EndSelection;
                    effects.push(EventEffect::Command(Command::new_action(action)));
                }

                // Set visual kind
                effects.push(EventEffect::Mode(EventMode::Visual(SelectionKind::Cells)));

                tracing::debug!("[MOUSE] Start selection (left start drag)");
                // Start selection
                self.is_selecting = true;

                let pos = Position {
                    x: mouse.column,
                    y: mouse.row,
                };
                let action = Action::StartSelection {
                    pos: Some(pos),
                    kind: SelectionKind::Cells,
                    additive: has_ctrl,
                };
                effects.push(EventEffect::Command(Command::new_action(action)));

                // Move to the selected position
                effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                    mouse,
                ))));
            }

            // Start visual selection on mouse drag
            // (Drag(Left), _, false, _) | (Down(Left), true, false, _) => {}

            // Update visual selection on additional mouse drag
            (Drag(Left), _, true, _) => {
                tracing::debug!("[MOUSE] Update selection (left continue drag)");
                effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                    mouse,
                ))));
            }

            (Up(Left), _, is_selecting, _) => {
                tracing::debug!("[MOUSE] Finish selection (up end drag)");
                self.is_selecting = false;

                // Stop dragging when mouse released
                if is_selecting {
                    let action = Action::EndSelection;
                    effects.push(EventEffect::Command(Command::new_action(action)));
                }
                // Return to normal mode on another mouse click
                else {
                    tracing::debug!("[MOUSE] Back to normal (up no drag)");
                    effects.push(EventEffect::Mode(EventMode::Normal));
                    effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                        mouse,
                    ))));
                }
            }

            // Pass through other mouse events normally
            (kind, _, _, _) => {
                if !matches!(kind, MouseEventKind::Moved) {
                    tracing::debug!("[MOUSE] Other: {mouse:?}");
                    effects.push(EventEffect::Command(Command::new_motion(Motion::Mouse(
                        mouse,
                    ))));
                }
            }
        }

        EventResult { effects }
    }

    fn search_commands(
        &mut self,
        events: &[AppEvent],
        mode: &EventMode,
    ) -> Vec<Option<AppCommand<A>>> {
        tracing::trace!("Searching with {events:?}...");

        let mut commands = Vec::new();

        match self.actions.search(events) {
            // Perform action for known sequence
            EventSearchResult::Exact(TrieEntry::TextModifier(modifier)) => {
                tracing::trace!("\tSetting pending modifier: {modifier:?}");
                self.pending_modifier = Some(modifier);
            }

            EventSearchResult::Exact(entry @ TrieEntry::Operator(op))
            | EventSearchResult::ExactPrefix(entry @ TrieEntry::Operator(op)) => {
                if mode.is_visual() || !op.requires_motion() {
                    if mode.is_visual() {
                        tracing::trace!("\tEnding existing selection");
                        let action = Action::EndSelection;
                        commands.push(Some(Command::new_action(action)));
                    }

                    tracing::trace!("\tApplying {op:?} to {entry:?}");
                    commands.push(self.entry_command(entry));
                } else {
                    tracing::trace!("\tSetting pending operator: {op:?}");
                    self.pending_operator = Some(op);
                }
            }

            EventSearchResult::Exact(entry) => {
                tracing::trace!("\tExact");
                commands.push(self.entry_command(entry));
            }

            // Clear previous keys for unknown sequence but keep repeat
            EventSearchResult::None => {
                commands.push(self.fallback_literal());
            }

            // Wait for additional input for prefix sequence
            EventSearchResult::Prefix => {
                tracing::trace!("\tFound prefix, waiting...");
            }

            _ => {}
        }

        commands
    }

    fn entry_command(&mut self, entry: AppTrieEntry<A>) -> Option<AppCommand<A>> {
        let count = self.repeat.count().unwrap_or(1);

        let command = match entry {
            TrieEntry::Action(action) => Some(Command::Action { count, action }),
            TrieEntry::TextObject(obj) => {
                let op = self.pending_operator.take()?;
                let modifier = self.pending_modifier.take()?;

                Some(Command::TextObj {
                    count,
                    obj,
                    op,
                    modifier,
                })
            }
            TrieEntry::Motion(motion) => {
                let op = self.pending_operator.take();
                Some(Command::Motion { count, motion, op })
            }
            TrieEntry::Operator(op) => Some(Command::Operator(op)),

            TrieEntry::TextModifier(_) => None,
        };

        self.reset();
        command
    }

    fn key_event(&mut self, event: AppEvent, override_mode: Option<EventMode>) -> EventResult<A> {
        tracing::trace!("[EVENT] {event} (with buffer {:?})", self.buffer);
        self.last_insert = Instant::now();

        let mut effects = Vec::new();

        // Check for mode switching keys
        if override_mode.is_none()
            && let Event::Key(key) = *event
        {
            tracing::trace!("Key event: {key:?} (while override {override_mode:?})");

            let mut add_visual_effects = |kind: SelectionKind| {
                effects.push(EventEffect::Mode(EventMode::Visual(kind)));

                let action = Action::StartSelection {
                    pos: None,
                    additive: false,
                    kind,
                };
                effects.push(EventEffect::Command(Command::new_action(action)));
            };

            match (self.mode, key.code, key.modifiers) {
                // -> Normal
                (mode, KeyCode::Esc, _) if !matches!(mode, EventMode::Normal) => {
                    effects.push(EventEffect::Mode(EventMode::Normal));
                }

                // -> Insert
                (
                    EventMode::Normal,
                    KeyCode::Char('i')
                    | KeyCode::Char('a')
                    | KeyCode::Char('I')
                    | KeyCode::Char('A'),
                    _,
                ) => {
                    effects.push(EventEffect::Mode(EventMode::Insert));
                }

                // -> Visual
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::NONE) => {
                    add_visual_effects(SelectionKind::Cells);
                }
                (EventMode::Normal, KeyCode::Char('V'), KeyModifiers::SHIFT) => {
                    add_visual_effects(SelectionKind::Rows);
                }
                (EventMode::Normal, KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    add_visual_effects(SelectionKind::Cols);
                }
                _ => {}
            };

            if !effects.is_empty() {
                return EventResult { effects };
            }
        }
        // Intercept leading digits (note: 0 is not a command if following another digit)
        if let Event::Key(key) = *event
            && let KeyCode::Char(ch) = key.code
            && ch.is_ascii_digit()
            && self.buffer.is_empty()
            && (ch != '0' || !self.repeat.is_empty())
        {
            tracing::trace!("\tLeading digit {ch} found, ignoring event");
            let digit = ch as u8;
            self.repeat.push_digit(digit);

            return EventResult { effects };
        }

        tracing::trace!("\tPush {event:?}");
        self.buffer.push(event.clone());

        for mut command in self
            .search_commands(&self.buffer.to_vec(), &self.mode.clone())
            .into_iter()
            .flatten()
        {
            let mut next_mode = None;

            // Ignore found command and interpret it as literal for override mode
            if command.is_mode_changing()
                && override_mode.is_some()
                && let Event::Key(key) = *event
            {
                command = Command::new_action(Action::Literal(key.code));
            }
            // Otherwise stick with the command and determine whether to switch modes
            else {
                next_mode = self.handle_mode_switch(&command);
            }

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

                    return first
                        .key()
                        .map(|key| Command::new_action(Action::Literal(key.code)));
                }
                EventSearchResult::None => continue,
            }
        }

        // Still emit the first action if it is a key
        self.buffer
            .remove(0)
            .key()
            .map(|key| Command::new_action(Action::Literal(key.code)))
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

            // Literal
            code => EventEffect::Command(Command::new_action(Action::Literal(code))),
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

            // Literal
            code => EventEffect::Command(Command::new_action(Action::Literal(code))),
        };

        EventResult {
            effects: vec![effect],
        }
    }

    fn handle_mode_switch(&mut self, command: &AppCommand<A>) -> Option<EventMode> {
        tracing::trace!("\tHandling mode switch for result {command:?}");

        let mode = match command {
            // Switch to the next mode explicitly
            // Switch implicitly based on e.g. the operator
            Command::Operator(op)
            | Command::Motion { op: Some(op), .. }
            | Command::TextObj { op, .. } => {
                tracing::trace!("\tNext mode from operator {op:?}");
                let next_mode = match op {
                    Operator::Change | Operator::ChangeSingle => EventMode::Insert,
                    _ => EventMode::Normal,
                };

                Some(next_mode)
            }

            _ => None,
        };

        tracing::trace!("\tNext mode: {mode:?}");
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
