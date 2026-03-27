use puzzled_core::Color as CoreColor;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::{Palette, PaletteColor};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum ThemeColor {
    Palette(PaletteColor),
    Color(CoreColor),
}
impl ThemeColor {
    pub fn resolve(&self, palette: &Palette) -> Color {
        match self {
            ThemeColor::Palette(color) => palette.get(*color),
            ThemeColor::Color(color) => Color::Rgb(color.red, color.green, color.blue),
        }
    }
}
