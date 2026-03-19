mod behavior;
mod handle;

pub use behavior::*;
pub use handle::*;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TextObject<T> {
    // Custom (for puzzle specific text objects)
    #[serde(untagged)]
    Custom(T),
}
