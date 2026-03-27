use serde::Deserialize;

use crate::{GridOptions, Load, NamedTheme};

#[derive(Debug, Default, Deserialize)]
pub struct OptionsConfig {
    #[serde(default)]
    pub grid: GridOptions,
}

#[derive(Deserialize)]
pub struct GridConfig {
    pub cell_width: Option<u16>,
    pub cell_height: Option<u16>,
    pub inner_borders: Option<u16>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Options {
    pub grid: GridOptions,

    pub theme: NamedTheme,
}

impl Load<'_> for Options {
    const FILE_NAME: &'static str = "options";
}
