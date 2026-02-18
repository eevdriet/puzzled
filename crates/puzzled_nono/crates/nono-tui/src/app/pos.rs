use nono::Position as PuzzlePosition;
use ratatui::layout::Position as AppPosition;

pub fn puzzle_to_app(pos: PuzzlePosition) -> AppPosition {
    AppPosition {
        x: pos.col,
        y: pos.row,
    }
}

pub fn app_to_puzzle(pos: AppPosition) -> PuzzlePosition {
    PuzzlePosition {
        row: pos.y,
        col: pos.x,
    }
}
