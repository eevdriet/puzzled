use puzzled_tui::{AppTypes, Popup, align_area};
use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    prelude::{Buffer, Rect},
    widgets::{Block, Clear, Widget},
};

pub struct HelloPopup;

impl<A: AppTypes> Popup<A> for HelloPopup {
    type State = ();

    fn render(&mut self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
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
