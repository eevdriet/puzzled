use puzzled_tui::EventMode;

pub struct AppState {
    _mode: EventMode,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            _mode: EventMode::Normal,
        }
    }
}
