mod actions;

use nono::{Fill, Line, Rule, Run};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, run_style, status_info, widgets::rules::RuleInfo};

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
        let cell_width = puz_state.style.cell_width;

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

            let info = RuleInfo {
                rule,
                line,
                validation,
            };

            let run_area = Rect {
                x,
                y,
                width: cell_width,
                height: area.height,
            };

            self.draw_runs(&info, false, run_area, buf, state);

            if cursor.x == col && !matches!(state.focus, Focus::RulesLeft) {
                let o = rule_state.overflow_area;
                let run_area = Rect {
                    y,
                    width: 2 * cell_width,
                    ..o
                };

                self.draw_runs(&info, true, run_area, buf, state);
            }

            self.draw_status(&info, x, area, buf, state);

            // Advance to next viewport column and skip grid dividors
            x += cell_width;

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
        info: &RuleInfo,
        continue_to_right: bool,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) {
        let RuleInfo { rule, line, .. } = info;

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

        let mut x = area.x;
        let mut y = area.y;

        // tracing::info!("[{continue_to_right}] Drawing {len} runs in {area:?} ({line:?})");

        for r in 0..len {
            if y >= area.bottom() {
                if !continue_to_right {
                    // tracing::info!("\tReached bottom");
                    return;
                }

                // tracing::info!("\tContinue to right");
                x += cell_width as u16;
                y = area.y;
            }

            if r + 1 < len && y + 2 >= area.bottom() {
                if !continue_to_right {
                    // tracing::info!("\tNo more space ⋯⋯");
                    let text = format!("{:>cell_width$}", "⋯");
                    buf.set_string(x, y, text, Style::default());
                    return;
                }

                // tracing::info!("\tContinue to right");
                x += cell_width as u16;
                y = area.y;
            }

            let run = runs[r as usize];
            let text = format!("{:>cell_width$}", run.count);
            let style = run_style(info, run.fill, r, state);

            buf.set_string(x, y, text, style);
            y += 1;
        }
    }

    fn draw_status(&self, info: &RuleInfo, x: u16, area: Rect, buf: &mut Buffer, state: &AppState) {
        let cell_width = state.puzzle.style.cell_width;
        let (style, symbol) = status_info(info, state);

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
