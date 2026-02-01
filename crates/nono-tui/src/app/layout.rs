use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{App, ComputeLayout};

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
        // Add 2 for border around puzzle and space between puzzle and rules
        let puzzle_size = self.state.puzzle.size();
        let rules_width = self.state.rules_left.width();
        let rules_height = self.state.rules_top.height();

        tracing::debug!("Rules width: {rules_width}");
        tracing::debug!("Rules height: {rules_height}");

        tracing::debug!("Puzzle size: {puzzle_size:?}");

        let max_cols = puzzle_size.width + rules_width;

        let col_perc = 90 * root.width / 100;
        let width = max_cols.min(col_perc);

        let [outer, _] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(width), Constraint::Min(0)])
            .areas(root);

        tracing::debug!("Outer: {outer:?}");

        // Try to display the full puzzle + top rules if possible, otherwise clamp to root
        let max_rows = puzzle_size.height + rules_height;
        let height = max_rows.min(root.height);

        let [inner] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(height)])
            .areas(outer);

        tracing::debug!("Inner: {inner:?}");

        // Try to display all row rules and fill the remainder with the puzzle
        // Otherwise, fill the space smartly based on both dimensions
        let width = rules_width.min(inner.width);

        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(width),
                Constraint::Max(puzzle_size.width),
            ])
            .areas(inner);

        tracing::debug!("Left: {left:?}");
        tracing::debug!("Right: {right:?}");

        // Similarly, try to display all column rules and fill the remainder with the puzzle
        let height = rules_height.min(right.height);

        let [rules_top_area, puzzle_area, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(height),
                Constraint::Max(puzzle_size.height),
                Constraint::Min(4),
            ])
            .areas(right);

        tracing::debug!("Rules top: {rules_top_area:?}");
        tracing::debug!("Puzzle: {puzzle_area:?}");

        // Finally, use the puzzle height to split into the left rules and info section
        let [info_area, rules_left_area, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(puzzle_area.height),
                Constraint::Length(footer_area.height),
            ])
            .areas(left);

        tracing::debug!("Rules left: {rules_left_area:?}");
        tracing::debug!("Info: {info_area:?}");

        self.state.puzzle.area = puzzle_area;
        self.state.puzzle.viewport = self.state.puzzle.create_viewport(puzzle_area);
        self.state.rules_top.area = rules_top_area;
        self.state.rules_left.area = rules_left_area;
        self.footer_area = footer_area;
    }
}
