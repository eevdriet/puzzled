use std::marker::PhantomData;

use crossterm::event::MouseEventKind;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{
    Block, HighlightSpacing, List, ListDirection, ListItem, ListState, StatefulWidget,
};

use crate::{AppCommand, AppContext, AppResolver, AppTypes, Command, Motion, Widget as AppWidget};

pub trait ListRender<A: AppTypes>: Sized {
    type State;

    fn render_items<'a>(
        &self,
        ctx: &AppContext<A>,
        state: &'a Self::State,
    ) -> impl Iterator<Item = ListItem<'a>>;

    fn render_list<'a>(
        &self,
        list: List<'a>,
        _ctx: &AppContext<A>,
        _state: &'a Self::State,
    ) -> List<'a> {
        list
    }

    fn on_command(
        &mut self,
        _command: AppCommand<A>,
        _resolver: AppResolver<A>,
        _ctx: &AppContext<A>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}

pub trait ListRenderState {
    fn get(&self) -> ListState;
    fn set(&mut self, state: ListState);
}

impl ListRenderState for ListState {
    fn get(&self) -> ListState {
        *self
    }
    fn set(&mut self, state: ListState) {
        *self = state;
    }
}

pub struct ListWidget<A, R> {
    // Render
    pub render: R,
    pub _r: PhantomData<A>,

    // Style
    pub block: Option<Block<'static>>,
    pub style: Style,
    pub direction: ListDirection,
    pub highlight_style: Style,
    pub highlight_symbol: Option<Line<'static>>,
    pub repeat_highlight_symbol: bool,
    pub highlight_spacing: HighlightSpacing,
    pub scroll_padding: usize,
}

impl<A, R> ListWidget<A, R>
where
    A: AppTypes,
    R: ListRender<A>,
{
    pub fn new(render: R) -> Self {
        Self {
            render,
            _r: PhantomData,

            block: None,
            style: Style::default(),
            direction: ListDirection::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            repeat_highlight_symbol: false,
            highlight_spacing: HighlightSpacing::default(),
            scroll_padding: 0,
        }
    }

    pub fn block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style<St: Into<Style>>(mut self, style: St) -> Self {
        self.style = style.into();
        self
    }

    pub fn highlight_symbol<L: Into<Line<'static>>>(mut self, highlight_symbol: L) -> Self {
        self.highlight_symbol = Some(highlight_symbol.into());
        self
    }

    pub fn highlight_style<St: Into<Style>>(mut self, style: St) -> Self {
        self.highlight_style = style.into();
        self
    }

    pub const fn repeat_highlight_symbol(mut self, repeat: bool) -> Self {
        self.repeat_highlight_symbol = repeat;
        self
    }

    pub const fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    pub const fn direction(mut self, direction: ListDirection) -> Self {
        self.direction = direction;
        self
    }

    pub const fn scroll_padding(mut self, padding: usize) -> Self {
        self.scroll_padding = padding;
        self
    }
}

impl<A, R> ListWidget<A, R>
where
    A: AppTypes,
{
    pub fn create_list(&self) -> List<'static> {
        let mut list = List::default();

        // Optional properties
        if let Some(block) = &self.block {
            list = list.block(block.clone());
        }
        if let Some(symbol) = &self.highlight_symbol {
            list = list.highlight_symbol(symbol.clone());
        }

        // Other properties
        list.style(self.style)
            .direction(self.direction)
            .highlight_style(self.highlight_style)
            .repeat_highlight_symbol(self.repeat_highlight_symbol)
            .highlight_spacing(self.highlight_spacing.clone())
            .scroll_padding(self.scroll_padding)
    }
}

#[derive(Debug)]
pub struct ListContext<'a, S> {
    pub state: S,
    pub list: &'a mut ListState,
}

impl<'a, S> ListContext<'a, S> {
    pub fn new(state: S, list: &'a mut ListState) -> Self {
        Self { state, list }
    }
}

impl<A, R> AppWidget<A> for ListWidget<A, R>
where
    A: AppTypes,
    R: ListRender<A>,
    R::State: ListRenderState,
{
    type State = R::State;

    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        let mut list_state = state.get();

        {
            // We pass `state` which has the lifetime `'a` into `render_state`
            let items = self.render.render_items(ctx, state);
            let list = self.create_list().items(items);
            let list = self.render.render_list(list, ctx, state);

            // Now we use the correct `ListState` reference
            list.render(area, buf, &mut list_state);
        }

        state.set(list_state);
    }

    fn render_size(&self, _area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Size {
        let items = self.render.render_items(ctx, state);

        items.fold(Size::ZERO, |mut size, item| {
            size.width = size.width.max(item.width() as u16);
            size.height += item.height() as u16;

            size
        })
    }

    fn on_command(
        &mut self,
        command: AppCommand<A>,
        resolver: AppResolver<A>,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) -> bool {
        if self
            .render
            .on_command(command.clone(), resolver, ctx, state)
        {
            return true;
        }

        let mut list = state.get();

        match command {
            Command::Motion { count, motion, .. } => {
                let count = count as u16;

                match motion {
                    Motion::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollDown => list.scroll_down_by(count),
                        MouseEventKind::ScrollUp => list.scroll_up_by(count),
                        _ => return false,
                    },
                    Motion::ColStart => list.select_first(),
                    Motion::ColEnd => list.select_last(),
                    Motion::Down => list.scroll_down_by(count),
                    Motion::Up => list.scroll_up_by(count),
                    _ => return false,
                }
            }
            _ => return false,
        }

        state.set(list);
        true
    }
}
