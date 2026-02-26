use std::marker::PhantomData;

use crate::{Grid, Lattice, LatticeError};

/// Used to indicate that a partially-constructed [`Puzzle`] is missing the height of its grid
pub struct Unset;
/// Used to indicate that a partially-constructed [`Puzzle`] has set the height of its grid
pub struct Set;

pub struct Builder<C, V, E, H, HC, HV, HEH, HEV> {
    dims: Option<(usize, usize)>,

    cells: Option<Grid<C>>,
    vertices: Option<Grid<V>>,
    h_edges: Option<Grid<E>>,
    v_edges: Option<Grid<E>>,

    _any: PhantomData<H>,
    _cells: PhantomData<HC>,
    _vertices: PhantomData<HV>,
    _h_edges: PhantomData<HEH>,
    _v_edges: PhantomData<HEV>,
}

impl<C, V, E> Default for Builder<C, V, E, Unset, Unset, Unset, Unset, Unset> {
    fn default() -> Builder<C, V, E, Unset, Unset, Unset, Unset, Unset> {
        Self {
            dims: None,

            cells: None,
            vertices: None,
            h_edges: None,
            v_edges: None,

            _any: PhantomData,
            _cells: PhantomData,
            _vertices: PhantomData,
            _h_edges: PhantomData,
            _v_edges: PhantomData,
        }
    }
}

impl<C, V, E, H, HC, HV, HEH, HEV> Builder<C, V, E, H, HC, HV, HEH, HEV> {
    fn update_dims(&mut self, kind: String, rows: usize, cols: usize) -> Result<(), LatticeError> {
        match self.dims {
            None => {
                self.dims = Some((rows, cols));
                Ok(())
            }
            Some((r, c)) => {
                if r == rows && c == cols {
                    Ok(())
                } else {
                    Err(LatticeError::InvalidDimensions {
                        kind,
                        found: (r, c),
                        expected: (rows, cols),
                    })
                }
            }
        }
    }
}

impl<C, V, E, HC, HV, HEH, HEV> Builder<C, V, E, Set, HC, HV, HEH, HEV> {
    pub fn build(self) -> Lattice<C, V, E> {
        Lattice::new(self.cells, self.vertices, self.h_edges, self.v_edges)
            .expect("Builder builds Lattice correctly")
    }
}

impl<C, V, E, H, HV, HEH, HEV> Builder<C, V, E, H, Unset, HV, HEH, HEV> {
    #[allow(clippy::type_complexity)]
    pub fn cells(
        mut self,
        cells: Grid<C>,
    ) -> Result<Builder<C, V, E, Set, Set, HV, HEH, HEV>, LatticeError> {
        let kind = "vertices".to_string();

        self.update_dims(kind, cells.rows(), cells.cols())?;
        self.cells = Some(cells);

        Ok(Builder {
            // State
            dims: self.dims,
            cells: self.cells,
            vertices: self.vertices,
            h_edges: self.h_edges,
            v_edges: self.v_edges,

            // Markers
            _any: PhantomData,
            _cells: PhantomData,
            _vertices: self._vertices,
            _h_edges: self._h_edges,
            _v_edges: self._v_edges,
        })
    }
}

impl<C, V, E, H, HC, HEH, HEV> Builder<C, V, E, H, HC, Unset, HEH, HEV> {
    #[allow(clippy::type_complexity)]
    pub fn vertices(
        mut self,
        vertices: Grid<V>,
    ) -> Result<Builder<C, V, E, Set, HC, Set, HEH, HEV>, LatticeError> {
        let kind = "vertices".to_string();
        let dims = (vertices.rows(), vertices.cols());

        let rows = vertices
            .rows()
            .checked_sub(1)
            .ok_or(LatticeError::DimensionUnderflow {
                kind: kind.clone(),
                found: dims,
            })?;
        let cols = vertices
            .cols()
            .checked_sub(1)
            .ok_or(LatticeError::DimensionUnderflow {
                kind: kind.clone(),
                found: dims,
            })?;

        self.update_dims(kind.clone(), rows, cols)?;
        self.vertices = Some(vertices);

        Ok(Builder {
            // State
            dims: self.dims,
            cells: self.cells,
            vertices: self.vertices,
            h_edges: self.h_edges,
            v_edges: self.v_edges,

            // Markers
            _any: PhantomData,
            _cells: self._cells,
            _vertices: PhantomData,
            _h_edges: self._h_edges,
            _v_edges: self._v_edges,
        })
    }
}

impl<C, V, E, H, HC, HV, HEV> Builder<C, V, E, H, HC, HV, Unset, HEV> {
    #[allow(clippy::type_complexity)]
    pub fn horizontal_edges(
        mut self,
        edges: Grid<E>,
    ) -> Result<Builder<C, V, E, Set, HC, HV, Set, HEV>, LatticeError> {
        let kind = "horizontal edges".to_string();
        let dims = (edges.rows(), edges.cols());

        let rows = edges
            .rows()
            .checked_sub(1)
            .ok_or(LatticeError::DimensionUnderflow {
                kind: kind.clone(),
                found: dims,
            })?;

        let cols = edges
            .cols()
            .checked_sub(1)
            .ok_or(LatticeError::DimensionUnderflow {
                kind: kind.clone(),
                found: dims,
            })?;

        self.update_dims(kind.clone(), rows, cols)?;
        self.h_edges = Some(edges);

        Ok(Builder {
            // State
            dims: self.dims,
            cells: self.cells,
            vertices: self.vertices,
            h_edges: self.h_edges,
            v_edges: self.v_edges,

            // Markers
            _any: PhantomData,
            _cells: self._cells,
            _vertices: self._vertices,
            _h_edges: PhantomData,
            _v_edges: self._v_edges,
        })
    }
}

impl<C, V, E, H, HC, HV, HEH> Builder<C, V, E, H, HC, HV, HEH, Unset> {
    #[allow(clippy::type_complexity)]
    pub fn vertical_edges(
        mut self,
        edges: Grid<E>,
    ) -> Result<Builder<C, V, E, Set, HC, HV, HEH, Set>, LatticeError> {
        let kind = "vertical edges".to_string();
        let dims = (edges.rows(), edges.cols());

        let rows = edges
            .rows()
            .checked_sub(1)
            .ok_or(LatticeError::DimensionUnderflow {
                kind: kind.clone(),
                found: dims,
            })?;

        let cols = edges
            .cols()
            .checked_sub(1)
            .ok_or(LatticeError::DimensionUnderflow {
                kind: kind.clone(),
                found: dims,
            })?;

        self.update_dims(kind.clone(), rows, cols)?;
        self.v_edges = Some(edges);

        Ok(Builder {
            // State
            dims: self.dims,
            cells: self.cells,
            vertices: self.vertices,
            h_edges: self.h_edges,
            v_edges: self.v_edges,

            // Markers
            _any: PhantomData,
            _cells: self._cells,
            _vertices: self._vertices,
            _h_edges: self._h_edges,
            _v_edges: PhantomData,
        })
    }
}
