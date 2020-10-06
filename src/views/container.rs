use crate::types::{native_view::NativeView, view::View};
use std::{
    ffi::{c_void, CString},
    mem::forget,
};

pub struct Container {
    pub name: String,
    pub children: Vec<Box<dyn View>>,
}

impl View for Container {
    fn to_native(&self) -> *const NativeView {
        let key = CString::new("text").unwrap();
        let key_ptr = key.as_ptr() as *const c_void;
        forget(key);

        let value = CString::new(self.name.clone()).unwrap();
        let value_ptr = value.as_ptr() as *const c_void;
        forget(value);

        let kind = CString::new("string").unwrap();
        let kind_ptr = kind.as_ptr() as *const c_void;
        forget(kind);

        let attributes = vec![key_ptr, value_ptr, kind_ptr];
        let attributes_size = attributes.len();
        let attributes_ptr = attributes.as_ptr();
        forget(attributes);

        let children = self
            .children
            .iter()
            .map(|child| child.to_native())
            .collect::<Vec<_>>();
        let children_size = children.len();
        let children_ptr = children.as_ptr();
        forget(children);

        Box::into_raw(Box::new(NativeView {
            attributes_ptr,
            attributes_size,
            children_ptr,
            children_size,
        })) as *const NativeView
    }
}
