use std::collections::BTreeSet;

use crate::{Comparison, Grid, Line, Position, SatisfiesLineConstraint};

pub struct LineConstraint {
    pub line: LineKind,
    pub kind: LineConstraintKind,
}

pub enum LineKind {
    Line(Line),
    Free(Vec<Position>),
}

pub enum LineConstraintKind {
    Consecutive,            // Renban
    MinDifference(usize),   // Whispers
    Comparison(Comparison), // Thermometer
    Palindrome,
    Parity,
    RegionSum,
    Sandwich(usize),
    Skyscraper(usize),
    Sum, // Arrow
}

impl SatisfiesLineConstraint for Grid<u8> {
    fn satisfies_line_constraint(&self, constraint: &LineConstraint) -> bool {
        let values: Vec<_> = match constraint.line {
            LineKind::Line(line) => self.iter_line(line).cloned().collect(),
            LineKind::Free(ref positions) => {
                let mut values = Vec::new();

                for pos in positions {
                    let Some(value) = self.get(*pos) else {
                        return false;
                    };
                    values.push(*value);
                }

                values
            }
        };

        match constraint.kind {
            LineConstraintKind::Consecutive => {
                // Make sure no duplicates appear among the numbers
                let mut seen: BTreeSet<u8> = BTreeSet::new();

                for val in values {
                    if seen.contains(&val) {
                        return false;
                    }
                    seen.insert(val);
                }

                // Make sure the numbers are consecutive
                let mut prev = None;
                for curr in seen {
                    if let Some(prev) = prev
                        && 1 + prev != curr
                    {
                        return false;
                    }
                    prev = Some(curr);
                }

                true
            }
            LineConstraintKind::MinDifference(diff) => values
                .windows(2)
                .all(|window| window[0].abs_diff(window[1]) as usize >= diff),
            LineConstraintKind::Comparison(cmp) => values
                .windows(2)
                .all(|window| cmp.satisfies(&window[0], &window[1])),

            LineConstraintKind::Sum => {
                let mut iter = values.iter().map(|val| *val as usize);
                let Some(first) = iter.next() else {
                    return false;
                };

                first == iter.sum()
            }

            LineConstraintKind::Skyscraper(visible) => {
                let mut max_height = 0;
                let mut count = 0;

                for height in values {
                    if height > max_height {
                        count += 1;
                        max_height = height;
                    }
                }

                count == visible
            }

            LineConstraintKind::Sandwich(between) => {
                let min = 1;
                let max = values.len();

                let mut result = 0;
                let mut inbetween = false;

                for val in values {
                    let val = val as usize;

                    if val == min || val == max {
                        if inbetween {
                            break;
                        }
                        inbetween = true;
                    } else if inbetween {
                        result += val;
                    }
                }

                result == between
            }

            _ => false,
        }
    }
}
