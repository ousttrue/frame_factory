extern crate scene;
pub mod asset_manager;
pub mod com_util;
pub mod error;
pub mod resource;
use std::{ffi::CStr};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
