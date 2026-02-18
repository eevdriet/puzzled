use puzzled_nono::{Fill, Order};
use ratatui::layout::Rect;

use crate::Region;

#[derive(Debug, Default)]
pub struct FooterState {
    pub order_region: Region<Order>,
    pub fill_regions: Vec<Region<Fill>>,

    pub area: Rect,
}

impl FooterState {
    pub fn new() -> Self {
        Self::default()
    }
}
