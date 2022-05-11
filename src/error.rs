use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub struct TryFromListingCurrenciesError {
    pub fract: f32,
}

impl std::error::Error for TryFromListingCurrenciesError {}

impl fmt::Display for TryFromListingCurrenciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Currencies contains fractional value: {}", self.fract)
    }
}

#[derive(Debug)]
pub enum ParseError {
    NoCurrencies,
    Invalid,
    ParseNumeric(String),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloat(error)
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NoCurrencies => write!(f, "No currencies could be parsed from string"),
            ParseError::Invalid => write!(f, "Invalid currencies string"),
            ParseError::ParseNumeric(e) => write!(f, "{}", e),
            ParseError::ParseInt(e) => write!(f, "{}", e),
            ParseError::ParseFloat(e) => write!(f, "{}", e),
        }
    }
}