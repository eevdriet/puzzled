use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    prelude::{Buffer, Rect},
    text::Text,
    widgets::{Block, Widget},
};

use crate::align_area;

pub struct TextBlock<'a> {
    pub text: Text<'a>,
    pub block: Block<'a>,

    pub h_align: HorizontalAlignment,
    pub v_align: VerticalAlignment,
}

impl Widget for TextBlock<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Render the block first
        let inner = self.block.inner(area);
        self.block.render(area, buf);

        // Then render the text inside according to its alignment
        let text_size = Size {
            width: self.text.width() as u16,
            height: self.text.height() as u16,
        };
        let text_area = align_area(inner, text_size, self.h_align, self.v_align);
        self.text.render(text_area, buf);
    }
}
