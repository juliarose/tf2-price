use crate::error::ParseError;
use crate::types::Currency;
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL, ONE_REF, ONE_REF_FLOAT};
use crate::Rounding;
use serde::{Deserialize, Deserializer};

/// Converts currencies to a metal value using the given key price (represented as weapons). This
/// method is saturating.
pub fn to_metal(
    metal: Currency,
    keys: Currency,
    key_price: Currency,
) -> Currency {
    keys.saturating_mul(key_price).saturating_add(metal)
}

/// Converts currencies to a metal value using the given key price (represented as weapons).
/// In cases where the result overflows or underflows beyond the limit for [`Currency`], `None` 
/// is returned.
pub fn checked_to_metal(
    metal: Currency,
    keys: Currency,
    key_price: Currency,
) -> Option<Currency> {
    metal.checked_add(keys.checked_mul(key_price)?)
}

/// Deserializes float weapon values as weapons.
pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: Deserializer<'de>
{
    // get the metal value as a float e.g. 2.55 ref
    let metal_refined_float = f32::deserialize(deserializer)?;
    // will fit it into the nearest weapon value
    let metal = (metal_refined_float * ONE_REF_FLOAT).round() as Currency;
    
    Ok(metal)
}

/// Serialzies and deserializes cents.
pub mod cents {
    use serde::{Serializer, Deserialize, Deserializer};
    use crate::types::Currency;
    use super::cents_to_dollars;
    
