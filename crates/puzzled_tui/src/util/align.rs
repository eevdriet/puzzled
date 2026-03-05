use ratatui::layout::{HorizontalAlignment, Rect, Size, VerticalAlignment};

pub fn align_area(
    size: Size,
    parent: Rect,
    h_align: HorizontalAlignment,
    v_align: VerticalAlignment,
) -> Rect {
    let (x, width) = align_horizontally(size.width, parent.left(), parent.right(), h_align);
    let (y, height) = align_vertically(size.height, parent.top(), parent.bottom(), v_align);

    Rect::new(x, y, width, height)
}

pub fn align_horizontally(
    width: u16,
    left: u16,
    right: u16,
    align: HorizontalAlignment,
) -> (u16, u16) {
    let parent_width = right.saturating_sub(left);

    let x = match align {
        HorizontalAlignment::Left => left,
        HorizontalAlignment::Center => left + parent_width.saturating_sub(width) / 2,
        HorizontalAlignment::Right => right.saturating_sub(width),
    };
    let width = width.min(right.saturating_sub(x));

    (x, width)
}

pub fn align_vertically(
    height: u16,
    top: u16,
    bottom: u16,
    align: VerticalAlignment,
) -> (u16, u16) {
    let parent_height = bottom.saturating_sub(top);
    let height = height.min(parent_height);

    let y = match align {
        VerticalAlignment::Top => top,
        VerticalAlignment::Center => top + parent_height.saturating_sub(height) / 2,
        VerticalAlignment::Bottom => bottom.saturating_sub(height),
    };
    let height = height.min(bottom.saturating_sub(y));

    (y, height)
}
