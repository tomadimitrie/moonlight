use std::ffi::c_void;

#[repr(C)]
pub struct NativeView {
    pub attributes_ptr: *const *const c_void,
    pub attributes_size: usize,
    pub children_ptr: *const *const NativeView,
    pub children_size: usize,
}
