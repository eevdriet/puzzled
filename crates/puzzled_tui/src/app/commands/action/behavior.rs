use std::fmt::Debug;

use crossterm::event::MouseButton;
use ratatui::layout::Position;

use crate::Action;

pub trait ActionBehavior: Clone + PartialEq + Eq + Send + Debug + Sized {
    fn is_mouse(&self) -> bool;

    fn is_focus(&self) -> bool {
        false
    }

    fn is_other(&self) -> bool {
        true
    }

    fn variants() -> Vec<Self>;
}

impl ActionBehavior for () {
    fn variants() -> Vec<Self> {
        vec![]
    }

    fn is_focus(&self) -> bool {
        false
    }

    fn is_mouse(&self) -> bool {
        false
    }
}

impl<A> ActionBehavior for Action<A>
where
    A: ActionBehavior,
{
    fn is_mouse(&self) -> bool {
        match self {
            // Mouse actions
            Action::Click { .. } | Action::Drag { .. } => true,

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

    fn is_other(&self) -> bool {
        false
    }

    fn variants() -> Vec<Self> {
        let pos = Position::default();
        let button = MouseButton::Left;

        let mut variants = vec![
            // Lifetime management
            Action::Quit,
            Action::Select,
            Action::Cancel,
            // Mouse
            Action::Click { pos, button },
            Action::Drag { pos, button },
            // Focus
            Action::FocusDown,
            Action::FocusLeft,
            Action::FocusRight,
            Action::FocusUp,
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
