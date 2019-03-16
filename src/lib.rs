//! # Pyxel
//!
//! Pyxel is a library for loading [PyxelEdit](https://pyxeledit.com) documents in Rust. Current only the latest (v0.4.8) version of PyxelEdit is officially supported.

#![allow(unknown_lints)]
#![deny(clippy::all)]
#![deny(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)]

use std::{fs::File, io::Cursor, path::Path};

mod deserialization;
mod error;
mod pyxel;

pub use crate::error::PyxelError;
pub use crate::pyxel::*;

/// Load a Pyxel document from a byte slice.
///
/// # Examples
///
/// ```
/// use std::fs;
/// # fn main() -> Result<(), pyxel::PyxelError> {
/// let buf = fs::read("resources/doc.pyxel")?;
/// let doc = pyxel::load_from_memory(&buf)?;
/// # Ok(())
/// # }
/// ```
pub fn load_from_memory(buf: &[u8]) -> Result<Pyxel, PyxelError> {
    let cursor = Cursor::new(buf);
    load(cursor)
}

/// Open the Pyxel document located at the path specified.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), pyxel::PyxelError> {
/// let doc = pyxel::open("resources/doc.pyxel")?;
/// # Ok(())
/// # }
/// ```
pub fn open<P>(path: P) -> Result<Pyxel, PyxelError>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    load(file)
}
