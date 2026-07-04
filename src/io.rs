/// Impotrs
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, bail};
use std::{fs, path::Path};

/// Returns config path
pub fn config_path() -> miette::Result<Utf8PathBuf> {
    match dirs::config_dir() {
        Some(path) => match Utf8PathBuf::from_path_buf(path) {
            Ok(path) => Ok(path),
            Err(path) => bail!("non-utf8 path: {path:?}"),
        },
        None => bail!("unable to find configs dir"),
    }
}

/// Reads a file, if it exists
pub fn read_file<P>(path: P) -> miette::Result<String>
where
    P: AsRef<Path>,
{
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(err) => bail!("unable to read file: {err}"),
    }
}

/// Writes text to the file
pub fn write_file<P>(path: P, text: String) -> miette::Result<()>
where
    P: AsRef<Path>,
{
    match fs::write(path, text) {
        Ok(_) => Ok(()),
        Err(err) => bail!("unable to read file: {err}"),
    }
}

/// Creates all the directories recursively
pub fn create_dir<P>(path: P) -> miette::Result<()>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(path).into_diagnostic()
}
