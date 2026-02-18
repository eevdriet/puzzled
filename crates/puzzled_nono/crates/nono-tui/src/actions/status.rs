use std::fmt::Debug;

use crate::UndoAction;

#[derive(Debug)]
pub enum ActionOutcome {
    Exit,
    Ignored,
    Consumed,
    Command(Box<dyn UndoAction>),

    LoseFocus,
    RequestFocus,
}
