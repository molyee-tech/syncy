use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Custom(Box<str>),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        s.to_owned().into()
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s.into_boxed_str())
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;