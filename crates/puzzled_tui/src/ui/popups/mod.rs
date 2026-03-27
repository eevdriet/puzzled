mod keys;

pub use keys::*;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Clear, Widget},
};

use crate::{Action, AppCommand, AppContext, AppResolver, AppTypes, Command, Widget as AppWidget};

pub trait Popup<A: AppTypes>: AppWidget<A> {
    fn render_popup(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        // Dim the surrounding area
        buf.set_style(area, Style::default().dim());

        // Clear the area in which the popup is to be rendered
        let clear_area = self.render_area(area, ctx, state);
        Clear.render(clear_area, buf);

        // Then render the popup as a widget
        self.render(area, buf, ctx, state);
    }

    fn on_popup_command(
        &mut self,
        command: AppCommand<A>,
        resolver: AppResolver<A>,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) -> bool {
        if let Command::Action {
            action: Action::Quit | Action::Cancel | Action::Click { .. },
            ..
        } = command
        {
            resolver.close_popup();
            true
        } else {
            tracing::info!("Pause command");
            self.on_command(command, resolver, ctx, state)
        }
    }
}
