use nono::{Axis, Position, Rule};
use ratatui::layout::{Position as AppPosition, Rect};

use crate::{RuleDisplay, Selection, puzzle_to_app};

#[derive(Debug, Default)]
pub struct RuleState {
    pub rules: Vec<Rule>,

    pub display: RuleDisplay,

    pub cursor: AppPosition,

    pub selection: Selection,

    pub axis: Axis,

    pub area: Rect,
    pub overflow_area: Rect,
}

impl RuleState {
    pub fn new(rules: Vec<Rule>, axis: Axis) -> Self {
        Self {
            rules,
            axis,
            selection: Selection::empty(axis),
            ..Default::default()
        }
    }

    pub fn height(&self) -> u16 {
        self.rules
            .iter()
            .map(|rule: &Rule| rule.runs().len() as u16)
            .max()
            .unwrap_or_default()

        // match self.display {
        //     RuleDisplay::Auto => median(heights),
        //     _ => heights.iter().max().copied().unwrap_or_default(),
        // }
    }

    pub fn width(&self) -> u16 {
        self.rules
            .iter()
            .map(|rule: &Rule| {
                let runs = rule.runs();

                runs.len().saturating_sub(1) as u16
                    + runs
                        .iter()
                        .map(|run| run.count.to_string().len() as u16)
                        .sum::<u16>()
            })
            .max()
            .unwrap_or_default()
    }

    pub fn follow_puzzle_cursor(&mut self, cursor: Position) {
        let cursor = match self.axis {
            Axis::Row => {
                let row = cursor.row;
                let col = self.rules[row as usize].min_run(cursor.col);

                Position { row, col }
            }
            Axis::Col => {
                let col = cursor.col;
                let row = self.rules[col as usize].min_run(cursor.row);

                Position { row, col }
            }
        };

        self.cursor = puzzle_to_app(cursor);
    }
}

fn median(nums: Vec<u16>) -> u16 {
    let mut nums = nums;
    nums.sort();
    let mid = nums.len() / 2;

    nums[mid]
}
