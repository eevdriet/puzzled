use ratatui::layout::{Position, Rect, Size};

use crate::{AsApp, Selection, SelectionKind};

#[derive(Debug, Clone)]
pub struct MultiSelection {
    pub selections: Vec<Selection>,
    pub active: Option<Selection>,
}

impl MultiSelection {
    pub fn new() -> Self {
        Self {
            selections: Vec::new(),
            active: None,
        }
    }

    pub fn active(&self) -> Option<&Selection> {
        self.active.as_ref()
    }

    pub fn active_mut(&mut self) -> Option<&mut Selection> {
        self.active.as_mut()
    }

    pub fn reset(&mut self) {
        self.selections.clear();
        self.active = None;
    }

    pub fn start(&mut self, pos: Position, kind: SelectionKind, additive: bool) {
        if !additive {
            self.reset();
        }

        let mut sel = Selection::default();
        sel.set(pos, pos);
        sel.set_kind(kind);

        self.active = Some(sel);
    }

    pub fn set(&mut self, start: Position, end: Position) {
        if let Some(sel) = &mut self.active {
            sel.set(start, end);
        }
    }

    pub fn update(&mut self, pos: Position) {
        if let Some(sel) = &mut self.active {
            sel.update(pos);
        }
    }

    pub fn finish(&mut self) {
        if let Some(sel) = self.active.take() {
            self.selections.push(sel);
        }
    }

    pub fn contains<P, S>(&self, pos: P, size: &S) -> bool
    where
        P: AsApp<Position>,
        S: AsApp<Size>,
    {
        self.selections
            .iter()
            .any(|sel| sel.range(&size.as_app()).contains(pos.as_app()))
    }

    pub fn ranges<S>(&self, size: &S) -> impl Iterator<Item = Rect>
    where
        S: AsApp<Size>,
    {
        self.selections.iter().map(|sel| sel.range(&size.as_app()))
    }

    pub fn positions<S>(&self, size: &S) -> impl Iterator<Item = Position>
    where
        S: AsApp<Size>,
    {
        self.selections
            .iter()
            .flat_map(|sel| sel.range(&size.as_app()).positions())
    }
}

impl Default for MultiSelection {
    fn default() -> Self {
        Self::new()
    }
}
