use std::error::Error;

/// Errors, InvalidPath is returned if the path is invalid, IOError is used for if the path could
/// not be read, or the process could not be spawned.
#[derive(Debug)]
pub enum UnraidNotifierError {
    InvalidPath,
    IOError(std::io::Error),
}

impl std::fmt::Display for UnraidNotifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnraidNotifierError::InvalidPath => write!(f, "Invalid path"),
            UnraidNotifierError::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for UnraidNotifierError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self{
            Self::IOError(err) => Some(err),
            _ => None
        }
    }
}
