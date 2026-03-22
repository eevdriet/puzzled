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

impl<T, A: AppTypes> HandleMode<A> for Grid<T> {
    type State = GridRenderState;

    fn handle_mode(
        &mut self,
        mode: EventMode,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) -> bool {
        state.mode = mode;

        if !mode.is_visual() {
            state.selection.reset();
        }

        true
    }
}