    pub fn serialize<S>(value: &Currency, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_f32(cents_to_dollars(*value))
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Currency, D::Error>
    where
        D: Deserializer<'de>
    {
        let usd = f32::deserialize(deserializer)?;
        let cents = (usd * 100.0).round() as Currency;
        
        Ok(cents)
    }
}

/// Converts cents to dollars.
pub fn cents_to_dollars(cents: Currency) -> f32 {
    (cents as f32) / 100.0
}

/// Pluralizes a value using an integer as the test.
pub fn pluralize<'a>(amount: Currency, singular: &'a str, plural: &'a str) -> &'a str {
    if amount == 1 {
        singular
    } else {
        plural
    }
}

/// Pluralizes a value using a float as the test.
pub fn pluralize_float<'a>(amount: f32, singular: &'a str, plural: &'a str) -> &'a str {
    if amount == 1.0 {
        singular
    } else {
        plural
    }
}

/// Prints a float as either an integer if it contains no fractional values or with 2 decimal 
/// places if it does.
pub fn print_float(amount: f32) -> String {
    if amount.fract() == 0.0 {
        (amount.round() as Currency).to_string()
    } else {
        format!("{:.2}", amount)
    }
}

/// Adds thousands places to a number string e.g. "1000" becomes "1,000".
pub fn thousands(string: String) -> String {
    // from https://crates.io/crates/separator
    let index = match string.find('.') {
        Some(i) => i,
        None => string.len()
    };
    let int_part = &string[..index];
    let fract_part = &string[index..];
    let mut output = String::new();
    let magnitude = if let Some(stripped) = int_part.strip_prefix('-') {
        output.push('-');
        stripped.to_owned()
    } else {
        int_part.to_owned()
    };
    let mut place = magnitude.len();
    let mut later_loop = false;
    
    for ch in magnitude.chars() {
        if later_loop && place % 3 == 0 {
            output.push(',');
        }
        
        output.push(ch);
        later_loop = true;
        place -= 1;
    };
    
    output.push_str(fract_part);
    output
}

/// Converts a value in weapons into its float value.
///
/// # Examples
/// ```
/// assert_eq!(tf2_price::get_metal_float_from_weapons(6), 0.33);
/// ```
pub fn get_metal_float_from_weapons(value: Currency) -> f32 {
    f32::trunc((value as f32 / ONE_REF_FLOAT) * 100.0) / 100.0
}

/// Converts a float value into a metal value (represented as weapons).
///
/// # Examples
/// ```
/// assert_eq!(tf2_price::get_weapons_from_metal_float(0.33), 6);
/// ```
pub fn get_weapons_from_metal_float(value: f32) -> Currency {
    (value * ONE_REF_FLOAT).round() as Currency
}

/// Converts a float value into a metal value.
/// 
/// Checks for safe conversion.
///
/// # Examples
/// ```
/// assert_eq!(tf2_price::checked_get_weapons_from_metal_float(0.33), Some(6));
/// ```
pub fn checked_get_weapons_from_metal_float(value: f32) -> Option<Currency> {
    let metal = (value * ONE_REF_FLOAT).round();
    
    strict_f32_to_currency(metal)
}

/// Converts an `f32` into a `Currency` safely.
pub fn strict_f32_to_currency(value: f32) -> Option<Currency> {
    // We don't want to allow NaN or infinite values.
    if value.is_nan() || value.is_infinite() {
        return None
    }
    
    // https://stackoverflow.com/a/71431182
    // Check if fractional component is 0 and that it can map to an integer
    // Using fract() is equivalent to using `as Currency as f32` and checking it matches
    if value.fract() != 0.0 {
        return None;
    }
    
    // Check if the value is out of bounds of a Currency.
    if value < Currency::MIN as f32 || value > Currency::MAX as f32 {
        return None;
    }
    
    Some(value.trunc() as Currency)
}

/// Parses currencies from a string.
fn parse_currencies(
    string: &str,
) -> Result<(Option<&str>, Option<&str>), ParseError> {
    let mut keys = None;
    let mut metal = None;
    
    for element in string.split(',') {
        let mut element_split = element.trim().split(' ');
        let count_str = element_split.next().ok_or(ParseError::MissingCount)?;
        let currency_name = element_split.next().ok_or(ParseError::MissingCurrencyName)?;
        
        // We don't expect another element after the currency name.
        if element_split.next().is_some() {
            return Err(ParseError::UnexpectedToken);
        }
        
        if currency_name.eq_ignore_ascii_case(METAL_SYMBOL) {
            metal = Some(count_str);
        } else if currency_name.eq_ignore_ascii_case(KEYS_SYMBOL) || currency_name.eq_ignore_ascii_case(KEY_SYMBOL) {
            keys = Some(count_str);
        } else {
            return Err(ParseError::InvalidCurrencyName);
        }
    }
    
    if keys.is_none() && metal.is_none() {
        return Err(ParseError::NoCurrenciesDetected);
    }
    
    Ok((keys, metal))
}

/// Parses currencies from a string.
pub fn parse_currency_from_string(
    string: &str,
) -> Result<(Currency, Currency), ParseError> {
    let (keys, metal) = parse_currencies(string)?;
    let keys = keys
        .map(|s| s.parse::<Currency>())
        .transpose()?
        .unwrap_or_default();
    let metal = metal
        .map(|s| s.parse::<f32>())
        .transpose()?
        // Convert the metal value to a weapon value.
        .map(get_weapons_from_metal_float)
        .unwrap_or_default();
    
    Ok((keys, metal))
}

/// Parses currencies from a string.
pub fn parse_float_from_string(
    string: &str,
) -> Result<(f32, f32), ParseError> {
    let (keys, metal) = parse_currencies(string)?;
    let keys = keys
        .map(|s| s.parse::<f32>())
        .transpose()?
        .unwrap_or_default();
    let metal = metal
        .map(|s| s.parse::<f32>())
        .transpose()?
        .unwrap_or_default();
    
    Ok((keys, metal))
}

/// Rounds a metal value.
pub fn round_metal(metal: Currency, rounding: &Rounding) -> Currency {
    if metal == 0 {
        return metal;
    }
    
    match *rounding {
        Rounding::UpScrap => if metal % 2 != 0{
            metal + 1
        } else {
            // No rounding needed if the metal value is an even number.
            metal
        },
        Rounding::DownScrap => if metal % 2 != 0 {
            metal - 1
        } else {
            // No rounding needed if the metal value is an even number.
            metal
        },
        Rounding::Refined => {
            let value = metal + ONE_REF / 2;
            
            value - (value % ONE_REF)
        },
        Rounding::UpRefined => {
            let remainder = metal % ONE_REF;
            
            if remainder != 0 {
                if metal > 0 {
                    metal - (remainder + -ONE_REF)
                } else {
                    metal - remainder
                }
            } else {
                metal
            }
        },
        Rounding::DownRefined => {
            let remainder = metal % ONE_REF;
            
            if remainder != 0 {
                if metal > 0 {
                    metal - remainder
                } else {
                    metal - (remainder + ONE_REF)
                }
            } else {
                metal
            }
        },
        Rounding::None => {
            metal
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrap;
    
    #[test]
    fn converts_strict_f32_to_currency() {
        assert!(strict_f32_to_currency(Currency::MAX as f32).is_some());
    }
    
    #[test]
    fn prints_float_rounded_whole_number() {
        assert_eq!("1", print_float(1.0));
    }
    
    #[test]
    fn prints_float_proper_decimal_places() {
        assert_eq!("1.56", print_float(1.55555));
    }
    
    #[test]
    fn converts_from_metal_float() {
        assert_eq!(scrap!(3), get_weapons_from_metal_float(0.33));
    }
    
    #[test]
    fn converts_to_metal_float() {
        assert_eq!(0.33, get_metal_float_from_weapons(6));
    }
}