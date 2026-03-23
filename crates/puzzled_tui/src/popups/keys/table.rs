use std::collections::HashMap;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect, Size},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Padding, Row, StatefulWidget, Table, TableState},
};

use crate::{AppTypes, Command, KeyMap, Keys, Motion, Popup, TrieEntry, Widget as AppWidget};

pub struct KeysTablePopup<'a, A: AppTypes> {
    pub keys: &'a Keys<A>,
    pub map: &'a KeyMap<A>,
}

impl<'a, A: AppTypes> KeysTablePopup<'a, A> {
    pub fn new(keys: &'a Keys<A>, map: &'a KeyMap<A>) -> Self {
        Self { keys, map }
    }
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

impl<'a, A: AppTypes> AppWidget<A> for KeysTablePopup<'a, A> {
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

impl<'a, A: AppTypes> Popup<A> for KeysTablePopup<'a, A> {}

impl<'a, A: AppTypes> KeysTablePopup<'a, A> {
    fn rows_and_widths(
        &self,
        state: &KeysTablePopupState,
    ) -> (HashMap<KeysTab, Vec<Row<'_>>>, usize, usize, usize) {
        let [name, keys, desc] = self.titles(state);

        let mut max_name_width = name.len();
        let mut max_keys_width = keys.len();
        let mut max_desc_width = desc.len();

        let mut rows: HashMap<KeysTab, Vec<Row<'_>>> = HashMap::default();

        self.add_entries(
            self.keys.actions.iter(),
            KeysTab::Actions,
            &mut rows,
            &mut max_name_width,
            &mut max_keys_width,
            &mut max_desc_width,
        );
        self.add_entries(
            self.keys.motions.iter(),
            KeysTab::Motions,
            &mut rows,
            &mut max_name_width,
            &mut max_keys_width,
            &mut max_desc_width,
        );
        self.add_entries(
            self.keys.text_objects.iter(),
            KeysTab::TextObjects,
            &mut rows,
            &mut max_name_width,
            &mut max_keys_width,
            &mut max_desc_width,
        );

        (rows, max_name_width, max_keys_width, max_desc_width)
    }

    fn add_entries<I, T>(
        &self,
        iter: I,
        tab: KeysTab,
        rows: &mut HashMap<KeysTab, Vec<Row<'a>>>,
        max_name_width: &mut usize,
        max_keys_width: &mut usize,
        max_desc_width: &mut usize,
    ) where
        I: Iterator<Item = &'a (String, String, T)>,
        T: Clone + Into<TrieEntry<A::Action, A::TextObject, A::Motion>> + 'a,
    {
        let base = Style::default();
        let entry_style = match tab {
            KeysTab::Actions => base.fg(Color::Yellow),
            KeysTab::Motions => base.fg(Color::Blue),
            KeysTab::TextObjects => base.fg(Color::Green),
        };
        let sep_style = base.fg(Color::White).dim();

        for (name, desc, t) in iter {
            *max_name_width = name.len().max(*max_name_width);
            *max_desc_width = desc.len().max(*max_desc_width);

            let entry = t.clone().into();
            let mut row = vec![Line::styled(name, base.fg(Color::White))];

            let keys = self
                .map
                .get_merged(&entry, entry_style, sep_style)
                .unwrap_or(Line::raw(""));
            let keys_len: usize = keys.iter().map(|span| span.width()).sum();
            *max_keys_width = keys_len.max(*max_keys_width);

            row.push(keys);
            row.push(Line::styled(desc, base.fg(Color::White)));

            rows.entry(tab).or_default().push(Row::new(row));
        }
    }
}

impl<'a, A: AppTypes> KeysTablePopup<'a, A> {
    fn titles(&self, state: &KeysTablePopupState) -> [String; 3] {
        [
            format!("{:?}", state.tab),
            "Key(s)".to_string(),
            "Description".to_string(),
        ]
    }
}
