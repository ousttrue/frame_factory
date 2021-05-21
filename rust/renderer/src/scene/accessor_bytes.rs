pub struct AccessorBytes {
    pub bytes: Vec<u8>,
    pub stride: u32,
    pub count: u32,
}

impl AccessorBytes {
    pub fn new(bytes: Vec<u8>, stride: u32, count: u32) -> AccessorBytes {
        AccessorBytes {
            bytes,
            stride,
            count,
        }
    }

    pub fn from<T: Sized>(src: &[T]) -> AccessorBytes {
        let stride = std::mem::size_of::<T>();
        let mut bytes: Vec<u8> = Vec::with_capacity(stride * src.len());
        let len = bytes.len();
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr() as *const std::ffi::c_void,
                bytes.as_mut_ptr() as *mut std::ffi::c_void,
                bytes.len(),
            );
        }
        Self::new(bytes, stride as u32, len as u32)
    }
}
