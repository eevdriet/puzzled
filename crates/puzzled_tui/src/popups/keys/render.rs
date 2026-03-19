use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Row, StatefulWidget, StatefulWidgetRef, Table},
};

use crate::{
    ActionBehavior, KeysPopup, KeysPopupState, MotionBehavior, TextObjectBehavior, TrieEntry,
};

impl<'a, A, T, M, S> StatefulWidgetRef for KeysPopup<'a, A, T, M, S>
where
    A: ActionBehavior,
    T: TextObjectBehavior,
    M: MotionBehavior,
{
    type State = KeysPopupState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered().title(" Keys ");
        let rows = match state.tab {
            0 => self
                .actions
                .iter()
                .map(|(desc, action)| {
                    let entry = TrieEntry::Action(action.clone());
                    let entry_str = self.map.get_merged(&entry).unwrap_or_default();

                    Row::new(vec![desc.to_string(), entry_str])
                })
                .collect(),
            _ => vec![],
        };

        let widths = vec![Constraint::Fill(1), Constraint::Fill(1)];

        Table::new(rows, widths)
            .block(block)
            .render(area, buf, &mut state.table);
    }
}
