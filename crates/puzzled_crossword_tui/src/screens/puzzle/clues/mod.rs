mod list;

use crossterm::event::MouseEventKind;
use puzzled_crossword::ClueDirection;
use puzzled_tui::{
    Action, Command, EventMode, HandleCommand, Motion, RenderSize, Widget as AppWidget,
};
use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Margin, Size},
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, StatefulWidgetRef, Widget},
};

use crate::{
    AppState, CrosswordAction, CrosswordCommand, CrosswordMotion, CrosswordResolver,
    CrosswordTextObject, Focus, PuzzleScreenState, screens::puzzle::clues::list::CluesListWidget,
};

pub struct CluesWidget {
    across_down: CluesListWidget,
    across: CluesListWidget,
    down: CluesListWidget,
}

impl Default for CluesWidget {
    fn default() -> Self {
        Self {
            across_down: CluesListWidget::new(None),
            across: CluesListWidget::new(Some(ClueDirection::Across)),
            down: CluesListWidget::new(Some(ClueDirection::Down)),
        }
    }
}

impl AppWidget<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState> for CluesWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Render the outside block with the tabs
        let base_style = Style::default();
        let selected_style = base_style.fg(Color::Yellow);
        let unselected_style = base_style;

        let border_style = if matches!(state.focus.get(), Focus::Clues) {
            selected_style
        } else {
            unselected_style
        };

        let title = " Clues ";
        let block = Block::new()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(border_style)
            .title(title)
            .title_alignment(HorizontalAlignment::Center);

        let area = block.inner(root);
        block.render(root, buf);

        // Render the clue list(s)
        let text_margin = Margin::new(0, 1);

        if state.clue_dir.is_none() {
            self.across_down.render(area.inner(text_margin), buf, state);
            return;
        }

        let [across, _, down] = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        // Render across clues
        let [across_title, across] =
            Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0)]).areas(across);

        Text::from("   Across")
            .fg(Color::Green)
            .render(across_title, buf);

        self.across.render(across, buf, state);

        // Render down clues
        let [down_title, down] =
            Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0)]).areas(down);

        Text::from("   Down")
            .fg(Color::Blue)
            .render(down_title, buf);

        self.down.render(down, buf, state);
    }

    fn render_size(&self, state: &Self::State) -> Size {
        let mut size = self.across_down.render_size(state);

        let across_size = self.across.render_size(state);
        let down_size = self.down.render_size(state);

        size.width = size.width.max(across_size.width + down_size.width + 2);
        size.height = size
            .height
            .max(2 + across_size.height.max(down_size.height));

        // Border
        size.width += 2;
        size.height += 2;

        size
    }

    fn override_mode(&self) -> Option<EventMode> {
        Some(EventMode::Normal)
    }

    fn on_command(
        &mut self,
        command: CrosswordCommand,
        resolver: CrosswordResolver,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Action {
                action: Action::Select,
                ..
            } => {
                state.focus.set(Focus::Crossword);
                resolver.set_mode(EventMode::Insert);
            }
            Command::Motion { motion, .. } => {
                let is_across = matches!(state.clue_dir, Some(ClueDirection::Across));
                let is_down = matches!(state.clue_dir, Some(ClueDirection::Down));

                match motion {
                    // Across -> down
                    Motion::Right if is_across => {
                        state.clue_dir = Some(ClueDirection::Down);
                        state.update_cursor_from_clues();
                    }
                    Motion::Mouse(mouse)
                        if is_across && mouse.kind == MouseEventKind::ScrollRight =>
                    {
                        state.clue_dir = Some(ClueDirection::Down);
                        state.update_cursor_from_clues();
                    }

                    // Down -> across
                    Motion::Left if is_down => {
                        state.clue_dir = Some(ClueDirection::Across);
                        state.update_cursor_from_clues();
                    }
                    Motion::Mouse(mouse) if is_down && mouse.kind == MouseEventKind::ScrollLeft => {
                        state.clue_dir = Some(ClueDirection::Across);
                        state.update_cursor_from_clues();
                    }
                    _ => {
                        return match state.clue_dir {
                            Some(ClueDirection::Across) => {
                                self.across.on_command(command, resolver, state)
                            }
                            Some(ClueDirection::Down) => {
                                self.down.on_command(command, resolver, state)
                            }
                            None => self.across_down.on_command(command, resolver, state),
                        };
                    }
                }
            }

            _ => {
                return false;
            }
        }

        true
    }
}
