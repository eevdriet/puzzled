mod engine;
mod trie;

use std::{hash::Hash, ops::Deref};

pub use engine::*;
use nono::Fill;
pub use trie::*;

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseEvent, MouseEventKind,
};

#[derive(Debug, Clone)]
pub struct AppEvent(Event);

impl AppEvent {
    pub fn new(event: Event) -> Self {
        Self(event)
    }

    pub fn key(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self(Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }))
    }

    pub fn mouse(kind: MouseEventKind, modifiers: KeyModifiers) -> Self {
        Self(Event::Mouse(MouseEvent {
            kind,
            modifiers,
            column: 0,
            row: 0,
        }))
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

impl TryFrom<AppEvent> for Fill {
    type Error = ();

    fn try_from(event: AppEvent) -> Result<Self, Self::Error> {
        let Event::Key(key) = event.0 else {
            return Err(());
        };

        let KeyCode::Char(ch) = key.code else {
            return Err(());
        };

        let fill = match ch {
            '.' => Fill::Blank,
            'x' | '0' => Fill::Cross,
            '1'..'9' => Fill::Color(ch as u16 - b'0' as u16),
            'a'..'x' | 'x'..='z' => Fill::Color(ch as u16 - b'a' as u16 + 9),
            _ => return Err(()),
        };

        Ok(fill)
    }
}
