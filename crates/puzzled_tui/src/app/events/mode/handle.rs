use puzzled_core::Grid;

use crate::{AppContext, AppResolver, AppTypes, EventMode, GridRenderState};

pub trait HandleMode<A: AppTypes> {
    type State;

    fn handle_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        _state: &mut Self::State,
    ) -> bool;
}

impl<C, A: AppTypes> HandleMode<A> for Grid<C> {
    type State = GridRenderState;

    fn handle_mode(
        &mut self,
        mode: EventMode,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) -> bool {
        tracing::info!("Changing grid mode {} -> {}", state.mode, mode);
        state.mode = mode;

        if !mode.is_visual() {
            if let Some(active) = state.selection.active()
                && let Some(start) = active.start()
                && let Some(cursor) = state.to_grid(start)
            {
                state.cursor = cursor;
            }

            state.selection.reset();
        }

        true
    }
}
