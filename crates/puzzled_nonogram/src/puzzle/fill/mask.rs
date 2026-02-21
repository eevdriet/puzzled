use bitvec::vec::BitVec;

use crate::{ColorId, Fill};

/// Mask that represents a collection of filles that have been used (1) or not (0)
/// The
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FillMask(BitVec);

impl FillMask {
    pub fn new() -> Self {
        Self(BitVec::EMPTY)
    }

    /// Add the given fill to the mask
    ///
    /// * `fill`: Fill to add
    pub fn add(&mut self, fill: Fill) {
        if let Some(idx) = Into::<Option<usize>>::into(fill) {
            let len = idx + 1;

            if len > self.0.len() {
                self.0.resize(len, false);
            }

            self.0.set(idx, true);
        }
    }

    /// Remove the given fill from the mask
    ///
    /// * `fill`: Fill to remove
    pub fn remove(&mut self, fill: Fill) {
        if let Some(idx) = Into::<Option<usize>>::into(fill) {
            self.0.set(idx, false);
        }
    }

    /// Iterate over the colors of the fill
    pub fn iter_colors(&self) -> impl Iterator<Item = Fill> {
        self.0
            .iter_ones()
            .filter(|&idx| idx != 0)
            .map(|idx| Fill::Color(idx as ColorId))
    }

    // Retrieve the underlying bits of the mask
    pub fn bits(&self) -> &BitVec {
        &self.0
    }
}
