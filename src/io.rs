/// Impotrs
use camino::Utf8PathBuf;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

/// Defines IO Error
#[derive(Error, Debug)]
pub enum IoError {
    #[error("non-utf8 path: `{0:?}`")]
    NonUtf8Path(PathBuf),
    #[error("unable to find config directory")]
    UnableToFindConfigDirectory,
    #[error("unable to read file: `{0}`")]
    UnableToRead(io::Error),
    #[error("unable to write file: `{0}`")]
    UnableToWrite(io::Error),
    #[error("unable to create dir: `{0}`")]
    UnableToCreateDir(io::Error),
    #[error("no parent found for: `{0}`")]
    NoParent(PathBuf),
    #[error("io error: `{0}")]
    Unknown(io::Error),
}

/// Defines IO result
pub type IoResult<T> = Result<T, IoError>;

/// Returns config path
pub fn config_path() -> IoResult<Utf8PathBuf> {
    match dirs::config_dir() {
        Some(path) => match Utf8PathBuf::from_path_buf(path) {
            Ok(path) => Ok(path),
            Err(path) => Err(IoError::NonUtf8Path(path)),
        },
        None => Err(IoError::UnableToFindConfigDirectory),
    }
}

/// Reads a file, if it exists
pub fn read_file<P>(path: P) -> IoResult<String>
where
    P: AsRef<Path>,
{
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(err) => Err(IoError::UnableToRead(err)),
    }
}

/// Writes text to the file
pub fn write_file<P>(path: P, text: String) -> IoResult<()>
where
    P: AsRef<Path>,
{
    match fs::write(path, text) {
        Ok(_) => Ok(()),
        Err(err) => Err(IoError::UnableToWrite(err)),
    }
}

/// Creates all the directories recursively
pub fn create_dir<P>(path: P) -> IoResult<()>
where
    P: AsRef<Path>,
{
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(IoError::UnableToCreateDir(err)),
    }
}

/// Returns parent of path
pub fn parent<P>(path: P) -> IoResult<Utf8PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_path_buf();
    match path.parent() {
        Some(path) => {
            let buf = path.to_path_buf();
            match Utf8PathBuf::from_path_buf(buf.clone()) {
                Ok(path) => Ok(path),
                Err(_) => Err(IoError::NonUtf8Path(buf)),
            }
        }
        None => Err(IoError::NoParent(path.to_path_buf())),
    }
}
