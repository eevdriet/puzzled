use puzzled_core::{
    Entry, Grid, GridState, Position, Puzzle, Square, SquareGridRef, SquareGridState,
};

use crate::{
    ActionHistory, AppResolver, AppTypes, Command, EntryAction, EntryChange, EventMode,
    GridRenderState, HandleCustomMotion, HandleMotion, Operator,
};

pub trait HandleOperator {
    type Position;

    fn handle_operator<I>(&mut self, op: Operator, positions: I, state: &mut ActionHistory<Self>)
    where
        Self: Sized,
        I: IntoIterator<Item = Self::Position>;
}

impl<P> HandleOperator for GridState<P>
where
    P: Puzzle<Position = Position> + 'static,
{
    type Position = Position;

    fn handle_operator<I>(&mut self, op: Operator, positions: I, state: &mut ActionHistory<Self>)
    where
        I: IntoIterator<Item = Self::Position>,
    {
        let changes = positions
            .into_iter()
            .filter_map(|pos| {
                let entry = self.entries.get(pos)?;
                let before = entry.entry().cloned();
                let after = match op {
                    Operator::Reveal => self.solutions.get(pos).and_then(|sol| sol.clone()),
                    _ => None,
                };

                Some(EntryChange { pos, before, after })
            })
            .collect();

        let action = Box::new(EntryAction::<P>::new(op, changes));
        state.execute(action, self);
    }
}

impl<P> HandleOperator for SquareGridState<P>
where
    P: Puzzle<Position = Position> + 'static,
{
    type Position = Position;

    fn handle_operator<I>(&mut self, op: Operator, positions: I, state: &mut ActionHistory<Self>)
    where
        I: IntoIterator<Item = Self::Position>,
    {
        let changes = positions
            .into_iter()
            .filter_map(|pos| {
                let entry = self.entries.get_fill(pos)?;
                let before = entry.entry().cloned();
                let after = match op {
                    Operator::Delete | Operator::Change => None,
                    Operator::Reveal => self.solutions.get_fill(pos).and_then(|sol| sol.clone()),
                    _ => None,
                };

                Some(EntryChange { pos, before, after })
            })
            .collect();

        let action = Box::new(EntryAction::new(op, changes));
        state.execute(action, self);
    }
}

pub fn handle_grid_command<A: AppTypes, S>(
    command: Command<A::Action, A::TextObject, A::Motion>,
    resolver: AppResolver<A>,
    render: &mut GridRenderState,
    solve: &mut GridState<A::Puzzle>,
    custom_state: &mut S,
    history: &mut ActionHistory<GridState<A::Puzzle>>,
) -> bool
where
    A::Puzzle: Puzzle<Position = Position> + 'static,
    // NOTE: Entries grid can handle a motion with custom state S
    Grid<Entry<<A::Puzzle as Puzzle>::Value>>:
        HandleMotion<A::Motion, GridRenderState, S, Position>,
{
    let (positions, op) = match command {
        Command::Operator(op) => {
            // Use all visually selected positions
            if render.mode.is_visual() {
                let positions: Vec<_> = render
                    .selection
                    .positions(render.viewport)
                    .filter_map(|pos| render.to_grid(pos))
                    .collect();

                (positions, Some(op))
            } else if !op.requires_motion() {
                let positions = vec![render.cursor];
                (positions, Some(op))
            } else {
                return false;
            }
        }
        Command::Motion { count, motion, op } => {
            let entries = &solve.entries;
            let positions: Vec<_> = entries
                .handle_motion(count, motion, render, custom_state)
                .into_iter()
                .collect();

            (positions, op)
        }
        _ => return false,
    };

    if let Some(op) = op {
        solve.handle_operator(op, positions, history);

        // Possibly change the mode after applying the operator
        let mode = match op {
            Operator::Change => EventMode::Insert,
            _ => EventMode::Normal,
        };

        resolver.set_mode(mode);
    }

    true
}

pub fn handle_square_grid_command<'a, A, S>(
    command: Command<A::Action, A::TextObject, A::Motion>,
    resolver: AppResolver<A>,
    render: &mut GridRenderState,
    solve: &mut SquareGridState<A::Puzzle>,
    custom_state: &mut S,
    history: &mut ActionHistory<SquareGridState<A::Puzzle>>,
) -> bool
where
    A: AppTypes,
    A::Puzzle: Puzzle<Position = Position> + 'static,
    // NOTE: Entries grid can handle a motion with custom state S
    SquareGridRef<'a, <A::Puzzle as Puzzle>::Value>:
        HandleMotion<A::Motion, GridRenderState, S, Position>,
    Grid<Square<Entry<<A::Puzzle as Puzzle>::Value>>>:
        HandleCustomMotion<A::Motion, GridRenderState, S, Position>,
{
    let (positions, op) = match command {
        Command::Operator(op) => {
            // Use all visually selected positions
            if render.mode.is_visual() {
                let positions: Vec<_> = render
                    .selection
                    .positions(render.viewport)
                    .filter_map(|pos| render.to_grid(pos))
                    .collect();

                (positions, Some(op))
            } else if !op.requires_motion() {
                let positions = vec![render.cursor];
                (positions, Some(op))
            } else {
                return false;
            }
        }
        Command::Motion { count, motion, op } => {
            let grid = SquareGridRef(&solve.entries);

            let positions: Vec<_> = grid
                .handle_motion(count, motion, render, custom_state)
                .into_iter()
                .collect();

            (positions, op)
        }
        _ => return false,
    };

    if let Some(op) = op {
        solve.handle_operator(op, positions, history);

        // Possibly change the mode after applying the operator
        let mode = match op {
            Operator::Change => EventMode::Insert,
            _ => EventMode::Normal,
        };

        resolver.set_mode(mode);
    }

    true
}
