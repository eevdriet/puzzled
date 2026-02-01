use nono::{Axis, Fill, Puzzle, Rules};
use ratatui::layout::Position as AppPosition;

use crate::{Action, ActionInput, Focus, PuzzleState, PuzzleStyle, RuleState, Selection, Settings};

#[derive(Debug)]
pub struct AppState {
    // Common
    pub settings: Settings,
    pub focus: Focus,

    // Widget specific
    pub puzzle: PuzzleState,
    pub rules_left: RuleState,
    pub rules_top: RuleState,
}

impl AppState {
    pub fn new(puzzle: Puzzle, rules: Rules, style: PuzzleStyle, settings: Settings) -> Self {
        // let mut settings = settings;
        // settings.rule_display = RuleDisplay::TryMax;
        let start_fill = Fill::Color(1);

        Self {
            settings,
            puzzle: PuzzleState::new(puzzle, style, start_fill),
            focus: Focus::default(),
            rules_left: RuleState::new(rules.rows.clone(), Axis::Row),
            rules_top: RuleState::new(rules.cols.clone(), Axis::Col),
        }
    }

    pub fn selection(&mut self) -> &mut Selection {
        match self.focus {
            Focus::Puzzle => &mut self.puzzle.selection,
            _ => &mut self.rules_left.selection,
        }
    }

    pub fn cursor(&mut self) -> &mut AppPosition {
        match self.focus {
            Focus::Puzzle => &mut self.puzzle.cursor,
            _ => &mut self.rules_left.cursor,
        }
    }

    pub fn switch_focus(&mut self, input: ActionInput) {
        let action = input.action;
        let focus = match (self.focus, action) {
            (Focus::Footer, Action::FocusUp) => Focus::Puzzle,
            (Focus::Footer, Action::FocusLeft) => Focus::RulesLeft,
            // (Focus::Puzzle, Action::FocusDown) => Focus::Footer,
            (Focus::Puzzle, Action::FocusLeft) => Focus::RulesLeft,
            (Focus::Puzzle, Action::FocusUp) => Focus::RulesTop,
            (Focus::RulesLeft, Action::FocusRight) => Focus::Puzzle,
            (Focus::RulesLeft, Action::FocusUp) => Focus::RulesTop,
            // (Focus::RulesLeft, Action::FocusDown) => Focus::RulesTop,
            (Focus::RulesTop, Action::FocusDown) => Focus::Puzzle,
            (Focus::RulesTop, Action::FocusLeft) => Focus::RulesLeft,
            _ => {
                tracing::debug!(
                    "Unknown focus request encountered with action {action:?} (prev focus {:?})",
                    self.focus
                );

                self.focus
            }
        };

        tracing::debug!("Focus set to {focus:?} (prev focus {:?})", self.focus);
        self.focus = focus;
    }
}
