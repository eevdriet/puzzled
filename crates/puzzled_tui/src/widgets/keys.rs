use std::borrow::Cow;

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Widget, WidgetRef},
};

use crate::{ActionBehavior, KeyMap, MotionBehavior, TextObjectBehavior, TrieEntry};

pub struct KeyLineWidget<'a, A, T, M> {
    pub keys: Vec<(Cow<'a, str>, TrieEntry<A, T, M>)>,
    pub map: &'a KeyMap<A, T, M>,
}

impl<'a, A, T, M> KeyLineWidget<'a, A, T, M> {
    pub fn new<S>(keys: Vec<(S, TrieEntry<A, T, M>)>, map: &'a KeyMap<A, T, M>) -> Self
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

impl<'a, A, T, M> WidgetRef for KeyLineWidget<'a, A, T, M>
where
    A: ActionBehavior,
    T: TextObjectBehavior,
    M: MotionBehavior,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let base = Style::default().fg(Color::Gray);
        let mut spans = Vec::new();

        for (desc, key) in self.keys.iter() {
            let Some(keys) = self.map.get_merged(key) else {
                continue;
            };
            if !spans.is_empty() {
                spans.push(Span::styled(" | ", base));
            }

            spans.push(Span::styled(format!("{desc}: "), base));
            spans.push(Span::styled(keys, base.fg(Color::Yellow)));
        }

        Line::from(spans).render(area, buf);
    }
}
