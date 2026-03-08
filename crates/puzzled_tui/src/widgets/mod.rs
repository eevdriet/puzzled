mod board;
mod list;

// mod settings;

pub use board::*;
pub use list::*;
// pub use settings::*;

use ratatui::layout::Size;

pub trait RenderSize {
    type State;

    fn render_size(&self, state: &Self::State) -> Size;
}
