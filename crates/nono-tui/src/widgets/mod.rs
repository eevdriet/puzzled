mod footer;
mod layout;
mod minimap;
mod puzzle;
mod region;
mod rules;

use std::fmt::Display;

pub use footer::*;
pub use layout::*;
pub use minimap::*;
pub use puzzle::*;
pub use region::*;
pub use rules::*;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Position as AppPosition, Rect},
    style::Style,
};

pub fn safe_draw_str<T>(buf: &mut Buffer, pos: AppPosition, content: T, style: Style)
where
    T: AsRef<str> + Display,
{
    let right = pos.x + content.as_ref().len().saturating_sub(1) as u16;
    let final_pos = AppPosition::new(right, pos.y);

    if !buf.area.contains(final_pos) {
        tracing::debug!(
            "Not writing {content} at {pos}-{final_pos}, falls outside the area {:?}",
            buf.area
        );
        return;
    }

    buf.set_string(pos.x, pos.y, content, style);
}

pub fn x_aligned(area: Rect, width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Left => area.x,
        Alignment::Right => area.right().saturating_sub(width),
        Alignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
    }
}
