use std::error::Error;
use std::fmt;

use esm::esm_error::EsmError;

#[derive(Debug)]
pub enum AlarmError {
    Io(std::io::Error),
    InvalidArgument(String),
    Esm(EsmError),
}

impl fmt::Display for AlarmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlarmError::Io(err) => write!(f, "IO error: {}", err),
            AlarmError::InvalidArgument(msg) => write!(f, "Invalid argument received: {}", msg),
            AlarmError::Esm(msg) => write!(f, "ESM error: {}", msg),
        }
    }
}

impl Error for AlarmError {
    // source() allows you to "reach through" your error to the underlying cause
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AlarmError::Io(err) => Some(err),
            AlarmError::Esm(err) => err.source(),
            AlarmError::InvalidArgument(_) => None,
        }
    }
}

impl From<std::io::Error> for AlarmError {
    fn from(err: std::io::Error) -> AlarmError {
        AlarmError::Io(err)
    }
}

impl From<EsmError> for AlarmError {
        fn from(err: EsmError) -> AlarmError {
        AlarmError::Esm(err)
    }
}
