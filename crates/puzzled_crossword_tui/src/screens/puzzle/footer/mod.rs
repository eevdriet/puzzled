use puzzled_core::Timer;
use puzzled_tui::{EventMode, TimerWidget, Widget as AppWidget};
use ratatui::{
    layout::Margin,
    prelude::{Buffer, Rect, Size},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::CrosswordApp;

pub struct FooterWidget;

pub struct FooterState {
    pub mode: EventMode,
    pub timer: Timer,
}

impl AppWidget<CrosswordApp> for FooterWidget {
    type State = FooterState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" ? ", Style::new().fg(Color::Yellow)),
            Span::raw("to display the key bindings that are defined"),
        ])
        .render(area, buf);

        let timer = TimerWidget { timer: state.timer };
        timer.render(area.inner(Margin::new(0, 1)), buf);
    }

    fn render_size(&self, _state: &Self::State) -> Size {
        let width = "Press ? to display the key bindings that are defined".len() as u16;
        Size::new(width, 2)
    }
}
