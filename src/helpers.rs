use crate::error::ParseError;
use crate::types::Currency;
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL, ONE_REF_FLOAT};

/// Converts currencies to a metal value using the given key price (represented as weapons). This
/// method is saturating.
#[inline]
pub fn to_metal(
    metal: Currency,
    keys: Currency,
    key_price_weapons: Currency,
) -> Currency {
    keys.saturating_mul(key_price_weapons).saturating_add(metal)
}

/// Converts currencies to a metal value using the given key price (represented as weapons).
/// In cases where the result overflows or underflows beyond the limit for [`Currency`], `None`
/// is returned.
#[inline]
pub fn checked_to_metal(
    metal: Currency,
    keys: Currency,
    key_price_weapons: Currency,
) -> Option<Currency> {
    metal.checked_add(keys.checked_mul(key_price_weapons)?)
}

/// Converts a value in weapons into its float value.
///
/// # Examples
/// ```
/// assert_eq!(tf2_price::get_metal_float_from_weapons(6), 0.33);
/// ```
#[inline]
pub fn get_metal_float_from_weapons(weapons: Currency) -> f32 {
    f32::trunc((weapons as f32 / ONE_REF_FLOAT) * 100.0) / 100.0
}

/// Converts a float value into a metal value (represented as weapons).
///
/// # Examples
/// ```
/// assert_eq!(tf2_price::get_weapons_from_metal_float(0.33), 6);
/// ```
#[inline]
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
#[inline]
pub fn checked_get_weapons_from_metal_float(value: f32) -> Option<Currency> {
    strict_f32_to_currency((value * ONE_REF_FLOAT).round())
}

/// Converts an `f32` into a `Currency` safely.
#[inline]
pub fn strict_f32_to_currency(value: f32) -> Option<Currency> {
    let as_currency = value as Currency;
    
    if 
        // We don't want to allow NaN or infinite values.
        value.is_finite() &&
        value == as_currency as f32 &&
        // Check if the value is out of bounds of a Currency.
        value >= Currency::MIN as f32 &&
        value <= Currency::MAX as f32
    {
        return Some(as_currency);
    }
    
    None
}

/// Parses currencies from a string.
fn parse_currencies(
    string: &str,
) -> Result<(Option<&str>, Option<&str>), ParseError> {
    let mut keys = None;
    let mut metal = None;
    
    for element in string.split(',') {
        let mut element_split = element.split_whitespace();
        let count_str = element_split.next().ok_or(ParseError::MissingCount)?;
        let currency_name = element_split.next().ok_or(ParseError::MissingCurrencyName)?;
        
        // We don't expect another element after the currency name.
        if let Some(token) = element_split.next() {
            let pos = string.find(token).unwrap_or_default();
            
            return Err(ParseError::UnexpectedToken {
                span: pos..(pos + token.len()),
            });
        }
        
        if currency_name.eq_ignore_ascii_case(METAL_SYMBOL) {
            metal = Some(count_str);
        } else if currency_name.eq_ignore_ascii_case(KEYS_SYMBOL) || currency_name.eq_ignore_ascii_case(KEY_SYMBOL) {
            keys = Some(count_str);
        } else {
            let pos = string.find(currency_name).unwrap_or_default();
            
            return Err(ParseError::InvalidCurrencyName {
                span: pos..(pos + currency_name.len()),
            });
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
        .map(str::parse::<Currency>)
        .transpose()?
        .unwrap_or_default();
    let metal = metal
        .map(str::parse::<f32>)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ref_to_weps;
    
    #[test]
    fn converts_strict_f32_to_currency() {
        assert!(strict_f32_to_currency(Currency::MAX as f32).is_some());
    }
    
    #[test]
    fn converts_from_metal_float() {
        assert_eq!(ref_to_weps!(0.33), get_weapons_from_metal_float(0.33));
    }
    
    #[test]
    fn converts_to_metal_float() {
        assert_eq!(0.33, get_metal_float_from_weapons(6));
    }
}
