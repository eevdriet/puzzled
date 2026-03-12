use derive_more::Display;
use serde::Deserialize;

use crate::SelectionKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Display, Hash)]
pub enum EventMode {
    Normal,
    Insert,
    Replace,
    Visual(SelectionKind),
}

impl EventMode {
    pub fn is_visual(&self) -> bool {
        matches!(self, Self::Visual(_))
    }
}
