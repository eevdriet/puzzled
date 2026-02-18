mod focus;
mod layout;
mod load;
mod mode;
mod pos;
mod selection;
mod state;

pub use focus::*;
pub use load::*;
pub use mode::*;
pub use pos::*;
pub use selection::*;
pub use state::*;

use crossterm::{
    event::{self as t_event, EnableMouseCapture, Event},
    execute,
    terminal::EnterAlternateScreen,
};
use puzzled_nono::{Puzzle, Rules, Solver};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Margin, Position, Rect},
    style::{Color, Style},
    widgets::{FrameExt, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use std::time::Duration;

use crate::{
    ActionEngine, ActionInput, ActionOutcome, ActionResult, AppEvent, ColRulesWidget,
    ComputeLayout, Config, EventEngine, FooterWidget, HandleAction, MiniMapWidget, PuzzleStyle,
    PuzzleWidget, Result, RowRulesWidget,
};

const POLL_DURATION: Duration = Duration::from_millis(30);
const TICK_DURATION: Duration = Duration::from_millis(200);

pub struct App {
    // State
    pub state: AppState,
    pub solver: Solver,

    // Input
    pub events: EventEngine,
    pub actions: ActionEngine,

    // Widgets
    puzzle_widget: PuzzleWidget,
    rules_left: RowRulesWidget,
    rules_top: ColRulesWidget,
    footer: FooterWidget,
    minimap: MiniMapWidget,
}

impl App {
    pub fn new(puzzle: Puzzle, rules: Rules, style: PuzzleStyle, config: Config) -> Self {
        let rules_left = RowRulesWidget::new("Rules [Rows]".to_string(), rules.rows.clone());
        let rules_top = ColRulesWidget::new("Rules [Cols]".to_string(), rules.cols.clone());

        let state = AppState::new(puzzle, rules, style, config.settings);
        let events = EventEngine::new(config.actions.clone(), TICK_DURATION);

        Self {
            state,
            events,
            actions: ActionEngine::default(),

            solver: Solver::default(),
            puzzle_widget: PuzzleWidget,
            rules_left,
            rules_top,
            footer: FooterWidget,
            minimap: MiniMapWidget,
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        self.init()?;

        loop {
            // Render
            {
                term.draw(|frame| {
                    self.compute_layout(frame.area());
                    self.render(frame)
                })?;
            }

            // Poll for events
            if t_event::poll(POLL_DURATION)? {
                // Read the terminal event
                let event = t_event::read()?;
                let app_event = AppEvent::new(event);

                // See whether the application handles it and whether it needs action
                if let Some(input) = self.events.push(app_event.clone()) {
                    let status = self.handle_with_engine(input)?;

                    if matches!(status, ActionOutcome::Exit) {
                        break;
                    }
                }
            }

            if let Some(input) = self.events.tick() {
                let status = self.handle_with_engine(input)?;
                if matches!(status, ActionOutcome::Exit) {
                    break;
                }
            }
        }

        self.exit()
    }

    fn handle_with_engine(&mut self, input: ActionInput) -> ActionResult {
        let focus = self.resolve_focus(&input);

        let outcome = match focus {
            Focus::Puzzle => {
                self.actions
                    .handle_action_with(&self.puzzle_widget, input.clone(), &mut self.state)
            }
            Focus::RulesLeft => {
                self.actions
                    .handle_action_with(&self.rules_left, input.clone(), &mut self.state)
            }
            Focus::RulesTop => {
                self.actions
                    .handle_action_with(&self.rules_top, input.clone(), &mut self.state)
            }
            Focus::Footer => {
                self.actions
                    .handle_action_with(&self.footer, input.clone(), &mut self.state)
            }
        }?;

        // If a focus change is requested,
        if matches!(
            outcome,
            ActionOutcome::RequestFocus | ActionOutcome::LoseFocus
        ) {
            self.state.switch_focus(input);
        }

        Ok(outcome)
    }

    fn resolve_focus(&self, input: &ActionInput) -> Focus {
        if let Event::Mouse(mouse) = *input.event {
            let pos = Position::new(mouse.column, mouse.row);

            if self.state.puzzle.area.contains(pos) {
                return Focus::Puzzle;
            }
            if self.state.rules_left.area.contains(pos) {
                return Focus::RulesLeft;
            }
            if self.state.rules_top.area.contains(pos) {
                return Focus::RulesTop;
            }
            if self.state.footer.area.contains(pos) {
                return Focus::Footer;
            }
        }

        self.state.focus
    }

    fn init(&self) -> Result<()> {
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        Ok(())
    }

    fn exit(&self) -> Result<()> {
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.render_stateful_widget_ref(
            &self.puzzle_widget,
            self.state.puzzle.area,
            &mut self.state,
        );
        frame.render_stateful_widget_ref(
            &self.rules_left,
            self.state.rules_left.area,
            &mut self.state,
        );
        frame.render_stateful_widget_ref(
            &self.rules_top,
            self.state.rules_top.area,
            &mut self.state,
        );

        frame.render_stateful_widget_ref(&self.footer, self.state.footer.area, &mut self.state);
        frame.render_stateful_widget_ref(&self.minimap, self.state.minimap.area, &mut self.state);
    }

    fn draw_puzzle_scrollbars(&mut self, frame: &mut Frame, area: Rect) {
        // Common properties for both scrollbars
        let style = Style::default().fg(Color::Gray);
        let vp = &self.state.puzzle.viewport;

        // Display scrollbar to scroll through puzzle rows
        let rows = self.state.puzzle.puzzle.rows() as usize;
        let visible_rows = vp.visible_rows() as usize;
        let row = self.state.puzzle.scroll.row as usize;

        if rows > visible_rows {
            let scroll_rows_bar = Scrollbar::new(ScrollbarOrientation::VerticalLeft)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .thumb_symbol("#")
                .style(style);

            let mut scroll_rows_state = ScrollbarState::new(rows - visible_rows)
                .viewport_content_length(visible_rows)
                .position(row);

            frame.render_stateful_widget(
                scroll_rows_bar,
                area.inner(Margin::new(0, 1)),
                &mut scroll_rows_state,
            );
        }

        // Display scrollbar to scroll through puzzle columns
        let cols = self.state.puzzle.puzzle.cols() as usize;
        let visible_cols = vp.visible_cols() as usize;
        let col = self.state.puzzle.scroll.col as usize;

        if cols > visible_cols {
            let scroll_cols_bar = Scrollbar::new(ScrollbarOrientation::HorizontalTop)
                .begin_symbol(Some("←"))
                .end_symbol(Some("→"))
                .thumb_symbol("#")
                .style(style);

            let mut scroll_cols_state = ScrollbarState::new(cols - visible_cols)
                .viewport_content_length(visible_cols)
                .position(col);

            frame.render_stateful_widget(
                scroll_cols_bar,
                area.inner(Margin::new(1, 0)),
                &mut scroll_cols_state,
            );
        }
    }

    pub fn actions(&self) -> &ActionEngine {
        &self.actions
    }
}

impl HandleAction for &mut App {
    fn handle_action(&self, _input: ActionInput, _: &mut AppState) -> Result<ActionOutcome> {
        Ok(ActionOutcome::Consumed)
    }
}
