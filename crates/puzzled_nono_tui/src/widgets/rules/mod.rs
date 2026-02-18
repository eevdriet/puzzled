mod actions;
mod left;
mod state;
mod top;

pub use actions::*;
pub use left::*;
pub use state::*;
pub use top::*;

use nono::{Fill, Line, LineValidation, Rule};
use ratatui::{
    layout::Position,
    style::{Color, Modifier, Style},
};

use crate::{AppState, Focus};

pub struct RuleInfo<'a> {
    rule: &'a Rule,
    line: Line,
    validation: LineValidation,
}

pub fn run_style(info: &RuleInfo, fill: Fill, idx: u16, state: &AppState) -> Style {
    let RuleInfo {
        line, validation, ..
    } = info;

    let color = state
        .puzzle
        .style
        .fill_color(fill)
        .expect("Fill {fill:?} should have a defined color");

    let base = Style::default().fg(color);

    let mut style = match validation {
        // Cross out solved lines
        LineValidation::Solved => base
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM | Modifier::CROSSED_OUT),

        // Shade invalid rules in red
        v if !v.is_valid() => base
            .fg(Color::Red)
            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),

        _ => base,
    };

    let focus = state.focus;
    let cursor = state.cursor();
    let pos = match line {
        Line::Row(row) => Position::new(idx, *row),
        Line::Col(col) => Position::new(*col, idx),
    };
    let is_left = matches!(line, Line::Row(_)) && matches!(focus, Focus::RulesLeft);
    let is_top = matches!(line, Line::Col(_)) && matches!(focus, Focus::RulesTop);

    let is_active = match focus {
        // Highlight all runs in the active row/column when puzzle focused
        Focus::Puzzle => match line {
            Line::Row(row) => cursor.y == *row,
            Line::Col(col) => cursor.x == *col,
        },

        // Highlight the cursor run in the active line when rules focused
        _ if is_left || is_top => pos == cursor,
        _ => false,
    };

    let selection = state.selection();
    let is_selected = (is_left || is_top) && selection.contains(pos);

    if is_selected {
        style = style.add_modifier(Modifier::UNDERLINED);
    }

    if is_active || is_selected {
        style = style.add_modifier(Modifier::BOLD).not_dim();
    }

    style
}

pub fn status_info(info: &RuleInfo, state: &AppState) -> (Style, char) {
    let RuleInfo {
        line, validation, ..
    } = info;

    let cursor = state.puzzle.cursor;
    let base = Style::default().fg(Color::White);

    let mut style = match validation {
        LineValidation::Solved => base.fg(Color::Green),
        val if !val.is_valid() => base.fg(Color::Red),
        _ => base,
    };

    let is_active = match line {
        Line::Row(row) => {
            cursor.y == *row && matches!(state.focus, Focus::Puzzle | Focus::RulesLeft)
        }
        Line::Col(col) => {
            cursor.x == *col && matches!(state.focus, Focus::Puzzle | Focus::RulesTop)
        }
    };

    if is_active {
        style = style.add_modifier(Modifier::BOLD).not_dim();
    }

    let mut symbol = validation.symbol();
    if symbol == ' ' && is_active {
        symbol = match line {
            Line::Row(_) => '<',
            Line::Col(_) => '^',
        }
    };

    (style, symbol)
}
