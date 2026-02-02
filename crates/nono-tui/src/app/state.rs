use std::time::Instant;

use nono::{Axis, Fill, Puzzle, Rules};
use ratatui::layout::Position as AppPosition;

use crate::{Action, ActionInput, Focus, PuzzleState, PuzzleStyle, RuleState, Selection, Settings};

#[derive(Debug)]
pub struct AppState {
    // Common
    pub settings: Settings,
    pub focus: Focus,
    pub start_time: Instant,

    // Widget specific
    pub puzzle: PuzzleState,
    pub rules_left: RuleState,
    pub rules_top: RuleState,
}

impl AppState {
    pub fn new(puzzle: Puzzle, rules: Rules, style: PuzzleStyle, settings: Settings) -> Self {
        let start_fill = Fill::Color(1);

        Self {
            settings,
            puzzle: PuzzleState::new(puzzle, style, start_fill),
            focus: Focus::default(),
            start_time: Instant::now(),
            rules_left: RuleState::new(rules.rows.clone(), Axis::Row),
            rules_top: RuleState::new(rules.cols.clone(), Axis::Col),
        }
    }

    pub fn selection(&self) -> Selection {
        match self.focus {
            Focus::RulesLeft => self.rules_left.selection,
            Focus::RulesTop => self.rules_top.selection,
            _ => self.puzzle.selection,
        }
    }

    pub fn mut_selection(&mut self) -> &mut Selection {
        match self.focus {
            Focus::RulesLeft => &mut self.rules_left.selection,
            Focus::RulesTop => &mut self.rules_top.selection,
            _ => &mut self.puzzle.selection,
        }
    }

    pub fn cursor(&self) -> AppPosition {
        match self.focus {
            Focus::RulesLeft => self.rules_left.cursor,
            Focus::RulesTop => self.rules_top.cursor,
            _ => self.puzzle.cursor,
        }
    }

    pub fn mut_cursor(&mut self) -> &mut AppPosition {
        match self.focus {
            Focus::RulesLeft => &mut self.rules_left.cursor,
            Focus::RulesTop => &mut self.rules_top.cursor,
            _ => &mut self.puzzle.cursor,
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
