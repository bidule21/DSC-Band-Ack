use std::fmt;



#[derive(Debug)]
pub enum TickError {
    /// Device has not send a response
    NoAnswerFromDevice,
    InvalidAnswer,
    UnkknownError,
}

impl TickError {
    pub fn from_u8(n: u8) -> Option<TickError> {
        match n {
            0 => None,
            1 => Some(TickError::NoAnswerFromDevice),
            2 => Some(TickError::InvalidAnswer),
            _ => Some(TickError::UnkknownError),
        }
    }
}

impl fmt::Display for TickError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TickError::NoAnswerFromDevice =>
                write!(f, "No answer from device was recived"),
            TickError::InvalidAnswer =>
                write!(f, "Device answer was not valid"),
            TickError::UnkknownError =>
                write!(f, "Unkknown error"),
        }

    }
}
