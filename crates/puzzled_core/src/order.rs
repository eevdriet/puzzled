#[derive(Debug, Clone, Copy, Default)]
pub enum Order {
    #[default]
    RowMajor,

    ColMajor,
}

impl Order {
    pub fn switched(&self) -> Self {
        match self {
            Self::RowMajor => Self::ColMajor,
            Self::ColMajor => Self::RowMajor,
        }
    }
    pub fn switch(&mut self) {
        *self = self.switched()
    }
}
