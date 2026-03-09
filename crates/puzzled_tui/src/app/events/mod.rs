mod engine;
mod handle;
mod mode;
mod trie;

use std::{fmt, hash::Hash, ops::Deref};

pub use engine::*;
pub use handle::*;
pub use mode::*;
pub use trie::*;

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};

use crate::Action;

#[derive(Debug, Clone)]
pub struct AppEvent(Event);

impl AppEvent {
    pub fn new(event: Event) -> Self {
        Self(event)
    }

    pub fn new_key(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self(Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }))
    }

    pub fn new_mouse(kind: MouseEventKind, modifiers: KeyModifiers) -> Self {
        Self(Event::Mouse(MouseEvent {
            kind,
            modifiers,
            column: 0,
            row: 0,
        }))
    }

    pub fn key(&self) -> Option<KeyEvent> {
        match self.0 {
            Event::Key(key) => Some(key),
            _ => None,
        }
    }

    pub fn mouse(&self) -> Option<MouseEvent> {
        match self.0 {
            Event::Mouse(mouse) => Some(mouse),
            _ => None,
        }
    }
}

impl Deref for AppEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for AppEvent {
    fn eq(&self, other: &Self) -> bool {
        use MouseEventKind::*;

        match (self.deref(), other.deref()) {
            (Event::Key(key1), Event::Key(key2)) => {
                if key1.modifiers != key2.modifiers {
                    return false;
                }

                match (key1.code, key2.code) {
                    (KeyCode::Char(ch1), KeyCode::Char(ch2)) => ch1.eq_ignore_ascii_case(&ch2),
                    (code1, code2) => code1 == code2,
                }
            }

            (Event::Mouse(mouse1), Event::Mouse(mouse2)) => {
                // Only compare mouse buttons, not their event (is dynamically done when actions fired)
                let buttons_equal = match (mouse1.kind, mouse2.kind) {
                    (
                        Down(button1) | Up(button1) | Drag(button1),
                        Down(button2) | Up(button2) | Drag(button2),
                    ) => button1 == button2,
                    _ => true,
                };

                buttons_equal && mouse1.modifiers == mouse2.modifiers
            }
            (event1, event2) => event1 == event2,
        }
    }
}

impl Eq for AppEvent {}

impl Hash for AppEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.deref() {
            Event::Key(key) => {
                0u8.hash(state);

                match key.code {
                    KeyCode::Char(ch) => ch.to_ascii_lowercase().hash(state),
                    code => code.hash(state),
                }

                key.modifiers.hash(state);
            }
            Event::Mouse(mouse) => {
                1u8.hash(state);
                mouse.kind.hash(state);
                mouse.modifiers.hash(state);
            }
            event => {
                event.hash(state);
            }
        }
    }
}

impl fmt::Display for AppEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Left => "←".to_string(),
                        KeyCode::Backspace => "← BS".to_string(),
                        KeyCode::Right => "→".to_string(),
                        KeyCode::Up => "↑".to_string(),
                        KeyCode::Down => "↓".to_string(),
                        KeyCode::Enter => "↵".to_string(),
                        KeyCode::Tab => "↹".to_string(),
                        code => code.to_string(),
                    }
                }
                Event::Mouse(mouse) => {
                    let button = match mouse.kind {
                        MouseEventKind::Down(button)
                        | MouseEventKind::Up(button)
                        | MouseEventKind::Drag(button) => button,
                        _ => return Err(fmt::Error),
                    };

                    match button {
                        MouseButton::Left => "mouse1",
                        MouseButton::Middle => "mouse2",
                        MouseButton::Right => "mouse3",
                    }
                    .to_string()
                }
                _ => "".to_string(),
            }
        )
    }
}

