use std::path::Path;

use crate::{Nonogram, io};

pub struct TextLoader;

impl io::PuzzleLoader for TextLoader {
    fn load_nonogram(_path: &Path) -> io::Result<Nonogram> {
        todo!()
    }
}

// fn parse_from_puzzle(buffer: &str, config: &Config) -> Result<(Puzzle, Rules, PuzzleStyle)> {
//     let mut fills = Vec::<Fill>::with_capacity(buffer.len());
//     let lines = buffer.lines();
//
//     let mut rows = 0;
//     let cols = lines.map(|line| line.len() as u16).max().unwrap();
//
//     for line in buffer.lines() {
//         let mut col_count = 0;
//         rows += 1;
//
//         for ch in line.chars() {
//             col_count += 1;
//
//             let fill = match ch {
//                 cell if cell.is_whitespace() || cell == '.' => Fill::Blank,
//                 _ => Fill::Color(ch as u16),
//             };
//             fills.push(fill);
//         }
//
//         for _ in col_count..cols {
//             fills.push(Fill::Blank);
//         }
//     }
//
//     let puzzle = Puzzle::new(rows, cols, fills).expect("checked rows and cols");
//     let rules = Rules::from_puzzle(&puzzle);
//     let style = config.styles.clone();
//
//     Ok((puzzle, rules, style))
// }
