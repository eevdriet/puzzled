use std::ops::RangeInclusive;

use puzzled_core::{Grid, Value};

use crate::{Fill, Line, LinePosition};

#[derive(Debug, Clone, Copy)]
pub enum FindDirection {
    Forwards,
    Backwards,
}

fn none_or_fill(fill: Option<&Fill>, target: Fill) -> bool {
    fill.is_none_or(|f| *f == target)
}
fn some_and_fill(cell: Option<&Fill>, fill: Fill) -> bool {
    cell.is_some_and(|f| *f == fill)
}

pub trait FillsFind {
    fn find_run_start(&self, line_pos: LinePosition) -> Option<LinePosition>;
    fn find_run_end(&self, line_pos: LinePosition) -> Option<LinePosition>;

    fn find_directed_run_start(
        &self,
        pos: LinePosition,
        direction: FindDirection,
    ) -> Option<LinePosition>;

    fn find_directed_run_end(
        &self,
        pos: LinePosition,
        direction: FindDirection,
    ) -> Option<LinePosition>;

    fn find_run_range(&self, pos: LinePosition) -> Option<RangeInclusive<LinePosition>> {
        let start = self.find_run_start(pos)?;
        let end = self.find_run_end(pos)?;

        Some(start..=end)
    }

    fn find_fill(
        &self,
        pos: LinePosition,
        fill: Fill,
        direction: FindDirection,
    ) -> Option<LinePosition>;

    fn find_first_non_blank_fill(
        &self,
        line: Line,
        direction: FindDirection,
    ) -> Option<LinePosition>;
}

