use std::fmt::Debug;

use crate::{Action, Describe};

pub trait ActionBehavior:
    Clone + PartialEq + Eq + PartialOrd + Ord + Send + Debug + Sized + Describe
{
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
}

impl<A> Describe for Action<A>
where
    A: Describe,
{
    fn describe(&self) -> Option<String> {
        Some(
            match self {
                Action::Quit => "Quit the application",
                Action::Select => "Select the active item",
                Action::Cancel => "Cancel the active item",
                // Focus
                Action::FocusDown => "Focus the widget beneath the active one",
                Action::FocusLeft => "Focus the widget to the left of the active one",
                Action::FocusRight => "Focus the widget to the right of the active one",
                Action::FocusUp => "Focus the widget above the active one",

                // Viewport
                Action::BottomViewport => "Scroll to the bottom of the viewport",
                Action::CenterViewport => "Scroll to the center of the viewport",
                Action::TopViewport => "Scroll to the top of the viewport",
                // Commands
                Action::Undo => "Undo the last action",
                Action::Redo => "Redo the last action",
                _ => "",
            }
            .to_string(),
        )
    }
}

impl<A> ActionBehavior for Action<A>
where
    A: ActionBehavior,
{
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
        let mut variants = vec![
            // Lifetime management
            Action::Quit,
            Action::Select,
            Action::Cancel,
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
