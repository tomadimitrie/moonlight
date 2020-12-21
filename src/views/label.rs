use crate::types::view::View;
use std::any::Any;

pub struct Label {
    pub text: String,
}

impl View for Label {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}
