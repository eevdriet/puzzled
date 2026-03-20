use puzzled_core::Timer;
use puzzled_tui::{EventMode, TimerWidget};
use ratatui::{
    layout::Margin,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{StatefulWidgetRef, Widget},
};

use crate::CrosswordKeys;

pub struct FooterWidget<'a> {
    pub keys: &'a CrosswordKeys,
}

pub struct FooterState {
    pub mode: EventMode,
    pub timer: Timer,
}

impl<'a> StatefulWidgetRef for FooterWidget<'a> {
    type State = FooterState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" ? ", Style::new().fg(Color::Yellow)),
            Span::raw("to display the key bindings that are defined"),
        ])
        .render(area, buf);

        let timer = TimerWidget { timer: state.timer };
        timer.render(area.inner(Margin::new(0, 1)), buf);
    }
}
