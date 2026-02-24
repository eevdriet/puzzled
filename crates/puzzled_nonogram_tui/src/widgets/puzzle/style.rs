use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleStyle {
    #[serde(default)]
    pub grid_size: Option<usize>,

    #[serde(default = "default_cell_width")]
    pub cell_width: usize,

    #[serde(default = "default_cell_height")]
    pub cell_height: usize,
}

fn default_cell_width() -> usize {
    2
}
fn default_cell_height() -> usize {
    1
}

impl Default for PuzzleStyle {
    fn default() -> Self {
        Self {
            grid_size: None,
            cell_width: default_cell_width(),
            cell_height: default_cell_height(),
        }
    }
}
