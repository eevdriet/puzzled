use std::fmt::Debug;

use crate::Motion;

pub trait MotionBehavior: Clone + Debug + Sized {
    fn variants() -> Vec<Self>;
}

impl MotionBehavior for () {
    fn variants() -> Vec<Self> {
        vec![]
    }
}

impl<M> MotionBehavior for Motion<M>
where
    M: MotionBehavior,
{
    fn variants() -> Vec<Self> {
        let mut variants = vec![
            Motion::None,
            // Left-right
            Motion::Col(0),
            Motion::Left,
            Motion::Right,
            Motion::RowEnd,
            Motion::RowStart,
            // Up-down
            Motion::ColEnd,
            Motion::ColStart,
            Motion::Down,
            Motion::Row(0),
            Motion::Up,
            // Word
            Motion::Word,
            // Other (for puzzle specific motions)
        ];

        let other_variants = M::variants().into_iter().map(Motion::Other);
        variants.extend(other_variants);

        variants
    }
}
