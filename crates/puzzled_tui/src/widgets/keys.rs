use std::borrow::Cow;

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Widget, WidgetRef},
};

use crate::{AppTrieEntry, AppTypes, KeyMap};

pub struct KeyLineWidget<'a, A: AppTypes> {
    pub keys: Vec<(Cow<'a, str>, AppTrieEntry<A>)>,
    pub map: &'a KeyMap<A>,
}

impl<'a, A: AppTypes> KeyLineWidget<'a, A> {
    pub fn new<S>(keys: Vec<(S, AppTrieEntry<A>)>, map: &'a KeyMap<A>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            keys: keys
                .into_iter()
                .map(|(key, entry)| (key.into(), entry))
                .collect(),
            map,
        }
    }
}

impl<'a, A> WidgetRef for KeyLineWidget<'a, A>
where
    A: AppTypes,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let base = Style::default().fg(Color::Gray);
        let entry_style = base.fg(Color::Yellow);
        let sep_style = base.fg(Color::White);

        let mut spans = Vec::new();

        for (desc, key) in self.keys.iter() {
            let Some(keys) = self.map.get_merged(key, entry_style, sep_style) else {
                continue;
            };

            if !spans.is_empty() {
                spans.push(Span::styled(" | ", base));
            }

            spans.push(Span::styled(format!("{desc}: "), base));
            spans.extend(keys);
        }

        Line::from(spans).render(area, buf);
    }
}
