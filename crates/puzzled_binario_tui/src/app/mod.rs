use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::EnterAlternateScreen,
};
use derive_more::{Deref, DerefMut};
use puzzled_binario::Bit;
use puzzled_core::{Position, grid};
use puzzled_tui::{Action, CellRender, GridOptions, GridState, GridWidget, HandleAction, Viewport};
use ratatui::{
    Terminal,
    layout::Size,
    prelude::CrosstermBackend,
    style::{Modifier, Style},
    text::Text,
};

pub struct App {
    term: Terminal<CrosstermBackend<Stdout>>,
}

#[derive(Debug, Deref, DerefMut)]
pub struct RenderBit(pub Bit);

impl CellRender<GridState> for RenderBit {
    fn render_cell(&self, pos: Position, state: &GridState) -> Text<'_> {
        let symbol = match pos == state.cursor {
            true => 'E'.to_string(),
            false => self.to_string(),
        };

        let mut style = Style::default();
        if pos == state.cursor {
            style = style.add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK);
        }

        Text::from(symbol).style(style)
    }
}

const T: Bit = Bit::One;
const F: Bit = Bit::Zero;
const POLL_DURATION: Duration = Duration::from_millis(30);

impl App {
    pub fn new() -> Self {
        Self {
            term: ratatui::init(),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        let grid = grid!(
            [T, T, F, F, F, F],
            [F, F, F, F, F, F],
            [F, F, F, T, T, F],
            [F, F, F, T, T, F],
            [F, F, F, T, T, F],
            [F, F, F, F, F, F],
        )
        .map(RenderBit);

        let mut state = GridState {
            options: GridOptions::default(),
            viewport: Viewport::from_grid(&grid),
            cursor: Position::default(),
        };
        state.options.cell_width = 4;
        state.options.cell_height = 2;
        state.options.inner = Some(Size::new(1, 1));
        // state.options.inner = None;
        state.options.draw_inner_borders = true;

        loop {
            let widget = GridWidget(&grid);

            self.term.draw(|frame| {
                frame.render_stateful_widget(widget, frame.area(), &mut state);
            })?;

            if event::poll(POLL_DURATION)? {
                let ev = event::read()?;

                if let Event::Key(key) = ev
                    && let KeyCode::Char(ch) = key.code
                {
                    let action: Action = match ch {
                        'q' => break,
                        'h' => Action::MoveLeft,
                        'j' => Action::MoveDown,
                        'k' => Action::MoveUp,
                        'l' => Action::MoveRight,
                        _ => Action::Quit,
                    };

                    let _ = widget.handle_action(action, 1, &mut state);
                }
            }
        }

        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
