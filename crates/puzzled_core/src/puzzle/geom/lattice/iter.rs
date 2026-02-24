use crate::{Lattice, Position};

impl<C, V, E> Lattice<C, V, E> {
    pub fn cell_edges(&self, pos: Position) -> [Option<&E>; 4] {
        let Position { row, col } = pos;

        [
            self.horizontal_edges
                .as_ref()
                .expect("Top edge")
                .get(Position::new(row, col)),
            self.vertical_edges
                .as_ref()
                .expect("Right edge")
                .get(Position::new(row, col + 1)),
            self.horizontal_edges
                .as_ref()
                .expect("Bottom edge")
                .get(Position::new(row + 1, col)),
            self.vertical_edges
                .as_ref()
                .expect("Left edge")
                .get(Position::new(row, col)),
        ]
    }
}
