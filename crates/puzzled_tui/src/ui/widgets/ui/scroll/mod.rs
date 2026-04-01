mod bar;
mod state;

pub use state::*;

use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget},
};
use tui_scrollview::ScrollbarVisibility;

use crate::{AppContext, AppTypes, Widget as AppWidget};

#[derive(Debug, Default, Clone)]
pub struct ScrollWidget {
    buf: Buffer,
    size: Size,

    pub vertical_scrollbar_visibility: ScrollbarVisibility,
    pub horizontal_scrollbar_visibility: ScrollbarVisibility,
}

impl ScrollWidget {
    pub fn new(size: Size) -> Self {
        let area = Rect::from(size);

        Self {
            buf: Buffer::empty(area),
            size,
            vertical_scrollbar_visibility: ScrollbarVisibility::default(),
            horizontal_scrollbar_visibility: ScrollbarVisibility::default(),
        }
    }

    pub fn set_size(&mut self, size: Size) {
        if self.size != size {
            tracing::info!("NOPE NOEP: {} <-> {size}", self.size);
            self.size = size;

            let area = Rect::from(size);
            self.buf = Buffer::empty(area);
        }
    }

    /// The content size of the scroll view
    pub const fn size(&self) -> Size {
        self.size
    }

    /// The area of the buffer that is available to be scrolled
    pub const fn area(&self) -> Rect {
        self.buf.area
    }

    /// The buffer containing the contents of the scroll view
    pub const fn buf(&self) -> &Buffer {
        &self.buf
    }

    /// The mutable buffer containing the contents of the scroll view
    ///
    /// This can be used to render widgets into the buffer programmatically
    pub const fn buf_mut(&mut self) -> &mut Buffer {
        &mut self.buf
    }

    /// Set the visibility of the vertical scrollbar
    ///
    /// See [`ScrollbarVisibility`] for all the options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    pub const fn vertical_scrollbar_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.vertical_scrollbar_visibility = visibility;
        self
    }

    /// Set the visibility of the horizontal scrollbar
    ///
    /// See [`ScrollbarVisibility`] for all the options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    pub const fn horizontal_scrollbar_visibility(
        mut self,
        visibility: ScrollbarVisibility,
    ) -> Self {
        self.horizontal_scrollbar_visibility = visibility;
        self
    }

    /// Set the visibility of both vertical and horizontal scrollbars
    ///
    /// See [`ScrollbarVisibility`] for all the options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    pub const fn scrollbars_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.vertical_scrollbar_visibility = visibility;
        self.horizontal_scrollbar_visibility = visibility;
        self
    }
}

impl ScrollWidget {
    /// Render needed scrollbars and return remaining area relative to
    /// scrollview's buffer area.
    fn render_scrollbars(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) -> Rect {
        // fit value per direction
        //   > 0 => fits
        //  == 0 => exact fit
        //   < 0 => does not fit
        let horizontal_space = area.width as i32 - self.size.width as i32;
        let vertical_space = area.height as i32 - self.size.height as i32;

        // if it fits in that direction, reset state to reflect it
        if horizontal_space > 0 {
            state.offset.x = 0;
        }
        if vertical_space > 0 {
            state.offset.y = 0;
        }

        let (show_horizontal, show_vertical) =
            self.visible_scrollbars(horizontal_space, vertical_space);

        let new_height = if show_horizontal {
            // if both bars are rendered, avoid the corner
            let width = area.width.saturating_sub(show_vertical as u16);
            let render_area = Rect { width, ..area };
            // render scrollbar, update available space
            self.render_horizontal_scrollbar(render_area, buf, state);
            area.height.saturating_sub(1)
        } else {
            area.height
        };

        let new_width = if show_vertical {
            // if both bars are rendered, avoid the corner
            let height = area.height.saturating_sub(show_horizontal as u16);
            let render_area = Rect { height, ..area };
            // render scrollbar, update available space
            self.render_vertical_scrollbar(render_area, buf, state);
            area.width.saturating_sub(1)
        } else {
            area.width
        };

        Rect::new(state.offset.x, state.offset.y, new_width, new_height)
    }

