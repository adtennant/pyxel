use std::{error::Error, fmt};

/// An enumeration of Pyxel errors.
#[derive(Debug)]
pub enum PyxelError {
    /// An error occured during an IO operation.
    Io(std::io::Error),

    /// An error occured during a zip operation.
    Zip(zip::result::ZipError),

    /// An error occured during deserialization.
    Serde(serde_json::error::Error),

    /// An error occured whilst loading an image.
    Image(image::ImageError),
}

impl fmt::Display for PyxelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PyxelError::Io(ref e) => e.fmt(f),
            PyxelError::Zip(ref e) => e.fmt(f),
            PyxelError::Serde(ref e) => e.fmt(f),
            PyxelError::Image(ref e) => e.fmt(f),
        }
    }
}

impl Error for PyxelError {
    fn description(&self) -> &str {
        match *self {
            PyxelError::Io(ref e) => e.description(),
            PyxelError::Zip(ref e) => e.description(),
            PyxelError::Serde(ref e) => e.description(),
            PyxelError::Image(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PyxelError::Io(ref e) => Some(e),
            PyxelError::Zip(ref e) => Some(e),
            PyxelError::Serde(ref e) => Some(e),
            PyxelError::Image(ref e) => Some(e),
        }
    }
}

impl std::convert::From<std::io::Error> for PyxelError {
    fn from(err: std::io::Error) -> PyxelError {
        PyxelError::Io(err)
    }
}

impl std::convert::From<zip::result::ZipError> for PyxelError {
    fn from(err: zip::result::ZipError) -> PyxelError {
        PyxelError::Zip(err)
    }
}

impl std::convert::From<serde_json::error::Error> for PyxelError {
    fn from(err: serde_json::error::Error) -> PyxelError {
        PyxelError::Serde(err)
    }
}

impl std::convert::From<image::ImageError> for PyxelError {
    fn from(err: image::ImageError) -> PyxelError {
        PyxelError::Image(err)
    }
}
