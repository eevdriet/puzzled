use ratatui::widgets::TableState;

use crate::SettingValue;

const ITEM_HEIGHT: usize = 4;

pub struct SettingsWidget {
    settings: Vec<Setting>,
    state: TableState,
}

pub struct Setting {
    key: String,
    description: String,
    value: SettingValue,
}
