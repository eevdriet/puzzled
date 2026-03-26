// Trait
pub trait Word {
    fn is_word(&self) -> bool;
}

macro_rules! impl_word {
    ($($ty:ty)+) => {
        $(
            impl Word for $ty {
                fn is_word(&self) -> bool {
                    true
                }
            }
        )+
    };
}

// Implementations
impl_word!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 String);

impl Word for char {
    fn is_word(&self) -> bool {
        self.is_whitespace() && self.is_ascii_punctuation()
    }
}
