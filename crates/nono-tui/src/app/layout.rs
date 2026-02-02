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
        tracing::debug!("Root: {root:?}");

        // Determine how many columns it takes to display the full puzzle + rules
        let puzzle_size = self.state.puzzle.size();

        // Rules based their length on the run digits and spacing for status column/row
        let rules_width = self.state.rules_left.width() + 1;
        let rules_height = self.state.rules_top.height() + 3;

        let cell_width = self.state.puzzle.style.cell_width;
        let cell_height = self.state.puzzle.style.cell_height;

        // The width is the left rules + puzzle + offset rule + spacing
        let width = puzzle_size.width + rules_width + cell_width + 2;
        let width = root.width.min(width);

        // Calculate the offset to horizontally center the puzzle
        let center_width = root.width.saturating_sub(puzzle_size.width) / 2;
        let center_offset = center_width.saturating_sub(rules_width);

        tracing::debug!("Rules width: {rules_width}");
        tracing::debug!("Rules height: {rules_height}");

        tracing::debug!("Puzzle size: {puzzle_size:?}");

        let [_, outer, rules_top_overflow_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(center_offset),
                Constraint::Length(width),
                Constraint::Length(cell_width),
            ])
            .areas(root);

        tracing::debug!("Outer: {outer:?}");

        // Also Calculate the offset to vertically center the puzzle
        let center_height = root.height.saturating_sub(puzzle_size.height) / 2;
        let center_offset = center_height.saturating_sub(rules_height);

        // Try to display the full puzzle + top rules if possible, otherwise clamp to root
        let height = puzzle_size.height + rules_height + cell_height + FOOTER_HEIGHT;
        let height = root.height.min(height);

        let [_, inner] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(center_offset),
                Constraint::Length(height),
            ])
            .areas(outer);

        tracing::debug!("Inner: {inner:?}");

        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(rules_width),
                Constraint::Length(puzzle_size.width),
            ])
            .areas(inner);

        tracing::debug!("Left: {left:?}");
        tracing::debug!("Right: {right:?}");

        // Similarly, try to display all column rules and fill the remainder with the puzzle
        let height = rules_height.min(right.height);

        let [rules_top_area, puzzle_area, _, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Max(puzzle_size.height),
                Constraint::Length(cell_height),
                Constraint::Min(FOOTER_HEIGHT),
            ])
            .areas(right);

        let [_, _, rules_left_overflow_area, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Max(puzzle_size.height),
                Constraint::Length(cell_height),
                Constraint::Min(FOOTER_HEIGHT),
            ])
            .areas(inner);

        tracing::debug!("Rules top: {rules_top_area:?}");
        tracing::debug!("Puzzle: {puzzle_area:?}");

        // Finally, use the puzzle height to split into the left rules and info section
        let [info_area, rules_left_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Length(puzzle_size.height),
            ])
            .areas(left);

        tracing::debug!("Rules left: {rules_left_area:?}");
        tracing::debug!("Info: {info_area:?}");

        self.state.puzzle.area = puzzle_area;
        self.state.puzzle.viewport = self.state.puzzle.create_viewport(puzzle_area);

        self.state.rules_top.area = rules_top_area;
        self.state.rules_top.overflow_area = rules_top_overflow_area;

        self.state.rules_left.area = rules_left_area;
        self.state.rules_left.overflow_area = rules_left_overflow_area;

        self.footer_area = footer_area;
    }
}
