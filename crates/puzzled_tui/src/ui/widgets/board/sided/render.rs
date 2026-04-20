use puzzled_core::{Position, Side};
use ratatui::layout::Rect;

use crate::{GridRenderState, ScrollViewState, Selection};

pub struct SidedGridRenderState {
    pub grid: GridRenderState,
    pub sides: SidesRenderState,
}

impl SidedGridRenderState {
    pub fn new(grid: GridRenderState, sides: SidesRenderState) -> Self {
        Self { grid, sides }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SideRenderState {
    pub margin: u16,
    pub side: Side,
    pub max_len: Option<u16>,

    pub selection: Selection,
    pub cursor: Position,
    pub viewport: Rect,

    pub scroll: ScrollViewState,
}

#[derive(Debug, Clone, Copy)]
pub struct SidesRenderState {
    pub top: SideRenderState,
    pub right: SideRenderState,
    pub bottom: SideRenderState,
    pub left: SideRenderState,
}

impl SidesRenderState {
    pub fn new(
        top: SideRenderState,
        right: SideRenderState,
        bottom: SideRenderState,
        left: SideRenderState,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn top(mut self, mut state: SideRenderState) -> Self {
        state.side = Side::Top;
        self.top = state;

        self
    }

    pub fn right(mut self, mut state: SideRenderState) -> Self {
        state.side = Side::Right;
        self.right = state;

        self
    }
    pub fn bottom(mut self, mut state: SideRenderState) -> Self {
        state.side = Side::Bottom;
        self.bottom = state;

        self
    }
    pub fn left(mut self, mut state: SideRenderState) -> Self {
        state.side = Side::Left;
        self.left = state;

        self
    }

    pub fn get(&self, side: Side) -> &SideRenderState {
        match side {
            Side::Top => &self.top,
            Side::Right => &self.right,
            Side::Bottom => &self.bottom,
            Side::Left => &self.left,
        }
    }

    pub fn get_mut(&mut self, side: Side) -> &mut SideRenderState {
        match side {
            Side::Top => &mut self.top,
            Side::Right => &mut self.right,
            Side::Bottom => &mut self.bottom,
            Side::Left => &mut self.left,
        }
    }
}

impl Default for SidesRenderState {
    fn default() -> Self {
        let base = SideRenderState::default();
        let state = SideRenderState {
            margin: 1,
            ..Default::default()
        };

        Self {
            top: base,
            right: state,
            bottom: base,
            left: state,
        }
    }
}
