use std::fmt::Debug;

use crate::Motion;

pub trait MotionBehavior: Clone + PartialEq + Eq + PartialOrd + Ord + Send + Debug + Sized {
    fn variants() -> Vec<Self>;

    fn is_mouse(&self) -> bool {
        false
    }
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

        let other_variants = M::variants().into_iter().map(Motion::Custom);
        variants.extend(other_variants);

        variants
    }

    fn is_mouse(&self) -> bool {
        match self {
            Motion::Custom(custom) => custom.is_mouse(),
            motion => motion.is_mouse(),
        }
    }
}
