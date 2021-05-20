extern crate serde;
extern crate serde_json;

pub mod generated;
pub use generated::*;

impl glTF {
    pub fn get_accessor_bytes<'a>(&self, bin: &'a [u8], accessor_index: i32) -> Option<&'a [u8]> {
        Some(bin)
    }
}
