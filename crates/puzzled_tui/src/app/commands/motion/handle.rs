use std::fmt::Debug;

use puzzled_core::{
    Direction, Entry, Grid, GridIndexedIter, GridIter, GridLinearIter, GridPositionsIter,
    GridState, Position, Puzzle, Square, SquareGridRef, SquareGridState, Word,
};

use crate::{GridRenderState, Motion, MotionBehavior};

pub trait HandleMotion<M, S, S2, P> {
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        state: &mut S,
        custom_state: &mut S2,
    ) -> impl IntoIterator<Item = P>;
}

pub trait HandleCustomMotion<M, S, S2, P> {
    fn handle_custom_motion(
        &self,
        count: usize,
        motion: M,
        state: &mut S,
        custom_state: &mut S2,
    ) -> impl IntoIterator<Item = P>;
}

impl<T, S, S2, P> HandleCustomMotion<(), S, S2, P> for T {
    fn handle_custom_motion(
        &self,
        _count: usize,
        _motion: (),
        _state: &mut S,
        _custom_state: &mut S2,
    ) -> impl IntoIterator<Item = P> {
        std::iter::empty()
    }
}

impl<M, T, S> HandleMotion<M, GridRenderState, S, Position> for Grid<T>
where
    T: Clone + Debug + Word,
    M: MotionBehavior,
    Grid<T>: HandleCustomMotion<M, GridRenderState, S, Position>,
{
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        render: &mut GridRenderState,
        custom_state: &mut S,
    ) -> impl IntoIterator<Item = Position> {
        // Determine where to start the motion from and in which direction
        let start = render.cursor;
        let next_dir = motion.apply_to_dir(render.direction);

        // Perform the motion by collecting all covered positions in the grid
        let iter = grid_motion(self, count, motion, start, next_dir, render, custom_state);
        perform_motion_from_iter(iter, next_dir, render, |_pos: &Position| true)
    }
}

impl<M, P, S> HandleMotion<M, GridRenderState, S, Position> for GridState<P>
where
    P: Puzzle<Position = Position>,
    M: MotionBehavior,
    Grid<Entry<P::Value>>: HandleMotion<M, GridRenderState, S, Position>,
{
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        state: &mut GridRenderState,
        custom_state: &mut S,
    ) -> impl IntoIterator<Item = Position> {
        self.entries
            .handle_motion(count, motion, state, custom_state)
    }
}

impl<M, T, S> HandleMotion<M, GridRenderState, S, Position> for SquareGridRef<'_, T>
where
    T: Clone + Word + Debug,
    M: MotionBehavior,
    Grid<Square<T>>: HandleCustomMotion<M, GridRenderState, S, Position>,
{
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        render: &mut GridRenderState,
        custom_state: &mut S,
    ) -> impl IntoIterator<Item = Position> {
        // Go to the first filled square in the given direction to perform the motion
        let mut pos = render.cursor;
        let is_visual = render.mode.is_visual();
        let next_dir = motion.apply_to_dir(render.direction);

        if count > 0 && !self.0.is_in_bounds(pos) {
            while let Some(next) = pos + next_dir
                && self.0.is_in_bounds(next)
            {
                // NOTE: since count > 0, some motion always moves the position past the currently non-fill pos
                if self.0.is_fill(next) {
                    break;
                }

                pos = next;
            }
        }

        // Perform the motion by collecting all covered positions in the grid
        let iter = square_grid_motion(self.0, count, motion, pos, next_dir, render, custom_state);
        perform_motion_from_iter(iter, next_dir, render, move |pos: &Position| {
            is_visual || self.0.is_fill(*pos)
        })
    }
}

impl<M, P, S> HandleMotion<M, GridRenderState, S, Position> for SquareGridState<P>
where
    P: Puzzle<Position = Position>,
    M: MotionBehavior,
    Entry<P::Value>: Debug,
    Grid<Square<P::Value>>: HandleMotion<M, GridRenderState, S, Position>,
{
    fn handle_motion(
        &self,
        _count: usize,
        _motion: Motion<M>,
        _state: &mut GridRenderState,
        _custom_state: &mut S,
    ) -> impl IntoIterator<Item = Position> {
        let _grid = SquareGridRef(&self.entries);
        std::iter::empty()
    }
}

