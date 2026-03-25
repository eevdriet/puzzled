use puzzled_crossword::{Clue, ClueDirection, CluesSolveState};
use puzzled_tui::{
    AppCommand, AppResolver, EventMode, ListRender, ListWidget, Widget as AppWidget,
};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{List, ListItem},
};

use crate::{CrosswordApp, PuzzleScreenState};

pub struct CluesListWidget {
    list: ListWidget<CrosswordApp, CluesListRender>,
}

impl CluesListWidget {
    pub fn new(dir: Option<ClueDirection>) -> Self {
        let render = CluesListRender { dir };
        let list = ListWidget::new(render).highlight_symbol(">> ");

        Self { list }
    }
}

impl AppWidget<CrosswordApp> for CluesListWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.list.render(area, buf, state);
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

pub struct CluesListRender {
    dir: Option<ClueDirection>,
}

impl ListRender<CrosswordApp> for CluesListRender {
    type State = PuzzleScreenState;

    fn render_list<'a>(&self, list: List<'a>, state: &'a Self::State) -> List<'a> {
        let base_style = Style::default();
        let selected_style = base_style.fg(Color::Yellow).bold();

        let highlight_style = if self.dir == state.clue_dir {
            selected_style
        } else {
            base_style
        };

        list.highlight_style(highlight_style)
    }

    fn render_items<'a>(&self, state: &'a Self::State) -> impl Iterator<Item = ListItem<'a>> {
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

            let clue_text = if state.popup.is_some() {
                format!("{:<width$}", "...", width = clue.text().len())
            } else {
                clue.text().to_owned()
            };

            let style = clue_style(clue, state);
            let text = Text::styled(format!("{clue_id} {clue_text}"), style);

            ListItem::new(text)
        })
    }
}

fn clue_style(clue: &Clue, state: &PuzzleScreenState) -> Style {
    let mut base = Style::default();
    let iter = state.solve.entries.iter_clue(clue);
    let entries = iter.clone().filter_map(|entry| entry.entry());

    // No filled squares or paused -> apply no styling
    if entries.count() == 0 || state.popup.is_some() {
        return base;
    }

    // All filled squares -> strikethrough
    let all_filled = iter.clone().all(|entry| entry.is_filled());
    if all_filled {
        base = base.fg(Color::DarkGray).crossed_out().dim();
    }

    // Incorrect -> red
    if all_filled && iter.clone().all(|entry| entry.is_incorrect()) {
        base = base.fg(Color::Red);
    }

    // Revealed -> blue
    if all_filled && iter.clone().all(|entry| entry.is_revealed()) {
        base = base.fg(Color::Blue);
    }

    // Otherwise don't apply styling either
    base
}
