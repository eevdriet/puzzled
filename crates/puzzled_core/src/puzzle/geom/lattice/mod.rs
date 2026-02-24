mod builder;
mod error;
mod iter;
mod reveal;

pub use error::Error as LatticeError;

use crate::{
    Grid,
    puzzle::geom::lattice::builder::{Builder, Unset},
};

#[derive(Debug)]
pub struct Lattice<C, V, E> {
    // Dimensions (RxC cells)
    rows: usize,
    cols: usize,

    cells: Option<Grid<C>>,
    vertices: Option<Grid<V>>,
    horizontal_edges: Option<Grid<E>>,
    vertical_edges: Option<Grid<E>>,
}

impl<C, V, E> Default for Lattice<C, V, E> {
    fn default() -> Self {
        Self {
            rows: 0,
            cols: 0,
            cells: None,
            vertices: None,
            horizontal_edges: None,
            vertical_edges: None,
        }
    }
}

impl<C, V, E> Lattice<C, V, E> {
    pub(crate) fn new(
        cells: Option<Grid<C>>,
        vertices: Option<Grid<V>>,
        h_edges: Option<Grid<E>>,
        v_edges: Option<Grid<E>>,
    ) -> Result<Self, LatticeError> {
        // 1. Determine base dimensions
        let dims = if let Some(ref g) = cells {
            Some((g.rows(), g.cols()))
        } else if let Some(ref g) = vertices {
            Some((
                g.rows(),
                g.cols()
                    .checked_sub(1)
                    .ok_or(LatticeError::DimensionUnderflow {
                        kind: "vertices".into(),
                        found: (g.rows(), g.cols()),
                    })?,
            ))
        } else if let Some(ref g) = h_edges {
            Some((g.rows(), g.cols()))
        } else if let Some(ref g) = v_edges {
            Some((
                g.rows(),
                g.cols()
                    .checked_sub(1)
                    .ok_or(LatticeError::DimensionUnderflow {
                        kind: "vertical edges".into(),
                        found: (g.rows(), g.cols()),
                    })?,
            ))
        } else {
            return Err(LatticeError::MissingDimensions);
        };

        let (rows, cols) = dims.unwrap();

        // helper closure
        let check = |kind: &str, r: usize, c: usize| {
            if r == rows && c == cols {
                Ok(())
            } else {
                Err(LatticeError::InvalidDimensions {
                    kind: kind.into(),
                    found: (r, c),
                    expected: (rows, cols),
                })
            }
        };

        // 2. Validate each grid

        if let Some(ref g) = cells {
            check("cells", g.rows(), g.cols())?;
        }

        if let Some(ref g) = vertices {
            check("vertices", g.rows(), g.cols() - 1)?;
        }

        if let Some(ref g) = h_edges {
            check("horizontal edges", g.rows(), g.cols())?;
        }

        if let Some(ref g) = v_edges {
            check("vertical edges", g.rows(), g.cols() - 1)?;
        }

        Ok(Self {
            rows,
            cols,
            cells,
            vertices,
            horizontal_edges: h_edges,
            vertical_edges: v_edges,
        })
    }

    pub fn builder() -> Builder<C, V, E, Unset, Unset, Unset, Unset, Unset> {
        Builder::default()
    }

    pub fn cells(&self) -> Option<&Grid<C>> {
        self.cells.as_ref()
    }

    pub fn cells_mut(&mut self) -> Option<&mut Grid<C>> {
        self.cells.as_mut()
    }

    pub fn vertices(&self) -> Option<&Grid<V>> {
        self.vertices.as_ref()
    }

    pub fn vertices_mut(&mut self) -> Option<&mut Grid<V>> {
        self.vertices.as_mut()
    }

    pub fn horizontal_edges(&self) -> Option<&Grid<E>> {
        self.horizontal_edges.as_ref()
    }

    pub fn horizontal_edges_mut(&mut self) -> Option<&mut Grid<E>> {
        self.horizontal_edges.as_mut()
    }

    pub fn vertical_edges(&self) -> Option<&Grid<E>> {
        self.vertical_edges.as_ref()
    }

    pub fn vertical_edges_mut(&mut self) -> Option<&mut Grid<E>> {
        self.vertical_edges.as_mut()
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn cells_dim(&self) -> Option<(usize, usize)> {
        self.cells
            .as_ref()
            .map(|cells| (cells.rows(), cells.cols()))
    }

    pub fn vertices_dim(&self) -> Option<(usize, usize)> {
        self.vertices
            .as_ref()
            .map(|cells| (cells.rows(), cells.cols()))
    }

    pub fn horizontal_edges_dim(&self) -> Option<(usize, usize)> {
        self.horizontal_edges
            .as_ref()
            .map(|cells| (cells.rows(), cells.cols()))
    }

    pub fn vertical_edges_dim(&self) -> Option<(usize, usize)> {
        self.vertical_edges
            .as_ref()
            .map(|cells| (cells.rows(), cells.cols()))
    }
}
