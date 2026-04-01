use crate::{AppTypes, SidedGridWidget};

impl<'a, A, CT, TT, RT, BT, LT, C, E> SidedGridWidget<'a, A, CT, TT, RT, BT, LT, C, E>
where
    A: AppTypes,
{
    pub fn with_top<U>(self, top: &'a Vec<U>) -> SidedGridWidget<'a, A, CT, U, RT, BT, LT, C, E> {
        let SidedGridWidget {
            grid,
            right,
            bottom,
            left,
            _marker,
            _cell,
            _edge,
            ..
        } = self;

        SidedGridWidget {
            top: Some(top),
            grid,
            right,
            bottom,
            left,
            _marker,
            _cell,
            _edge,
        }
    }

    pub fn with_right<U>(
        self,
        right: &'a Vec<U>,
    ) -> SidedGridWidget<'a, A, CT, TT, U, BT, LT, C, E> {
        let SidedGridWidget {
            _cell,
            _edge,
            _marker,
            bottom,
            grid,
            left,
            top,
            ..
        } = self;

        SidedGridWidget {
            right: Some(right),
            _cell,
            _edge,
            _marker,
            bottom,
            grid,
            left,
            top,
        }
    }

    pub fn with_bottom<U>(
        self,
        bottom: &'a Vec<U>,
    ) -> SidedGridWidget<'a, A, CT, TT, RT, U, LT, C, E> {
        let SidedGridWidget {
            _cell,
            _edge,
            _marker,
            grid,
            left,
            right,
            top,
            ..
        } = self;

        SidedGridWidget {
            bottom: Some(bottom),
            _cell,
            _edge,
            _marker,
            grid,
            left,
            right,
            top,
        }
    }

    pub fn with_left<U>(self, left: &'a Vec<U>) -> SidedGridWidget<'a, A, CT, TT, RT, BT, U, C, E> {
        let SidedGridWidget {
            _cell,
            _edge,
            _marker,
            bottom,
            grid,
            right,
            top,
            ..
        } = self;

        SidedGridWidget {
            left: Some(left),
            _cell,
            _edge,
            _marker,
            bottom,
            grid,
            right,
            top,
        }
    }
}