fn perform_motion_from_iter<'a, T, F>(
    iter: GridIndexedIter<'a, T>,
    next_dir: Direction,
    render: &mut GridRenderState,
    end_predicate: F,
) -> impl IntoIterator<Item = Position>
where
    T: Clone,
    F: FnMut(&Position) -> bool,
{
    // If currently going in another direction, change the direction
    if render.use_direction
        && !render.mode.is_visual()
        && iter.clone().count() <= 2
        && ![next_dir, !next_dir].contains(&render.direction)
    {
        render.direction = next_dir;
    }
    // Otherwise, move to the end and make sure it stays visible
    else if let Some(end) = iter.clone().map(|(pos, _)| pos).rfind(end_predicate) {
        tracing::info!(
            "Move to end {end:?}: {:?} ({:?})",
            render.to_app(end),
            render.selection
        );
        render.cursor = end;
        render.ensure_cursor_visible(end);

        if let Some(app_end) = render.to_app(end) {
            tracing::trace!("Update selection");
            render.selection.update(app_end);
        }
    }

    iter.map(|(pos, _)| pos)
}

fn grid_motion<'a, T, M, S>(
    grid: &'a Grid<T>,
    count: usize,
    motion: Motion<M>,
    start: Position,
    dir: Direction,
    state: &mut GridRenderState,
    custom_state: &mut S,
) -> GridIndexedIter<'a, T>
where
    Grid<T>: HandleCustomMotion<M, GridRenderState, S, Position>,
    T: Word + Debug,
{
    let iter_dir_remaining = |dir: Direction, remaining: usize| {
        GridIter::Linear(GridLinearIter::new_with_remaining(
            grid,
            start,
            dir.into(),
            remaining,
        ))
    };
    let iter_remaining = |remaining: usize| iter_dir_remaining(dir, remaining);
    let iter_direction = |dir: Direction| GridIter::Linear(grid.iter_segment(start, dir));
    let iter_single = |pos: Position| GridIter::Linear(GridLinearIter::new_single(grid, pos));
    let iter_empty = || GridIter::Linear(GridLinearIter::new_empty(grid));
    let iter_positions =
        |positions: Vec<Position>| GridIter::Positions(GridPositionsIter::new(grid, positions));

    let cursor = state.cursor;
    let iter = match motion {
        // Linear
        Motion::Mouse(mouse) => {
            let app_pos = ratatui::layout::Position {
                x: mouse.column,
                y: mouse.row,
            };

            match state.to_grid(app_pos) {
                Some(pos) => iter_single(pos),
                None => iter_empty(),
            }
        }
        Motion::Down | Motion::Left | Motion::Right | Motion::Up => iter_remaining(count + 1),
        Motion::Forwards => iter_dir_remaining(dir, count + 1),
        // NOTE: dir is already reversed before passing to this function for backwards motions
        Motion::Backwards => iter_dir_remaining(dir, count + 1),
        Motion::RowStart | Motion::RowEnd | Motion::ColStart | Motion::ColEnd => {
            iter_direction(dir)
        }

        // Position based
        Motion::Row(_) => {
            let diff = (cursor.row as isize) - count.saturating_sub(1) as isize;
            let dir = if diff >= 0 {
                Direction::Up
            } else {
                Direction::Down
            };

            iter_dir_remaining(dir, diff.unsigned_abs() + 1)
        }
        Motion::Col(_) => {
            let diff = (cursor.col as isize) - count.saturating_sub(1) as isize;
            let dir = if diff >= 0 {
                Direction::Left
            } else {
                Direction::Right
            };

            iter_dir_remaining(dir, diff.unsigned_abs() + 1)
        }
        Motion::WordEndBackwards
        | Motion::WordEndForwards
        | Motion::WordStartBackwards
        | Motion::WordStartForwards => {
            let iter = GridIndexedIter::new(iter_direction(dir));

            tracing::info!("Word motion for {:?}", iter.collect::<Vec<_>>());
            let target = &grid[cursor];

            let iter = GridIndexedIter::new(iter_direction(dir));
            grid_word_motion(grid, iter, target, count)
        }

        Motion::Custom(custom) => {
            let positions: Vec<_> = grid
                .handle_custom_motion(count, custom, state, custom_state)
                .into_iter()
                .collect();
            iter_positions(positions)
        }
    };

    GridIndexedIter::new(iter)
}

fn grid_word_motion<'a, T>(
    grid: &'a Grid<T>,
    mut iter: GridIndexedIter<'a, T>,
    target: &T,
    count: usize,
) -> GridIter<'a, T>
where
    T: Word,
{
    let mut positions = Vec::default();
    let mut word_count = 0;

    let mut prev_is_word = target.is_word();

    while let Some((pos, curr)) = iter.next()
        && word_count < count + (target.is_word() as usize)
    {
        let curr_is_word = curr.is_word();

        tracing::info!(
            "\t{pos:?} | count {word_count}/{} | prev {:?}, curr {}, target {prev_is_word})",
            count + (target.is_word() as usize),
            prev_is_word,
            curr_is_word
        );

        // Word -> Non-word
        if prev_is_word != curr_is_word {
            word_count += 1;
        }

        prev_is_word = curr_is_word;

        positions.push(pos);
    }

    tracing::info!("\tPositions: {positions:?}");

    let iter = GridPositionsIter::new(grid, positions);
    GridIter::Positions(iter)
}

