use crate::{Action, SelectionKind};

#[derive(Debug, Clone, Copy, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,

    Visual(SelectionKind),
}

impl TryFrom<Action> for Mode {
    type Error = ();

    fn try_from(action: Action) -> Result<Self, Self::Error> {
        let mode = match action {
            Action::EnterNormal => Mode::Normal,
            Action::EnterInsert => Mode::Insert,
            Action::ExitInsert => Mode::Normal,
            Action::EnterCellsVisual => Mode::Visual(SelectionKind::Cells),
            Action::EnterRowsVisual => Mode::Visual(SelectionKind::Rows),
            Action::EnterColsVisual => Mode::Visual(SelectionKind::Cols),
            Action::ExitVisual => Mode::Normal,
            _ => return Err(()),
        };

        Ok(mode)
    }
}
