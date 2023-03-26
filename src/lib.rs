pub mod echonet;
pub mod error;

pub type Result<T> = core::result::Result<T, error::Error>;
