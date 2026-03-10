use puzzled_crossword::ClueDirection;
use puzzled_tui::{
    ActionResolver, Command, HandleCommand, ListRender, ListWidget, Motion, RenderSize,
};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, StatefulWidget, StatefulWidgetRef},
};

use crate::{AppState, CrosswordAction, CrosswordMotion, Focus, PuzzleScreenState};

pub struct CluesWidget {
    direction: ClueDirection,
    focus: Focus,
}

impl CluesWidget {
    pub fn new(direction: ClueDirection, focus: Focus) -> Self {
        Self { direction, focus }
    }
}

impl RenderSize<PuzzleScreenState> for CluesWidget {
    fn render_size(&self, state: &PuzzleScreenState) -> Size {
        let mut size = ListWidget::new(self).render_size(state);

        // Border
        size.width += 2;
        size.height += 2;

        size
    }
}

impl ListRender for CluesWidget {
    type State = PuzzleScreenState;

    fn render_items(
        &self,
        state: &Self::State,
    ) -> impl Iterator<Item = ratatui::widgets::ListItem<'_>> {
        state
            .puzzle
            .clues()
            .iter_direction(self.direction)
            .map(|clue| {
                let clue_text = format!("{:>2} {}", clue.num(), clue.text());
                ListItem::new(clue_text)
            })
    }
}

impl StatefulWidgetRef for CluesWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let clues = state.puzzle.clues();

        let items: Vec<_> = clues
            .iter_direction(self.direction)
            .map(|clue| format!("{:>2} {}", clue.num(), clue.text()))
            .collect();

        let mut highlight_style = Style::default();
        let curr_dir = ClueDirection::from(state.render.direction);

        if curr_dir == self.direction {
            highlight_style = highlight_style.fg(Color::Yellow).italic();
        }

        let border_style = if state.focus.current() == &self.focus {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let list = match self.direction {
            ClueDirection::Across => &mut state.across,
            ClueDirection::Down => &mut state.down,
        };

        List::new(items)
            .block(
                Block::bordered()
                    .border_style(border_style)
                    .title(format!(" {:?} ", self.direction)),
            )
            .highlight_style(highlight_style)
            .highlight_symbol(">> ")
            .render(area, buf, list);
    }
}

impl HandleCommand<CrosswordMotion, CrosswordAction, AppState> for CluesWidget {
    type State = PuzzleScreenState;

    fn on_command(
        &mut self,
        command: Command<CrosswordMotion, CrosswordAction>,
        _resolver: ActionResolver<CrosswordMotion, CrosswordAction, AppState>,
        state: &mut Self::State,
    ) -> bool {
        let count = command.count() as u16;
        let Some(motion) = command.motion() else {
            return false;
        };

        let list = state.list(self.direction);

        match motion {
            Motion::ColStart => list.select_first(),
            Motion::ColEnd => list.select_last(),
            Motion::Down => list.scroll_down_by(count),
            Motion::Up => list.scroll_up_by(count),
            _ => return false,
        }

        state.update_cursor_from_clues();
        true
    }
}
