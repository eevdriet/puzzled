use puzzled_core::{Direction, Side};
use tui_scrollview::ScrollViewState;

#[derive(Debug, Clone, Copy, Default)]
pub struct SideRenderState {
    pub margin: u16,
    pub direction: Direction,
    pub max_len: Option<u16>,

    pub scroll: ScrollViewState,
}

#[derive(Debug, Clone, Copy)]
pub struct SidesRenderState {
    top: SideRenderState,
    right: SideRenderState,
    bottom: SideRenderState,
    left: SideRenderState,

    focus: Option<Direction>,
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
            focus: None,
        }
    }

    pub fn top(mut self, mut state: SideRenderState) -> Self {
        state.direction = Direction::Up;
        self.top = state;

        self
    }

    pub fn right(mut self, mut state: SideRenderState) -> Self {
        state.direction = Direction::Right;
        self.right = state;

        self
    }
    pub fn bottom(mut self, mut state: SideRenderState) -> Self {
        state.direction = Direction::Down;
        self.bottom = state;

        self
    }
    pub fn left(mut self, mut state: SideRenderState) -> Self {
        state.direction = Direction::Left;
        self.left = state;

        self
    }

    pub fn vertical(self, state: SideRenderState) -> Self {
        self.top(state).bottom(state)
    }

    pub fn horizontal(self, state: SideRenderState) -> Self {
        self.left(state).right(state)
    }

    pub fn upper(self, state: SideRenderState) -> Self {
        self.top(state).left(state)
    }

    pub fn lower(self, state: SideRenderState) -> Self {
        self.bottom(state).right(state)
    }

    pub fn get(&self, dir: Side) -> &SideRenderState {
        match dir {
            Side::Top => &self.top,
            Side::Right => &self.right,
            Side::Bottom => &self.bottom,
            Side::Left => &self.left,
        }
    }

    pub fn get_mut(&mut self, dir: Direction) -> &mut SideRenderState {
        match dir {
            Direction::Up => &mut self.top,
            Direction::Right => &mut self.right,
            Direction::Down => &mut self.bottom,
            Direction::Left => &mut self.left,
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
