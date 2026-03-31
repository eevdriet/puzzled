mod fills;

pub use fills::*;

use puzzled_nonogram::{Colors, Fill};
use puzzled_tui::{Action, AppContext, TrieEntry, Widget as AppWidget};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::{Buffer, Rect, Size},
    text::{Line, Span},
    widgets::Widget,
};

use crate::NonogramApp;

#[derive(Debug)]
pub struct FooterWidget<'a> {
    fills: FillsWidget<'a>,
}

pub struct FooterState<'a> {
    pub colors: &'a Colors,
    pub fill: &'a Fill,
}

impl<'a> AppWidget<NonogramApp> for FooterWidget<'a> {
    type State = FooterState<'a>;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<NonogramApp>,
        state: &mut Self::State,
    ) {
        let fills_size = self.fills.render_size(root, ctx, state);

        let [help_line, _stats_line, fills_line, _progress_line] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(fills_size.height),
            Constraint::Length(1),
        ])
        .areas(root);

        self.render_help(help_line, buf, ctx, state);
        self.fills.render(fills_line, buf, ctx, state);
        // self.render_stats(stats_line, buf, state);
        // self.render_progress(progress_line, buf, state);
    }

    fn render_size(
        &self,
        area: Rect,
        _ctx: &AppContext<NonogramApp>,
        _state: &mut Self::State,
    ) -> Size {
        Size::new(area.width, 3)
    }
}

impl<'a> FooterWidget<'a> {
    pub const HEIGHT: u16 = 4;

    pub fn new() -> Self {
        Self {
            fills: FillsWidget::default(),
        }
    }

    fn render_help(
        &self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<NonogramApp>,
        _state: &mut FooterState<'a>,
    ) {
        let entry = TrieEntry::Action(Action::Cancel);
        let pause_key = ctx.keys.get_merged_str(&entry).unwrap_or_default();

        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" ? ", ctx.theme.primary),
            Span::raw("for help and "),
            Span::styled(pause_key, ctx.theme.primary),
            Span::raw(" to pause the puzzle"),
        ])
        .alignment(Alignment::Center)
        .render(area, buf);
    }

    // fn render_stats(&self, area: Rect, buf: &mut Buffer, state: &mut FooterState<'a>) {}
    // fn render_progress(&self, area: Rect, buf: &mut Buffer, state: &mut FooterState<'a>) {}
}

impl<'a> Default for FooterWidget<'a> {
    fn default() -> Self {
        Self::new()
    }
}
