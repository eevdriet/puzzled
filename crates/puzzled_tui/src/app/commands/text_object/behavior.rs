use crate::{AppTypeTraits, Describe, TextObject};

pub trait TextObjectBehavior: AppTypeTraits {
    fn variants() -> Vec<Self>;
}

impl TextObjectBehavior for () {
    fn variants() -> Vec<Self> {
        vec![]
    }
}

impl<T> Describe for TextObject<T>
where
    T: Describe,
{
    fn describe(&self) -> Option<String> {
        None
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
