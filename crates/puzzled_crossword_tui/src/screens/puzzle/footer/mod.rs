use puzzled_core::Timer;
use puzzled_tui::{EventMode, TimerWidget};
use ratatui::{
    layout::Margin,
    prelude::{Buffer, Rect},
    text::Text,
    widgets::{StatefulWidgetRef, Widget},
};

pub struct FooterWidget;

pub struct FooterState {
    pub mode: EventMode,
    pub timer: Timer,
}

impl StatefulWidgetRef for FooterWidget {
    type State = FooterState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text = format!("{:?}", state.mode);
        Text::from(text).render(area, buf);

        let timer = TimerWidget { timer: state.timer };
        timer.render(area.inner(Margin::new(0, 1)), buf);
    }
}
