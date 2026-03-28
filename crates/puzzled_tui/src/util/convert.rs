use puzzled_core::{Color as CoreColor, Position as CorePosition, Size as CoreSize};
use ratatui::{
    layout::{Position as AppPosition, Size as AppSize},
    style::Color as AppColor,
};

pub trait AsApp<T> {
    fn as_app(&self) -> T;
}

pub trait AsCore<T> {
    fn as_core(&self) -> T;
}

impl<T> AsApp<T> for T
where
    T: Copy,
{
    fn as_app(&self) -> T {
        *self
    }
}

impl<T> AsCore<T> for T
where
    T: Copy,
{
    fn as_core(&self) -> T {
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

impl AsApp<AppSize> for CoreSize {
    fn as_app(&self) -> AppSize {
        AppSize {
            width: self.cols as u16,
            height: self.rows as u16,
        }
    }
}

impl AsApp<AppColor> for CoreColor {
    fn as_app(&self) -> AppColor {
        AppColor::Rgb(self.red, self.green, self.blue)
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
