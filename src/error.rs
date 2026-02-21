// SPDX-FileCopyrightText: 2023 CELESTIFYX Team
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io,
    error,
    path::PathBuf,

    fmt::{
        Display,
        Formatter
    }
};

#[derive(Debug)]
pub enum Error {
    JsonError(String),
    IoError(io::Error),
    ValidationError(String),
    ColorParseError(String),
    FileNotFound(PathBuf)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::ValidationError(e) => write!(f, "Validation error: {}", e),
            Error::ColorParseError(e) => write!(f, "Color parse error: {}", e),
            Error::FileNotFound(path) => write!(f, "File not found: {:?}", path)
        }
    }
}

impl error::Error for Error {
    // TODO: NOTHING
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err.to_string())
    }
}
