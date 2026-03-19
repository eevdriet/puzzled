use puzzled_tui::{AppContext, Popup, align_area};
use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    prelude::{Buffer, Rect},
    widgets::{Block, Clear, Widget},
};

pub struct HelloPopup;

impl<A, T, M, S> Popup<A, T, M, S> for HelloPopup {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _ctx: &mut AppContext<S>) {
        let size = Size::new(10, 10);
        let inner = align_area(
            area,
            size,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
        );

        Clear {}.render(inner, buf);
        Block::bordered().title("Hello").render(inner, buf);
    }
}
