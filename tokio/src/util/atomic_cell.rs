use crate::loom::sync::atomic::AtomicPtr;

use std::ptr;
use std::sync::atomic::Ordering::AcqRel;

pub(crate) struct AtomicCell<T> {
    data: AtomicPtr<T>,
}

impl<T> AtomicCell<T> {
    pub(crate) fn new(data: Option<Box<T>>) -> AtomicCell<T> {
        AtomicCell {
            data: AtomicPtr::new(to_raw(data)),
        }
    }

    pub(crate) fn take(&self) -> Option<Box<T>> {
        self.swap(None)
    }

    pub(crate) fn swap(&self, val: Option<Box<T>>) -> Option<Box<T>> {
        let old = self.data.swap(to_raw(val), AcqRel);
        from_raw(old)
    }
}

fn to_raw<T>(data: Option<Box<T>>) -> *mut T {
    data.map_or(ptr::null_mut(), Box::into_raw)
}

fn from_raw<T>(val: *mut T) -> Option<Box<T>> {
    if val.is_null() {
        None
    } else {
        Some(unsafe { Box::from_raw(val) })
    }
}