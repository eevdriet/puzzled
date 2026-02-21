#[derive(Debug, thiserror::Error, Clone)]
pub enum GridError {
    #[error("Row {row} in the grid has an invalid width of {found} (expected {expected})")]
    InvalidWidth { row: u8, found: u8, expected: u8 },

    #[error("The grid has an invalid height of {found} (expected {expected})")]
    InvalidHeight { found: u8, expected: u8 },

    #[error(
        "The grid has invalid dimensions ({rows} rows and {cols} columns). Make sure the size divides the number of columns"
    )]
    InvalidDimensions { cols: u8, rows: u8 },

    #[error("Row {row} has an invalid format: {reason}")]
    InvalidRow { row: u8, reason: String },

    #[error("The length of the data ({len}) is not divisible by the number of columns ({cols})")]
    ColDivisibility { len: usize, cols: usize },

    #[error("Size overflow from trying to construct grid with {rows} rows and {cols} cols")]
    SizeOverflow { rows: usize, cols: usize },
}
