mod board;
mod keys;
mod list;
mod timer;

// mod settings;

pub use board::*;
pub use keys::*;
pub use list::*;
pub use timer::*;
// pub use settings::*;

use ratatui::layout::Size;

pub trait RenderSize<S> {
    fn render_size(&self, state: &S) -> Size;
}
