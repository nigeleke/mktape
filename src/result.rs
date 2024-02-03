#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidArgsCount,
    InvalidTapeFilename,
    Io(String),
    EnvVar(String),
    InvalidInputSpec,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error.to_string())
    }
}

#[cfg(test)]
impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::EnvVar(error.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Error::InvalidInputSpec
    }
}

pub type Result<T> = std::result::Result<T, Error>;
