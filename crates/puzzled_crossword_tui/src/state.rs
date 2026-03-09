use puzzled_tui::EventMode;

pub struct AppState {
    mode: EventMode,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: EventMode::Normal,
        }
    }
}
