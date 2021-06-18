extern crate serde;
extern crate serde_json;

pub mod generated;

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
