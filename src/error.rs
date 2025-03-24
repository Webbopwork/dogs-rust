use std::io;

pub fn throw_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}