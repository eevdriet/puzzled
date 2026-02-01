use serde::Deserialize;

mod engine;
mod history;
mod kind;
mod motion;
mod status;
mod r#trait;

pub use engine::*;
pub use history::*;
pub use kind::*;
pub use motion::*;
pub use status::*;
pub use r#trait::*;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /* -- App -- */
    Quit,

    FocusLeft,
    FocusDown,
    FocusUp,
    FocusRight,

    /* -- Puzzle -- */
    // Mouse
    Click,
    Drag,
    ScrollLeft,
    ScrollRight,
    ScrollDown,
    ScrollUp,

    // Fills
    Measure,
    Fill,
    Cross,
    DeleteSingle,
    Delete,

    // Normal mode
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,

    // Fill jumps
    FindFillForwards,
    FindFillBackwards,
    FindTilFillForwards,
    FindTilFillBackwards,

    // Row jumps
    // - all rows
    JumpRow,
    JumpRowStart,
    JumpRowEnd,
    JumpCol,
    JumpColStart,
    JumpColEnd,

    JumpEndBackwards,
    JumpEndForwards,
    JumpFirstNonBlank,
    JumpLastNonBlank,
    JumpStartBackwards,
    JumpStartForwards,

    // Viewport
    CenterViewport,
    BottomViewport,
    TopViewport,
    // ShiftViewportLeft,
    // ShiftViewportRight,
    // ShiftViewportUp,
    // ShiftViewportDown,

    // History
    Undo,
    Redo,

    // Other
    SwitchAxis,
    SampleFill,
    SwitchFill,

    // Modes
    EnterNormal,
    ExitNormal,
    EnterInsert,
    ExitInsert,
    EnterCellsVisual,
    EnterLinesVisual,
    ExitVisual,
}

impl Action {
    pub fn kind(&self) -> ActionKind {
        use Action::*;

        match self {
            // Commands
            Quit | Undo | Redo | CenterViewport | BottomViewport | TopViewport | SwitchAxis
            | FocusLeft | FocusDown | FocusRight | FocusUp | SampleFill | SwitchFill => {
                ActionKind::Command
            }

            // Operators
            Fill | Cross | DeleteSingle | Delete | Measure => ActionKind::Operator,

            // Modes
            EnterNormal | ExitNormal | EnterInsert | ExitInsert | EnterCellsVisual
            | EnterLinesVisual | ExitVisual => ActionKind::Mode,

            // Motions
            Click | Drag | FindFillBackwards | FindFillForwards | FindTilFillBackwards
            | FindTilFillForwards | JumpCol | JumpColEnd | JumpColStart | JumpRow | JumpRowEnd
            | JumpEndBackwards | JumpEndForwards | JumpFirstNonBlank | JumpLastNonBlank
            | JumpRowStart | JumpStartBackwards | JumpStartForwards | MoveDown | MoveLeft
            | MoveRight | MoveUp | ScrollDown | ScrollLeft | ScrollUp | ScrollRight => {
                ActionKind::Motion
            }
        }
    }

    pub fn is_motionless_op(&self) -> bool {
        matches!(
            (self.kind(), self),
            (ActionKind::Operator, Action::Fill | Action::DeleteSingle)
        )
    }

    pub fn requires_operand(&self) -> bool {
        matches!(
            self,
            Action::FindFillForwards | Action::FindFillBackwards | Action::SwitchFill
        )
    }
}
