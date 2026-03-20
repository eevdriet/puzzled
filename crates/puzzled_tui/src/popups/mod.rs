mod keys;

pub use keys::*;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{Action, AppCommand, AppContext, AppResolver, AppTypes, Command};

pub trait Popup<A: AppTypes> {
    type State;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State);

    fn on_command(
        &mut self,
        command: AppCommand<A>,
        resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
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
