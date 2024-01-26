//! Error types.

use std::num::{ParseFloatError, ParseIntError};

/// Error converting listing currencies to currencies.
#[derive(Debug, thiserror::Error)]
#[error("Currencies contains fractional value: {fract}")]
pub struct TryFromListingCurrenciesError {
    /// Fractional key values are invalid.
    pub fract: f32,
}

/// An error occurred parsing a string into a currency.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// String was invalid.
    #[error("No currencies could be parsed from string")]
    Invalid,
    /// A value expected to be number failed to parse. 
    #[error(r#"Failed to parse "{}" as numeric"#, .0)]
    ParseNumeric(String),
    /// A string failed to parse to an integer.
    #[error("{}", .0)]
    ParseInt(#[from] ParseIntError),
    /// A string failed to parse to a float.
    #[error("{}", .0)]
    ParseFloat(#[from] ParseFloatError),
}