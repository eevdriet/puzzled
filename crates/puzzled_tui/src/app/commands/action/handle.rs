use std::fmt::Debug;

use puzzled_core::Grid;

use crate::{Action, GridRenderState};

pub trait HandleBaseAction<A, S, S2> {
    fn handle_action(&mut self, action: Action<A>, state: &mut S, custom_state: &mut S2) -> bool;
}

pub trait HandleCustomAction<A, S, S2> {
    fn handle_custom_action(&mut self, action: A, state: &mut S, custom_state: &mut S2) -> bool;
}

impl<T, S, S2> HandleCustomAction<(), S, S2> for T {
    fn handle_custom_action(
        &mut self,
        _action: (),
        _state: &mut S,
        _custom_state: &mut S2,
    ) -> bool {
        true
    }
}

impl<A, T, S> HandleBaseAction<A, GridRenderState, S> for Grid<T>
where
    A: Debug,
    Grid<T>: HandleCustomAction<A, GridRenderState, S>,
{
    fn handle_action(
        &mut self,
        action: Action<A>,
        state: &mut GridRenderState,
        custom_state: &mut S,
    ) -> bool {
        tracing::info!("Handling grid base action: {action:?}");

        match action {
            Action::StartSelection {
                pos,
                additive,
                kind,
            } => match pos {
                Some(start) => {
                    if let Some(cursor) = state.to_grid(start) {
                        tracing::info!("Starting selection @ {cursor}");
                        state.cursor = cursor;
                        state.selection.start(start, kind, additive);
                    }
                }
                None => {
                    if let Some(start) = state.to_app(state.cursor) {
                        state.selection.start(start, kind, additive);
                    }
                }
            },
            Action::EndSelection => {
                state.selection.finish();
            }
            Action::Custom(custom) => {
                return self.handle_custom_action(custom, state, custom_state);
            }
            _ => return false,
        }

        true
    }
}
