mod color;
mod grid;
mod palette;
mod style;

pub use color::*;
pub use grid::*;
pub use palette::*;
pub use style::*;

use ratatui::style::Style;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Theme {
    // General
    pub primary: Style,
    pub secondary: Style,

    // - UI
    pub highlighted: Style,
    pub cursor: Style,
    pub selection: Style,

    // - Solving
    pub revealed: Style,
    pub correct: Style,
    pub incorrect: Style,
    pub circled: Style,

    pub grid: GridTheme,

    pub palette: Palette,
}

impl Theme {
    pub fn from_definition(def: ThemeDef, palette: Palette) -> Self {
        Self {
            // General
            primary: def.primary.resolve(&palette),
            secondary: def.secondary.resolve(&palette),
            // - UI
            highlighted: def.highlighted.resolve(&palette),
            cursor: def.cursor.resolve(&palette),
            selection: def.selection.resolve(&palette),

            // - Solving
            revealed: def.revealed.resolve(&palette),
            correct: def.correct.resolve(&palette),
            incorrect: def.incorrect.resolve(&palette),
            circled: def.circled.resolve(&palette),

            grid: def.grid.resolve(&palette),
            palette,
        }
    }

    pub fn from_palette(palette: Palette) -> Self {
        let base = Style::default();

        Self {
            // General
            primary: base.fg(palette.yellow).bold(),
            secondary: base.fg(palette.magenta).bold(),
            // - UI
            highlighted: base,
            cursor: base.fg(palette.yellow).bold(),
            selection: base.fg(palette.green).bold(),

            // - Solving
            revealed: base.fg(palette.blue),
            correct: base.fg(palette.green),
            incorrect: base.fg(palette.red).bold(),
            circled: base,

            palette,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ThemeDef {
    // General
    pub primary: StyleDef,
    pub secondary: StyleDef,

    // - UI
    pub highlighted: StyleDef,
    pub cursor: StyleDef,
    pub selection: StyleDef,

    // - Solving
    pub revealed: StyleDef,
    pub correct: StyleDef,
    pub incorrect: StyleDef,
    pub circled: StyleDef,

    pub grid: GridThemeDef,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(untagged)]
pub enum NamedTheme {
    #[default]
    Nord,

    Solarized,

    Custom(String),
}

pub trait ResolveTheme<T> {
    fn resolve(self, palette: &Palette) -> T;
}
