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
