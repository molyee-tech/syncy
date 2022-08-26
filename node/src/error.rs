use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
