use ratatui::style::{Modifier, Style};
use serde::{Deserialize, Serialize};

use crate::{Palette, ResolveTheme, ThemeColor};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StyleDef {
    /// The foreground color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg: Option<ThemeColor>,
    /// The background color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<ThemeColor>,
    /// The underline color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline_color: Option<ThemeColor>,
    /// The modifiers to add.
    #[serde(
        default,
        skip_serializing_if = "Modifier::is_empty",
        deserialize_with = "deserialize_modifier"
    )]
    pub add_modifier: Modifier,
    /// The modifiers to remove.
    #[serde(
        default,
        skip_serializing_if = "Modifier::is_empty",
        deserialize_with = "deserialize_modifier"
    )]
    pub sub_modifier: Modifier,
}

fn deserialize_modifier<'de, D>(deserializer: D) -> Result<Modifier, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    Option::<Modifier>::deserialize(deserializer)
        .map(|modifier| modifier.unwrap_or_else(Modifier::empty))
}

impl ResolveTheme<Style> for StyleDef {
    fn resolve(self, palette: &Palette) -> Style {
        let mut style = Style::default();

        if let Some(fg) = self.fg {
            style = style.fg(fg.resolve(palette));
        }

        if let Some(bg) = self.bg {
            style = style.bg(bg.resolve(palette));
        }

        style = style.add_modifier(self.add_modifier);
        style = style.remove_modifier(self.sub_modifier);

        style
    }
}
