use puzzled_nono::{Axis, Fill};
use ratatui::layout::Rect;

use crate::Region;

#[derive(Debug, Default)]
pub struct FooterState {
    pub axis_region: Region<Axis>,
    pub fill_regions: Vec<Region<Fill>>,

    pub area: Rect,
}

impl FooterState {
    pub fn new() -> Self {
        Self::default()
    }
}
