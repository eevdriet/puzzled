#[derive(Debug, Clone, Copy, Default)]
pub enum Axis {
    #[default]
    Row,

    Col,
}

impl Axis {
    pub fn switch(&mut self) {
        *self = match self {
            Axis::Row => Axis::Col,
            Axis::Col => Axis::Row,
        }
    }
}
