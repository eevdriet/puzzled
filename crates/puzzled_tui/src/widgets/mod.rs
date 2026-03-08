mod board;
// mod settings;

pub use board::*;
// pub use settings::*;

use ratatui::layout::Size;

pub trait RenderSize {
    type State;

    fn render_size(&self, state: &Self::State) -> Size;
}
