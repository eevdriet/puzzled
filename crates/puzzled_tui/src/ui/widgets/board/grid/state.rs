use crate::GridRenderState;

pub struct GridWidgetState<'a, C> {
    pub render: &'a mut GridRenderState,
    pub cell_state: &'a mut C,
}

impl<'a, C> GridWidgetState<'a, C> {
    pub fn new(render: &'a mut GridRenderState, cell_state: &'a mut C) -> Self {
        Self { render, cell_state }
    }
}
