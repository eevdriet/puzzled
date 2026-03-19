use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, HorizontalAlignment, Rect, Size, VerticalAlignment},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Clear, Row, StatefulWidget, Table, Widget},
};

use crate::{
    ActionBehavior, KeysPopup, KeysPopupState, MotionBehavior, Popup, TextObjectBehavior,
    TrieEntry, align_area,
};

impl<A, T, M, S> Popup<A, T, M, S> for KeysPopup<A, T, M>
where
    A: ActionBehavior,
    T: TextObjectBehavior,
    M: MotionBehavior,
{
    type State = KeysPopupState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let titles = vec!["Action", "Key(s)", "Description"];
        let mut max_name_width = titles[0].len();
        let mut max_keys_width = titles[1].len();
        let mut max_desc_width = titles[2].len();

        let base = Style::default();
        let header = Row::new(titles).style(Style::new().bold()).bottom_margin(1);

        let rows = match state.tab {
            0 => self
                .actions
                .iter()
                .map(|(name, desc, action)| {
                    let entry = TrieEntry::Action(action.clone());
                    let entry_str = self.map.get_merged(&entry).unwrap_or_default();

                    max_name_width = name.len().max(max_name_width);
                    max_keys_width = entry_str.len().max(max_keys_width);
                    max_desc_width = desc.len().max(max_desc_width);

                    Row::new(vec![
                        Span::styled(name, base.fg(Color::White)),
                        Span::styled(entry_str, base.fg(Color::Yellow)),
                        Span::styled(desc, base.fg(Color::White)),
                    ])
                })
                .collect(),
            _ => vec![],
        };

        let widths = vec![
            Constraint::Length(max_name_width as u16),
            Constraint::Length(max_keys_width as u16),
            Constraint::Length(max_desc_width as u16),
        ];

        // Sizing
        let size = Size::new(
            (max_name_width + max_keys_width + max_desc_width) as u16 + 2 + 2,
            rows.len() as u16 + 2,
        );
        let area = align_area(
            area,
            size,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
        );

        // Widgets
        let block = Block::bordered()
            .title(" Keys ")
            .title_alignment(Alignment::Center);

        let table = Table::new(rows, widths)
            .block(block)
            .header(header)
            .column_spacing(1);

        // Render
        Clear.render(area, buf);
        StatefulWidget::render(table, area, buf, &mut state.table);
    }
}
