use crate::types::native_view::NativeView;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn end_app(tree: *const NativeView) {
    let tree = &*tree;
    let children = Vec::from_raw_parts(
        tree.children_ptr as *mut *mut NativeView,
        tree.children_size,
        tree.children_size,
    );
    let attributes = Vec::from_raw_parts(
        tree.attributes_ptr as *mut *mut i8,
        tree.attributes_size,
        tree.attributes_size,
    );
    for attribute in attributes {
        CString::from_raw(attribute as *mut i8);
    }
    children.iter().for_each(|child| end_app(*child));
    let tree = tree as *const NativeView as *mut NativeView;
    Box::from_raw(tree);
}
