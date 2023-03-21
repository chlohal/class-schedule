use std::io::{Error, ErrorKind};

pub fn err(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidData, msg)
}
