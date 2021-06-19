use com_ptr::ComPtr;
use winapi::{shared::ntdef::HRESULT, Interface};

#[derive(Debug, Clone)]
pub enum ComError {
    HResult(i32),
    Message(String),
    StaticMessage(&'static str),
}

pub trait ComCreate {
    type Item: Interface;
    fn create_if_success<F>(f: F) -> Result<ComPtr<Self::Item>, ComError>
    where
        F: FnOnce(*mut *mut Self::Item) -> HRESULT;
}

impl<T: Interface> ComCreate for ComPtr<T> {
    type Item = T;

    fn create_if_success<F>(f: F) -> Result<ComPtr<Self::Item>, ComError>
    where
        F: FnOnce(*mut *mut Self::Item) -> HRESULT,
    {
        let mut ptr: *mut T = std::ptr::null_mut();
        let hr = f(&mut ptr);
        if hr == 0 {
            Ok(unsafe { ComPtr::from_raw(ptr) })
        } else {
            Err(ComError::HResult(hr))
        }
    }
}
