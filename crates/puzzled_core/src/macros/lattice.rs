/*
   [ + - + - O - + - + - + - + - + - + - + - + - + ]
   [ |   |   |   |   |   |   |   |   |   |   |   | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | O | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + O + - + O + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . O . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
   [ | . | . | . | . | . | . | . | . | . | . | . | ]
   [ + - + - + - + - + - + - + - + - + - + - + - + ]
*/

#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! lattice {
    (
        [$($x0:tt),+ $(,)?]
        $(, [$($x:tt),+ $(,)?] $(,)?)*
    ) => {{
        // Count columns and verify size
        let mut _assert_width0 = [(); $crate::__count!($($x0)+)];
        let cols = $crate::__count!($($x0)+);
        let rows = 1usize;

        $(
            let _assert_width = [(); $crate::__count!($($x)+)];
            _assert_width0 = _assert_width;

            let rows = rows + 1usize;
        )*

        let size = cols * rows;

        // Construct the data to put into the lattice
        let mut cells = Vec::with_capacity(size);
        let mut vertices = Vec::with_capacity(size);
        let mut h_edges = Vec::with_capacity(size);
        let mut v_edges = Vec::with_capacity(size);

        $crate::lattice!(@rows true; cells vertices h_edges v_edges; [$($x0),+] $(,[$($x),+])*);

        let c = (cols - 1) / 2;

        // Construct the grids and build the lattice
        let cells = cells.iter().any(|v| v.is_some()).then_some($crate::Grid::from_vec(cells, c).expect("Valid cell grid"));
        let vertices = vertices.iter().any(|v| v.is_some()).then_some($crate::Grid::from_vec(vertices, c + 1).expect("Valid vertex grid"));
        let h_edges = h_edges.iter().any(|v| v.is_some()).then_some($crate::Grid::from_vec(h_edges, c).expect("Valid h-edges grid"));
        let v_edges = v_edges.iter().any(|v| v.is_some()).then_some($crate::Grid::from_vec(v_edges, c + 1).expect("Valid v-edges grid"));

        $crate::Lattice::new(cells, vertices, h_edges, v_edges).expect("Valid lattice")
    }};

    // Parse each row recursively
    (@rows $is_h_row:expr ; $c:ident $v:ident $eh:ident $ev:ident ; [$($x0:tt),+] $(,[$($x:tt),+])*) => {
        // Parse the first row, with alternating "first element"
        $crate::lattice!(@cols $is_h_row, true; $c $v $eh $ev; $($x0),+);

        // Continue with remaining rows, flipping is_h_row
        $crate::lattice!(@rows !$is_h_row; $c $v $eh $ev; $([$($x),+]),*);
    };

    (@rows $is_h_row:expr; $c:ident $v:ident $eh:ident $ev:ident;) => {

    };

    // Columns
    (@cols $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; $x:tt $(, $tail:tt)*) => {
        $crate::lattice!(@col $is_h_row, $is_first; $c $v $eh $ev; $x);
        $crate::lattice!(@cols $is_h_row, !$is_first; $c $v $eh $ev; $($tail),*);
    };
    (@cols $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident;) => {

    };

    (@col $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; +) => {
        if $is_h_row && $is_first {
            $v.push(None);
        }
        else {
            panic!("Vertices panic")
        }
    };

    (@col $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; -) => {
        if $is_h_row && !$is_first {
            $eh.push(None);
        }
        else {
            panic!("H-edges panic")
        }
    };

    (@col $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; |) => {
        if !$is_h_row && $is_first {
            $ev.push(None);
        }
        else {
            panic!("V-edges panic")
        }
    };

    (@col $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; .) => {
        if !$is_h_row && !$is_first {
            $c.push(None);
        }
        else {
            panic!("Cells panic")
        }
    };

    (@col $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; $x:expr) => {
        match ($is_h_row, $is_first) {
            (false, false) => {
                $c.push(Some(1));
            },
            (false, true) => {
                $ev.push(Some(1));
            },
            (true, false) => {
                $eh.push(Some(1));
            },
            (true, true) => {
                $v.push(Some(1));
            },
        };
    };

    (@col $is_h_row:expr, $is_first:expr; $c:ident $v:ident $eh:ident $ev:ident; $x:tt) => {
        match ($is_h_row, $is_first) {
            (false, false) => {
                $c.push(Some(1));
                // $v.push(None);
                // $eh.push(None);
                // $ev.push(None);
            },
            (false, true) => {
                // $c.push(None);
                $v.push(Some(1));
                // $eh.push(None);
                // $ev.push(None);
            },
            (true, false) => {
                // $c.push(None);
                // $v.push(None);
                $eh.push(Some(1));
                // $ev.push(None);
            },
            (true, true) => {
                // $c.push(None);
                // $v.push(None);
                // $eh.push(None);
                $ev.push(Some(1));
            },
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __count_h_row {
    () => (0usize);
    ( | $($rest:tt)* ) => (0usize);
    ( $first:tt $($rest:tt)* ) => (1usize + $crate::__count!($($rest)*));
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{Grid, Lattice};
    type A = Option<u8>;

    #[rstest]
    #[case(__count_h_row!(), 0)]
    #[case(__count_h_row!(|), 0)]
    #[case(__count_h_row!(1), 1)]
    fn h_row(#[case] lhs: usize, #[case] rhs: usize) {
        assert_eq!(lhs, rhs);
    }

    #[rstest]
    #[case(lattice!([+, -, +], [|, O, |]), None, None, None, Some(vec![Some(1)]))]
    fn lattice_len(
        #[case] lattice: Lattice<A, A, A>,
        #[case] vertices: Option<Vec<A>>,
        #[case] h_edges: Option<Vec<A>>,
        #[case] v_edges: Option<Vec<A>>,
        #[case] cells: Option<Vec<A>>,
    ) {
        let (_, cols) = lattice.dim();

        eprintln!("Vertices: {:?}", lattice.vertices());
        eprintln!("H-edges : {:?}", lattice.horizontal_edges());
        eprintln!("V-edges : {:?}", lattice.vertical_edges());
        eprintln!("Cells   : {:?}", lattice.cells());

        fn check(kind: &str, lhs: Option<&Grid<A>>, rhs: Option<Vec<A>>, cols: usize) {
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => {
                    let len = rhs.len();
                    let grid = Grid::from_vec(rhs, cols).unwrap_or_else(|_| {
                        panic!(
                            "{kind} could not be constructed with {cols} cols from {len} entries",
                        )
                    });

                    assert_eq!(*lhs, grid);
                }
                (None, None) => {}
                (lhs, rhs) => panic!(
                    "One side is set but the other isn't: lhs ({}) <-> rhs ({})",
                    lhs.is_some(),
                    rhs.is_some()
                ),
            }
        }

        check("vertices", lattice.vertices(), vertices, cols + 1);
        check("cells", lattice.cells(), cells, cols);
        check("h-edges", lattice.horizontal_edges(), h_edges, cols);
        check("v-edges", lattice.vertical_edges(), v_edges, cols + 1);
    }
}
