mod command;
mod handle;

pub use command::*;
pub use handle::*;

use crossterm::event::MouseEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action<A = ()> {
    // Screen management
    Quit,

    // Mouse
    Click(MouseEvent),
    Drag(MouseEvent),
    ScrollDown(MouseEvent),
    ScrollLeft(MouseEvent),
    ScrollRight(MouseEvent),
    ScrollUp(MouseEvent),

    // Focus
    FocusDown,
    FocusLeft,
    FocusRight,
    FocusUp,

    // Movement
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,

    MoveRow(usize),
    MoveRowStart,
    MoveRowEnd,

    MoveCol(usize),
    MoveColStart,
    MoveColEnd,

    // Viewport
    BottomViewport,
    CenterViewport,
    TopViewport,

    // Commands
    Select,
    Undo,
    Redo,

    // Other (for puzzle specific actions)
    Other(A),
}

pub trait ActionBehavior {
    fn is_mouse(&self) -> bool;
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
}
