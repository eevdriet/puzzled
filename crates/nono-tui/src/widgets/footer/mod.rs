mod actions;

use nono::{Axis, Fill};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{LineGauge, Paragraph, StatefulWidgetRef, Widget},
};

use crate::{AppState, Focus, MotionRange, PuzzleState};

#[derive(Debug)]
pub struct FooterWidget;

impl StatefulWidgetRef for &FooterWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let rect = |offset: u16| -> Rect {
            Rect {
                x: area.x,
                y: area.y + offset,
                width: area.width,
                height: 1,
            }
        };

        // Progress
        self.render_colors(rect(1), buf, state);
        self.render_stats(rect(2), buf, state);
        self.render_progress(rect(3), buf, state);
    }
}

impl FooterWidget {
    fn create_fill_spans(&self, fill: Fill, state: &PuzzleState) -> Vec<Span<'_>> {
        let mut spans: Vec<Span> = Vec::new();

        let mut style = match state.fill == fill {
            true => Style::default().bold().underlined(),
            _ => Style::default(),
        };
        let color = state
            .style
            .fill_color(fill)
            .expect("Fill {fill:?} should have a defined color");

        style = style.underline_color(color);

        // Color brush itself
        let symbol = fill.symbol();
        let span = Span::styled(format!("{symbol} "), style.fg(color));
        spans.push(span);

        // Id of the color
        let key = state
            .style
            .key_from_fill(fill)
            .expect("Fill {fill:?} should define a id char");

        let span = Span::styled(key.to_string(), style.fg(Color::White));
        spans.push(span);

        spans
    }

    fn render_colors(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // Show the available colors
        let mut spans: Vec<Span> = Vec::new();

        let fills: Vec<_> = (0..state.puzzle.style.colors.len())
            .map(|c| Fill::Color(c as u16 + 1))
            .collect();

        for fill in fills {
            spans.extend(self.create_fill_spans(fill, &state.puzzle));
            spans.push(Span::raw(" "));
        }

        let line = Line::from(spans);

        Paragraph::new(line)
            .alignment(Alignment::Center)
            .render(area, buf);

        // Show the current fill
        let fill = state.puzzle.fill;
        let fill_symbol = fill.symbol();
        let color = state
            .puzzle
            .style
            .fill_color(fill)
            .expect("Current fill {fill:?} should have a defined color");

        let axis_symbol = match state.puzzle.motion_axis {
            Axis::Row => "↔",
            Axis::Col => "↕",
        };

        Line::from(vec![
            Span::styled(
                fill_symbol.to_string().repeat(3),
                Style::default().fg(color),
            ),
            Span::raw(" "),
            Span::styled(axis_symbol.to_string(), Style::default().fg(Color::White)),
        ])
        .alignment(Alignment::Left)
        .render(area, buf);
    }

    fn render_progress(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // Determine how many of the cells are filled (non-blank)
        let fill_count = state
            .puzzle
            .puzzle
            .iter_cells()
            .filter(|fill| !matches!(fill, Fill::Blank))
            .count() as u16;

        let fill_perc = fill_count as f64 / state.puzzle.puzzle.size() as f64;

        // let gauge = Gauge::default().ratio(fill_perc);
        let gauge = LineGauge::default()
            .filled_style(Style::new().white().on_black().bold())
            .filled_symbol(symbols::line::THICK_HORIZONTAL)
            .ratio(fill_perc);

        gauge.render(area, buf);
    }

    fn render_stats(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let style = Style::default().fg(Color::White);
        let cursor = state.puzzle.cursor;

        // Left
        Span::styled(format!("{},{}", cursor.y + 1, cursor.x + 1), style)
            .into_left_aligned_line()
            .render(area, buf);

        // Middle
        let selection_span = self.selection_span(state).into_centered_line();
        selection_span.render(area, buf);

        // Right
        // Show the dimensions of the puzzle
        Span::styled(
            format!(
                "{},{}",
                state.puzzle.puzzle.rows(),
                state.puzzle.puzzle.cols()
            ),
            style,
        )
        .into_right_aligned_line()
        .render(area, buf);
    }

    fn selection_span(&self, state: &mut AppState) -> Span<'_> {
        let cursor = state.cursor();
        let style = Style::default().fg(Color::White);
        let range = state.selection().range();

        let row_rule = &state.rules_left.rules[cursor.y as usize];
        let col_rule = &state.rules_top.rules[cursor.x as usize];

        let text = match (state.focus, range) {
            /* -- Rules -- */
            // Show the selected runs on the active left rule
            (Focus::RulesLeft, MotionRange::Block(Rect { x, width, .. })) => {
                row_rule.slice(x..x + width).len().to_string()
            }
            // Show the selected runs on the active top rule
            (Focus::RulesTop, MotionRange::Block(Rect { y, height, .. })) => {
                col_rule.slice(y..y + height).len().to_string()
            }
            // Show length of the active rule
            (Focus::RulesLeft, _) => row_rule.len().to_string(),
            (Focus::RulesTop, _) => col_rule.len().to_string(),

            /* -- Puzzle -- */
            (Focus::Puzzle, MotionRange::Single(pos)) => format!("{},{}", pos.y, pos.x),

            (Focus::Puzzle, MotionRange::Block(rect)) => format!(
                "{},{} -> {},{}",
                rect.y,
                rect.x,
                rect.y + rect.height - 1,
                rect.x + rect.width - 1
            ),

            (
                Focus::Puzzle,
                MotionRange::Rows { start, end } | MotionRange::Cols { start, end },
            ) => {
                format!("{start} -> {end}")
            }

            // Show the length of both rules if nothing selected in the puzzle
            (Focus::Puzzle, _) => format!("{}R, {}C", row_rule.len(), col_rule.len()),

            // Show the selection in the puzzle

            // Don't display anything by default
            _ => String::new(),
        };

        Span::styled(text, style)
    }
}
