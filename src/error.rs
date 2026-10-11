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
    Json5Error(String),
    IoError(io::Error),
    ValidationError(String),
    ColorParseError(String),
    FileNotFound(PathBuf)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Json5Error(e) => write!(f, "JSON5 parsing error: {}", e),
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

impl From<json5::Error> for Error {
    fn from(err: json5::Error) -> Self {
        Error::Json5Error(err.to_string())
    }
}
