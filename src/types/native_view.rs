use crate::types::view::View;
use crate::views::{container::Container, label::Label};
use std::{
    ffi::{c_void, CString},
    mem,
};

#[repr(C)]
#[derive(Debug)]
pub struct NativeView {
    pub attributes_ptr: *const *const c_void,
    pub attributes_size: usize,
    pub children_ptr: *const *const NativeView,
    pub children_size: usize,
    pub is_wrapper: bool,
}

impl NativeView {
    pub fn into_ptr(self) -> *const Self {
        Box::into_raw(Box::new(self)) as *const Self
    }
    pub fn from_view(view: &Box<dyn View>) -> Self {
        let any = view.as_any();
        match any {
            Some(any) => match any.downcast_ref::<Container>() {
                Some(container) => {
                    let attributes = Vec::<*const c_void>::new();
                    let attributes_ptr = attributes.as_ptr();
                    let attributes_size = 0;
                    mem::forget(attributes);
                    let children = container
                        .children
                        .iter()
                        .map(|child| NativeView::from_view(child).into_ptr())
                        .collect::<Vec<_>>();
                    let children_ptr = children.as_ptr();
                    let children_size = children.len();
                    mem::forget(children);

                    NativeView {
                        attributes_ptr,
                        attributes_size,
                        children_ptr,
                        children_size,
                        is_wrapper: false,
                    }
                }
                None => match any.downcast_ref::<Label>() {
                    Some(label) => {
                        let children = Vec::<*const NativeView>::new();
                        let children_ptr = children.as_ptr();
                        let children_size = 0;
                        mem::forget(children);

                        let attributes: Vec<*const c_void> = vec![
                            CString::new("text").unwrap().into_raw() as *const c_void,
                            CString::new(label.text.clone()).unwrap().into_raw() as *const c_void,
                            CString::new("String").unwrap().into_raw() as *const c_void,
                        ];
                        let attributes_ptr = attributes.as_ptr();
                        let attributes_size = attributes.len();
                        mem::forget(attributes);

                        NativeView {
                            attributes_ptr,
                            attributes_size,
                            children_ptr,
                            children_size,
                            is_wrapper: false,
                        }
                    }
                    None => panic!(),
                },
            },
            None => {
                let attributes = Vec::<*const c_void>::new();
                let attributes_ptr = attributes.as_ptr();
                let attributes_size = 0;
                mem::forget(attributes);
                let mut children = Vec::<*const NativeView>::new();
                match view.render() {
                    Some(child) => children.push(NativeView::from_view(&child).into_ptr()),
                    None => {}
                };
                let children_ptr = children.as_ptr();
                let children_size = children.len();
                mem::forget(children);

                NativeView {
                    attributes_ptr,
                    attributes_size,
                    children_ptr,
                    children_size,
                    is_wrapper: true,
                }
            }
        }
    }
}
