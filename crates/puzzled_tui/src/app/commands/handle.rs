use std::fmt::Debug;

use puzzled_core::{
    Entry, Grid, GridState, Position, Puzzle, Square, SquareGridRef, SquareGridState,
};
use ratatui::layout::Rect;

use crate::{
    AppCommand, AppContext, AppResolver, AppTypes, AsCore, Command, EntryAction, EventMode,
    GridRenderState, HandleCustomMotion, HandleMotion, HandleOperator, Operator, Screen,
};

pub enum CommandOutcome<A: AppTypes> {
    // Handled externally
    Command(AppCommand<A>),
    Mode(EventMode),

    // Screen management
    Quit,
    PreviousScreen,
    NextScreen(Box<dyn Screen<A>>),

    // Popup management
    OpenPopup,
    ClosePopup,
}

pub trait HandleCommand<A: AppTypes> {
    type State;

    fn handle_command(
        &mut self,
        _command: AppCommand<A>,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        _state: &mut Self::State,
    ) -> bool;
}

pub fn handle_grid_command<A: AppTypes, MS>(
    command: Command<A::Action, A::TextObject, A::Motion>,
    resolver: AppResolver<A>,
    render: &mut GridRenderState,
    solve: &mut GridState<A::Puzzle>,
    motion_state: &mut MS,
    fill: Option<<A::Puzzle as Puzzle>::Value>,
) -> Option<Box<EntryAction<A::Puzzle>>>
where
    A::Puzzle: Puzzle<Position = Position> + 'static,
    // NOTE: Entries grid can handle a motion with custom state S
    Grid<Entry<<A::Puzzle as Puzzle>::Value>>:
        HandleMotion<A::Motion, GridRenderState, MS, Position>,
{
    let (positions, op) = match command {
        Command::Operator(op) => {
            let grid_area = Rect::from(render.size());

            // Use all visually selected positions
            if render.mode.is_visual() {
                let positions: Vec<_> = render
                    .selection
                    .positions(grid_area)
                    .map(|pos| pos.as_core())
                    .collect();

                (positions, Some(op))
            } else if !op.requires_motion() {
                let positions = vec![render.cursor];
                (positions, Some(op))
            } else {
                return None;
            }
        }
        Command::Motion { count, motion, op } => {
            let entries = &solve.entries;
            let positions: Vec<_> = entries
                .handle_motion(count, motion, render, motion_state)
                .into_iter()
                .collect();

            (positions, op)
        }
        _ => return None,
    };

    op.map(|op| {
        let action = solve.handle_operator(op, positions, &fill);

        // Possibly change the mode after applying the operator
        let mode = match op {
            Operator::Change => EventMode::Insert,
            _ => EventMode::Normal,
        };

        resolver.set_mode(mode);
        action
    })
}

pub fn handle_square_grid_command<'a, A, MS>(
    command: Command<A::Action, A::TextObject, A::Motion>,
    resolver: AppResolver<A>,
    render: &mut GridRenderState,
    solve: &mut SquareGridState<A::Puzzle>,
    motion_state: &mut MS,
    fill: Option<<A::Puzzle as Puzzle>::Value>,
) -> Option<Box<EntryAction<A::Puzzle>>>
where
    A: AppTypes,
    A::Puzzle: Puzzle<Position = Position> + 'static,
    <A::Puzzle as Puzzle>::Value: Debug,
    // NOTE: Entries grid can handle a motion with custom state S
    SquareGridRef<'a, <A::Puzzle as Puzzle>::Value>:
        HandleMotion<A::Motion, GridRenderState, MS, Position>,
    Grid<Square<Entry<<A::Puzzle as Puzzle>::Value>>>:
        HandleCustomMotion<A::Motion, GridRenderState, MS, Position>,
{
    render.dirty.clear();

    let (positions, op) = match command {
        Command::Operator(op) => {
            // Use all visually selected positions
            if render.mode.is_visual() {
                let area = Rect::from(render.size());
                let positions: Vec<_> = render
                    .selection
                    .positions(area)
                    .map(|pos| pos.as_core())
                    .collect();

                (positions, Some(op))
            } else if !op.requires_motion() {
                let positions = vec![render.cursor];
                (positions, Some(op))
            } else {
                return None;
            }
        }
        Command::Motion { count, motion, op } => {
            let grid = SquareGridRef(&solve.entries);

            let positions: Vec<_> = grid
                .handle_motion(count, motion, render, motion_state)
                .into_iter()
                .collect();

            (positions, op)
        }
        _ => return None,
    };

    render.dirty = positions;

    op.map(|op| {
        let action = solve.handle_operator(op, render.dirty.iter().cloned(), &fill);

        // Possibly change the mode after applying the operator
        let mode = match op {
            Operator::Change => EventMode::Insert,
            _ => EventMode::Normal,
        };

        resolver.set_mode(mode);
        action
    })
}
