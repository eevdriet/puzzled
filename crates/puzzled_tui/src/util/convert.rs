use puzzled_core::{Position as CorePosition, Size as CoreSize};
use ratatui::layout::{Position as AppPosition, Size as AppSize};

pub trait AsApp<T> {
    fn as_app(&self) -> T;
}

pub trait AsCore<T> {
    fn as_core(&self) -> T;
}

impl AsApp<AppPosition> for AppPosition {
    fn as_app(&self) -> AppPosition {
        *self
    }
}

impl AsApp<AppPosition> for CorePosition {
    fn as_app(&self) -> AppPosition {
        AppPosition {
            x: self.col as u16,
            y: self.row as u16,
        }
    }
}

impl AsApp<AppSize> for AppSize {
    fn as_app(&self) -> AppSize {
        *self
    }
}

impl AsApp<AppSize> for CoreSize {
    fn as_app(&self) -> AppSize {
        AppSize {
            width: self.cols as u16,
            height: self.rows as u16,
        }
    }
}

impl AsCore<CorePosition> for CorePosition {
    fn as_core(&self) -> CorePosition {
        *self
    }
}

impl AsCore<CorePosition> for AppPosition {
    fn as_core(&self) -> CorePosition {
        CorePosition {
            col: self.x as usize,
            row: self.y as usize,
        }
    }
}

impl AsCore<CoreSize> for AppSize {
    fn as_core(&self) -> CoreSize {
        CoreSize {
            rows: self.height as usize,
            cols: self.width as usize,
        }
    }
}
