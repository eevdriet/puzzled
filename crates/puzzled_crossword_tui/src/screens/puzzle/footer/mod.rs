use puzzled_tui::EventMode;
use ratatui::{
    prelude::{Buffer, Rect},
    text::Text,
    widgets::{StatefulWidgetRef, Widget},
};

pub struct FooterWidget;

pub struct FooterState {
    pub mode: EventMode,
}

impl StatefulWidgetRef for FooterWidget {
    type State = FooterState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text = format!("{:?}", state.mode);
        Text::from(text).render(area, buf);
    }
}
