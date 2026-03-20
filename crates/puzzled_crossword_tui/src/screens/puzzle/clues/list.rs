use crossterm::event::MouseEventKind;
use puzzled_crossword::ClueDirection;
use puzzled_tui::{
    Command, EventMode, ListRender, ListWidget, Motion, RenderSize, Widget as AppWidget,
};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{List, ListItem, StatefulWidget},
};

use crate::{
    AppState, CrosswordAction, CrosswordCommand, CrosswordMotion, CrosswordResolver,
    CrosswordTextObject, PuzzleScreenState,
};

pub struct CluesListWidget {
    dir: Option<ClueDirection>,
}

impl CluesListWidget {
    pub fn new(dir: Option<ClueDirection>) -> Self {
        Self { dir }
    }
}

impl AppWidget<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>
    for CluesListWidget
{
    type State = PuzzleScreenState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let visible = area.height as usize;
        let offset = state.clue_list(self.dir).offset();
        let render_state = ListRenderState {
            state,
            visible,
            offset,
            is_paused: state.is_paused,
        };

        let items = self.render_items(&render_state);

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

    fn render_size(&self, state: &Self::State) -> Size {
        let render_state = ListRenderState {
            state,
            offset: 0,
            visible: 0,
            is_paused: false,
        };

        ListWidget::new(self).render_size(&render_state)
    }

    fn override_mode(&self) -> Option<EventMode> {
        Some(EventMode::Normal)
    }

    fn on_command(
        &mut self,
        command: CrosswordCommand,
        _resolver: CrosswordResolver,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Motion { count, motion, .. } => {
                let count = count as u16;
                let list = state.clue_list(self.dir);

                match motion {
                    Motion::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollDown => list.scroll_down_by(count),
                        MouseEventKind::ScrollUp => list.scroll_up_by(count),
                        _ => return false,
                    },
                    Motion::ColStart => list.select_first(),
                    Motion::ColEnd => list.select_last(),
                    Motion::Down => list.scroll_down_by(count),
                    Motion::Up => list.scroll_up_by(count),
                    _ => return false,
                }

                state.update_cursor_from_clues();
                state.update_clues_from_cursor();
                true
            }
            _ => false,
        }
    }
}

pub struct ListRenderState<'a> {
    state: &'a PuzzleScreenState,
    offset: usize,
    visible: usize,
    is_paused: bool,
}

impl<'a> ListRender<'a> for CluesListWidget {
    type State = ListRenderState<'a>;

    fn render_items(
        &self,
        state: &Self::State,
    ) -> impl Iterator<Item = ratatui::widgets::ListItem<'_>> {
        let ListRenderState {
            state: screen_state,
            offset,
            visible,
            is_paused,
        } = state;

        let count = screen_state.clues(self.dir).count();
        let other = if offset + visible >= count {
            vec![]
        } else {
            vec![ListItem::new("...")]
        }
        .into_iter();

        screen_state
            .clues(self.dir)
            .map(|clue| {
                let clue_text = match (*is_paused, self.dir.is_some()) {
                    (true, _) => format!("{:>2} ...", clue.num()),
                    (false, true) => format!("{:>2} {}", clue.num(), clue.text()),
                    (false, false) => {
                        format!("{:>2}{} {}", clue.num(), clue.direction(), clue.text())
                    }
                };
                ListItem::new(clue_text)
            })
            .chain(other)
    }
}
