use puzzled_core::Timer;
use puzzled_tui::{EventMode, TimerWidget, Widget as AppWidget};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Size},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::Widget,
};

use crate::CrosswordApp;

pub struct FooterWidget;

pub struct FooterState {
    pub mode: EventMode,
    pub timer: Timer,
    pub pause_key: String,
}

impl AppWidget<CrosswordApp> for FooterWidget {
    type State = FooterState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [help_line, timer_line, mode_line] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(area);

        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" ? ", Style::new().fg(Color::Yellow)),
            Span::raw("for help and "),
            Span::styled(&state.pause_key, Style::new().fg(Color::Yellow)),
            Span::raw(" to pause the puzzle"),
        ])
        .render(help_line, buf);

        let timer = TimerWidget { timer: state.timer };
        timer.render(timer_line, buf);

        let mode = state.mode.to_string();
        Text::from(mode).render(mode_line, buf);
    }

    fn render_size(&self, area: Rect, _state: &Self::State) -> Size {
        Size::new(area.width, 3)
    }
}
