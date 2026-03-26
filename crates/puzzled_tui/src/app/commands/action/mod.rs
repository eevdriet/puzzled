mod behavior;
mod description;
mod handle;

use std::cmp::Ordering;

pub use behavior::*;
pub use handle::*;

use crossterm::event::{KeyCode, MouseButton};
use derive_more::{Debug, Eq};
use ratatui::layout::Position;
use serde::Deserialize;

use crate::SelectionKind;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Action<A> {
    // Lifetime management
    Quit,
    Select,
    Cancel,

    // -- Normal -- //
    // Mouse
    StartSelection {
        pos: Option<Position>,
        kind: SelectionKind,
        additive: bool,
    },
    EndSelection,
    Click {
        button: MouseButton,
        #[debug(skip)]
        pos: Position,
    },

    // Focus
    FocusDown,
    FocusLeft,
    FocusRight,
    FocusUp,

    // Fills
    Fill(u8),
    FillNext,
    FillPrev,

    // Viewport
    BottomViewport,
    CenterViewport,
    TopViewport,

    // -- Command -- //
    Undo,
    Redo,

    ShowHelp,

    // Literal
    Literal(KeyCode),

    // Other (for puzzle specific actions)
    #[serde(untagged)]
    Custom(A),
}

impl<A> Action<A> {
    fn as_usize(&self) -> usize {
        use Action::*;

        match self {
            Quit => 00,
            Select => 10,
            Cancel => 20,

            // -- Normal -- //
            // Mouse
            StartSelection { .. } => 30,
            EndSelection => 40,
            Click { .. } => 50,

            // Focus
            FocusDown => 60,
            FocusLeft => 70,
            FocusRight => 80,
            FocusUp => 90,

            // Fills
            Fill(_) => 100,
            FillNext => 110,
            FillPrev => 120,

            // Viewport
            BottomViewport => 130,
            CenterViewport => 140,
            TopViewport => 150,

            // -- Command -- //
            Undo => 160,
            Redo => 170,

            ShowHelp => 180,

            // Literal
            Literal(..) => 190,

            // Other (for puzzle specific actions)
            Custom { .. } => 200,
        }
    }
}

impl<A> Ord for Action<A>
where
    A: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Action::Literal(lhs), Action::Literal(rhs)) => {
                lhs.partial_cmp(rhs).unwrap_or(Ordering::Less)
            }
            (Action::Custom(lhs), Action::Custom(rhs)) => lhs.cmp(rhs),
            (lhs, rhs) => lhs.as_usize().cmp(&rhs.as_usize()),
        }
    }
}

impl<A> PartialOrd for Action<A>
where
    A: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
