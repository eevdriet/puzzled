mod actions;
mod state;

pub use state::*;

use puzzled_nono::{Axis, Fill};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{LineGauge, StatefulWidgetRef, Widget},
};

use crate::{AppState, Focus, MotionRange, PuzzleState, Region, x_aligned};

#[derive(Debug)]
pub struct FooterWidget;

impl StatefulWidgetRef for &FooterWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let line = |offset: u16| -> Rect {
            Rect {
                x: area.x,
                y: area.y + offset,
                width: area.width,
                height: 1,
            }
        };

        // Progress
        self.draw_colors(line(0), Alignment::Center, buf, state);
        self.draw_current_fill(line(0), Alignment::Left, buf, state);
        self.draw_game_time(line(0), Alignment::Right, buf, state);

        self.render_stats(line(1), buf, state);
        self.render_progress(line(2), buf, state);
    }
}

impl FooterWidget {
    fn create_fill_spans(&self, fill: Fill, state: &PuzzleState) -> Vec<(Span<'_>, Option<Fill>)> {
        let mut spans: Vec<(Span, Option<Fill>)> = Vec::new();

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
        spans.push((span, Some(fill)));

        // Id of the color
        let key = state
            .style
            .key_from_fill(fill)
            .expect("Fill {fill:?} should define a id char");

        let span = Span::styled(key.to_string(), style.fg(Color::White));
        spans.push((span, Some(fill)));

        spans
    }

    fn draw_colors(
        &self,
        area: Rect,
        alignment: Alignment,
        buf: &mut Buffer,
        state: &mut AppState,
    ) {
        // Show the available colors
        let mut fill_spans: Vec<(Span, Option<Fill>)> = Vec::new();

        let fills: Vec<_> = (0..state.puzzle.style.colors.len())
            .map(|c| Fill::Color(c as u16 + 1))
            .collect();

        for (f, fill) in fills.iter().enumerate() {
            fill_spans.extend(self.create_fill_spans(*fill, &state.puzzle));

            if f != fills.len() - 1 {
                fill_spans.push((Span::raw(" "), None));
            }
        }

        // Determine the span width
        let width = |pos: usize| -> u16 {
            let w = match pos.rem_euclid(3) {
                // symbol + space
                0 => 2,

                // index (one-digit) or space
                _ => 1,
            };

            w as u16
        };

        // Create clickable regions to set the fill
        let spans: Vec<_> = fill_spans.iter().map(|(span, _)| span).cloned().collect();
        let content_width: u16 = (0..spans.len()).map(width).sum();

        let mut x = x_aligned(area, content_width, alignment);
        let y = area.y;

        let mut regions = Vec::new();

        for (r, (_, maybe_fill)) in fill_spans.iter().enumerate() {
            let w = width(r);

            if let Some(fill) = maybe_fill {
                let region = Region {
                    data: *fill,
                    area: Rect {
                        x,
                        y,
                        width: w,
                        height: 1,
                    },
                };

                regions.push(region);
            }

            x += w;
        }

        state.footer.fill_regions = regions;

        // Finally render the line
        Line::from(spans).alignment(alignment).render(area, buf);
    }

    fn draw_current_fill(
        &self,
        area: Rect,
        alignment: Alignment,
        buf: &mut Buffer,
        state: &mut AppState,
    ) {
        // Show the current fill
        let fill = state.puzzle.fill;
        let fill_symbol = fill.symbol();
        let color = state
            .puzzle
            .style
            .fill_color(fill)
            .expect("Current fill {fill:?} should have a defined color");

        let axis = state.puzzle.motion_axis;
        let axis_symbol = match axis {
            Axis::Row => "↔",
            Axis::Col => "↕",
        };
        let axis_span = Span::styled(axis_symbol.to_string(), Style::default().fg(Color::White));

        let fill_repeat = 3;
        Line::from(vec![
            Span::styled(
                fill_symbol.to_string().repeat(fill_repeat as usize),
                Style::default().fg(color),
            ),
            Span::raw(" "),
            axis_span,
        ])
        .alignment(alignment)
        .render(area, buf);

        let x = x_aligned(area, fill_repeat, alignment) + fill_repeat + 1;

        state.footer.axis_region = Region {
            data: axis,
            area: Rect {
                x,
                y: area.y,
                width: 1,
                height: 1,
            },
        };
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

    fn draw_game_time(
        &self,
        area: Rect,
        alignment: Alignment,
        buf: &mut Buffer,
        state: &mut AppState,
    ) {
        let time = state.puzzle.start_time.elapsed();
        let secs = time.as_secs();
        let time_str = format!(
            "{:02}:{:02}:{:02}",
            (secs / 3600).rem_euclid(60),
            (secs / 60).rem_euclid(60),
            secs.rem_euclid(60)
        );

        let style = Style::default().fg(Color::Gray);
        let span = Span::styled(time_str, style);

        Line::from(span).alignment(alignment).render(area, buf);
    }
}
