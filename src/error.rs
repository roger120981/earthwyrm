// error.rs
//
// Copyright (c) 2019-2020  Minnesota Department of Transportation
//
use std::net::AddrParseError;
use std::num::TryFromIntError;
use std::{fmt, io};

/// Earthwyrm error types
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid network address error
    InvalidAddress(AddrParseError),
    /// I/O error
    Io(io::Error),
    /// MVT error
    Mvt(mvt::Error),
    /// PostgreSQL error
    Pg(postgres::Error),
    /// R2D2 connection pool error
    R2D2(r2d2::Error),
    /// Tile empty
    TileEmpty(),
    /// TOML deserializing error
    Toml(toml::de::Error),
    /// TryFrom conversion error
    TryFromInt(TryFromIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidAddress(e) => e.fmt(f),
            Error::Io(e) => e.fmt(f),
            Error::Mvt(e) => e.fmt(f),
            Error::Pg(e) => e.fmt(f),
            Error::R2D2(e) => e.fmt(f),
            Error::TileEmpty() => write!(f, "Tile empty"),
            Error::Toml(e) => e.fmt(f),
            Error::TryFromInt(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidAddress(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Mvt(e) => Some(e),
            Error::Pg(e) => Some(e),
            Error::R2D2(e) => Some(e),
            Error::TileEmpty() => None,
            Error::Toml(e) => Some(e),
            Error::TryFromInt(e) => Some(e),
        }
    }
}

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Error::InvalidAddress(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<mvt::Error> for Error {
    fn from(e: mvt::Error) -> Self {
        Error::Mvt(e)
    }
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Self {
        Error::Pg(e)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::R2D2(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Toml(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}
