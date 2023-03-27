use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum Error {
    /// Catches all io Errors.
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Malformed property: {0}")]
    MalformedProperty(String),

    #[error("Tried to access an unset property: {0}")]
    UnsetProperty(String),
}
