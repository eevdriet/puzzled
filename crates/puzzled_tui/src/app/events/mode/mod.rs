mod handle;

pub use handle::*;

use derive_more::Display;
use serde::Deserialize;

use crate::{AppContext, AppResolver, SelectionKind};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Display, Hash)]
pub enum EventMode {
    #[default]
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
