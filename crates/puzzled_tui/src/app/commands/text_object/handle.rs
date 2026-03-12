use crate::TextObject;

pub trait HandleBaseTextObject<T, S> {
    type Position;

    fn handle_base_text_object(
        &self,
        count: usize,
        obj: TextObject<T>,
        state: &mut S,
    ) -> impl IntoIterator<Item = Self::Position>;
}

pub trait HandleCustomTextObject<T, S> {
    type Position;

    fn handle_base_text_object(
        &self,
        count: usize,
        obj: TextObject<T>,
        state: &mut S,
    ) -> impl IntoIterator<Item = Self::Position>;
}
