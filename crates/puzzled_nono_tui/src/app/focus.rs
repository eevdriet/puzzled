#[derive(Debug, Default, Clone, Copy)]
pub enum Focus {
    #[default]
    Puzzle,

    RulesLeft,
    RulesTop,

    Footer,
}
