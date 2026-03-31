mod render;

use puzzled_nonogram::{Fill, Rule, Rules};
pub use render::*;

pub fn fills_texts(rule: &Rule) -> (Vec<Fill>, Vec<String>) {
    rule.runs()
        .iter()
        .map(|run| (run.fill, run.count.to_string()))
        .unzip()
}

pub fn rule_display_lengths(mut width: u16, mut height: u16, rules: &Rules) -> (u16, u16) {
    // Determine the longest rule to display in both directions
    let max_row_rule_width = rules
        .iter_rows()
        .map(|(_, rule)| rule.display_len())
        .max()
        .unwrap_or_default();

    let max_col_rule_height = rules
        .iter_cols()
        .map(|(_, rule)| rule.display_len())
        .max()
        .unwrap_or_default();

    loop {
        let w = (max_row_rule_width as f64 / width as f64).ceil() as u16;
        let h = (max_col_rule_height as f64 / height as f64).ceil() as u16;

        if w > 1 {
            width -= 1;
        }
        if h > 1 {
            height -= 1;
        }

        if w == 1 && h == 1 {
            return (w, h);
        }
    }
}
