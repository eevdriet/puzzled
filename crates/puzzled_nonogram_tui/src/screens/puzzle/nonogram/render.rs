use puzzled_core::{Entry, Position, Side};
use puzzled_nonogram::{Colors, Fill, Rule};
use puzzled_tui::{
    AppContext, AsApp, CellRender, GridRenderState, LineRender, SidesRenderState, ThemeStyled,
};
use ratatui::{
    prelude::Widget,
    style::{Color, Style},
    text::{Line, Span, Text},
};

use crate::NonogramApp;

#[derive(Debug)]
pub struct RenderFill<'a> {
    pub fill: &'a Fill,
}

impl<'a> ThemeStyled for RenderFill<'a> {}

#[derive(Debug, Clone, Copy)]
pub struct RenderFillState<'a> {
    pub colors: &'a Colors,
    pub render: &'a GridRenderState,
}

impl<'a> CellRender<NonogramApp, RenderFillState<'a>> for Entry<RenderFill<'a>> {
    fn render_cell(
        &self,
        pos: Position,
        state: &RenderFillState,
        ctx: &AppContext<NonogramApp>,
    ) -> impl Widget {
        let mut style = self.theme_style(&ctx.theme);
        style = style.fg(ctx.theme.palette.light2);

        if let Some(render) = self.entry()
            && let Some(color) = state.colors.get(render.fill)
        {
            style = style.fg(color.as_app());
        }

        let cell_style = state.render.cell_style(pos, &ctx.theme);
        style = style.patch(cell_style);

        let symbol = match self.entry() {
            Some(fill) => fill.fill.symbol(),
            _ => '◌',
        }
        .to_string();

        let text = vec![
            Line::from(symbol.repeat(state.render.options.cell_width as usize));
            state.render.options.cell_height as usize
        ];
        Text::from(text).style(style)
    }
}

#[derive(Debug)]
pub struct RenderRule<'a> {
    pub rule: &'a Rule,
}

impl<'a> RenderRule<'a> {
    fn fills_texts(&self) -> (Vec<Fill>, Vec<String>) {
        self.rule
            .runs()
            .iter()
            .map(|run| (run.fill, run.count.to_string()))
            .unzip()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderRuleState<'a> {
    pub colors: &'a Colors,
    pub render: &'a SidesRenderState,
}

impl<'a> LineRender<NonogramApp, RenderRuleState<'a>> for RenderRule<'a> {
    fn render_row(
        &self,
        _row: usize,
        state: &RenderRuleState<'a>,
        _ctx: &AppContext<NonogramApp>,
    ) -> Text<'_> {
        let mut width = 0;
        let max_width = state.render.get(Side::Left).max_len.unwrap_or(u16::MAX);

        let mut spans: Vec<Span> = Vec::new();
        let base = Style::default();

        let (fills, texts) = self.fills_texts();

        for (f, fill) in fills.iter().enumerate() {
            let text = texts[f].to_owned();
            let len = text.len() as u16;

            // Don't overflow the area if the rule is too long to draw
            if width >= max_width {
                break;
            }
            // Instead hide the remaining runs
            else if width + len + 1 >= max_width {
                spans.push(Span::raw("⋯"));
                break;
            }

            // Otherwise, draw the run normally
            width += len;

            let color: Color = state
                .colors
                .get(fill)
                .unwrap_or_else(|| {
                    panic!("Fill color {fill:?} should be defined: {:?}", state.colors)
                })
                .as_app();

            let style = base.fg(color);
            let span = Span::styled(text, style);
            spans.push(span);

            // Add a divider to the next run if it fits
            if f < fills.len() - 1 && (width + texts[f + 1].len() as u16) < max_width {
                spans.push(Span::raw(" "));
                width += 1;
            }
        }

        Text::from(Line::from(spans))
    }
    fn render_col(
        &self,
        _col: usize,
        state: &RenderRuleState<'a>,
        _ctx: &AppContext<NonogramApp>,
    ) -> Text<'_> {
        let mut height = 0;
        let max_height = state.render.get(Side::Top).max_len.unwrap_or(u16::MAX);

        let mut lines: Vec<Line> = Vec::new();
        let base = Style::default();

        let (fills, texts) = self.fills_texts();

        for (f, fill) in fills.iter().enumerate() {
            let text = texts[f].to_owned();
            let len = text.len() as u16;

            // Don't overflow the area if the rule is too tall to draw
            if height > max_height {
                break;
            }
            // Instead hide the remaining runs
            else if height + 1 > max_height {
                lines.push(Line::raw("⋯"));
                break;
            }

            // Otherwise, draw the run normally
            height += len;

            let color: Color = state
                .colors
                .get(fill)
                .unwrap_or_else(|| {
                    panic!("Fill color {fill:?} should be defined: {:?}", state.colors)
                })
                .as_app();

            let style = base.fg(color);
            let span = Line::styled(text, style);
            lines.push(span);
        }

        Text::from(lines)
    }
}
