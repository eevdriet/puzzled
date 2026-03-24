use crate::{Action, Description};

impl<A> Description<()> for Action<A>
where
    A: Description<()>,
{
    fn description(&self, state: &()) -> Option<String> {
        let desc = match self {
            // General
            Action::Quit => "Quit the application",
            Action::ShowHelp => "Show the help menu for the active widget",
            Action::Literal(_) => return None,
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
            // Selection
            Action::StartSelection { .. } => "Start a selection",
            Action::EndSelection => "End the current selection (if any)",
            // Custom
            Action::Custom(custom) => return custom.description(state),
        };

        Some(desc.to_string())
    }
}