    /// Resolve whether to render each scrollbar.
    ///
    /// Considers the visibility options set by the user and whether the scrollview size fits into
    /// the the available area on each direction.
    ///
    /// The space arguments are the difference between the scrollview size and the available area.
    ///
    /// Returns a bool tuple with (horizontal, vertical) resolutions.
    const fn visible_scrollbars(&self, horizontal_space: i32, vertical_space: i32) -> (bool, bool) {
        type V = ScrollbarVisibility;

        match (
            self.horizontal_scrollbar_visibility,
            self.vertical_scrollbar_visibility,
        ) {
            // straightfoward, no need to check fit values
            (V::Always, V::Always) => (true, true),
            (V::Never, V::Never) => (false, false),
            (V::Always, V::Never) => (true, false),
            (V::Never, V::Always) => (false, true),

            // Auto => render scrollbar only if it doesn't fit
            (V::Automatic, V::Never) => (horizontal_space < 0, false),
            (V::Never, V::Automatic) => (false, vertical_space < 0),

            // Auto => render scrollbar if:
            //   it doesn't fit; or
            //   exact fit (other scrollbar steals a line and triggers it)
            (V::Always, V::Automatic) => (true, vertical_space <= 0),
            (V::Automatic, V::Always) => (horizontal_space <= 0, true),

            // depends solely on fit values
            (V::Automatic, V::Automatic) => {
                if horizontal_space >= 0 && vertical_space >= 0 {
                    // there is enough space for both dimensions
                    (false, false)
                } else if horizontal_space < 0 && vertical_space < 0 {
                    // there is not enough space for either dimension
                    (true, true)
                } else if horizontal_space > 0 && vertical_space < 0 {
                    // horizontal fits, vertical does not
                    (false, true)
                } else if horizontal_space < 0 && vertical_space > 0 {
                    // vertical fits, horizontal does not
                    (true, false)
                } else {
                    // one is an exact fit and other does not fit which triggers both scrollbars to
                    // be visible because the other scrollbar will steal a line from the buffer
                    (true, true)
                }
            }
        }
    }

    fn render_vertical_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let scrollbar_height = self.size.height.saturating_sub(area.height);
        let mut scrollbar_state =
            ScrollbarState::new(scrollbar_height as usize).position(state.offset.y as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }

    fn render_horizontal_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let scrollbar_width = self.size.width.saturating_sub(area.width);
        let mut scrollbar_state =
            ScrollbarState::new(scrollbar_width as usize).position(state.offset.x as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }

    fn render_visible_area(&self, area: Rect, buf: &mut Buffer, visible_area: Rect) {
        // TODO: there's probably a more efficient way to do this
        for (src_row, dst_row) in visible_area.rows().zip(area.rows()) {
            for (src_col, dst_col) in src_row.columns().zip(dst_row.columns()) {
                buf[dst_col] = self.buf[src_col].clone();
            }
        }
    }
}

impl<A> AppWidget<A> for ScrollWidget
where
    A: AppTypes,
{
    type State = ScrollViewState;

    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        _ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        tracing::trace!("Scrollview");
        tracing::trace!("\tRender area: {area:?}");
        tracing::trace!("\tScroll buffer area: {:?}", self.buf.area());

        let (mut x, mut y) = state.offset.into();
        // ensure that we don't scroll past the end of the buffer in either direction
        let max_x_offset = self
            .buf
            .area
            .width
            .saturating_sub(area.width.saturating_sub(1));
        let max_y_offset = self
            .buf
            .area
            .height
            .saturating_sub(area.height.saturating_sub(1));

        x = x.min(max_x_offset);
        y = y.min(max_y_offset);

        state.offset = (x, y).into();
        state.size = Some(self.size);
        state.page_size = Some(area.into());
        let visible_area = self
            .render_scrollbars(area, buf, state)
            .intersection(self.buf.area);

        self.render_visible_area(area, buf, visible_area);
    }

    fn render_size(&self, _area: Rect, _ctx: &AppContext<A>, _state: &mut Self::State) -> Size {
        self.size
    }
}
