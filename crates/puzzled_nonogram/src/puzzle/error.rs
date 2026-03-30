use thiserror::Error;

use crate::Fill;

#[derive(Debug, Error)]
pub enum NonogramError {
    #[error("The nonogram does not define a color for fill {0}")]
    UndefinedFill(Fill),
}
