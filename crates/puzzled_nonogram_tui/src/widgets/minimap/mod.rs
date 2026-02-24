mod state;

use puzzled_nonogram::Fill;
pub use state::*;

use ratatui::{
    prelude::{Buffer, Rect},
    style::Color,
    symbols::Marker,
    widgets::{
        StatefulWidgetRef, Widget,
        canvas::{Canvas, Points},
    },
};

use crate::AppState;

#[derive(Debug, Copy, Clone)]
pub struct MiniMapWidget;

impl StatefulWidgetRef for &MiniMapWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let puzzle = &state.puzzle.puzzle;
        let colors = puzzle.colors();
        let cell_width = state.puzzle.style.cell_width;
        let cell_height = state.puzzle.style.cell_height;

        Canvas::default()
            .x_bounds([0.0, (cell_width * puzzle.cols()) as f64])
            .y_bounds([0.0, (cell_height * puzzle.rows()) as f64])
            .marker(Marker::Braille)
            .paint(|ctx| {
                for (r, row) in puzzle.fills().iter_rows().enumerate() {
                    let y_start = cell_height * (puzzle.rows() - r);

                    for (c, cell) in row.enumerate() {
                        let x_start = cell_width * c;
                        let fill = cell.fill();

                        if matches!(fill, Fill::Color(_))
                            && let Some(c) = colors.get(&fill)
                        {
                            let coords: Vec<_> = (x_start..x_start + cell_width)
                                .flat_map(move |x| {
                                    (y_start..y_start + cell_height)
                                        .map(move |y| (x as f64, y as f64))
                                })
                                .collect();

                            let points = Points {
                                coords: &coords,
                                color: Color::Rgb(c.red, c.green, c.blue),
                            };

                            ctx.draw(&points);
                        }
                    }
                }
            })
            .render(area, buf);
    }
}
