use derive_more::Display;
use serde::Deserialize;

#[derive(
    Debug, Default, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Display, PartialOrd, Ord,
)]
pub enum SelectionKind {
    #[default]
    Cells,

    Rows,
    Cols,
}
