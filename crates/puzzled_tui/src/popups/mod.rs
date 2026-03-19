mod commands;

pub use commands::*;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{Action, ActionResolver, AppContext, Command};

pub trait Popup<A, T, M, S> {
    fn render(&mut self, area: Rect, buf: &mut Buffer, ctx: &mut AppContext<A, T, M, S>);

    fn on_command(
        &mut self,
        command: Command<A, T, M>,
        resolver: ActionResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
    ) -> bool {
        if let Command::Action { action, .. } = command {
            match action {
                Action::Quit => {
                    resolver.quit();
                }
                Action::Cancel => {
                    resolver.close_popup();
                }
                _ => {}
            }

            return true;
        }

        false
    }
}
