use puzzled_crossword::ClueDirection;
use puzzled_tui::{
    AppCommand, AppResolver, EventMode, ListRender, ListWidget, Widget as AppWidget,
};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
};

use crate::{CrosswordApp, PuzzleScreenState};

pub struct CluesListWidget {
    list: ListWidget<CluesListRender, CrosswordApp>,
}

impl CluesListWidget {
    pub fn new(dir: Option<ClueDirection>) -> Self {
        let render = CluesListRender { dir };
        let list = ListWidget::new(render);

        Self { list }
    }
}

impl AppWidget<CrosswordApp> for CluesListWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        AppWidget::<CrosswordApp>::render(&mut self.list, area, buf, state);
    }

    fn render_size(&self, state: &Self::State) -> Size {
        AppWidget::<CrosswordApp>::render_size(&self.list, state)
    }

    fn override_mode(&self) -> Option<EventMode> {
        Some(EventMode::Normal)
    }

    fn on_command(
        &mut self,
        command: AppCommand<CrosswordApp>,
        resolver: AppResolver<CrosswordApp>,
        state: &mut Self::State,
    ) -> bool {
        let is_handled = self.list.on_command(command, resolver, state);

        if is_handled {
            state.update_cursor_from_clues();
            state.update_clues_from_cursor();
        }

        is_handled
    }
}

struct CluesListRender {
    dir: Option<ClueDirection>,
}

impl ListRender for CluesListRender {
    type State = PuzzleScreenState;

    fn render_list(&self, state: &Self::State) -> List<'_> {
        let base_style = Style::default();
        let selected_style = base_style.fg(Color::Yellow);

        let highlight_style = if self.dir == state.clue_dir {
            selected_style
        } else {
            base_style
        };

        List::default()
            .highlight_style(highlight_style)
            .highlight_symbol(">> ")
    }

    fn render_items(&self, state: &Self::State) -> impl Iterator<Item = ListItem<'_>> {
        state.clues(self.dir).map(|clue| {
            let clue_text = match (state.is_paused, self.dir.is_some()) {
                (true, _) => format!("{:>2} ...", clue.num()),
                (false, true) => format!("{:>2} {}", clue.num(), clue.text()),
                (false, false) => {
                    format!("{:>2}{} {}", clue.num(), clue.direction(), clue.text())
                }
            };
            ListItem::new(clue_text)
        })
    }

    fn render_state<'a>(&self, state: &'a mut Self::State) -> &'a mut ListState {
        match self.dir {
            Some(ClueDirection::Across) => &mut state.across,
            Some(ClueDirection::Down) => &mut state.down,
            None => &mut state.across_down,
        }
    }
}
