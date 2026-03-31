use puzzled_core::{Direction, Position, Side};
use ratatui::layout::Rect;
use tui_scrollview::ScrollViewState;

use crate::Selection;

#[derive(Debug, Clone, Copy, Default)]
pub struct SideRenderState<E = ()> {
    pub edge_state: E,

    pub margin: u16,
    pub direction: Direction,
    pub max_len: Option<u16>,

    pub selection: Selection,
    pub cursor: Position,
    pub viewport: Rect,

    pub scroll: ScrollViewState,
}

#[derive(Debug, Clone, Copy)]
pub struct SidesRenderState<E = ()> {
    pub top: SideRenderState<E>,
    pub right: SideRenderState<E>,
    pub bottom: SideRenderState<E>,
    pub left: SideRenderState<E>,

    focus: Option<Direction>,
}

impl<E> SidesRenderState<E> {
    pub fn new(
        top: SideRenderState<E>,
        right: SideRenderState<E>,
        bottom: SideRenderState<E>,
        left: SideRenderState<E>,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
            focus: None,
        }
    }

    pub fn top(mut self, mut state: SideRenderState<E>) -> Self {
        state.direction = Direction::Up;
        self.top = state;

        self
    }

    pub fn right(mut self, mut state: SideRenderState<E>) -> Self {
        state.direction = Direction::Right;
        self.right = state;

        self
    }
    pub fn bottom(mut self, mut state: SideRenderState<E>) -> Self {
        state.direction = Direction::Down;
        self.bottom = state;

        self
    }
    pub fn left(mut self, mut state: SideRenderState<E>) -> Self {
        state.direction = Direction::Left;
        self.left = state;

        self
    }

    pub fn get(&self, dir: Side) -> &SideRenderState<E> {
        match dir {
            Side::Top => &self.top,
            Side::Right => &self.right,
            Side::Bottom => &self.bottom,
            Side::Left => &self.left,
        }
    }

    pub fn get_mut(&mut self, side: Side) -> &mut SideRenderState<E> {
        match side {
            Side::Top => &mut self.top,
            Side::Right => &mut self.right,
            Side::Bottom => &mut self.bottom,
            Side::Left => &mut self.left,
        }
    }

    pub fn focus_grid(&mut self) {
        self.focus = None
    }

    pub fn focus_side(&mut self, side: Direction) {
        self.focus = Some(side)
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
            right: base,
            bottom: base,
            left: state,
            focus: None,
        }
    }
}
