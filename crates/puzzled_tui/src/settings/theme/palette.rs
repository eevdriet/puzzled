use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default)]
pub struct Palette {
    // Light
    pub light0: Color,
    pub light1: Color,
    pub light2: Color,
    pub light3: Color,

    // Dark
    pub dark0: Color,
    pub dark1: Color,
    pub dark2: Color,
    pub dark3: Color,

    // Accents
    pub blue: Color,
    pub cyan: Color,
    pub green: Color,
    pub magenta: Color,
    pub orange: Color,
    pub red: Color,
    pub violet: Color,
    pub yellow: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            dark3: Color::Black,
            dark2: Color::DarkGray,
            dark1: Color::DarkGray,
            dark0: Color::DarkGray,
            light0: Color::Gray,
            light1: Color::Gray,
            light2: Color::Gray,
            light3: Color::White,

            yellow: Color::Yellow,
            red: Color::Red,
            magenta: Color::Magenta,
            blue: Color::Blue,
            cyan: Color::Cyan,
            green: Color::Green,

            orange: Color::Yellow,
            violet: Color::Magenta,
        }
    }
}

impl Palette {
    pub const SOLARIZED: Palette = Palette {
        dark3: Color::Rgb(0x00, 0x2b, 0x36),  // base03
        dark2: Color::Rgb(0x07, 0x36, 0x42),  // base02
        dark1: Color::Rgb(0x58, 0x6e, 0x75),  // base01
        dark0: Color::Rgb(0x65, 0x7b, 0x83),  // base00
        light0: Color::Rgb(0x83, 0x94, 0x96), // base0
        light1: Color::Rgb(0x93, 0xa1, 0xa1), // base1
        light2: Color::Rgb(0xee, 0xe8, 0xd5), // base2
        light3: Color::Rgb(0xfd, 0xf6, 0xe3), // base3

        yellow: Color::Rgb(0xb5, 0x89, 0x00),
        orange: Color::Rgb(0xcb, 0x4b, 0x16),
        red: Color::Rgb(0xdc, 0x32, 0x2f),
        magenta: Color::Rgb(0xd3, 0x36, 0x82),
        violet: Color::Rgb(0x6c, 0x71, 0xc4),
        blue: Color::Rgb(0x26, 0x8b, 0xd2),
        cyan: Color::Rgb(0x2a, 0xa1, 0x98),
        green: Color::Rgb(0x85, 0x99, 0x00),
    };

    pub const NORD: Palette = Palette {
        dark3: Color::Rgb(0x2e, 0x34, 0x40),  // nord0
        dark2: Color::Rgb(0x3b, 0x42, 0x52),  // nord1
        dark1: Color::Rgb(0x43, 0x4c, 0x5e),  // nord2
        dark0: Color::Rgb(0x4c, 0x56, 0x6a),  // nord3
        light0: Color::Rgb(0xd8, 0xde, 0xe9), // nord4
        light1: Color::Rgb(0xe5, 0xe9, 0xf0), // nord5
        light2: Color::Rgb(0xec, 0xef, 0xf4), // nord6
        light3: Color::Rgb(0xec, 0xef, 0xf4), // ..

        cyan: Color::Rgb(0x8f, 0xbc, 0xbb),    // nord7
        blue: Color::Rgb(0x54, 0x81, 0xac),    // nord10
        red: Color::Rgb(0xbf, 0x61, 0x6a),     // nord11
        orange: Color::Rgb(0xd0, 0x87, 0x70),  // nord12
        yellow: Color::Rgb(0xeb, 0xcb, 0x8b),  // nord13
        green: Color::Rgb(0xa3, 0xbe, 0x8c),   // nord14
        violet: Color::Rgb(0xb4, 0x8e, 0xad),  // nord15
        magenta: Color::Rgb(0xb4, 0x8e, 0xad), // ..
    };

    pub fn get(&self, color: PaletteColor) -> Color {
        match color {
            PaletteColor::Light0 => self.light0,
            PaletteColor::Light1 => self.light1,
            PaletteColor::Light2 => self.light2,
            PaletteColor::Light3 => self.light3,
            PaletteColor::Dark0 => self.dark0,
            PaletteColor::Dark1 => self.dark1,
            PaletteColor::Dark2 => self.dark2,
            PaletteColor::Dark3 => self.dark3,
            PaletteColor::Blue => self.blue,
            PaletteColor::Cyan => self.cyan,
            PaletteColor::Green => self.green,
            PaletteColor::Magenta => self.magenta,
            PaletteColor::Orange => self.orange,
            PaletteColor::Red => self.red,
            PaletteColor::Violet => self.violet,
            PaletteColor::Yellow => self.yellow,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PaletteColor {
    Light0,
    Light1,
    Light2,
    Light3,
    Dark0,
    Dark1,
    Dark2,
    Dark3,
    Blue,
    Cyan,
    Green,
    Magenta,
    Orange,
    Red,
    Violet,
    Yellow,
}
