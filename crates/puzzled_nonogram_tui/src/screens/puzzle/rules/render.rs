use puzzled_nonogram::{Colors, Fill, Rule};
use puzzled_tui::{AppContext, AsApp, EdgeRender, SidedGridRenderState};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::NonogramApp;

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
    pub is_active_rule: bool,
}

impl<'a> EdgeRender<NonogramApp, RenderRuleState<'a>> for RenderRule<'a> {
    fn render_row(
        &self,
        _row: usize,
        area: Rect,
        buf: &mut Buffer,
        _ctx: &AppContext<NonogramApp>,
        _render: &SidedGridRenderState,
        state: &RenderRuleState<'a>,
    ) {
        let mut x = area.x;

        let base = Style::default();
        let mut spans = Vec::new();

        let (fills, texts) = self.fills_texts();

        for (f, fill) in fills.iter().enumerate() {
            let text = texts[f].to_owned();
            let len = text.len() as u16;

            // Don't overflow the area if the rule is too long to draw
            if x >= area.right() {
                break;
            }
            // Instead hide the remaining runs
            if f < fills.len() - 1 && x + len + 1 >= area.right() {
                let span = Span::raw("⋯");
                spans.push(span);
                break;
            }

            // Otherwise, draw the run normally
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
            x += len;

            // Add a divider to the next run if it fits
            if f < fills.len() - 1 && x + 1 < area.right() {
                spans.push(Span::raw(" "));
                x += 1;
            }
        }

        let alignment = if state.is_active_rule {
            Alignment::Left
        } else {
            Alignment::Right
        };

        Line::from(spans).alignment(alignment).render(area, buf);
    }

    fn render_col(
        &self,
        _col: usize,
        area: Rect,
        buf: &mut Buffer,
        _ctx: &AppContext<NonogramApp>,
        render: &SidedGridRenderState,
        state: &RenderRuleState<'a>,
    ) {
        let mut x = area.x;
        let mut y = area.y;
        let cell_w = render.grid.options.cell_width;

        let base = Style::default();
        let alignment = Alignment::Right;

        let (fills, texts) = self.fills_texts();

        for (f, fill) in fills.iter().enumerate() {
            let text = texts[f].to_owned();
            let text_area = Rect {
                x,
                y,
                height: 1,
                width: cell_w,
            };

            // Don't overflow the area if the rule is too tall to draw
            if y >= area.bottom() {
                if state.is_active_rule {
                    x += cell_w;
                    y = area.y;
                } else {
                    break;
                }
            }
            // Instead hide the remaining runs
            else if f < fills.len() - 1 && y + 1 >= area.bottom() {
                if state.is_active_rule {
                } else {
                    Line::raw("⋯").alignment(alignment).render(text_area, buf);
                    break;
                }
            }

            // Otherwise, draw the run normally
            let color: Color = state
                .colors
                .get(fill)
                .unwrap_or_else(|| {
                    panic!("Fill color {fill:?} should be defined: {:?}", state.colors)
                })
                .as_app();

            let style = base.fg(color);

            Line::styled(text, style)
                .alignment(alignment)
                .render(text_area, buf);

            y += 1;
        }
    }

    fn height(
        &self,
        _area: Rect,
        _ctx: &AppContext<NonogramApp>,
        _render: &SidedGridRenderState,
        _state: &RenderRuleState<'a>,
    ) -> u16 {
        self.rule.runs().len() as u16
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<NonogramApp>,
        _render: &SidedGridRenderState,
        _state: &RenderRuleState<'a>,
    ) -> u16 {
        let mut width = 0;

        for (r, run) in self.rule.runs().iter().enumerate() {
            if r > 0 {
                width += 1;
            }
            width += run.count.to_string().len();
        }

        width as u16
    }
}
