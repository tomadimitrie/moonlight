use crate::types::view::View;
use moonlight_macros::NativeImpl;

#[derive(NativeImpl)]
pub struct Container {
    #[convert]
    pub name: String,
    #[convert]
    pub tag: usize,
    #[next]
    pub children: Vec<Box<dyn View>>,
}
