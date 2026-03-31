use crate::{GridWidgetState, SidedGridRenderState, SidesRenderState};

pub struct SidedGridWidgetState<'a, C, E> {
    pub grid: GridWidgetState<'a, C>,
    pub sides: &'a mut SidesRenderState,
    pub edge_state: &'a mut E,
}

impl<'a, C, E> SidedGridWidgetState<'a, C, E> {
    pub fn new(
        render: &'a mut SidedGridRenderState,
        cell_state: &'a mut C,
        edge_state: &'a mut E,
    ) -> Self {
        Self {
            grid: GridWidgetState::new(&mut render.grid, cell_state),
            sides: &mut render.sides,
            edge_state,
        }
    }

    pub fn render_state(&self) -> SidedGridRenderState {
        SidedGridRenderState {
            grid: self.grid.render.clone(),
            sides: *self.sides,
        }
    }
}
