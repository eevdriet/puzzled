use puzzled_core::Timer;
use ratatui::{
    prelude::{Buffer, Rect, Size},
    text::Text,
    widgets::Widget,
};

use crate::RenderSize;

pub struct TimerWidget {
    pub timer: Timer,
}

impl<S> RenderSize<S> for TimerWidget {
    fn render_size(&self, _state: &S) -> Size {
        Size {
            width: 8, // HH:mm:ss
            height: 1,
        }
    }
}

impl Widget for TimerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let elapsed = self.timer.elapsed().as_secs();
        let hours = elapsed / 3600;
        let minutes = (elapsed % 3600) / 60;
        let seconds = elapsed % 60;

        let display = format!("{hours:02}:{minutes:02}:{seconds:02}");
        Text::from(display).render(area, buf);
    }
}
