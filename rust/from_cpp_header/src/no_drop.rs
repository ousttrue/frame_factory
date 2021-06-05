use std::ops::{Deref, DerefMut};

pub struct NoDrop<T> {
    data: Option<Box<T>>,
}

impl<T> Drop for NoDrop<T> {
    fn drop(&mut self) {
        if let Some(box_data) = self.data.take() {
            Box::into_raw(box_data);
        }
    }
}

impl<T> Deref for NoDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().unwrap()
    }
}

impl<T> DerefMut for NoDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().unwrap()
    }
}

impl<T> NoDrop<T> {
    pub fn new(data: *mut T) -> NoDrop<T> {
        NoDrop::<T> {
            data: Some(unsafe { Box::from_raw(data) }),
        }
    }
}
