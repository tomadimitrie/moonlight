use std::any::Any;

pub trait View {
    fn render(&self) -> Option<Box<dyn View>> {
        None
    }
    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
}
