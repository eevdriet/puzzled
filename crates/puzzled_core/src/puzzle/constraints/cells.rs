use std::cmp::Ordering;

use crate::{Comparison, Grid, Position, SatisfiesCellsConstraint};

pub struct CellsConstraint {
    pub lhs: Position,
    pub rhs: Position,
    pub kind: CellsConstraintKind,
    pub ord: Ordering,
}

pub enum CellsConstraintKind {
    Difference(usize),
    Factor(usize),
    Comparison(Comparison),
    Parity,
}

impl SatisfiesCellsConstraint for Grid<u8> {
    fn satisfies_cells_constraint(&self, constraint: &CellsConstraint) -> bool {
        let (Some(lhs), Some(rhs)) = (self.get(constraint.lhs), self.get(constraint.rhs)) else {
            return false;
        };
        let lhs = *lhs as usize;
        let rhs = *rhs as usize;

        match constraint.kind {
            CellsConstraintKind::Difference(diff) => match constraint.ord {
                Ordering::Less => lhs.checked_sub(rhs).is_some_and(|result| result == diff),
                Ordering::Greater => rhs.checked_sub(lhs).is_some_and(|result| result == diff),
                Ordering::Equal => lhs.abs_diff(rhs) == diff,
            },
            CellsConstraintKind::Factor(factor) => {
                let lfactor = lhs.checked_mul(factor).is_some_and(|result| result == rhs);
                let rfactor = rhs.checked_mul(factor).is_some_and(|result| result == lhs);

                match constraint.ord {
                    Ordering::Less => lfactor,
                    Ordering::Greater => rfactor,
                    Ordering::Equal => lfactor || rfactor,
                }
            }
            CellsConstraintKind::Comparison(cmp) => cmp.satisfies(&lhs, &rhs),
            CellsConstraintKind::Parity => lhs.is_multiple_of(2) != rhs.is_multiple_of(2),
        }
    }
}
