mod command;
mod handle;
mod resolver;

pub use command::*;
pub use handle::*;
use ratatui::layout::Position;
pub use resolver::*;

use serde::Deserialize;

use crate::AppEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action<A = ()> {
    // Lifetime management
    Quit,

    // Mouse
    Click(#[serde(skip, default)] Position),
    Drag(#[serde(skip, default)] Position),
    ScrollDown(#[serde(skip, default)] Position),
    ScrollLeft(#[serde(skip, default)] Position),
    ScrollRight(#[serde(skip, default)] Position),
    ScrollUp(#[serde(skip, default)] Position),

    // Focus
    FocusDown,
    FocusLeft,
    FocusRight,
    FocusUp,

    // Movement
    MoveDown(#[serde(skip, default = "default_count")] usize),
    MoveLeft(#[serde(skip, default = "default_count")] usize),
    MoveRight(#[serde(skip, default = "default_count")] usize),
    MoveUp(#[serde(skip, default = "default_count")] usize),

    MoveRow(#[serde(skip, default)] usize),
    MoveRowStart,
    MoveRowEnd,

    MoveCol(#[serde(skip, default)] usize),
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
    #[serde(untagged)]
    Other(A),
}

fn default_count() -> usize {
    1
}

pub trait ActionKind {
    fn is_mouse(&self) -> bool;
}

impl<A> ActionKind for Action<A>
where
    A: ActionKind,
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

pub trait ActionHydrate: Sized {
    fn hydrate(self, _event: AppEvent, count: usize) -> Self;
}

impl<A> ActionHydrate for Action<A>
where
    A: ActionHydrate,
{
    fn hydrate(self, event: AppEvent, count: usize) -> Self {
        let mouse = event
            .mouse()
            .map(|mouse| Position::new(mouse.column, mouse.row));

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
}
