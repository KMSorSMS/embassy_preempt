//! Uniprocessor interior mutability primitives
use core::cell::{Ref, RefCell, RefMut};

/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`.
pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    /// Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
    /// set the inner data
    pub fn set(&self, value: T) {
        *self.inner.borrow_mut() = value;
    }
    /// get the unmutable reference of inner data
    pub fn get(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }
    /// set and return the old value
    pub fn swap(&self, value: T) -> T {
        let mut inner = self.inner.borrow_mut();
        core::mem::replace(&mut *inner, value)
    }
}
