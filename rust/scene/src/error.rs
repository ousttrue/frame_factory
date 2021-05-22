pub enum LoadError {
    InvalidHeader,
    UnknownVersion,
    UnknownChunkType,
}

pub enum Error {
    NotImpl,
    StaticMessage(&'static str),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    LoadError(LoadError),
}
