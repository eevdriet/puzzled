use std::collections::HashMap;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect, Size},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Padding, Row, StatefulWidget, Table, TableState},
};

use crate::{AppTypes, Command, Keys, Motion, Popup, TrieEntry, Widget as AppWidget};

pub struct KeysTablePopup<A: AppTypes> {
    pub keys: Keys<A>,
}

#[derive(Debug, Default)]
pub struct KeysTablePopupState {
    pub tab: KeysTab,
    pub table: TableState,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(usize)]
pub enum KeysTab {
    #[default]
    Actions = 0,
    Motions = 1,
    TextObjects = 2,
}

impl From<usize> for KeysTab {
    fn from(idx: usize) -> Self {
        match idx {
            0 => KeysTab::Actions,
            1 => KeysTab::Motions,
            _ => KeysTab::TextObjects,
        }
    }
}

impl<A: AppTypes> AppWidget<A> for KeysTablePopup<A> {
    type State = KeysTablePopupState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let titles = self.titles(state);
        let header = Row::new(titles).style(Style::new().bold()).bottom_margin(1);
        let (mut rows, name_width, keys_width, desc_width) = self.rows_and_widths(state);
        let rows = rows.remove(&state.tab).unwrap_or_default();

        let widths = vec![
            Constraint::Length(name_width as u16),
            Constraint::Length(keys_width as u16),
            Constraint::Length(desc_width as u16),
        ];

        // Widgets
        let block = Block::bordered()
            .title(" Keys ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

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

        let width = (max_name_width + max_keys_width + max_desc_width) as u16 + 2 + 4;
        let height = rows
            .values()
            .map(|keys| keys.len())
            .max()
            .unwrap_or_default() as u16
            + 2;

        Size::new(width, height)
    }

    fn on_command(
        &mut self,
        command: crate::AppCommand<A>,
        _resolver: crate::AppResolver<A>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Motion { motion, .. } => {
                match motion {
                    Motion::Left => {
                        let curr = state.tab as usize;
                        let next = (curr + 2).rem_euclid(3);
                        state.tab = KeysTab::from(next);
                    }
                    Motion::Right => {
                        let curr = state.tab as usize;
                        let next = (curr + 1).rem_euclid(3);
                        state.tab = KeysTab::from(next);
                    }
                    _ => return false,
                }
                true
            }
            _ => false,
        }
    }
}

impl<A: AppTypes> Popup<A> for KeysTablePopup<A> {}

impl<A: AppTypes> KeysTablePopup<A> {
    fn rows_and_widths(
        &self,
        state: &KeysTablePopupState,
    ) -> (HashMap<KeysTab, Vec<Row<'_>>>, usize, usize, usize) {
        let [name, keys, desc] = self.titles(state);

        let mut max_name_width = name.len();
        let mut max_keys_width = keys.len();
        let mut max_desc_width = desc.len();

        let base = Style::default();
        let mut rows: HashMap<KeysTab, Vec<Row<'_>>> = HashMap::default();

        for (name, desc, action) in self.keys.actions.iter() {
            let entry = TrieEntry::Action(action.to_owned());
            let entry_str = self.keys.map.get_merged(&entry).unwrap_or_default();

            max_name_width = name.len().max(max_name_width);
            max_desc_width = desc.len().max(max_desc_width);
            max_keys_width = entry_str.len().max(max_keys_width);

            rows.entry(KeysTab::Actions)
                .or_default()
                .push(Row::new(vec![
                    Span::styled(name, base.fg(Color::White)),
                    Span::styled(entry_str, base.fg(Color::Yellow)),
                    Span::styled(desc, base.fg(Color::White)),
                ]))
        }

        for (name, desc, motion) in self.keys.motions.iter() {
            let entry = TrieEntry::Motion(motion.to_owned());
            let entry_str = self.keys.map.get_merged(&entry).unwrap_or_default();

            max_name_width = name.len().max(max_name_width);
            max_desc_width = desc.len().max(max_desc_width);
            max_keys_width = entry_str.len().max(max_keys_width);

            rows.entry(KeysTab::Motions)
                .or_default()
                .push(Row::new(vec![
                    Span::styled(name, base.fg(Color::White)),
                    Span::styled(entry_str, base.fg(Color::Blue)),
                    Span::styled(desc, base.fg(Color::White)),
                ]))
        }

        for (name, desc, motion) in self.keys.text_objects.iter() {
            let entry = TrieEntry::TextObject(motion.to_owned());
            let entry_str = self.keys.map.get_merged(&entry).unwrap_or_default();

            max_name_width = name.len().max(max_name_width);
            max_desc_width = desc.len().max(max_desc_width);
            max_keys_width = entry_str.len().max(max_keys_width);

            rows.entry(KeysTab::TextObjects)
                .or_default()
                .push(Row::new(vec![
                    Span::styled(name, base.fg(Color::White)),
                    Span::styled(entry_str, base.fg(Color::Green)),
                    Span::styled(desc, base.fg(Color::White)),
                ]))
        }

        (rows, max_name_width, max_keys_width, max_desc_width)
    }
}

impl<A: AppTypes> KeysTablePopup<A> {
    fn titles(&self, state: &KeysTablePopupState) -> [String; 3] {
        [
            format!("{:?}", state.tab),
            "Key(s)".to_string(),
            "Description".to_string(),
        ]
    }
}