impl<T> FillsFind for Grid<T>
where
    T: Value<Fill>,
{
    fn find_run_start(&self, line_pos: LinePosition) -> Option<LinePosition> {
        let LinePosition { line, pos } = line_pos;

        if pos >= self.line_len(line) {
            return None;
        }

        let Some(fill) = &self[line_pos].value() else {
            return None;
        };

        let start = self
            .iter_line(line)
            .enumerate()
            .take(pos + 1)
            .rev()
            .take_while(|(_, cell)| some_and_fill(cell.value(), **fill))
            .map(|(i, _)| i)
            .last()
            .map(|start| LinePosition::new(line, start));

        tracing::debug!("Start {start:?} found from {line_pos:?}");
        start
    }

    fn find_run_end(&self, line_pos: LinePosition) -> Option<LinePosition> {
        let LinePosition { line, pos } = line_pos;

        if pos >= self.line_len(line) {
            return None;
        }

        let Some(fill) = &self[line_pos].value() else {
            return None;
        };

        self.iter_line(line)
            .enumerate()
            .skip(pos)
            .take_while(|(_, cell)| none_or_fill(cell.value(), **fill))
            .map(|(i, _)| i)
            .last()
            .map(|end| LinePosition::new(line, end))
    }

    fn find_run_range(&self, pos: LinePosition) -> Option<RangeInclusive<LinePosition>> {
        let start = self.find_run_start(pos)?;
        let end = self.find_run_end(pos)?;

        Some(start..=end)
    }

    fn find_directed_run_start(
        &self,
        pos: LinePosition,
        direction: FindDirection,
    ) -> Option<LinePosition> {
        let start = self.find_run_start(pos)?;
        let end = self.find_run_end(pos)?;
        let len = self.line_len(pos.line);

        match direction {
            // Jump to the start of the current run if not already there
            FindDirection::Backwards if pos != start => Some(start),

            // Otherwise, find the start of the run just before the current one
            FindDirection::Backwards if pos == start => self.find_run_start(start - 1),

            // Jump one past the current end to reach the next run
            FindDirection::Forwards if end.pos < len - 1 => self.find_run_start(end + 1),

            // Avoid jumping past the last run; jump to its end
            FindDirection::Forwards if end.pos >= len - 1 => Some(end.with_pos(len - 1)),

            _ => None,
        }
    }

    fn find_directed_run_end(
        &self,
        pos: LinePosition,
        direction: FindDirection,
    ) -> Option<LinePosition> {
        let start = self.find_run_start(pos)?;
        let end = self.find_run_end(pos)?;

        match direction {
            // Jump to the end of the current run if not already there
            FindDirection::Forwards if pos != end => Some(end),

            // Otherwise, find the end of the run just after the current one
            FindDirection::Forwards if pos == end => self.find_run_end(end + 1).or(Some(end)),

            // Jump one past the current start to reach the previous run
            FindDirection::Backwards if start.pos > 0 => self.find_run_end(start - 1),

            // Avoid jumping past the first run; jump to its start
            FindDirection::Backwards if start.pos == 0 => Some(start.with_pos(0)),

            _ => None,
        }
    }

    /// Find the first ocurrence of a fill in the current line
    fn find_fill(
        &self,
        pos: LinePosition,
        fill: Fill,
        direction: FindDirection,
    ) -> Option<LinePosition> {
        let line = pos.line;
        let offset = pos.pos;

        let iter = self.iter_line(line).enumerate();
        let found = match direction {
            FindDirection::Forwards => iter
                .skip(offset)
                .find(|(_, cell)| some_and_fill(cell.value(), fill)),

            FindDirection::Backwards => iter
                .take(offset.saturating_sub(1))
                .rev()
                .find(|(_, cell)| some_and_fill(cell.value(), fill)),
        };

        found.map(|(idx, _)| LinePosition::new(line, idx))
    }

    fn find_first_non_blank_fill(
        &self,
        line: Line,
        direction: FindDirection,
    ) -> Option<LinePosition> {
        let mut iter = self.iter_line(line).enumerate();
        let non_blank = |cell: Option<&Fill>| cell.is_some();

        match direction {
            FindDirection::Forwards => iter.find(|(_, cell)| non_blank(cell.value())),
            FindDirection::Backwards => iter.rev().find(|(_, cell)| non_blank(cell.value())),
        }
        .map(|(idx, _)| LinePosition::new(line, idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzled_core::{Cell, Grid};
    use rstest::{fixture, rstest};
    use tracing_test::traced_test;

    fn line_pos(pos: usize) -> LinePosition {
        LinePosition::new(Line::Row(0), pos)
    }

    const B: Option<Fill> = None;
    const C: Option<Fill> = Some(Fill::Color(1));

    #[fixture]
    fn line_fills() -> Grid<Cell<Fill>> {
        //                       0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18
        let cells: Vec<_> = vec![C, B, C, C, B, C, B, C, C, C, B, B, B, C, B, B, C, C, B]
            .into_iter()
            .map(Cell::new)
            .collect();
        let cols = cells.len();

        Grid::from_vec(cells, cols).expect("Single row")
    }

    #[rstest]
    #[case(1, Some(1))]
    #[case(2, Some(2))]
    #[case(3, Some(2))]
    #[case(4, Some(4))]
    #[case(5, Some(5))]
    #[case(6, Some(6))]
    #[case(7, Some(7))]
    #[case(8, Some(7))]
    #[case(9, Some(7))]
    #[case(10, Some(10))]
    #[case(11, Some(10))]
    #[case(12, Some(10))]
    #[case(13, Some(13))]
    #[case(14, Some(14))]
    #[case(15, Some(14))]
    #[case(16, Some(16))]
    #[case(17, Some(16))]
    #[case(18, Some(18))]
    #[case(19, None)]
    #[case(40, None)]
    fn find_run_start(#[case] offset: usize, #[case] expected: Option<usize>) {
        let pos = line_pos(offset);
        let start = line_fills().find_run_start(pos).map(|p| p.pos);

        assert_eq!(start, expected);
    }

    #[rstest]
    #[case(1, Some(1))]
    #[case(2, Some(3))]
    #[case(3, Some(3))]
    #[case(4, Some(4))]
    #[case(5, Some(5))]
    #[case(6, Some(6))]
    #[case(7, Some(9))]
    #[case(8, Some(9))]
    #[case(9, Some(9))]
    #[case(10, Some(12))]
    #[case(11, Some(12))]
    #[case(12, Some(12))]
    #[case(13, Some(13))]
    #[case(14, Some(15))]
    #[case(15, Some(15))]
    #[case(16, Some(17))]
    #[case(17, Some(17))]
    #[case(18, Some(18))]
    #[case(19, None)]
    #[case(40, None)]
    fn find_run_end(#[case] offset: usize, #[case] expected: Option<usize>) {
        let pos = line_pos(offset);
        let start = line_fills().find_run_end(pos).map(|p| p.pos);

        assert_eq!(start, expected);
    }

    #[rstest]
    #[case(1, Some(2))]
    #[case(2, Some(4))]
    #[case(3, Some(4))]
    #[case(4, Some(5))]
    #[case(5, Some(6))]
    #[case(6, Some(7))]
    #[case(7, Some(10))]
    #[case(8, Some(10))]
    #[case(9, Some(10))]
    #[case(10, Some(13))]
    #[case(11, Some(13))]
    #[case(12, Some(13))]
    #[case(13, Some(14))]
    #[case(14, Some(16))]
    #[case(15, Some(16))]
    #[case(16, Some(18))]
    #[case(17, Some(18))]
    #[case(18, Some(18))]
    #[case(19, None)]
    #[case(40, None)]
    fn find_forward_run_start(#[case] offset: usize, #[case] expected: Option<usize>) {
        let pos = line_pos(offset);
        let start = line_fills()
            .find_directed_run_start(pos, FindDirection::Forwards)
            .map(|p| p.pos);

        assert_eq!(start, expected);
    }

    #[rstest]
    #[case(1, Some(0))]
    #[case(2, Some(1))]
    #[case(3, Some(2))]
    #[case(4, Some(2))]
    #[case(5, Some(4))]
    #[case(6, Some(5))]
    #[case(7, Some(6))]
    #[case(8, Some(7))]
    #[case(9, Some(7))]
    #[case(10, Some(7))]
    #[case(11, Some(10))]
    #[case(12, Some(10))]
    #[case(13, Some(10))]
    #[case(14, Some(13))]
    #[case(15, Some(14))]
    #[case(16, Some(14))]
    #[case(17, Some(16))]
    #[case(18, Some(16))]
    #[case(19, None)]
    #[case(40, None)]
    fn find_backward_run_start(#[case] offset: usize, #[case] expected: Option<usize>) {
        let pos = line_pos(offset);
        let start = line_fills()
            .find_directed_run_start(pos, FindDirection::Backwards)
            .map(|p| p.pos);

        assert_eq!(start, expected);
    }

    #[rstest]
    #[case(1, Some(3))]
    #[case(2, Some(3))]
    #[case(3, Some(4))]
    #[case(4, Some(5))]
    #[case(5, Some(6))]
    #[case(6, Some(9))]
    #[case(7, Some(9))]
    #[case(8, Some(9))]
    #[case(9, Some(12))]
    #[case(10, Some(12))]
    #[case(11, Some(12))]
    #[case(12, Some(13))]
    #[case(13, Some(15))]
    #[case(14, Some(15))]
    #[case(15, Some(17))]
    #[case(16, Some(17))]
    #[case(17, Some(18))]
    #[case(18, Some(18))]
    #[case(19, None)]
    #[case(40, None)]
    fn find_forward_run_end(#[case] offset: usize, #[case] expected: Option<usize>) {
        let pos = line_pos(offset);
        let start = line_fills()
            .find_directed_run_end(pos, FindDirection::Forwards)
            .map(|p| p.pos);

        assert_eq!(start, expected);
    }

    #[traced_test]
    #[rstest]
    #[case(1, Some(0))]
    #[case(2, Some(1))]
    #[case(3, Some(1))]
    #[case(4, Some(3))]
    #[case(5, Some(4))]
    #[case(6, Some(5))]
    #[case(7, Some(6))]
    #[case(8, Some(6))]
    #[case(9, Some(6))]
    #[case(10, Some(9))]
    #[case(11, Some(9))]
    #[case(12, Some(9))]
    #[case(13, Some(12))]
    #[case(14, Some(13))]
    #[case(15, Some(13))]
    #[case(16, Some(15))]
    #[case(17, Some(15))]
    #[case(18, Some(17))]
    #[case(19, None)]
    #[case(40, None)]
    fn find_backward_run_end(#[case] offset: usize, #[case] expected: Option<usize>) {
        let pos = line_pos(offset);
        let start = line_fills()
            .find_directed_run_end(pos, FindDirection::Backwards)
            .map(|p| p.pos);

        assert_eq!(start, expected);
    }
}
