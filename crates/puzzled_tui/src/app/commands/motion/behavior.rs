use crate::{AppTypeTraits, Description, Motion};

pub trait MotionBehavior: AppTypeTraits {
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

impl<M, S> Description<S> for Motion<M>
where
    M: Description<S>,
{
    fn description(&self, state: &S) -> Option<String> {
        let desc = match self {
            // Left-right
            Motion::Col(_) => "Move to column <n> in the current row of the active widget",
            Motion::Left => "Move left in the active widget",
            Motion::Right => "Move right in the active widget",
            Motion::RowEnd => "Move to the end of the row in the active widget",
            Motion::RowStart => "Move to the start of the row in the active widget",

            // Up-down
            Motion::ColEnd => "Move to the end of the column in the active widget",
            Motion::ColStart => "Move to the start of the column in the active widget",
            Motion::Down => "Move down in the active widget",
            Motion::Row(_) => "Move to row <n> in the current column of the active widget",
            Motion::Up => "Move up in the active widget",

            // Custom
            Motion::Custom(custom) => return custom.description(state),
            _ => return None,
        };

        Some(desc.to_string())
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
            Motion::WordEndBackwards,
            Motion::WordEndForwards,
            Motion::WordStartBackwards,
            Motion::WordStartForwards,
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
