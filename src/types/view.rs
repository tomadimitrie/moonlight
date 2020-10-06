use crate::types::native_view::NativeView;

pub trait View {
    fn render(&self) -> Box<dyn View> {
        unimplemented!()
    }
    fn to_native(&self) -> *const NativeView {
        unimplemented!()
    }
}
