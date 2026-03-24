use puzzled_core::{GridState, Position, Puzzle, SquareGridState};

use crate::{
    ActionHistory, AppResolver, AppTypes, EntryAction, EntryChange, EventMode, GridRenderState,
    Operator,
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

pub fn handle_grid_operator<A: AppTypes>(
    op: Operator,
    resolver: AppResolver<A>,
    render: &GridRenderState,
    solve: &mut GridState<A::Puzzle>,
    history: &mut ActionHistory<GridState<A::Puzzle>>,
) -> bool
where
    A::Puzzle: Puzzle<Position = Position> + 'static,
{
    if render.mode.is_visual() {
        let positions = render
            .selection
            .positions(render.viewport)
            .filter_map(|pos| render.to_grid(pos));

        solve.handle_operator(op, positions, history);

        let mode = match op {
            Operator::Change => EventMode::Insert,
            _ => EventMode::Normal,
        };

        resolver.set_mode(mode);
        true
    } else if !op.requires_motion() {
        let positions = vec![render.cursor];

        solve.handle_operator(op, positions, history);
        true
    } else {
        false
    }
}

pub fn handle_square_grid_operator<A: AppTypes>(
    op: Operator,
    resolver: AppResolver<A>,
    render: &GridRenderState,
    solve: &mut SquareGridState<A::Puzzle>,
    history: &mut ActionHistory<SquareGridState<A::Puzzle>>,
) -> bool
where
    A::Puzzle: Puzzle<Position = Position> + 'static,
{
    if render.mode.is_visual() {
        let positions = render
            .selection
            .positions(render.viewport)
            .filter_map(|pos| render.to_grid(pos));

        solve.handle_operator(op, positions, history);

        let mode = match op {
            Operator::Change => EventMode::Insert,
            _ => EventMode::Normal,
        };

        resolver.set_mode(mode);
        true
    } else if !op.requires_motion() {
        let positions = vec![render.cursor];

        solve.handle_operator(op, positions, history);
        true
    } else {
        false
    }
}