pub(crate) fn parse_key<A>(key: &str, entry: &TrieEntry<A>) -> Result<Vec<AppEvent>, String> {
    use KeyCode::*;

    let mut s = key.trim().to_ascii_lowercase();
    let mut mods = KeyModifiers::empty();

    // Remove surrounding <...>
    if s.starts_with('<') && s.ends_with('>') {
        s = s[1..s.len() - 1].to_string();
    }

    // Check for modifiers
    let mut modifiers: Vec<_> = s.split('-').collect();

    let key_str = if modifiers.len() > 1 {
        // Last part is the key itself, e.g. <C-S-Enter> -> ['Control', 'Shift', 'Enter']
        let key_part = modifiers.pop().expect("Checked that length is > 1");

        // Apply all modifiers one by one
        for modifier in modifiers {
            match modifier {
                "c" => mods |= KeyModifiers::CONTROL,
                "s" => mods |= KeyModifiers::SHIFT,
                "a" => mods |= KeyModifiers::ALT,
                other => return Err(format!("Unknown modifier: {other}")),
            }
        }

        key_part
    }
    // Normal dash
    else {
        modifiers[0]
    };

    // Determine if a mouse button corresponds to the action
    let mouse = match key_str {
        "mouseleft" | "mouse1" => Some(MouseButton::Left),
        "mouseright" | "mouse2" => Some(MouseButton::Right),
        "mousemiddle" | "mouse3" => Some(MouseButton::Middle),
        _ => None,
    };

    // If so, determine the type of event based on the action
    if let Some(button) = mouse {
        let kind = match entry {
            TrieEntry::Action(Action::Click(_)) => MouseEventKind::Down(button),
            TrieEntry::Action(Action::Drag(_)) => MouseEventKind::Drag(button),
            TrieEntry::Action(Action::ScrollLeft(_)) => MouseEventKind::ScrollLeft,
            TrieEntry::Action(Action::ScrollUp(_)) => MouseEventKind::ScrollUp,
            TrieEntry::Action(Action::ScrollDown(_)) => MouseEventKind::ScrollDown,
            TrieEntry::Action(Action::ScrollRight(_)) => MouseEventKind::ScrollRight,
            _ => return Err("Invalid action {action:?} to be performed by {button:?}".to_string()),
        };

        return Ok(vec![AppEvent::new_mouse(kind, mods)]);
    }

    // Keep trying to take as much away from the key str as possible and collect events
    let mut key_events: Vec<AppEvent> = Vec::new();
    let mut idx = 0;

    'outer: while idx < key_str.len() {
        for (key, code) in SPECIAL_KEY_CODES {
            if key_str[idx..].starts_with(key) {
                let event = AppEvent::new_key(code, mods);
                key_events.push(event);

                break 'outer;
            }
        }

        let first = key_str[idx..].chars().next().expect("while-loop condition");

        for key in SHIFTED_KEY_CODES.chars() {
            if first == key || first == '_' {
                let event = AppEvent::new_key(Char(first), mods);
                key_events.push(event);

                break 'outer;
            }
        }

        if first.is_ascii_alphanumeric() || first.is_whitespace() {
            let event = AppEvent::new_key(Char(first), mods);
            key_events.push(event);

            idx += 1;
        } else {
            tracing::warn!("Unknown character '{first}' found, ignoring");
            break 'outer;
        }
    }

    Ok(key_events)
}

const SHIFTED_KEY_CODES: &str = "!@#$%^&*()_+{}|:\"<>?~";

const SPECIAL_KEY_CODES: [(&str, KeyCode); 16] = [
    (" ", KeyCode::Char(' ')),
    ("space", KeyCode::Char(' ')),
    ("backspace", KeyCode::Backspace),
    ("enter", KeyCode::Enter),
    ("left", KeyCode::Left),
    ("right", KeyCode::Right),
    ("up", KeyCode::Up),
    ("down", KeyCode::Down),
    ("home", KeyCode::Home),
    ("esc", KeyCode::Esc),
    ("end", KeyCode::End),
    ("pageup", KeyCode::PageUp),
    ("pagedown", KeyCode::PageDown),
    ("tab", KeyCode::Tab),
    ("delete", KeyCode::Delete),
    ("insert", KeyCode::Insert),
];
