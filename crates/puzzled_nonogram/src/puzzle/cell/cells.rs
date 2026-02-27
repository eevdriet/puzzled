use puzzled_core::{ColorId, Grid, Line, Value};

use crate::{Fill, Runs};

pub trait Fills {
    fn iter_line_runs<'a>(&'a self, line: Line) -> Runs<impl Iterator<Item = Fill> + 'a + Clone>;

    fn iter_colors(&self) -> impl Iterator<Item = &Fill>;

    fn colors_ids(&self) -> Vec<ColorId>;
}

impl<T> Fills for Grid<T>
where
    T: Clone + Value<Fill>,
{
    fn iter_line_runs<'a>(&'a self, line: Line) -> Runs<impl Iterator<Item = Fill> + 'a + Clone> {
        let fills = self
            .iter_line(line)
            .filter_map(|cell| cell.value().cloned());

        Runs::new(fills, true)
    }

    fn iter_colors(&self) -> impl Iterator<Item = &Fill> {
        self.iter().filter_map(|cell| match cell.value() {
            color @ Some(Fill::Color(_)) => color,
            _ => None,
        })
    }

    fn colors_ids(&self) -> Vec<ColorId> {
        let mut ids: Vec<_> = self
            .iter()
            .filter_map(|cell| match cell.value() {
                Some(Fill::Color(id)) => Some(*id),
                _ => None,
            })
            .collect();

        ids.dedup();
        ids.sort();

        ids
    }
}
