use puzzled_core::Grid;

use crate::{AppContext, AppResolver, EventMode, GridRenderState};

pub trait HandleMode<A, T, M, S> {
    type State;

    fn handle_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
        _state: &mut Self::State,
    ) -> bool;
}

impl<C, A, T, M, S> HandleMode<A, T, M, S> for Grid<C> {
    type State = GridRenderState;

    fn handle_mode(
        &mut self,
        mode: EventMode,
        _resolver: AppResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
        state: &mut Self::State,
    ) -> bool {
        tracing::info!("Changing grid mode {} -> {}", state.mode, mode);
        state.mode = mode;

        let selection = &mut state.selection;
        let cursor = state.cursor;

        match mode {
            EventMode::Visual(kind) => {
                selection.set_kind(kind);
                selection.set(cursor, cursor);
            }
            _ => {
                if let Some(start) = selection.start() {
                    state.cursor = start;
                }

                selection.reset();
            }
        }

        true
    }
}
