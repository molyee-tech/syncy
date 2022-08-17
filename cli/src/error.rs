use thiserror::Error;
use core::borrow::Borrow;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(Box<str>),
}

impl<S: Borrow<str>> From<S> for Error {
    fn from(s: S) -> Self {
        let m = s.borrow().to_owned().into_boxed_str();
        Error::Custom(m)
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;