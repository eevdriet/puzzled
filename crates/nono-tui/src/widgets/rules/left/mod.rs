mod actions;

use nono::{Fill, Line, Rule, Run};
use ratatui::{
    layout::{Alignment, Margin},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line as TextLine, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, run_style, status_info, widgets::rules::RuleInfo};

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

        let block = area.inner(Margin::new(0, 0));

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

    fn draw(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let rule_state = &state.rules_left;
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
            let validation = puz_state.puzzle.validate(rule, line);

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
            self.draw_runs(&info, Alignment::Right, inner, buf, state);

            if cursor.y == row && !matches!(state.focus, Focus::RulesTop) {
                let o = rule_state.overflow_area;
                let inner = Rect { x: o.x + 1, ..o };
                self.draw_runs(&info, Alignment::Left, inner, buf, state);
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
    ) {
        let RuleInfo { rule, .. } = info;

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

        tracing::debug!("Drawing runs for {area:?}");

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
                tracing::debug!("\tDrawing overflow and quitting");
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

        TextLine::from(spans)
            .alignment(alignment)
            .style(Style::reset())
            .render(area, buf);
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
