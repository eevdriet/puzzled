use ratatui::{style::Style, widgets::BorderType};
use serde::Deserialize;

use crate::{ResolveTheme, StyleDef};

use super::Palette;

#[derive(Debug, Default, Clone, Copy)]
pub struct GridTheme {
    pub inner_border: BorderType,
    pub outer_border: BorderType,

    pub inner_border_style: Style,
    pub outer_border_style: Style,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(default)]
pub struct GridThemeDef {
    pub inner_border: BorderType,
    pub outer_border: BorderType,

    pub inner_border_style: StyleDef,
    pub outer_border_style: StyleDef,
}

impl ResolveTheme<GridTheme> for GridThemeDef {
    fn resolve(self, palette: &Palette) -> GridTheme {
        GridTheme {
            inner_border: self.inner_border,
            outer_border: self.outer_border,
            inner_border_style: self.inner_border_style.resolve(palette),
            outer_border_style: self.outer_border_style.resolve(palette),
        }
    }
}
