use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub trait ComputeLayout {
    fn compute_layout(&mut self, root: Rect);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Offset the puzzle to be in the middle of the area
    // let x_center = (area.width as i32 - cols as i32).max(0) / 2;
    // let y_center = (area.height as i32 - rows as i32).max(0) / 2;
    // state.offset = Offset::new(origin.x as i32 + x_center, origin.y as i32 + y_center);

    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
