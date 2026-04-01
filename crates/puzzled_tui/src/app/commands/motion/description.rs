use puzzled_core::Grid;

use crate::{Description, Motion};

impl<M> Description<()> for Motion<M>
where
    M: Description<()>,
{
    fn description(&self, state: &()) -> Option<String> {
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
            Motion::Search(search) => return search.description(state),
            Motion::Custom(custom) => return custom.description(state),
            _ => return None,
        };

        Some(desc.to_string())
    }
}

impl<T, M> Description<Grid<T>> for Motion<M>
where
    M: Description<()>,
{
    fn description(&self, _state: &Grid<T>) -> Option<String> {
        let description = match self {
            Motion::ColEnd => "Move to the last square in the column",
            Motion::ColStart => "Move to the first square in the column",
            Motion::Left => "Move to the square left of the cursor",
            Motion::Right => "Move to the square right of the cursor",
            Motion::RowEnd => "Move to the last square in the row",
            Motion::RowStart => "Move to the first square in the row",
            motion => return motion.description(&()),
        };

        Some(description.to_string())
    }
}
