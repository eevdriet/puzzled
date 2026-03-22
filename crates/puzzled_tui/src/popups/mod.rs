mod keys;

pub use keys::*;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Clear, Widget},
};

use crate::{Action, AppCommand, AppResolver, AppTypes, Command, Widget as AppWidget};

pub trait Popup<A: AppTypes>: AppWidget<A> {
    fn render_popup(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Dim the surrounding area
        buf.set_style(area, Style::default().dim());

        // Clear the area in which the popup is to be rendered
        let clear_area = self.render_area(area, state);
        Clear.render(clear_area, buf);

        // Then render the popup as a widget
        self.render(area, buf, state);
    }

    fn on_popup_command(
        &mut self,
        command: AppCommand<A>,
        resolver: AppResolver<A>,
        state: &mut Self::State,
    ) -> bool {
        match &command {
            Command::Action { action, .. } => {
                match action {
                    Action::Quit | Action::Cancel => {
                        resolver.close_popup();
                    }
                    _ => return self.on_command(command, resolver, state),
                }

                true
            }

            _ => false,
        }
    }
}
