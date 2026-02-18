mod actions;

use nono::{Fill, Line, Rule, Run};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line as TextLine, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, Region, run_style, status_info, widgets::rules::RuleInfo, x_aligned};

#[derive(Debug)]
pub struct RowRulesWidget {
    name: String,
    rules: Vec<Rule>,
}

impl StatefulWidgetRef for &RowRulesWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let mut style = Style::default().fg(Color::Gray).dim();
        if matches!(state.focus, Focus::RulesLeft) {
            style = style.fg(Color::White).not_dim().bold();
        }

        self.draw(area, buf, state);

        let block = Rect {
            width: area.width - 1,
            ..area
        };

        Block::new()
            .borders(Borders::TOP)
            .title(" Rows ")
            .border_style(style)
            .title_alignment(Alignment::Center)
            .title_position(TitlePosition::Top)
            .render(block, buf);
    }
}

impl RowRulesWidget {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        state.rules_left.fill_regions.clear();

        let puz_state = &state.puzzle;
        let cursor = state.cursor();

        let vp = &puz_state.viewport;
        let rows = puz_state.puzzle.rows();
        let cell_height = puz_state.style.cell_height;

        // Keep track of the vertical position
        let mut y = vp.area.y;

        for row in vp.row_start..vp.row_end {
            let rule = &self.rules[row as usize];
            let line = Line::Row(row);
            let validation = state.solver[line];

            let info = RuleInfo {
                rule,
                line,
                validation,
            };

            // NOTE: this breaks the coloring of the runs if status is drawn after the runs
            self.draw_status(&info, y, area, buf, state);

            let inner = Rect {
                y,
                width: area.width - 2,
                height: cell_height,
                ..area
            };

            let regions = self.draw_runs(&info, Alignment::Right, inner, buf, state);
            state.rules_left.fill_regions.extend(regions);

            if cursor.y == row && !matches!(state.focus, Focus::RulesTop) {
                let o = state.rules_left.overflow_area;
                let inner = Rect { x: o.x + 1, ..o };

                let regions = self.draw_runs(&info, Alignment::Left, inner, buf, state);
                state.rules_left.fill_regions.extend(regions);
            }

            // Advance to next viewport row and skip grid dividors
            y += cell_height;

            if let Some(size) = puz_state.style.grid_size
                && (row + 1).is_multiple_of(size)
                && row != rows - 1
            {
                y += 1;
            }
        }
    }

    fn draw_runs(
        &self,
        info: &RuleInfo,
        alignment: Alignment,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) -> Vec<Region<Fill>> {
        let RuleInfo { rule, .. } = info;
        let cell_height = state.puzzle.style.cell_height;

        let mut spans: Vec<Span> = Vec::new();
        let runs = match rule.runs().len() {
            0 => &vec![Run {
                count: 0,
                fill: Fill::Blank,
            }],
            _ => rule.runs(),
        };

        // Skip grid dividor row
        let mut width = 0;
        let max_width = area.width;

        tracing::trace!("Drawing runs for {area:?}");

        let (fills, texts): (Vec<_>, Vec<_>) = runs
            .iter()
            .map(|run| (run.fill, run.count.to_string()))
            .unzip();

        for r in 0..runs.len() {
            let fill = fills[r];
            let text = &texts[r];
            let len = text.len() as u16;

            // Don't overflow the area if the rule is too long to draw
            if width >= max_width {
                break;
            }
            // Instead hide the remaining runs
            else if width + len + 1 >= max_width {
                spans.push(Span::raw("⋯"));
                break;
            } else {
                // If not, draw the run
                width += len;

                let style = run_style(info, fill, r as u16, state);
                let span = Span::styled(text, style);

                spans.push(span);

                // Add a dividor to the next run if it fits
                if r != runs.len() - 1 && (width + texts[r + 1].len() as u16) < max_width {
                    spans.push(Span::raw(" "));
                    width += 1;
                }
            }
        }

        let content_width: u16 = spans.iter().map(|span| span.content.len() as u16).sum();

        let mut x = x_aligned(area, content_width, alignment);
        let y = area.y;

        let mut regions = Vec::new();

        for (r, span) in spans.iter().enumerate() {
            let w = span.content.len() as u16;

            // Only create region for run numbers, not spaces/ellipsis
            if r.is_multiple_of(2) {
                let fill = runs[r / 2].fill;

                let region = Region {
                    data: fill,
                    area: Rect {
                        x,
                        y,
                        width: w,
                        height: cell_height,
                    },
                };

                regions.push(region);
            }

            x += w;
        }

        TextLine::from(spans)
            .alignment(alignment)
            .style(Style::reset())
            .render(area, buf);

        regions
    }

    fn draw_status(&self, info: &RuleInfo, y: u16, area: Rect, buf: &mut Buffer, state: &AppState) {
        let cell_height = state.puzzle.style.cell_height;
        let (style, symbol) = status_info(info, state);

        let area = Rect {
            x: area.x,
            y,
            width: area.width,
            height: cell_height,
        };

        Paragraph::new(format!("{symbol}"))
            .alignment(Alignment::Right)
            .style(style)
            .render(area, buf);
    }
}
