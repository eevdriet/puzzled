mod clues;
mod crossword;
mod state;

pub use clues::*;
pub use crossword::*;
use puzzled_core::Direction;
pub use state::*;

use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, VerticalAlignment},
    prelude::{Buffer, Rect},
    widgets::{ListState, StatefulWidgetRef},
};

use puzzled_crossword::{ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{
    Action, ActionBehavior, ActionResolver, AppContext, Command, CommandHistory, EventMode,
    FocusManager, GridRenderState, HandleCommand, RenderSize, StatefulScreen, align_area,
};

use crate::{AppState, CrosswordAction, CrosswordMotion};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Focus {
    #[default]
    Crossword,

    AcrossClues,
    DownClues,
    Footer,
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
        let mut focus = FocusManager::from_mode_nodes([
            (Focus::AcrossClues, EventMode::Normal),
            (Focus::DownClues, EventMode::Normal),
        ]);

        focus.link_right(Focus::Crossword, &[Focus::AcrossClues]);
        focus.link_right(Focus::AcrossClues, &[Focus::DownClues]);
        focus.link_below(Focus::Footer, &[Focus::AcrossClues, Focus::DownClues]);

        let list = ListState::default().with_selected(Some(0));
        let mut state = PuzzleScreenState {
            puzzle,
            solve: solve_state,
            render: render_state,
            across: list,
            down: list,
            focus,
        };
        state.update_clues_from_cursor();

        Self {
            state,
            crossword: CrosswordWidget,
            across_clues: CluesWidget::new(ClueDirection::Across, Focus::AcrossClues),
            down_clues: CluesWidget::new(ClueDirection::Down, Focus::DownClues),

            commands: CommandHistory::default(),
        }
    }
}

impl StatefulScreen<CrosswordMotion, CrosswordAction, AppState> for PuzzleScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _state: &mut AppContext<AppState>) {
        let gap = Constraint::Length(2);

        // Crossword on the left
        let [crossword, _, right] = Layout::horizontal(vec![
            Constraint::Length(self.crossword.render_size(&self.state).width),
            gap,
            Constraint::Min(0),
        ])
        .areas(area);

        // Render widgets
        let crossword_size = self.crossword.render_size(&self.state);
        let crossword = align_area(
            crossword,
            crossword_size,
            HorizontalAlignment::Center,
            VerticalAlignment::Top,
        );

        self.crossword.render_ref(crossword, buf, &mut self.state);

        // Clues on the right
        let [across_clues, _, down_clues] =
            Layout::horizontal(vec![Constraint::Fill(1), gap, Constraint::Fill(1)]).areas(right);

        self.across_clues
            .render_ref(across_clues, buf, &mut self.state);
        self.down_clues.render_ref(down_clues, buf, &mut self.state);
    }

    fn on_command(
        &mut self,
        command: Command<CrosswordMotion, CrosswordAction>,
        resolver: ActionResolver<CrosswordMotion, CrosswordAction, AppState>,
        ctx: &mut AppContext<AppState>,
    ) -> bool {
        tracing::info!("Command reached crossword: {command:?}");

        if let Some(action) = command.action() {
            match action {
                // Lifetime actions
                Action::Cancel => resolver.prev_screen(),
                Action::Quit => resolver.quit(),
                Action::Undo => self.commands.undo(&mut self.state.solve),
                Action::Redo => self.commands.redo(&mut self.state.solve),

                // Focus change actions
                action if action.is_focus() => {
                    let changed_focus =
                        self.state
                            .focus
                            .on_command(command, resolver, &mut ctx.mode);

                    // Update the grid direction to reflect the active clue widget
                    if changed_focus {
                        match self.state.focus.current() {
                            Focus::AcrossClues => {
                                self.state.render.direction = ClueDirection::Across.into();
                            }
                            Focus::DownClues => {
                                self.state.render.direction = ClueDirection::Down.into();
                            }
                            _ => {}
                        }
                    }

                    return changed_focus;
                }
                _ => {}
            }
        }

        match self.state.focus.current() {
            Focus::Crossword => self
                .crossword
                .on_command(command, resolver, &mut self.state),
            Focus::AcrossClues => self
                .across_clues
                .on_command(command, resolver, &mut self.state),
            Focus::DownClues => self
                .down_clues
                .on_command(command, resolver, &mut self.state),
            Focus::Footer => self
                .crossword
                .on_command(command, resolver, &mut self.state),
        }
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<AppState>) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<AppState>) {
        self.state.solve.timer.start();
    }
}
