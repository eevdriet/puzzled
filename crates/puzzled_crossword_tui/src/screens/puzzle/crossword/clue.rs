use puzzled_crossword::{Clue, Clues};
use puzzled_tui::RenderSize;
use ratatui::{
    layout::{Margin, Rect},
    prelude::Size,
    style::{Color, Style},
    text::Text,
    widgets::{Paragraph, Widget, Wrap},
};

pub struct ClueWidget<'a> {
    pub clue: &'a Clue,
}

pub struct CluesSizeWidget<'a> {
    pub clues: &'a Clues,
}

impl<'a> RenderSize<Rect> for CluesSizeWidget<'a> {
    fn render_size(&self, area: &Rect) -> Size {
        let max_height = self
            .clues
            .values()
            .map(|clue| {
                let clue_id_len = format!("{}{}  ", clue.num(), clue.direction()).len() as u16;
                let rows = clue.text().len() as f64 / (area.width - clue_id_len) as f64;

                rows.ceil() as u16
            })
            .max()
            .unwrap_or(0);

        Size {
            width: area.width,
            height: max_height,
        }
    }
}

impl<'a> Widget for ClueWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let base = Style::default();

        // Clue identifier
        let clue_text = format!("{}{}  ", self.clue.num(), self.clue.direction(),);
        let x_offset = clue_text.len() as u16;
        Text::styled(clue_text, base.fg(Color::White).bold()).render(area, buf);

        // Clue text
        let clue_area = area.inner(Margin::new(x_offset, 0));

        Paragraph::new(self.clue.text().clone())
            .style(base.fg(Color::White).bold())
            .wrap(Wrap { trim: true })
            .render(clue_area, buf);
    }
}
