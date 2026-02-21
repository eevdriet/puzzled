mod actions;

use puzzled_nonogram::{Fill, Line, Rule, Run};
use ratatui::{
    layout::{Alignment, Position},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{
    AppState, Focus, Region, run_style, safe_draw_str, status_info, widgets::rules::RuleInfo,
};

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

    fn draw(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        state.rules_top.fill_regions.clear();

        let puz_state = &state.puzzle;
        let cursor = state.cursor();

        let vp = puz_state.viewport.clone();
        let cols = puz_state.puzzle.cols();
        let cell_width = puz_state.style.cell_width;

        // Keep track of the horizontal position
        let mut x = vp.area.x;
        let y = area.y;

        for col in vp.col_start..vp.col_end {
            let col = col as usize;
            if x >= vp.area.right() {
                break;
            }

            let rule = &self.rules[col];
            let line = Line::Col(col);
            let validation = state.solver[line];

            let info = RuleInfo {
                rule,
                line,
                validation,
            };

            let run_area = Rect {
                x,
                y,
                width: cell_width as u16,
                height: area.height,
            };

            let regions = self.draw_runs(&info, false, run_area, buf, state);
            state.rules_top.fill_regions.extend(regions);

            if cursor.x as usize == col && !matches!(state.focus, Focus::RulesLeft) {
                let o = state.rules_top.overflow_area;
                let run_area = Rect { y, ..o };

                let regions = self.draw_runs(&info, true, run_area, buf, state);
                state.rules_top.fill_regions.extend(regions);
            }

            self.draw_status(&info, x, area, buf, state);

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
        info: &RuleInfo,
        continue_to_right: bool,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) -> Vec<Region<Fill>> {
        let RuleInfo { rule, .. } = info;
        let mut regions = Vec::new();

        let runs = match rule.runs().len() {
            0 => &vec![Run {
                count: 0,
                fill: Fill::Blank,
            }],
            _ => rule.runs(),
        };

        //
        let cell_width = state.puzzle.style.cell_width;
        let len = runs.len() as u16;

        let mut x = area.x;
        let mut y = area.y;

        for r in 0..len {
            if y + 1 >= area.bottom() {
                if !continue_to_right {
                    // tracing::info!("\tReached bottom");
                    return regions;
                }

                // tracing::info!("\tContinue to right");
                x += cell_width as u16;
                y = area.top();
            }

            if r + 1 < len && y + 2 >= area.bottom() {
                if !continue_to_right {
                    // tracing::info!("\tNo more space ⋯⋯");
                    let text = format!("{:>cell_width$}", "⋯");

                    safe_draw_str(buf, Position::new(x, y), text, Style::default());
                    return regions;
                }

                // tracing::info!("\tContinue to right");
                x += cell_width as u16;
                y = area.top();
            }

            let run = runs[r as usize];
            let text = format!("{:>cell_width$}", run.count);
            let style = run_style(info, run.fill, r, state);

            let region = Region::<Fill> {
                data: run.fill,
                area: Rect {
                    x,
                    y,
                    width: cell_width as u16,
                    height: 1,
                },
            };
            regions.push(region);

            safe_draw_str(buf, Position::new(x, y), text, style);
            y += 1;
        }

        regions
    }

    fn draw_status(&self, info: &RuleInfo, x: u16, area: Rect, buf: &mut Buffer, state: &AppState) {
        let cell_width = state.puzzle.style.cell_width;
        let (style, symbol) = status_info(info, state);

        let area = Rect {
            x,
            y: area.bottom() - 1,
            width: cell_width as u16,
            height: 1,
        };

        Paragraph::new(format!("{symbol}"))
            .alignment(Alignment::Right)
            .style(style)
            .render(area, buf);
    }
}
