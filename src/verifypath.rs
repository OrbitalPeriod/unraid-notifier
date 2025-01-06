use std::path::{Path, PathBuf};

use crate::unraidnotifiererror::UnraidNotifierError;

/// Verify if there exists a file at the described path, fails if not.
pub trait VerifyPath {
    fn verify_path(&self) -> Result<(), UnraidNotifierError>;
}

impl VerifyPath for PathBuf {
    fn verify_path(&self) -> Result<(), UnraidNotifierError> {
        self.as_path().verify_path()
    }
}
impl VerifyPath for &Path{
    fn verify_path(&self) -> Result<(), UnraidNotifierError> {
        if !&self.try_exists().map_err(UnraidNotifierError::IOError)?{
            return  Err(UnraidNotifierError::InvalidPath);
        }else {
            Ok(())
        }
    }
}

impl VerifyPath for &str{
    fn verify_path(&self) -> Result<(), UnraidNotifierError> {
        Path::new(self).verify_path()
    }
}

impl VerifyPath for String{
    fn verify_path(&self) -> Result<(), UnraidNotifierError> {
        self.as_str().verify_path()
    }
}
