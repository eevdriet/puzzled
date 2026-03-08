mod clues;
mod crossword;

pub use clues::*;
pub use crossword::*;

use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    widgets::StatefulWidgetRef,
};

use puzzled_crossword::{ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{
    Action, ActionBehavior, ActionResolver, AppEvent, CommandHistory, FocusManager,
    GridRenderState, HandleAction, RenderSize, StatefulScreen, clamp_area,
};

use crate::{AppState, CrosswordAction};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Focus {
    #[default]
    Crossword,

    AcrossClues,
    DownClues,
    Footer,
}

pub struct PuzzleScreenState {
    // Solve state
    puzzle: Crossword,
    solve: CrosswordState,
    render: GridRenderState,

    // UI state
    focus: FocusManager<Focus>,
}

pub struct PuzzleScreen {
    state: PuzzleScreenState,

    // Widgets
    crossword: CrosswordWidget,
    across_clues: CluesWidget,
    down_clues: CluesWidget,

    // Other
    commands: CommandHistory<CrosswordState>,
}

impl PuzzleScreen {
    pub fn new(
        puzzle: Crossword,
        solve_state: CrosswordState,
        render_state: GridRenderState,
    ) -> Self {
        let mut focus = FocusManager::default();
        focus.link_right(Focus::Crossword, &[Focus::AcrossClues]);
        focus.link_right(Focus::AcrossClues, &[Focus::DownClues]);
        focus.link_below(Focus::Footer, &[Focus::AcrossClues, Focus::DownClues]);

        Self {
            state: PuzzleScreenState {
                puzzle,
                solve: solve_state,
                render: render_state,
                focus: FocusManager::default(),
            },

            crossword: CrosswordWidget,
            across_clues: CluesWidget::new(ClueDirection::Across),
            down_clues: CluesWidget::new(ClueDirection::Down),

            commands: CommandHistory::default(),
        }
    }
}

impl StatefulScreen<CrosswordAction, AppState> for PuzzleScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _state: &mut AppState) {
        let gap = Constraint::Length(2);

        // Crossword on the left
        let [crossword, _, right] = Layout::horizontal(vec![
            Constraint::Length(self.crossword.render_size(&self.state).width),
            gap,
            Constraint::Min(0),
        ])
        .areas(area);

        // Render widgets
        let crossword = clamp_area(crossword, self.crossword.render_size(&self.state));
        self.crossword.render_ref(crossword, buf, &mut self.state);

        // Clues on the right
        let [across_clues, _, down_clues] =
            Layout::horizontal(vec![Constraint::Fill(1), gap, Constraint::Fill(1)]).areas(right);

        self.across_clues
            .render_ref(across_clues, buf, &mut self.state);
        self.down_clues.render_ref(down_clues, buf, &mut self.state);
    }

    fn on_action(
        &mut self,
        action: Action<CrosswordAction>,
        resolver: ActionResolver<CrosswordAction, AppState>,
        _state: &mut AppState,
    ) {
        match action {
            // Lifetime actions
            Action::Cancel => resolver.prev_screen(),
            Action::Quit => resolver.quit(),
            Action::Undo => self.commands.undo(&mut self.state.solve),
            Action::Redo => self.commands.redo(&mut self.state.solve),

            // Focus change actions
            action if action.is_focus() => self.state.focus.on_action(action, resolver, &mut ()),

            // Widget actions
            action => match self.state.focus.current() {
                Focus::Crossword => self.crossword.on_action(action, resolver, &mut self.state),
                Focus::AcrossClues => {
                    self.across_clues
                        .on_action(action, resolver, &mut self.state)
                }
                Focus::DownClues => self.down_clues.on_action(action, resolver, &mut self.state),
                Focus::Footer => self.crossword.on_action(action, resolver, &mut self.state),
            },
        }
    }

    fn on_event(
        &mut self,
        _event: AppEvent,
        _resolver: ActionResolver<CrosswordAction, AppState>,
        _state: &mut AppState,
    ) {
    }

    fn on_pause(&mut self, _state: &mut AppState) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _state: &mut AppState) {
        self.state.solve.timer.start();
    }
}
