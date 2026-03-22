use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect, Size},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Row, StatefulWidget, Table, TableState},
};

use crate::{AppTypes, Keys, Popup, TrieEntry, Widget as AppWidget};

pub struct KeysTablePopup<A: AppTypes> {
    pub keys: Keys<A>,
}

#[derive(Debug, Default)]
pub struct KeysTablePopupState {
    pub tab: usize,
    pub table: TableState,
}

const TITLES: [&str; 3] = ["Action", "Key(s)", "Description"];

impl<A: AppTypes> AppWidget<A> for KeysTablePopup<A> {
    type State = KeysTablePopupState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header = Row::new(TITLES).style(Style::new().bold()).bottom_margin(1);
        let (rows, name_width, keys_width, desc_width) = self.rows_and_widths(state);
        let widths = vec![
            Constraint::Length(name_width as u16),
            Constraint::Length(keys_width as u16),
            Constraint::Length(desc_width as u16),
        ];

        // Widgets
        let block = Block::bordered()
            .title(" Keys ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let table = Table::new(rows, widths)
            .block(block)
            .header(header)
            .column_spacing(1);

        // Render
        let table_area = self.render_area(area, state);
        StatefulWidget::render(table, table_area, buf, &mut state.table);
    }

    fn render_size(&self, _area: Rect, state: &Self::State) -> Size {
        let (rows, max_name_width, max_keys_width, max_desc_width) = self.rows_and_widths(state);

        // Sizing
        Size::new(
            (max_name_width + max_keys_width + max_desc_width) as u16 + 2 + 2,
            rows.len() as u16 + 2,
        )
    }
}

impl<A: AppTypes> Popup<A> for KeysTablePopup<A> {}

impl<A: AppTypes> KeysTablePopup<A> {
    fn rows_and_widths(&self, state: &KeysTablePopupState) -> (Vec<Row<'_>>, usize, usize, usize) {
        let mut max_name_width = TITLES[0].len();
        let mut max_keys_width = TITLES[1].len();
        let mut max_desc_width = TITLES[2].len();

        let base = Style::default();

        let rows = match state.tab {
            0 => self
                .keys
                .actions
                .iter()
                .map(|(name, desc, action)| {
                    let entry = TrieEntry::Action(action.clone());
                    let entry_str = self.keys.map.get_merged(&entry).unwrap_or_default();

                    max_name_width = name.len().max(max_name_width);
                    max_desc_width = desc.len().max(max_desc_width);
                    max_keys_width = entry_str.len().max(max_keys_width);

                    Row::new(vec![
                        Span::styled(name, base.fg(Color::White)),
                        Span::styled(entry_str, base.fg(Color::Yellow)),
                        Span::styled(desc, base.fg(Color::White)),
                    ])
                })
                .collect(),
            _ => vec![],
        };

        (rows, max_name_width, max_keys_width, max_desc_width)
    }
}
