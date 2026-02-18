use crossterm::event::{KeyCode, KeyEvent};
use puzzled_nono::{Color, Fill};
use ratatui::style::Color as RColor;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleStyle {
    #[serde(default)]
    pub colors: Vec<Color>,

    #[serde(default)]
    pub grid_size: Option<u16>,

    #[serde(default = "default_cell_width")]
    pub cell_width: u16,

    #[serde(default = "default_cell_height")]
    pub cell_height: u16,
}

fn default_cell_width() -> u16 {
    2
}
fn default_cell_height() -> u16 {
    1
}

impl Default for PuzzleStyle {
    fn default() -> Self {
        Self {
            colors: Vec::new(),
            grid_size: None,
            cell_width: default_cell_width(),
            cell_height: default_cell_height(),
        }
    }
}

impl PuzzleStyle {
    pub fn fill_color(&self, fill: Fill) -> Option<RColor> {
        match fill {
            Fill::Color(id) if id > 0 => self
                .colors
                .get(id as usize - 1)
                .copied()
                .map(|(r, g, b)| RColor::Rgb(r, g, b)),
            _ => Some(RColor::DarkGray),
        }
    }

    pub fn key_from_fill(&self, fill: Fill) -> Option<char> {
        let color_count = self.colors.len() as u16;
        fill.key(Some(color_count))
    }

    pub fn fill_from_key(&self, key: KeyEvent) -> Option<Fill> {
        let KeyCode::Char(ch) = key.code else {
            return None;
        };

        let idx = match ch {
            '.' => return Some(Fill::Blank),
            'x' | '0' => return Some(Fill::Cross),
            i @ '1'..='9' => usize::from(i as u8 - b'1'),
            i @ 'a'..='z' => usize::from(i as u8 - b'a' + 9),
            _ => return None,
        };

        (0..self.colors.len())
            .contains(&idx)
            .then_some(Fill::Color(idx as u16 + 1))
    }
}
