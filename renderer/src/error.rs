// #[derive(std::error::Error)]
pub enum Error {
    NotImpl,
    StaticMessage(&'static str),
    ComError(com_ptr_util::ComError),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    LoadError(scene::LoadError),
    ImageError(image::ImageError),
}
