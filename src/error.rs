//! Error types.

use std::num::{ParseFloatError, ParseIntError};
use std::fmt;

/// Error converting float currencies to currencies.
#[derive(Debug)]
pub enum TryFromFloatCurrenciesError {
    /// For currencies which contain fractional values.
    Fractional {
        /// Fractional key values are invalid.
        fract: f32,
    },
    /// For values which are out of bounds.
    OutOfBounds {
        /// The value that was out of bounds.
        value: f32,
    },
}

impl fmt::Display for TryFromFloatCurrenciesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TryFromFloatCurrenciesError::Fractional { fract } => {
                write!(f, "Currencies contains fractional value: {}", fract)
            }
            TryFromFloatCurrenciesError::OutOfBounds { value } => {
                write!(f, "Conversion of {} was out of integer bounds", value)
            }
        }
    }
}

/// An error occurred parsing a string into a currency.
#[derive(Debug)]
pub enum ParseError {
    /// String was invalid.
    NoCurrenciesDetected,
    /// A number was expected, but none was found.
    MissingCount,
    /// A currency name was expected, but none was found.
    MissingCurrencyName,
    /// An unexpected element was found.
    UnexpectedToken,
    /// An invalid currency name was found.
    InvalidCurrencyName,
    /// A string failed to parse to an integer.
    ParseInt(ParseIntError),
    /// A string failed to parse to a float.
    ParseFloat(ParseFloatError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoCurrenciesDetected => write!(f, "No currencies could be parsed from string"),
            ParseError::MissingCount => write!(f, "Expected a number, but none was found"),
            ParseError::MissingCurrencyName => write!(f, "Expected a currency name, but none was found"),
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::InvalidCurrencyName => write!(f, "Invalid currency name"),
            ParseError::ParseInt(e) => write!(f, "{}", e),
            ParseError::ParseFloat(e) => write!(f, "{}", e),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::ParseInt(e)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        ParseError::ParseFloat(e)
    }
}