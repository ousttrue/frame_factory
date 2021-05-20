extern crate serde;
extern crate serde_json;

pub mod generated;
use std::usize;

pub use generated::*;

impl Accessor {
    pub fn stride(&self) -> i32 {
        let c = match self.componentType {
            Some(5120) => 1,
            Some(5121) => 1,
            Some(5122) => 2,
            Some(5123) => 2,
            Some(5125) => 4,
            Some(5126) => 4,
            _ => panic!(),
        };

        let t = match self.r#type.as_str() {
            "SCALAR" => 1,
            "VEC2" => 2,
            "VEC3" => 3,
            "VEC4" => 4,
            "MAT2" => 4,
            "MAT3" => 9,
            "MAT4" => 16,
            _ => panic!(),
        };

        c * t
    }
}

impl glTF {
    pub fn get_accessor_bytes<'a>(
        &self,
        bin: &'a [u8],
        accessor_index: i32,
    ) -> Option<(&'a [u8], i32, i32)> {
        let accessor = &self.accessors[accessor_index as usize];
        let buffer_view = &self.bufferViews[accessor.bufferView.unwrap() as usize];

        if buffer_view.buffer? != 0 {
            return None;
        }

        let start = buffer_view.byteOffset.unwrap_or(0) as usize;
        let end = start + buffer_view.byteLength.unwrap() as usize;
        let bytes = &bin[start..end];

        let start = accessor.byteOffset.unwrap_or(0) as usize;
        let stride = accessor.stride();
        let count = accessor.count?;
        let end = (start + count as usize * stride as usize) as usize;

        Some((&bytes[start..end], stride, count))
    }
}