fn square_grid_word_motion<'a, T>(
    grid: &'a Grid<Square<T>>,
    mut iter: GridIndexedIter<'a, Square<T>>,
    target: &Square<T>,
    count: usize,
) -> GridIndexedIter<'a, Square<T>>
where
    T: Word,
{
    let mut positions = Vec::default();
    let mut word_count = 0;

    let mut prev_is_word = !target.is_word();
    let mut between_squares = target.is_none();

    tracing::info!("Word motion");

    while let Some((pos, square)) = iter.next()
        && word_count < count + (target.is_word() as usize)
    {
        tracing::info!("{pos:?}");

        if let Some(curr) = square.as_ref() {
            if between_squares {
                tracing::info!("\tAfter non-square");
                word_count += 1;
            } else {
                tracing::info!("\tSquare");
                let curr_is_word = curr.is_word();

                // Word -> Non-word
                if prev_is_word != curr_is_word {
                    word_count += 1;
                }

                prev_is_word = curr_is_word;
            }
        }

        between_squares = square.is_none();
        positions.push(pos);
    }

    tracing::info!("\tPositions: {positions:?}");

    let iter = GridPositionsIter::new(grid, positions);
    GridIndexedIter::new_positions(iter)
}

fn square_grid_motion<'a, T, M, S>(
    grid: &'a Grid<Square<T>>,
    count: usize,
    motion: Motion<M>,
    start: Position,
    dir: Direction,
    state: &mut GridRenderState,
    custom_state: &mut S,
) -> GridIndexedIter<'a, Square<T>>
where
    Grid<Square<T>>: HandleCustomMotion<M, GridRenderState, S, Position>,
    T: Clone + Word + Debug,
{
    let iter_direction = |dir: Direction| GridIter::Linear(grid.iter_segment(start, dir));
    let iter_dir_remaining = |dir: Direction, remaining: usize| {
        let iter = GridIter::Linear(GridLinearIter::new_with_remaining(
            grid,
            start,
            dir.into(),
            remaining,
        ));

        GridIndexedIter::new(iter)
    };

    let cursor = state.cursor;

    match motion {
        // Position based
        Motion::Row(_) => {
            let goal1 = Position {
                row: count,
                ..cursor
            };
            let row = find_nth_some(grid.iter_col(cursor.col), count);

            let goal = Position { row, ..cursor };

            let diff = (cursor.row as isize) - row as isize;
            let dir = if diff >= 0 {
                Direction::Up
            } else {
                Direction::Down
            };

            tracing::trace!(
                "{cursor:?} <-> {goal1:?} ({goal:?}) => {dir:?} with a difference {diff}"
            );

            iter_dir_remaining(dir, diff.unsigned_abs() + 1)
        }
        Motion::Col(_) => {
            let goal1 = Position {
                col: count,
                ..cursor
            };
            let col = find_nth_some(grid.iter_row(cursor.row), count);

            let goal = Position { col, ..cursor };

            let diff = (cursor.col as isize) - col as isize;
            let dir = if diff >= 0 {
                Direction::Left
            } else {
                Direction::Right
            };

            tracing::trace!(
                "{cursor:?} <-> {goal1:?} ({goal:?}) => {dir:?} with a difference {diff}"
            );

            iter_dir_remaining(dir, diff.unsigned_abs() + 1)
        }

        Motion::WordEndBackwards
        | Motion::WordEndForwards
        | Motion::WordStartBackwards
        | Motion::WordStartForwards => {
            let iter = GridIndexedIter::new(iter_direction(dir));
            let target = &grid[cursor];

            square_grid_word_motion(grid, iter, target, count)
        }

        _ => grid_motion(grid, count, motion, start, dir, state, custom_state),
    }
}

fn find_nth_some<'a, T>(iter: GridLinearIter<'a, Square<T>>, goal: usize) -> usize {
    let mut count = 0;
    let mut last_idx = 0;

    for (idx, item) in iter.enumerate() {
        if count == goal {
            break;
        }

        if item.is_some() {
            count += 1;
        }

        last_idx = idx;
    }

    last_idx
}
