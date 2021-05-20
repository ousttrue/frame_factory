pub enum LoadError {
    InvalidHeader,
    UnknownVersion,
    UnknownChunkType,
}

// #[derive(std::error::Error)]
pub enum Error {
    NotImpl,
    ComError(crate::com_util::ComError),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    LoadError(LoadError),
}
