use puzzled_core::Direction;

#[derive(Debug, Clone, Copy, Default)]
pub struct SideRenderState {
    pub margin: u16,
    pub cursor: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SidesRenderState {
    top: SideRenderState,
    right: SideRenderState,
    bottom: SideRenderState,
    left: SideRenderState,
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

    pub fn top(mut self, state: SideRenderState) -> Self {
        self.top = state;
        self
    }

    pub fn right(mut self, state: SideRenderState) -> Self {
        self.right = state;
        self
    }
    pub fn bottom(mut self, state: SideRenderState) -> Self {
        self.bottom = state;
        self
    }
    pub fn left(mut self, state: SideRenderState) -> Self {
        self.left = state;
        self
    }

    pub fn vertical(mut self, state: SideRenderState) -> Self {
        self.top = state;
        self.bottom = state;

        self
    }

    pub fn horizontal(mut self, state: SideRenderState) -> Self {
        self.left = state;
        self.right = state;

        self
    }

    pub fn upper(mut self, state: SideRenderState) -> Self {
        self.top = state;
        self.left = state;

        self
    }

    pub fn lower(mut self, state: SideRenderState) -> Self {
        self.bottom = state;
        self.right = state;

        self
    }

    pub fn get(&self, dir: Direction) -> &SideRenderState {
        match dir {
            Direction::Up => &self.top,
            Direction::Right => &self.right,
            Direction::Down => &self.bottom,
            Direction::Left => &self.left,
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
}

impl Default for SidesRenderState {
    fn default() -> Self {
        Self {
            top: SideRenderState::default(),
            right: SideRenderState {
                margin: 1,
                ..Default::default()
            },
            bottom: SideRenderState::default(),
            left: SideRenderState {
                margin: 1,
                ..Default::default()
            },
        }
    }
}
