//! Error types.

use std::num::{ParseFloatError, ParseIntError};

/// Error converting float currencies to currencies.
#[derive(Debug, thiserror::Error)]
pub enum TryFromFloatCurrenciesError {
    /// For currencies which contain fractional values.
    #[error("Currencies contains fractional value: {fract}")]
    Fractional {
        /// Fractional key values are invalid.
        fract: f32,
    },
    /// For when converting from a float metal value into a weapon metal value is out of bounds.
    #[error("Metal value is out of bounds for conversion into weapon value: {metal}")]
    MetalOutOfBounds {
        /// The amount of metal.
        metal: f32,
    },
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