use std::marker::PhantomData;

use puzzled_tui::{AppContext, AsApp, Widget as AppWidget};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect, Size},
    style::Style,
    text::{Line, Span, Text},
    widgets::Widget,
};

use crate::{FooterState, NonogramApp};

#[derive(Debug)]
pub struct FillsWidget<'a> {
    _marker: &'a PhantomData<()>,
}

impl<'a> AppWidget<NonogramApp> for FillsWidget<'a> {
    type State = FooterState<'a>;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<NonogramApp>,
        state: &mut Self::State,
    ) {
        // All fills
        let fills_text = self.fills_text(root, ctx, state);
        fills_text.render(root, buf);
    }

    fn render_size(
        &self,
        root: Rect,
        ctx: &AppContext<NonogramApp>,
        state: &mut Self::State,
    ) -> Size {
        let fills_text = self.fills_text(root, ctx, state);
        Size::new(fills_text.width() as u16, fills_text.height() as u16)
    }
}

impl<'a> FillsWidget<'a> {
    pub fn new() -> Self {
        Self {
            _marker: &PhantomData,
        }
    }

    pub fn fills_text(
        &self,
        root: Rect,
        ctx: &AppContext<NonogramApp>,
        state: &FooterState<'a>,
    ) -> Text<'a> {
        let base = Style::default();
        let colors = state.colors;

        let mut line = Line::default();
        let mut text = Text::default().alignment(Alignment::Center);

        for (f, (fill, color)) in colors.iter().enumerate() {
            let style = if state.fill == fill {
                base.bold().underlined()
            } else {
                base
            };

            // Fill identifier
            let fill_id = fill.index().expect("Fill should be valid").to_string();
            let fill_span = Span::styled(fill_id, style.fg(ctx.theme.palette.light3));

            // Color brush
            let color = color.as_app();
            let color_span = Span::styled(
                fill.symbol().to_string(),
                style.fg(color).underline_color(color),
            );

            // Continue on the next line if the spans do not fit
            let gap = if f + 1 < colors.len() { 1 } else { 0 };
            let width = line.width() + fill_span.width() + gap + color_span.width();

            if width > root.width as usize {
                text.push_line(line);
                line = Line::default();
            }

            // Add the spans to the line
            line.push_span(fill_span);
            line.push_span(color_span);
            line.push_span(Span::raw(" ".repeat(gap)));
        }

        if line.width() > 0 {
            text.push_line(line);
        }

        text
    }
}

impl<'a> Default for FillsWidget<'a> {
    fn default() -> Self {
        Self::new()
    }
}
