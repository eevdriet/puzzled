use std::fmt::Display;

use ratatui::layout::Position;

use crate::{Action, AppEvent};

pub trait ActionBehavior: Display + Sized {
    fn is_mouse(&self) -> bool;

    fn is_focus(&self) -> bool {
        false
    }

    fn hydrate(self, _events: Vec<AppEvent>, _count: usize) -> Self {
        self
    }

    fn variants() -> Vec<Self>;
}

impl<A> ActionBehavior for Action<A>
where
    A: ActionBehavior,
{
    fn is_mouse(&self) -> bool {
        match self {
            // Mouse actions
            Action::Click(_)
            | Action::Drag(_)
            | Action::ScrollLeft(_)
            | Action::ScrollRight(_)
            | Action::ScrollDown(_)
            | Action::ScrollUp(_) => true,

            // Mouse actions for other type of action
            Action::Other(other) => other.is_mouse(),

            _ => false,
        }
    }

    fn is_focus(&self) -> bool {
        matches!(
            self,
            Action::FocusLeft | Action::FocusDown | Action::FocusRight | Action::FocusUp
        )
    }

    fn hydrate(self, events: Vec<AppEvent>, count: usize) -> Self {
        let mouse = events.last().and_then(|event| {
            event
                .mouse()
                .map(|mouse| Position::new(mouse.column, mouse.row))
        });

        match self {
            // Counted actions
            Action::MoveDown(_) => Action::MoveDown(count),
            Action::MoveLeft(_) => Action::MoveLeft(count),
            Action::MoveRight(_) => Action::MoveRight(count),
            Action::MoveUp(_) => Action::MoveUp(count),

            Action::MoveRow(_) => Action::MoveRow(count),
            Action::MoveCol(_) => Action::MoveCol(count),

            // Mouse actions
            Action::Click(_) if mouse.is_some() => Action::Click(mouse.expect("Checked mouse")),
            Action::Drag(_) if mouse.is_some() => Action::Drag(mouse.expect("Checked mouse")),
            Action::ScrollDown(_) if mouse.is_some() => {
                Action::ScrollDown(mouse.expect("Checked mouse"))
            }
            Action::ScrollLeft(_) if mouse.is_some() => {
                Action::ScrollLeft(mouse.expect("Checked mouse"))
            }
            Action::ScrollRight(_) if mouse.is_some() => {
                Action::ScrollRight(mouse.expect("Checked mouse"))
            }
            Action::ScrollUp(_) if mouse.is_some() => {
                Action::ScrollUp(mouse.expect("Checked mouse"))
            }

            // Other actions remain the same
            _ => self,
        }
    }

    fn variants() -> Vec<Self> {
        let pos = Position::default();
        let count = 1;

        let mut variants = vec![
            // Lifetime management
            Action::Quit,
            Action::Select,
            Action::Cancel,
            // Mouse
            Action::Click(pos),
            Action::Drag(pos),
            Action::ScrollDown(pos),
            Action::ScrollLeft(pos),
            Action::ScrollRight(pos),
            Action::ScrollUp(pos),
            // Focus
            Action::FocusDown,
            Action::FocusLeft,
            Action::FocusRight,
            Action::FocusUp,
            // Movement
            Action::MoveDown(count),
            Action::MoveLeft(count),
            Action::MoveRight(count),
            Action::MoveUp(count),
            Action::MoveRow(count),
            Action::MoveRowStart,
            Action::MoveRowEnd,
            Action::MoveCol(count),
            Action::MoveColStart,
            Action::MoveColEnd,
            // Solving
            Action::Reveal,
            Action::RevealAll,
            // Viewport
            Action::BottomViewport,
            Action::CenterViewport,
            Action::TopViewport,
            // Commands
            Action::Undo,
            Action::Redo,
        ];

        let other_variants = A::variants().into_iter().map(Action::Other);
        variants.extend(other_variants);

        variants
    }
}
