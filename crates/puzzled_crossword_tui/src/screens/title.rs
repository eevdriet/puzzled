use std::io;

use puzzled_crossword::Crossword;
use puzzled_io::TxtPuzzle;
use puzzled_tui::{
    Action, AppCommand, Command, EventMode, EventTrie, GridRenderState, ListRender, ListWidget,
    Screen, Widget as AppWidget, center_area,
};
use puzzled_tui::{AppContext, AppResolver};
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem, ListState, Paragraph, Widget};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{CrosswordApp, PuzzleScreen};

pub struct TitleScreen {
    list: ListWidget<TitleRender, CrosswordApp>,
    state: ListState,
}

const ITEMS: [&str; 4] = ["New game", "Continue", "About", "Quit"];

impl Default for TitleScreen {
    fn default() -> Self {
        let list = ListWidget::new(TitleRender);
        let mut state = ListState::default();
        state.select_first();

        Self { list, state }
    }
}

const TITLE: [&str; 6] = [
    r#"   _____                                                 _ "#,
    r#"  / ____|                                               | |"#,
    r#" | |      _ __  ___   ___  ___ __      __ ___   _ __  __| |"#,
    r#" | |     | '__|/ _ \ / __|/ __|\ \ /\ / // _ \ | '__|/ _` |"#,
    r#" | |____ | |  | (_) |\__ \\__ \ \ V  V /| (_) || |  | (_| |"#,
    r#"  \_____||_|   \___/ |___/|___/  \_/\_/  \___/ |_|   \__,_| "#,
];

impl Screen<CrosswordApp> for TitleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, _ctx: &mut AppContext<CrosswordApp>) {
        let lines: Vec<_> = TITLE.into_iter().map(Line::from).collect();
        let width = lines[0].width() as u16;

        let [area] = Layout::horizontal(vec![Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(root);
        let [title_area, _, list_area] = Layout::vertical(vec![
            Constraint::Length(lines.len() as u16),
            Constraint::Length(2),
            Constraint::Length(ITEMS.len() as u16),
        ])
        .flex(Flex::Center)
        .areas(area);

        // Render
        Paragraph::new(lines).render(title_area, buf);

        let mut size = self.list.render_size(root, &self.state);
        size.width += 3; // Add on highlight symbol width

        let list_area = center_area(list_area, size);
        AppWidget::render(&mut self.list, list_area, buf, &mut self.state);
    }

    fn on_command(
        &mut self,
        command: AppCommand<CrosswordApp>,
        resolver: AppResolver<CrosswordApp>,
        _ctx: &mut AppContext<CrosswordApp>,
    ) -> bool {
        match command {
            Command::Action { action, .. } => match action {
                // Selection hotkeys
                Action::Literal('n') => self.state.select(Some(0)),
                Action::Literal('C') => self.state.select(Some(1)),
                Action::Literal('a') => self.state.select(Some(2)),
                Action::Quit | Action::Literal('q') => self.state.select(Some(3)),

                // Select actions
                Action::Select => {
                    if let Some(selected) = self.state.selected() {
                        match selected {
                            0 => {
                                let Ok(screen) = create_puzzle_screen() else {
                                    return false;
                                };

                                resolver.next_screen(Box::new(screen));
                            }
                            1 => resolver.prev_screen(),
                            3 => resolver.quit(),
                            _ => {}
                        }
                    }
                }
                _ => return false,
            },
            command => return self.list.on_command(command, resolver, &mut self.state),
        }

        true
    }

    fn override_mode(&self) -> Option<EventMode> {
        Some(EventMode::Normal)
    }
}

struct TitleRender;

impl ListRender for TitleRender {
    type State = ListState;

    fn render_list(&self, _state: &Self::State) -> ratatui::widgets::List<'_> {
        List::default().highlight_symbol(">> ")
    }

    fn render_items(
        &self,
        _state: &Self::State,
    ) -> impl Iterator<Item = ratatui::widgets::ListItem<'_>> {
        ITEMS.into_iter().map(ListItem::new)
    }

    fn render_state<'a>(&self, state: &'a mut Self::State) -> &'a mut ListState {
        state
    }
}

fn create_puzzle_screen() -> io::Result<PuzzleScreen> {
    let (puzzle, solve_state) = Crossword::load_text("2026-03-08-nyt").map_err(io::Error::other)?;

    let mut render_state = GridRenderState {
        use_direction: true,
        ..Default::default()
    };

    let opts = &mut render_state.options;

    opts.cell_width = 5;
    opts.cell_height = 3;

    let events: EventTrie<CrosswordApp> = EventTrie::from_config::<Crossword>()?;
    let keys = events.action_keys();
    let screen = PuzzleScreen::new(puzzle, solve_state, render_state, keys.clone());

    Ok(screen)
}
