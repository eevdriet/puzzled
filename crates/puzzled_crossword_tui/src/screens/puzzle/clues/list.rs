use puzzled_crossword::ClueDirection;
use puzzled_tui::{
    ActionResolver, AppContext, Command, HandleCommand, ListRender, ListWidget, Motion, RenderSize,
};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{List, ListItem, StatefulWidget, StatefulWidgetRef},
};

use crate::{AppState, CrosswordAction, CrosswordMotion, PuzzleScreenState};

pub struct CluesListWidget {
    dir: Option<ClueDirection>,
}

impl CluesListWidget {
    pub fn new(dir: Option<ClueDirection>) -> Self {
        Self { dir }
    }
}

impl RenderSize<PuzzleScreenState> for CluesListWidget {
    fn render_size(&self, state: &PuzzleScreenState) -> Size {
        ListWidget::new(self).render_size(state)
    }
}

impl ListRender for CluesListWidget {
    type State = PuzzleScreenState;

    fn render_items(
        &self,
        state: &Self::State,
    ) -> impl Iterator<Item = ratatui::widgets::ListItem<'_>> {
        state.clues(self.dir).map(|clue| {
            let clue_text = match self.dir.is_some() {
                true => format!("{:>2} {}", clue.num(), clue.text()),
                false => format!("{:>2}{} {}", clue.num(), clue.direction(), clue.text()),
            };
            ListItem::new(clue_text)
        })
    }
}

impl StatefulWidgetRef for CluesListWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items = self.render_items(state);

        let base_style = Style::default();
        let selected_style = base_style.fg(Color::Yellow);

        let highlight_style = if self.dir == state.clue_dir {
            selected_style
        } else {
            base_style
        };

        List::new(items)
            .highlight_style(highlight_style)
            .highlight_symbol(">> ")
            .render(area, buf, state.clue_list(self.dir));
    }
}

impl HandleCommand<CrosswordMotion, CrosswordAction, AppState> for CluesListWidget {
    type State = PuzzleScreenState;

    fn handle_command(
        &mut self,
        command: Command<CrosswordMotion, CrosswordAction>,
        _resolver: ActionResolver<CrosswordMotion, CrosswordAction, AppState>,
        _ctx: &mut AppContext<AppState>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Motion { count, motion, .. } => {
                let count = count as u16;
                let list = state.clue_list(self.dir);

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
            _ => false,
        }
    }
}
