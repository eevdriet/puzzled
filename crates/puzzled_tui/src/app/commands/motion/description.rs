use puzzled_core::SquareGridRef;

use crate::{Description, GridRef, Motion};

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

impl<T, M> Description<(Option<String>, GridRef<'_, T>)> for Motion<M>
where
    M: Description<()>,
{
    fn description(&self, (fill, _grid): &(Option<String>, GridRef<'_, T>)) -> Option<String> {
        let fill = fill
            .as_ref()
            .map(|f| f.to_owned())
            .unwrap_or("cell".to_string());

        let description = match self {
            Motion::ColEnd => format!("Move to the last {fill} in the column"),
            Motion::ColStart => format!("Move to the first {fill} in the column"),
            Motion::Left => format!("Move to the {fill} left of the cursor"),
            Motion::Right => format!("Move to the {fill} right of the cursor"),
            Motion::RowEnd => format!("Move to the last {fill} in the row"),
            Motion::RowStart => format!("Move to the first {fill} in the row"),
            motion => return motion.description(&()),
        };

        Some(description)
    }
}

impl<T, M> Description<(Option<String>, SquareGridRef<'_, T>)> for Motion<M>
where
    M: Description<()>,
{
    fn description(
        &self,
        (fill, _grid): &(Option<String>, SquareGridRef<'_, T>),
    ) -> Option<String> {
        let fill = match fill {
            Some(fill) => format!("{fill} square"),
            _ => "square".to_string(),
        };

        let description = match self {
            Motion::ColEnd => format!("Move to the last {fill} in the column"),
            Motion::ColStart => format!("Move to the first {fill} in the column"),
            Motion::Left => format!("Move to the {fill} left of the cursor"),
            Motion::Right => format!("Move to the {fill} right of the cursor"),
            Motion::RowEnd => format!("Move to the last {fill} in the row"),
            Motion::RowStart => format!("Move to the first {fill} in the row"),
            motion => return motion.description(&()),
        };

        Some(description)
    }
}
