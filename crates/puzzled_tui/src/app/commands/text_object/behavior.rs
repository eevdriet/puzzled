use crate::{AppTypeTraits, Description, TextObject};

pub trait TextObjectBehavior: AppTypeTraits {
    fn variants() -> Vec<Self>;
}

impl TextObjectBehavior for () {
    fn variants() -> Vec<Self> {
        vec![]
    }
}

impl<T, S> Description<S> for TextObject<T>
where
    T: Description<S>,
{
    fn description(&self, state: &S) -> Option<String> {
        let desc = match self {
            TextObject::Word => "Word under the cursor",
            TextObject::Custom(custom) => return custom.description(state),
        };

        Some(desc.to_string())
    }
}

impl<T> TextObjectBehavior for TextObject<T>
where
    T: TextObjectBehavior,
{
    fn variants() -> Vec<Self> {
        let mut variants = vec![TextObject::Word];

        let other_variants = T::variants().into_iter().map(TextObject::Custom);
        variants.extend(other_variants);

        variants
    }
}
