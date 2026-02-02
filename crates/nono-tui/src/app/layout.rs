use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{App, ComputeLayout};

const FOOTER_HEIGHT: u16 = 4;

impl ComputeLayout for App {
    /*
    ┌────── [O] Outer ────────────────────────────────────────────────────────────────────────┐
    │┌────── [I] Inner ───────────┬── Max 70% or P + RR width ────────────┐                   │
    ││                            │                                       │ Min 30%           │
    ││                            │                                       │                   │
    ││     App information        │           [RC] Column rules           │  [H] Help         │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││────────────────────────────┼───────────────────────────────────────┤                   │
    ││                            │ Prefer fitting RR/RC fully, otherwise │                   │
    ││                            │ set preferred size through config     │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││      [RR] Row rules        │              [P] Puzzle               │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    ││                            │                                       │                   │
    │└────────────────────────────┴───────────────────────────────────────┘                   │
    └─────────────────────────────────────────────────────────────────────────────────────────┘
    */

    fn compute_layout(&mut self, root: Rect) {
        // Determine how many columns it takes to display the full puzzle + rules
        let puzzle_size = self.state.puzzle.size();

        // Rules based their length on the run digits and spacing for status column/row
        let rules_width = self.state.rules_left.width();
        let rules_height = self.state.rules_top.height();

        let max_rules_width = (rules_width + 3).max(15).min(root.width / 4);
        let max_rules_height = (rules_height + 3).max(15).min(4 * root.height / 10);

        tracing::debug!("Ruleshhh: {max_rules_height}");

        let cell_width = self.state.puzzle.style.cell_width;
        let cell_height = self.state.puzzle.style.cell_height;

        // The width is the left rules + puzzle + offset rule + spacing
        let width = puzzle_size.width + max_rules_width + cell_width;
        let width = root.width.min(width);

        // Calculate the offset to horizontally center the puzzle
        let center_width = root.width.saturating_sub(puzzle_size.width) / 2;
        let center_offset = center_width.saturating_sub(max_rules_width);

        // Calculate how much space is needed to display the longest overflowing column rule
        let overflow_height = max_rules_height + puzzle_size.height;
        let overflow_count = (rules_height as f64 / overflow_height as f64).ceil() as u16;
        let overflow_top = overflow_count * cell_width;

        let [_, outer, _, rules_top_overflow_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(center_offset),
                Constraint::Length(width),
                Constraint::Length(1),
                Constraint::Length(overflow_top),
            ])
            .areas(root);

        // Also Calculate the offset to vertically center the puzzle
        let center_height = root.height.saturating_sub(puzzle_size.height) / 2;
        let center_offset = center_height.saturating_sub(max_rules_height);

        // Try to display the full puzzle + top rules if possible, otherwise clamp to root
        let height = puzzle_size.height + max_rules_height + cell_height + FOOTER_HEIGHT;
        let height = root.height.min(height);

        let [_, inner] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(center_offset),
                Constraint::Length(height),
            ])
            .areas(outer);

        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(max_rules_width),
                Constraint::Length(puzzle_size.width),
            ])
            .areas(inner);

        // Similarly, try to display all column rules and fill the remainder with the puzzle
        let height = max_rules_height.min(right.height);

        let [rules_top_area, puzzle_area, _, _, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Max(puzzle_size.height),
                Constraint::Length(1),
                Constraint::Length(cell_height),
                Constraint::Min(FOOTER_HEIGHT),
            ])
            .areas(right);

        // Calculate how much space is needed to display the longest overflowing row rule
        let overflow_count = (rules_width as f64 / inner.width as f64).ceil() as u16;
        let overflow_left = overflow_count * cell_height;

        let [_, _, _, rules_left_overflow_area, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Max(puzzle_size.height),
                Constraint::Length(1),
                Constraint::Length(overflow_left),
                Constraint::Min(FOOTER_HEIGHT),
            ])
            .areas(inner);

        // Finally, use the puzzle height to split into the left rules and info section
        let [info_area, rules_left_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Length(puzzle_size.height),
            ])
            .areas(left);

        tracing::debug!("Info                  : {info_area:?}");
        tracing::debug!("Inner                 : {inner:?}");
        tracing::debug!("Left                  : {left:?}");
        tracing::debug!("Outer                 : {outer:?}");
        tracing::debug!("Puzzle size           : {puzzle_size:?}");
        tracing::debug!("Puzzle                : {puzzle_area:?}");
        tracing::debug!("Right                 : {right:?}");
        tracing::debug!("Root                  : {root:?}");
        tracing::debug!("Rules height          : {max_rules_height}");
        tracing::debug!("Rules left            : {rules_left_area:?}");
        tracing::debug!("Rules left (overflow) : {rules_left_overflow_area:?}");
        tracing::debug!("Rules top             : {rules_top_area:?}");
        tracing::debug!("Rules top (overflow)  : {rules_top_overflow_area:?}");
        tracing::debug!("Rules width           : {max_rules_width}");

        self.state.puzzle.area = puzzle_area;
        self.state.puzzle.viewport = self.state.puzzle.create_viewport(puzzle_area);

        self.state.rules_top.area = rules_top_area;
        self.state.rules_top.overflow_area = rules_top_overflow_area;

        self.state.rules_left.area = rules_left_area;
        self.state.rules_left.overflow_area = rules_left_overflow_area;

        self.footer_area = footer_area;
    }
}
