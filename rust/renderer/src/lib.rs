extern crate scene;
extern crate image;
pub mod asset_manager;
pub mod com_util;
pub mod error;
pub mod resource;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
