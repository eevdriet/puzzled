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
    list: ListWidget<CrosswordApp, CluesListRender>,
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

    fn render_size(&self, area: Rect, state: &Self::State) -> Size {
        self.list.render_size(area, state)
    }

    fn override_mode(&self) -> Option<EventMode> {
        self.list.override_mode()
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

impl ListRender<CrosswordApp> for CluesListRender {
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
        let max_num = state
            .clues(self.dir)
            .last()
            .map(|clue| clue.num())
            .unwrap_or(0);
        let max_width = max_num.to_string().len();

        state.clues(self.dir).map(move |clue| {
            let clue_id = if self.dir.is_some() {
                format!("{:>width$}", clue.num(), width = max_width)
            } else {
                format!(
                    "{:>width$} {}",
                    clue.num(),
                    clue.direction(),
                    width = max_width
                )
            };

            let clue_text = if state.is_paused {
                format!("{:<width$}", "...", width = clue.text().len())
            } else {
                clue.text().to_owned()
            };

            ListItem::new(format!("{clue_id} {clue_text}"))
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
