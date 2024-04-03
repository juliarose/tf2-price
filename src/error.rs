//! Error types.

use std::num::{ParseFloatError, ParseIntError};

/// Error converting float currencies to currencies.
#[derive(Debug, thiserror::Error)]
pub enum TryFromFloatCurrenciesError {
    /// For currencies which contain fractional values.
    #[error("Currencies contains fractional value: {}", .fract)]
    Fractional {
        /// Fractional key values are invalid.
        fract: f32,
    },
}

/// An error occurred parsing a string into a currency.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// String was invalid.
    #[error("No currencies could be parsed from string")]
    NoCurrenciesDetected,
    /// A number was expected, but none was found.
    #[error("Expected a number, but none was found")]
    MissingCount,
    /// A currency name was expected, but none was found.
    #[error("Expected a currency name, but none was found")]
    MissingCurrencyName,
    /// An unexpected element was found.
    #[error("Unexpected token")]
    UnexpectedToken,
    /// An invalid currency name was found.
    #[error("Invalid currency name")]
    InvalidCurrencyName,
    /// A string failed to parse to an integer.
    #[error("{}", .0)]
    ParseInt(#[from] ParseIntError),
    /// A string failed to parse to a float.
    #[error("{}", .0)]
    ParseFloat(#[from] ParseFloatError),
}