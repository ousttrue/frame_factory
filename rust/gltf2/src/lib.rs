extern crate serde;
extern crate serde_json;

pub mod generated;
pub use generated::*;

impl glTF {
    pub fn get_accessor_bytes<T>(&self, bin: &[u8], accessor_index: i32) -> Option<&[T]> {
        let p = bin.as_ptr() as *const T;
        unsafe { Some(&std::slice::from_raw_parts(p, 1)) }
    }
}
