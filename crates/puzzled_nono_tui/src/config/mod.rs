mod parser;

use serde::Deserialize;

use crate::{EventTrie, PuzzleStyle};

#[derive(Debug)]
pub struct Config {
    pub settings: Settings,
    pub actions: EventTrie,

    pub styles: PuzzleStyle,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub rule_display: RuleDisplay,
}

#[derive(Debug, Default, Deserialize)]
pub enum RuleDisplay {
    #[default]
    /// Automatically fit the rules based on the puzzle dimensions
    Auto,

    TryMax,
}
