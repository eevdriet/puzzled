use crate::{Action, AppTypeTraits};

pub trait ActionBehavior: AppTypeTraits {
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
            Action::ShowHelp,
            Action::Select,
            Action::Cancel,
            // Focus
            Action::FocusDown,
            Action::FocusLeft,
            Action::FocusRight,
            Action::FocusUp,
            // Fills
            Action::Fill(0),
            Action::FillNext,
            Action::FillPrev,
            // Viewport
            Action::BottomViewport,
            Action::CenterViewport,
            Action::TopViewport,
            // Commands
            Action::Undo,
            Action::Redo,
        ];

        let other_variants = A::variants().into_iter().map(Action::Custom);
        variants.extend(other_variants);

        variants
    }
}
