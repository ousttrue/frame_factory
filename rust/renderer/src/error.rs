pub enum LoadError {
    FileNotFound,
    InvalidHeader,
    UnknownVersion,
    UnknownChunkType,
    // ShaderAsset,
}

// #[derive(std::error::Error)]
pub enum Error {
    NotImpl,
    ComError(u32),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    LoadError(LoadError),
}
