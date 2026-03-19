use puzzled_core::Timer;
use puzzled_tui::{Action, EventMode, Motion, TimerWidget, TrieEntry};
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
        let base = Style::default();

        Line::from(vec![
            Span::styled("Help: ", base.fg(Color::Gray)),
            Span::styled(
                self.keys
                    .get_merged(&TrieEntry::Action(Action::ShowHelp))
                    .unwrap_or_default(),
                base.fg(Color::Yellow),
            ),
            Span::styled(" | Move left: ", base.fg(Color::Gray)),
            Span::styled(
                self.keys
                    .get_merged(&TrieEntry::Motion(Motion::Left))
                    .unwrap_or_default(),
                base.fg(Color::Yellow),
            ),
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
