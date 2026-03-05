mod board;

pub use board::*;

use ratatui::layout::Size;

pub trait RenderSize {
    type State;

    fn render_size(&self, state: &Self::State) -> Size;
}
