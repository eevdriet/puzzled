use std::collections::HashMap;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect, Size},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Padding, Row, StatefulWidget, Table, TableState},
};

use crate::{
    AppContext, AppTypes, Command, KeyMap, Keys, Motion, Popup, Theme, TrieEntry,
    Widget as AppWidget,
};

pub struct KeysTablePopup<'a, A: AppTypes> {
    pub keys: &'a Keys<A>,
}

impl<'a, A: AppTypes> KeysTablePopup<'a, A> {
    pub fn new(keys: &'a Keys<A>) -> Self {
        Self { keys }
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

#[derive(Debug, Default, Clone, Copy)]
pub struct TabWidths {
    name: usize,
    keys: usize,
    desc: usize,
}

impl TabWidths {
    pub fn total(&self) -> usize {
        self.name + self.keys + self.desc
    }
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

    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        let titles = self.titles(state);
        let header = Row::new(titles).style(Style::new().bold());
        let (mut rows, widths) = self.rows_and_widths(&ctx.keys, state, &ctx.theme);
        let rows = rows.remove(&state.tab).unwrap_or_default();

        let widths = vec![
            Constraint::Length(widths.name as u16),
            Constraint::Length(widths.keys as u16),
            Constraint::Length(widths.desc as u16),
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
        let table_area = self.render_area(area, ctx, state);
        StatefulWidget::render(table, table_area, buf, &mut state.table);
    }

    fn render_size(&self, _area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Size {
        let (rows, widths) = self.rows_and_widths(&ctx.keys, state, &ctx.theme);

        let width = widths.total() as u16 + 2 + 4;
        let height = rows
            .values()
            .map(|keys| keys.len())
            .max()
            .unwrap_or_default() as u16
            + 2
            + 1;

        Size::new(width, height)
    }

    fn on_command(
        &mut self,
        command: crate::AppCommand<A>,
        _resolver: crate::AppResolver<A>,
        _ctx: &mut AppContext<A>,
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
        map: &'a KeyMap<A>,
        state: &KeysTablePopupState,
        theme: &Theme,
    ) -> (HashMap<KeysTab, Vec<Row<'_>>>, TabWidths) {
        let [_, keys, desc] = self.titles(state);
        let mut widths = TabWidths {
            name: 0,
            keys: keys.len(),
            desc: desc.len(),
        };

        let mut rows: HashMap<KeysTab, Vec<Row<'_>>> = HashMap::default();

        self.add_entries(
            self.keys.actions.iter(),
            KeysTab::Actions,
            map,
            &mut rows,
            &mut widths,
            theme,
        );
        self.add_entries(
            self.keys.motions.iter(),
            KeysTab::Motions,
            map,
            &mut rows,
            &mut widths,
            theme,
        );
        self.add_entries(
            self.keys.text_objects.iter(),
            KeysTab::TextObjects,
            map,
            &mut rows,
            &mut widths,
            theme,
        );

        widths.name = rows
            .keys()
            .map(|name| format!("{name:?}").len())
            .max()
            .unwrap_or(widths.name);

        (rows, widths)
    }

    fn add_entries<I, T>(
        &self,
        iter: I,
        tab: KeysTab,
        map: &'a KeyMap<A>,
        rows: &mut HashMap<KeysTab, Vec<Row<'a>>>,
        widths: &mut TabWidths,
        theme: &Theme,
    ) where
        I: Iterator<Item = &'a (String, String, T)>,
        T: Clone + Into<TrieEntry<A::Action, A::TextObject, A::Motion>> + 'a,
    {
        let base = Style::default();
        let entry_style = match tab {
            KeysTab::Actions => base.fg(theme.palette.yellow),
            KeysTab::Motions => base.fg(theme.palette.blue),
            KeysTab::TextObjects => base.fg(theme.palette.green),
        };
        let sep_style = base.fg(Color::White).dim();

        for (name, desc, t) in iter {
            widths.name = name.len().max(widths.name);
            widths.desc = desc.len().max(widths.desc);

            let entry = t.clone().into();
            let mut row = vec![Line::styled(name, base.fg(Color::White))];

            let keys = map
                .get_merged(&entry, entry_style, sep_style)
                .unwrap_or(Line::raw(""));
            let keys_len: usize = keys.iter().map(|span| span.width()).sum();
            widths.keys = keys_len.max(widths.keys);

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
