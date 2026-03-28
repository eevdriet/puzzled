mod handle;

pub use handle::*;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    // -- Motion -- //
    // Text
    Change,
    ChangeSingle,

    Delete,
    DeleteLeft,
    DeleteRight,

    Yank,
    YankSingle,

    // Puzzle
    Fill(usize),
    Reveal,
    RevealSingle,
    Check,
    CheckSingle,
}

impl Operator {
    pub fn requires_motion(&self) -> bool {
        use Operator::*;

        matches!(self, Change | Delete | Yank | Reveal | Check)
    }

    pub fn is_mode_changing(&self) -> bool {
        use Operator::*;

        matches!(
            self,
            Change | Delete | ChangeSingle | DeleteLeft | DeleteRight
        )
    }
}
