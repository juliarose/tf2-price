//! Error types.

use std::num::{ParseFloatError, ParseIntError};
use std::fmt;

/// Error converting float currencies to currencies.
#[derive(Debug)]
pub enum TryFromFloatCurrenciesError {
    /// The `keys` part of the currencies contained a fractional value.
    Fractional {
        /// Fractional key values are invalid.
        fract: f32,
    },
    /// A value overflowed or underflowed the integer bounds.
    OutOfBounds {
        /// The value that was out of bounds.
        value: f32,
    },
}

impl std::error::Error for TryFromFloatCurrenciesError {}

impl fmt::Display for TryFromFloatCurrenciesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TryFromFloatCurrenciesError::Fractional { fract } => {
                write!(f, "Currencies contains fractional value: {fract}")
            }
            TryFromFloatCurrenciesError::OutOfBounds { value } => {
                write!(f, "Conversion of {value} was out of integer bounds")
            }
        }
    }
}

/// An error occurred parsing a currency from a string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// String was invalid.
    NoCurrenciesDetected,
    /// A number was expected, but none was found.
    MissingCount,
    /// A currency name was expected, but none was found.
    MissingCurrencyName,
    /// An unexpected element was found.
    UnexpectedToken {
        /// The span of the unexpected token.
        span: std::ops::Range<usize>,
    },
    /// An invalid currency name was found.
    InvalidCurrencyName {
        /// The span of the invalid currency name.
        span: std::ops::Range<usize>,
    },
    /// A string failed to parse to an integer.
    ParseInt(ParseIntError),
    /// A string failed to parse to a float.
    ParseFloat(ParseFloatError),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoCurrenciesDetected => write!(f, "No currencies could be parsed from string"),
            ParseError::MissingCount => write!(f, "Expected a number, but none was found"),
            ParseError::MissingCurrencyName => write!(f, "Expected a currency name, but none was found"),
            ParseError::UnexpectedToken {
                span
            } => write!(f, "Unexpected token at {}", span.start),
            ParseError::InvalidCurrencyName {
                span
            } => write!(f, "Invalid currency name at index {}", span.start),
            ParseError::ParseInt(e) => write!(f, "{e}"),
            ParseError::ParseFloat(e) => write!(f, "{e}"),
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
