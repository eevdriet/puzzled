use std::fmt::Debug;

use puzzled_core::Grid;

use crate::{Action, GridRenderState, SelectionKind};

pub trait HandleBaseAction<A, S> {
    fn handle_base_action(&mut self, _action: Action<A>, state: &mut S) -> bool;
}

pub trait HandleCustomAction<A, S> {
    fn handle_custom_action(&mut self, _action: A, _state: &mut S) -> bool;
}

impl<A, T> HandleBaseAction<A, GridRenderState> for Grid<T>
where
    A: Debug,
{
    fn handle_base_action(&mut self, action: Action<A>, state: &mut GridRenderState) -> bool {
        tracing::info!("Handling grid base action: {action:?}");

        match action {
            Action::StartSelection { pos, additive } => {
                state.selection.start(pos, SelectionKind::Cells, additive);

                if let Some(cursor) = state.to_grid(pos) {
                    state.cursor = cursor;
                }
            }
            Action::EndSelection => {
                state.selection.finish();
            }
            _ => return false,
        }

        true
    }
}
