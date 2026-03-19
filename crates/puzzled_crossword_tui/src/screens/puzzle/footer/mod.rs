use puzzled_core::Timer;
use puzzled_tui::{EventMode, TimerWidget};
use ratatui::{
    layout::Margin,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
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
        let base = Style::default();

        Line::from(vec![
            Span::styled("Help: ", base.fg(Color::Gray)),
            Span::styled("?", base.fg(Color::Yellow)),
        ])
        .render(area, buf);

        let timer = TimerWidget { timer: state.timer };
        timer.render(area.inner(Margin::new(0, 1)), buf);

        // let footer_text = vec![
        //     Line::from(""),
        //     Line::from(vec![
        //         Span::styled("Navigation: ", Style::default().fg(Color::Gray)),
        //         Span::styled("↑/k ", Style::default().fg(Color::Yellow)),
        //         Span::styled("↓/j ", Style::default().fg(Color::Yellow)),
        //         Span::styled("| Select: ", Style::default().fg(Color::Gray)),
        //         Span::styled("Enter/Space", Style::default().fg(Color::Yellow)),
        //         Span::styled(" | Help: ", Style::default().fg(Color::Gray)),
        //         Span::styled("?", Style::default().fg(Color::Yellow)),
        //     ]),
        //     Line::from(format!("Version: {}", version))
        //         .alignment(Alignment::Center)
        //         .style(Style::default().fg(Color::Gray)),
        // ];
        //
        // let footer = Paragraph::new(footer_text)
        //     .alignment(Alignment::Center)
        //     .block(Block::default());
        // }
    }
}
