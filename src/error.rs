use std::io;

pub fn thow_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}