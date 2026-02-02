mod actions;

use nono::{Fill, Line, LineValidation, Rule, Run};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, run_style, status_info};

#[derive(Debug)]
pub struct ColRulesWidget {
    name: String,
    rules: Vec<Rule>,
}

impl StatefulWidgetRef for &ColRulesWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let inner = Rect {
            y: area.y + 1,
            height: area.height - 1,
            ..area
        };

        let mut style = Style::default().fg(Color::DarkGray).dim();
        if matches!(state.focus, Focus::RulesTop) {
            style = style.fg(Color::White).not_dim().bold();
        }

        let block = Rect {
            x: area.x + 2,
            width: area.width - 3,
            ..area
        };

        Block::new()
            .borders(Borders::TOP)
            .title(" Cols ")
            .title_alignment(Alignment::Center)
            .title_position(TitlePosition::Top)
            .border_style(style)
            .render(block, buf);

        self.draw(inner, buf, state);
    }
}

impl ColRulesWidget {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let rule_state = &state.rules_top;
        let puz_state = &state.puzzle;
        let cursor = state.cursor();

        let vp = puz_state.viewport.clone();
        let cols = puz_state.puzzle.cols();
        let cell_width = puz_state.style.cell_width as usize;

        // Keep track of the horizontal position
        let mut x = vp.area.x;
        let y = area.y;

        for col in vp.col_start..vp.col_end {
            if x >= vp.area.right() {
                break;
            }

            let rule = &self.rules[col as usize];
            let line = Line::Col(col);
            let validation = puz_state.puzzle.validate(rule, line);

            let run_area = Rect {
                x,
                y,
                width: 1,
                height: area.height,
            };

            self.draw_runs(rule, &validation, line, run_area, buf, state);

            if cursor.x == col && !matches!(state.focus, Focus::RulesLeft) {
                let o = &rule_state.overflow_area;
                let run_area = Rect {
                    x: o.x,
                    y,
                    width: 1,
                    height: o.height - y.abs_diff(o.y),
                };

                self.draw_runs(rule, &validation, line, run_area, buf, state);
            }

            self.draw_status(line, &validation, x, area, buf, state);

            // Advance to next viewport column and skip grid dividors
            x += cell_width as u16;

            if let Some(size) = puz_state.style.grid_size
                && (col + 1).is_multiple_of(size)
                && col != cols - 1
            {
                x += 1;
            }
        }
    }

    fn draw_runs(
        &self,
        rule: &Rule,
        validation: &LineValidation,
        line: Line,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) -> bool {
        let runs = match rule.runs().len() {
            0 => &vec![Run {
                count: 0,
                fill: Fill::Blank,
            }],
            _ => rule.runs(),
        };

        //
        let cell_width = state.puzzle.style.cell_width as usize;
        let len = runs.len() as u16;
        let height = area.height.min(len);

        let x = area.x;
        let mut y = area.y;

        for r in 0..height {
            if y >= area.bottom() {
                return false;
            }

            if r + 1 < height && y + 2 >= area.bottom() {
                let text = format!("{:>cell_width$}", "⋯");
                buf.set_string(x, y, text, Style::default());

                return false;
            }

            let run = runs[r as usize];
            let text = format!("{:>cell_width$}", run.count);
            let style = run_style(run.fill, rule, r, line, validation, state);

            buf.set_string(x, y, text, style);
            y += 1;
        }

        true
    }

    fn draw_status(
        &self,
        line: Line,
        validation: &LineValidation,
        x: u16,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) {
        let cell_width = state.puzzle.style.cell_width;
        let (style, symbol) = status_info(line, validation, state);

        let area = Rect {
            x,
            y: area.bottom() - 1,
            width: cell_width,
            height: 1,
        };

        Paragraph::new(format!("{symbol}"))
            .alignment(Alignment::Right)
            .style(style)
            .render(area, buf);
    }
}
