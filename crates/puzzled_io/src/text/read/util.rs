pub trait CellText {
    fn is_cell_style(&self) -> bool;
    fn is_cell_contents(&self) -> bool;
}

const CELL_STYLE_CHARS: &str = "~!@*";
const CELL_CONTENT_CHARS: &str = "abcdefghijklmnopqrstuvwxyz\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
0123456789\
`#$%^&_=+[]\\{}|;:'\"<>/? "; // ~!@* () -.,

impl CellText for char {
    fn is_cell_style(&self) -> bool {
        CELL_STYLE_CHARS.contains(*self)
    }

    fn is_cell_contents(&self) -> bool {
        CELL_CONTENT_CHARS.contains(*self)
    }
}
