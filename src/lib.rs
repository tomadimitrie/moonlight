mod cleanup;
mod types;
mod views;
use crate::{
    types::view::View,
    views::{container::Container, label::Label},
};
use moonlight_macros::moonlight_main;

struct Wrapper;
impl View for Wrapper {
    fn render(&self) -> Option<Box<dyn View>> {
        Some(Box::new(Container {
            children: vec![Box::new(Label {
                text: String::from("hello"),
            })],
        }))
    }
}

#[moonlight_main]
fn run() -> Box<dyn View> {
    Box::new(Wrapper {})
}
