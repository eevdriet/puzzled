use std::io;

use puzzled_crossword::{Crossword, CrosswordState, crossword};
use puzzled_io::TxtPuzzle;
use puzzled_tui::{
    Action, AppCommand, Command, EventMode, EventTrie, GridRenderState, GridWidget, ListRender,
    ListWidget, Screen, Widget,
};
use puzzled_tui::{AppContext, AppResolver};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{CrosswordApp, PuzzleScreen, RenderSquareSolution, RenderSquareState};

pub struct TitleScreen {
    title: Crossword,
    list: ListWidget<TitleRender, CrosswordApp>,
    state: ListState,
}

const ITEMS: [&str; 4] = ["New game", "Continue", "About", "Quit"];

impl Default for TitleScreen {
    fn default() -> Self {
        let title = crossword!(
            [ C C C . R R . O O O . S S S . S S S ]
            [ C . . . R R . O . O . . S . . S . . ]
            [ C C C . R . R O O O . S S S . S S S ]
            [ . . . . . . . . . . . . . . . . . . ]
            [ . W . . W . O O O . R R . . D D . . ]
            [ . W . . W . O . O . R R . . D . D . ]
            [ W W W . W . O O O . R . R . D D . . ]
        );
        // let title = crossword!(
        //     [ C C C C C . R R R R . . O O O O O . S S S S S . S S S S S ]
        //     [ C . . . . . R . . R . . O . . . O . S . . . . . S . . . . ]
        //     [ C . . . . . R R R . . . O . . . O . S S S S S . S S S S S ]
        //     [ C . . . . . R . . R . . O . . . O . . . . . S . . . . . S ]
        //     [ C C C C C . R . . . R . O O O O O . S S S S S . S S S S S ]
        //     [ . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ]
        //     [ . . . W . . . W . O O O O O . R R R R . . D D D D . . . . ]
        //     [ . . . W . . . W . O . . . O . R . . R . . D . . . D . . . ]
        //     [ . . . W . W . W . O . . . O . R R R . . . D . . . D . . . ]
        //     [ . . . W . W . W . O . . . O . R . . R . . D . . . D . . . ]
        //     [ . . . W W W W W . O O O O O . R . . . R . D D D D . . . . ]
        //
        // );

        let list = ListWidget::new(TitleRender);
        let mut state = ListState::default();
        state.select_first();

        Self { title, list, state }
    }
}

impl Screen<CrosswordApp> for TitleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, _ctx: &mut AppContext<CrosswordApp>) {
        let state = CrosswordState::from(&self.title);
        let cell_state = RenderSquareState::new(self.title.squares(), self.title.clues());
        let mut render = GridRenderState::default();
        render.options.cell_width = 4;
        render.options.cell_height = 3;

        let grid = state.entries.map_ref(RenderSquareSolution);
        let _grid_widget = GridWidget::new(&grid, &cell_state);

        self.list.render(root, buf, &mut self.state);
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

    let mut render_state = GridRenderState::default();
    let opts = &mut render_state.options;

    opts.cell_width = 5;
    opts.cell_height = 3;

    let events: EventTrie<CrosswordApp> = EventTrie::from_config::<Crossword>()?;
    let keys = events.action_keys();
    let screen = PuzzleScreen::new(puzzle, solve_state, render_state, keys.clone());

    Ok(screen)
}
