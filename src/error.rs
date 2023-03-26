use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum Error {
    /// Catches all io Errors.
    #[error(transparent)]
    Io(#[from] io::Error),
}
