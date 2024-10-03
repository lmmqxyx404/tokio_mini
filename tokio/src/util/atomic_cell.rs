use crate::loom::sync::atomic::AtomicPtr;

use std::ptr;

pub(crate) struct AtomicCell<T> {
    data: AtomicPtr<T>,
}

impl<T> AtomicCell<T> {
    pub(crate) fn new(data: Option<Box<T>>) -> AtomicCell<T> {
        AtomicCell {
            data: AtomicPtr::new(to_raw(data)),
        }
    }
}

fn to_raw<T>(data: Option<Box<T>>) -> *mut T {
    data.map_or(ptr::null_mut(), Box::into_raw)
}
