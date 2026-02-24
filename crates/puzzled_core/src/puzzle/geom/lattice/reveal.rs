use crate::{Lattice, Line, Offset, Position, Reveal};

impl<C, V, E> Lattice<C, V, E>
where
    V: Reveal,
{
    fn reveal_vertex(&mut self, cell_pos: Position) -> bool {
        let Some(vertices) = self.vertices.as_mut() else {
            return false;
        };

        vertices.reveal(cell_pos)
    }

    pub fn reveal_top_left_vertex(&mut self, cell_pos: Position) -> bool {
        self.reveal_vertex(cell_pos)
    }

    pub fn reveal_top_right_vertex(&mut self, cell_pos: Position) -> bool {
        self.reveal_vertex(cell_pos + Offset::RIGHT)
    }

    pub fn reveal_bottom_left_vertex(&mut self, cell_pos: Position) -> bool {
        self.reveal_vertex(cell_pos + Offset::DOWN)
    }

    pub fn reveal_bottom_right_vertex(&mut self, cell_pos: Position) -> bool {
        self.reveal_vertex(cell_pos + Offset::DOWN + Offset::RIGHT)
    }

    pub fn reveal_vertices(&mut self, cell_pos: Position) -> bool {
        let top_left = self.reveal_top_left_vertex(cell_pos);
        let top_right = self.reveal_top_right_vertex(cell_pos);
        let bottom_left = self.reveal_bottom_left_vertex(cell_pos);
        let bottom_right = self.reveal_bottom_right_vertex(cell_pos);

        top_left && top_right && bottom_left && bottom_right
    }
}

impl<C, V, E> Lattice<C, V, E>
where
    E: Reveal,
{
    pub fn reveal_line_edges(&mut self, line: Line) -> bool {
        let opt_edges = match line {
            Line::Row(_) => self.horizontal_edges.as_mut(),
            Line::Col(_) => self.vertical_edges.as_mut(),
        };
        let Some(edges) = opt_edges else {
            return false;
        };

        for edge in edges.iter_line_mut(line) {
            edge.reveal();
        }

        true
    }
}
