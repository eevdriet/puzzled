#[macro_export]
macro_rules! __clue_value {
    (.) => {
        None
    };
    ($x:literal) => {
        Some($x as usize)
    };
}

#[macro_export]
macro_rules! __insert_clues {
    ($clues:ident, $values:expr, $direction:expr, Row) => {
        for (r, val) in $values.into_iter().enumerate() {
            if let Some(clue) = val {
                let id = $crate::ClueId::new($crate::Line::Row(r), $direction);
                $clues.insert(id, clue);
            }
        }
    };

    ($clues:ident, $values:expr, $direction:expr, Col) => {
        for (c, val) in $values.into_iter().enumerate() {
            if let Some(clue) = val {
                let id = $crate::ClueId::new($crate::Line::Col(c), $direction);
                $clues.insert(id, clue);
            }
        }
    };
}
