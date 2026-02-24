#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid dimensions {found:?} found for {kind}, expected {expected:?}")]
    InvalidDimensions {
        kind: String,
        found: (usize, usize),
        expected: (usize, usize),
    },

    #[error("Dimensions for {kind} must be >= (1, 1), found {found:?}")]
    DimensionUnderflow { kind: String, found: (usize, usize) },

    #[error("No grid set at lattice construction, cannot determine dimensions")]
    MissingDimensions,
}
