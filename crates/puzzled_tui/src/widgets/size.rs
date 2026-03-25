use ratatui::{
    layout::{Rect, Size},
    text::Text,
};

pub trait RenderSize<S> {
    fn render_size(&self, area: Rect, state: &S) -> Size;
}

impl<'a> RenderSize<()> for Text<'a> {
    fn render_size(&self, _area: Rect, _state: &()) -> Size {
        let (width, height) = self
            .lines
            .iter()
            .fold((0, 0), |(mut width, mut height), line| {
                width = line.width().max(width);
                height += 1;

                (width, height)
            });

        Size::new(width as u16, height)
    }
}
