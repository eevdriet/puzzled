use puzzled_core::{
    Direction, Grid, GridIndexedIter, GridIter, Position as CorePosition, SquareGridRef,
};

use crate::Motion;

pub trait BaseMotionRange<M> {
    type Position;

    fn base_motion_range(
        &self,
        start: Self::Position,
        count: usize,
        motion: &Motion<M>,
    ) -> impl IntoIterator<Item = Self::Position>;
}

pub trait CustomMotionRange<M, S>: BaseMotionRange<M> {
    fn custom_motion_range(
        &self,
        start: Self::Position,
        count: usize,
        motion: &M,
        state: &S,
    ) -> impl IntoIterator<Item = Self::Position>;
}

impl<M, T> BaseMotionRange<M> for Grid<T> {
    type Position = CorePosition;

    fn base_motion_range(
        &self,
        start: Self::Position,
        count: usize,
        motion: &Motion<M>,
    ) -> impl IntoIterator<Item = Self::Position> {
        let dir = Direction::try_from(motion).unwrap_or_default();

        let iter_remaining =
            |remaining: usize| GridIter::new_with_remaining(self, start, dir.into(), remaining);
        let iter_direction = |dir: Direction| self.iter_segment(start, dir);

        let iter = match motion {
            Motion::Down | Motion::Left | Motion::Right | Motion::Up => iter_remaining(count + 1),
            Motion::RowStart | Motion::RowEnd | Motion::ColStart | Motion::ColEnd => {
                iter_direction(dir)
            }
            _ => GridIter::new_empty(self),
        };

        GridIndexedIter::new(iter).map(|(pos, _)| pos)
    }
}

impl<M, T> BaseMotionRange<M> for SquareGridRef<'_, T> {
    type Position = CorePosition;

    fn base_motion_range(
        &self,
        start: Self::Position,
        count: usize,
        motion: &Motion<M>,
    ) -> impl IntoIterator<Item = Self::Position> {
        // Handle non-directed motions normally
        if count == 0 {
            return self.0.base_motion_range(start, count, motion);
        }

        let Ok(dir) = Direction::try_from(motion) else {
            return self.0.base_motion_range(start, count, motion);
        };

        // Otherwise, go to the first filled square in the given direction to perform the range
        let mut pos = start;

        while let Some(next) = pos + dir
            && self.0.is_in_bounds(next)
        {
            // NOTE: since count > 0, some motion always moves the position past the currently non-fill pos
            if self.0.is_fill(next) {
                break;
            }

            pos = next;
        }

        // Now, perform the motion range starting from the found position
        self.0.base_motion_range(pos, count, motion)
    }
}

impl<M> BaseMotionRange<M> for () {
    type Position = ();

    fn base_motion_range(
        &self,
        _start: Self::Position,
        _count: usize,
        _motion: &Motion<M>,
    ) -> impl IntoIterator<Item = Self::Position> {
        std::iter::empty()
    }
}

impl<M, S> CustomMotionRange<M, S> for () {
    fn custom_motion_range(
        &self,
        _start: Self::Position,
        _count: usize,
        _motion: &M,
        _state: &S,
    ) -> impl IntoIterator<Item = Self::Position> {
        std::iter::empty()
    }
}
