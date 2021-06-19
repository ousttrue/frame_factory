extern crate scene;
extern crate image;
extern crate gen;
extern crate com_ptr_util;

pub mod asset_manager;
pub mod error;
pub mod resource;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
