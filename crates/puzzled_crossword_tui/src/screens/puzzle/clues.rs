use puzzled_crossword::ClueDirection;
use puzzled_tui::{Action, ActionResolver, Command, HandleCommand, RenderSize};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListState, StatefulWidget, StatefulWidgetRef},
};

use crate::{AppState, CrosswordAction, PuzzleScreenState};

pub struct CluesWidget {
    direction: ClueDirection,
}

impl CluesWidget {
    pub fn new(direction: ClueDirection) -> Self {
        Self { direction }
    }
}

impl RenderSize for CluesWidget {
    type State = PuzzleScreenState;

    fn render_size(&self, state: &Self::State) -> Size {
        let clues = state.puzzle.clues();
        let clue_count = clues.iter_direction(self.direction).count();

        Size::new(10, 10)
    }
}

impl StatefulWidgetRef for CluesWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let clues = state.puzzle.clues();

        let (nums, items): (Vec<_>, Vec<_>) = clues
            .iter_direction(self.direction)
            .map(|clue| (clue.num(), format!("{:>2} {}", clue.num(), clue.text())))
            .unzip();

        let mut list_state = ListState::default();

        if let Some((across, down)) = clues.get_clues(state.render.cursor) {
            let num = match self.direction {
                ClueDirection::Across => across.num(),
                ClueDirection::Down => down.num(),
            };

            if let Ok(idx) = nums.binary_search(&num) {
                list_state.select(Some(idx));
            }
        }

        let mut highlight_style = Style::default();
        let curr_dir = ClueDirection::from(state.render.direction);

        if curr_dir == self.direction {
            highlight_style = highlight_style.fg(Color::Yellow).italic();
        }

        List::new(items)
            .block(Block::bordered().title(format!(" {:?} ", self.direction)))
            .highlight_style(highlight_style)
            .highlight_symbol(">> ")
            .render(area, buf, &mut list_state);
    }
}

impl HandleCommand<CrosswordAction, AppState> for CluesWidget {
    type State = PuzzleScreenState;

    fn on_command(
        &mut self,
        _command: Command<CrosswordAction>,
        _resolver: ActionResolver<CrosswordAction, AppState>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}
