#[derive(Debug, PartialEq)]
pub enum Error {
    Io(String),
    InvalidInputSpec,
    NotImplemented,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error.to_string())
    }
}

// impl From<std::num::ParseIntError> for Error {
//     fn from(_error: std::num::ParseIntError) -> Self {
//         Error::InvalidInputSpec
//     }
// }

pub(crate) type Result<T> = std::result::Result<T, Error>;
