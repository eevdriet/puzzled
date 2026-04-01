use puzzled_core::Side;

use crate::{GridWidgetState, SidedGridRenderState};

pub struct SidedGridWidgetState<'a, C, E> {
    pub render: &'a mut SidedGridRenderState,
    pub cell_state: &'a mut C,
    pub edge_state: &'a mut E,
    pub focus: Option<Side>,
}

impl<'a, C, E> SidedGridWidgetState<'a, C, E> {
    pub fn new(
        render: &'a mut SidedGridRenderState,
        cell_state: &'a mut C,
        edge_state: &'a mut E,
    ) -> Self {
        Self {
            render,
            cell_state,
            edge_state,
            focus: None,
        }
    }

    pub fn grid_state(&mut self) -> GridWidgetState<'_, C> {
        GridWidgetState {
            render: &mut self.render.grid,
            cell_state: self.cell_state,
        }
    }

    pub fn focus_grid(&mut self) {
        self.focus = None
    }

    pub fn focus_side(&mut self, side: Side) {
        self.focus = Some(side);
    }
}
