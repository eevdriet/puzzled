use std::ops::RangeInclusive;

use crate::{Fill, Line, LinePosition, Puzzle};

#[derive(Debug, Clone, Copy)]
pub enum FindDirection {
    Forwards,
    Backwards,
}

impl Puzzle {
    pub fn find_run_start(&self, pos: LinePosition) -> Option<LinePosition> {
        let line = pos.line;
        let offset = pos.offset;

        if offset >= self.line_len(line) {
            return None;
        }

        let fill = self[pos];

        let start = self
            .iter_line(line)
            .enumerate()
            .take(usize::from(offset + 1))
            .rev()
            .take_while(|(_, f)| **f == fill)
            .map(|(i, _)| i)
            .last()
            .map(|start| LinePosition::new(line, start as u16));

        tracing::debug!("Start {start:?} found from {pos:?}");
        start
    }

    pub fn find_run_end(&self, pos: LinePosition) -> Option<LinePosition> {
        let line = pos.line;
        let offset = pos.offset;

        if offset >= self.line_len(line) {
            return None;
        }

        let fill = self[pos];

        self.iter_line(line)
            .enumerate()
            .skip(usize::from(offset))
            .take_while(|(_, f)| **f == fill)
            .map(|(i, _)| i)
            .last()
            .map(|start| LinePosition::new(pos.line, start as u16))
    }

    pub fn find_run_range(&self, pos: LinePosition) -> Option<RangeInclusive<LinePosition>> {
        let start = self.find_run_start(pos)?;
        let end = self.find_run_end(pos)?;

        Some(start..=end)
    }

    pub fn find_directed_run_start(
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
            FindDirection::Forwards if end.offset < len - 1 => self.find_run_start(end + 1),

            // Avoid jumping past the last run; jump to its end
            FindDirection::Forwards if end.offset >= len - 1 => Some(end.with_offset(len - 1)),

            _ => None,
        }
    }

    pub fn find_directed_run_end(
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
            FindDirection::Backwards if start.offset > 0 => self.find_run_end(start - 1),

            // Avoid jumping past the first run; jump to its start
            FindDirection::Backwards if start.offset == 0 => Some(start.with_offset(0)),

            _ => None,
        }
    }

    /// Find the first ocurrence of a fill in the current line
    pub fn find_fill(
        &self,
        pos: LinePosition,
        fill: Fill,
        direction: FindDirection,
    ) -> Option<LinePosition> {
        let line = pos.line;
        let offset = pos.offset as usize;

        let iter = self.iter_line(line).enumerate();
        let found = match direction {
            FindDirection::Forwards => iter.skip(offset).find(|(_, f)| **f == fill),
            FindDirection::Backwards => iter
                .take(offset.saturating_sub(1))
                .rev()
                .find(|(_, f)| **f == fill),
        };

        found.map(|(idx, _)| LinePosition::new(line, idx as u16))
    }

    pub fn find_first_non_blank_fill(
        &self,
        line: Line,
        direction: FindDirection,
    ) -> Option<LinePosition> {
        let mut iter = self.iter_line(line).enumerate();
        let non_blank = |fill: &Fill| !matches!(fill, Fill::Blank);

        match direction {
            FindDirection::Forwards => iter.find(|(_, fill)| non_blank(fill)),
            FindDirection::Backwards => iter.rev().find(|(_, fill)| non_blank(fill)),
        }
        .map(|(idx, _)| LinePosition::new(line, idx as u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use tracing_test::traced_test;

    fn puzzle_from_fills(row: &[Fill]) -> Puzzle {
        Puzzle::new(1, row.len() as u16, row.to_vec()).unwrap()
    }

    fn line_pos(pos: u16) -> LinePosition {
        LinePosition::new(Line::Row(0), pos)
    }

    const B: Fill = Fill::Blank;
    const X: Fill = Fill::Cross;
    const C: Fill = Fill::Color(1);
    const C2: Fill = Fill::Color(2);

    #[fixture]
    fn line_puzzle() -> Puzzle {
        //               0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18
        let fills = vec![C, B, C, C, B, C, B, C, C, C, B, B, B, C, B, B, C, C, B];

        Puzzle::new(1, fills.len() as u16, fills).unwrap()
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
    fn find_run_start(#[case] offset: u16, #[case] expected: Option<u16>) {
        let pos = line_pos(offset);
        let start = line_puzzle().find_run_start(pos).map(|p| p.offset);

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
    fn find_run_end(#[case] offset: u16, #[case] expected: Option<u16>) {
        let pos = line_pos(offset);
        let start = line_puzzle().find_run_end(pos).map(|p| p.offset);

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
    fn find_forward_run_start(#[case] offset: u16, #[case] expected: Option<u16>) {
        let pos = line_pos(offset);
        let start = line_puzzle()
            .find_directed_run_start(pos, FindDirection::Forwards)
            .map(|p| p.offset);

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
    fn find_backward_run_start(#[case] offset: u16, #[case] expected: Option<u16>) {
        let pos = line_pos(offset);
        let start = line_puzzle()
            .find_directed_run_start(pos, FindDirection::Backwards)
            .map(|p| p.offset);

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
    fn find_forward_run_end(#[case] offset: u16, #[case] expected: Option<u16>) {
        let pos = line_pos(offset);
        let start = line_puzzle()
            .find_directed_run_end(pos, FindDirection::Forwards)
            .map(|p| p.offset);

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
    fn find_backward_run_end(#[case] offset: u16, #[case] expected: Option<u16>) {
        let pos = line_pos(offset);
        let start = line_puzzle()
            .find_directed_run_end(pos, FindDirection::Backwards)
            .map(|p| p.offset);

        assert_eq!(start, expected);
    }
}
