use puzzled_core::{Grid, Offset, Position, Value};

use crate::Bit;

pub trait Bits {
    fn is_candidate(&self, pos: Position) -> Option<Bit>;
}

impl<T> Bits for Grid<T>
where
    T: Value<Bit>,
{
    fn is_candidate(&self, pos: Position) -> Option<Bit> {
        // Already filled bits are never a candidate
        if self.get(pos).and_then(|bit| bit.value()).is_some() {
            return None;
        }

        // Candidate if U/D or L/R are the same bit
        let [up, right, down, left] = self
            .adjacent4(pos)
            .map(|adj| adj.and_then(|b| b.value().cloned()));

        if up.is_some() && down.is_some() && up == down {
            return up;
        }
        if left.is_some() && right.is_some() && left == right {
            return left;
        }

        // Candidate if D/D2 are the same bit for any direction D
        let bit2 = |offset: Offset, dir: Option<&Bit>| {
            let dir2 = self.get(pos + 2 * offset).and_then(|b| b.value());

            dir2.is_some() && dir == dir2
        };

        if bit2(Offset::UP, up.as_ref()) {
            return up;
        }
        if bit2(Offset::RIGHT, right.as_ref()) {
            return right;
        }

        if bit2(Offset::DOWN, down.as_ref()) {
            return down;
        }

        if bit2(Offset::LEFT, left.as_ref()) {
            return left;
        }

        // Otherwise, no
        None
    }
}
