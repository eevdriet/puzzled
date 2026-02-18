use puzzled_nono::Position as PuzzlePosition;
use ratatui::layout::Position as AppPosition;

pub fn puzzle_to_app(pos: PuzzlePosition) -> AppPosition {
    AppPosition {
        x: pos.col as u16,
        y: pos.row as u16,
    }
}

pub fn app_to_puzzle(pos: AppPosition) -> PuzzlePosition {
    PuzzlePosition {
        row: pos.y as usize,
        col: pos.x as usize,
    }
}
