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
