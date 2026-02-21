use std::fmt::Debug;

use ratatui::layout::Rect;

#[derive(Debug, Default)]
pub struct Region<T: Debug> {
    pub data: T,
    pub area: Rect,
}
