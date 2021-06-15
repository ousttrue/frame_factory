mod generated;
pub use generated::*;
mod sdl;
pub use sdl::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
