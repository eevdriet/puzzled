use crossterm::event::MouseEventKind;
use puzzled_crossword::ClueDirection;
use puzzled_tui::{AppContext, Command, HandleCommand, ListRender, ListWidget, Motion, RenderSize};
use ratatui::{
    layout::Size,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{List, ListItem, StatefulWidget, StatefulWidgetRef},
};

use crate::{
    AppState, CrosswordAction, CrosswordCommand, CrosswordContext, CrosswordMotion,
    CrosswordResolver, CrosswordTextObject, PuzzleScreenState,
};

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
        let render_state = ListRenderState {
            state,
            offset: 0,
            visible: 0,
        };

        ListWidget::new(self).render_size(&render_state)
    }
}

pub struct ListRenderState<'a> {
    state: &'a PuzzleScreenState,
    offset: usize,
    visible: usize,
}

impl<'a> ListRender<'a> for CluesListWidget {
    type State = ListRenderState<'a>;

    fn render_items(
        &self,
        state: &Self::State,
    ) -> impl Iterator<Item = ratatui::widgets::ListItem<'_>> {
        let ListRenderState {
            state,
            offset,
            visible,
        } = state;

        let count = state.clues(self.dir).count();
        let other = if offset + visible >= count {
            vec![]
        } else {
            vec![ListItem::new("...")]
        }
        .into_iter();

        state
            .clues(self.dir)
            .map(|clue| {
                let clue_text = match self.dir.is_some() {
                    true => format!("{:>2} {}", clue.num(), clue.text()),
                    false => format!("{:>2}{} {}", clue.num(), clue.direction(), clue.text()),
                };
                ListItem::new(clue_text)
            })
            .chain(other)
    }
}

impl StatefulWidgetRef for CluesListWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let visible = area.height as usize;
        let offset = state.clue_list(self.dir).offset();
        let render_state = ListRenderState {
            state,
            visible,
            offset,
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
}

impl HandleCommand<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>
    for CluesListWidget
{
    type State = PuzzleScreenState;

    fn handle_command(
        &mut self,
        command: CrosswordCommand,
        _resolver: CrosswordResolver,
        _ctx: &mut CrosswordContext,
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
