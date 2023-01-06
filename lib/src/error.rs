// #![allow(unused)]

use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    InvalidSignature,
    InvalidPixelData,
    UnsupportedCompression,
    UnsupportedBitDepth,
    InvalidRange(usize),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}
