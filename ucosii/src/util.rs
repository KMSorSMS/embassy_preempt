use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ptr;

pub(crate) struct UninitCell<T>(MaybeUninit<UnsafeCell<T>>);

#[allow(unused)]
impl<T> UninitCell<T> {
    pub const fn uninit() -> Self {
        Self(MaybeUninit::uninit())
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut T {
        (*self.0.as_ptr()).get()
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut(&self) -> &mut T {
        &mut *self.as_mut_ptr()
    }

    #[inline(never)]
    pub unsafe fn write_in_place(&self, func: impl FnOnce() -> T) {
        ptr::write(self.as_mut_ptr(), func())
    }

    pub unsafe fn drop_in_place(&self) {
        ptr::drop_in_place(self.as_mut_ptr())
    }
}

unsafe impl<T> Sync for UninitCell<T> {}

#[repr(transparent)]
pub struct SyncUnsafeCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

impl <T: PartialEq> PartialEq for SyncUnsafeCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_unmut() == other.get_unmut()
    }
}
impl <T: PartialOrd> PartialOrd for SyncUnsafeCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.get_unmut().partial_cmp(other.get_unmut())
    }
}
// imple clone for SyncUnsafeCell
impl<T: Clone> Clone for SyncUnsafeCell<T> {
    fn clone(&self) -> Self {
        Self {
            value: UnsafeCell::new(self.get_unmut().clone()),
        }
    }
}

impl<T> SyncUnsafeCell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub unsafe fn set(&self, value: T) {
        *self.value.get() = value;
    }

    pub unsafe fn get(&self) -> T
    where
        T: Copy,
    {
        *self.value.get()
    }
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
    pub fn get_unmut(&self) -> &T {
        unsafe { &*self.value.get() }
    }
    /// set and return the old value
    pub unsafe fn swap(&self, value: T) -> T {
        core::mem::replace(&mut *self.value.get(), value)
    }
}
