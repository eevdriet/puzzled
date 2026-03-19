use std::fmt::Debug;

use crate::TextObject;

pub trait TextObjectBehavior:
    Clone + PartialEq + Eq + PartialOrd + Ord + Send + Debug + Sized
{
    fn variants() -> Vec<Self>;
}

impl TextObjectBehavior for () {
    fn variants() -> Vec<Self> {
        vec![]
    }
}

impl<T> TextObjectBehavior for TextObject<T>
where
    T: TextObjectBehavior,
{
    fn variants() -> Vec<Self> {
        let mut variants = vec![];

        let other_variants = T::variants().into_iter().map(TextObject::Custom);
        variants.extend(other_variants);

        variants
    }
}
