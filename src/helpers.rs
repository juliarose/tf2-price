use std::{fmt, str::FromStr};
use crate::{
    error::ParseError,
    Rounding,
    constants::{
        KEYS_SYMBOL,
        KEY_SYMBOL,
        METAL_SYMBOL,
        ONE_REF,
    },
};
use serde::{Deserialize, Deserializer};

/// Determines if two integers have the same sign (both positive or both negative).
fn same_sign_i32(a: i32, b:i32) -> bool {
    (
        a >= 0 &&
        b >= 0
    ) ||
    (
        a < 0 &&
        b < 0
    )
}

/// Converts currencies to a metal value using the given key price (represented as weapons).
/// In cases where the result overflows or underflows beyond the limit for i32, the max or 
/// min i32 will be returned. In most cases values this high are not useful.
pub fn to_metal(metal: i32, keys: i32, key_price: i32) -> i32 {
    match keys.checked_mul(key_price) {
        // saturating_add will limit the addition to the lower or upper bounds
        Some(result) => metal.saturating_add(result),
        // two positives always equal a positive
        // and two negatives always equal a positive
        // return the maximum i32
        None if same_sign_i32(keys, key_price) => i32::MAX,
        // otherwise this number will be negative
        // return the minimum i32
        None => i32::MIN,
    }
}

pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>
{
    let float = f32::deserialize(deserializer)?;
    let metal = (float * (ONE_REF as f32)).round() as i32;
    
    Ok(metal)
}

pub mod cents {
    use serde::{Serializer, Deserialize, Deserializer};
    use super::cents_to_dollars;
    
    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_f32(cents_to_dollars(*value))
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>
    {
        let usd = f32::deserialize(deserializer)?;
        let cents = (usd * 100.0).round() as i32;
        
        Ok(cents)
    }
}

pub fn cents_to_dollars(cents: i32) -> f32 {
    (cents as f32) / 100.0
}

pub fn pluralize<'a>(amount: i32, singular: &'a str, plural: &'a str) -> &'a str {
    if amount == 1 {
        singular
    } else {
        plural
    }
}

pub fn pluralize_float<'a>(amount: f32, singular: &'a str, plural: &'a str) -> &'a str {
    if amount == 1.0 {
        singular
    } else {
        plural
    }
}

pub fn print_float(amount: f32) -> String {
    if amount % 1.0 == 0.0 {
        (amount.round() as i32).to_string()
    } else {
        format!("{:.2}", amount)
    }
}

/// Converts a metal value into its float value.
///
/// # Examples
///
/// ```
/// assert_eq!(tf2_price::get_metal_float(6), 0.33);
/// ```
pub fn get_metal_float(value: i32) -> f32 {
    f32::trunc((value as f32 / (ONE_REF as f32)) * 100.0) / 100.0
}

/// Converts a float value into a metal value.
///
/// # Examples
///
/// ```
/// assert_eq!(tf2_price::get_metal_from_float(0.33), 6);
/// ```
pub fn get_metal_from_float(value: f32) -> i32 {
    (value * (ONE_REF as f32)).round() as i32
}

pub fn parse_from_string<T>(string: &str) -> Result<(T, i32), ParseError>
where
    T: Default + FromStr + PartialEq,
    <T as FromStr>::Err: fmt::Display,
{
    let mut keys = T::default();
    let mut metal = 0;
    
    for element in string.split(", ") {
        let mut element_split = element.split(' ');
        let (
            count_str,
            currency_name,
        ) = (
            element_split.next(),
            element_split.next(),
        );
        
        if count_str.is_none() || currency_name.is_none() || element_split.next().is_some() {
            return Err(ParseError::Invalid);
        }
        
        let (
            count_str,
            currency_name,
        ) = (
            count_str.unwrap(),
            currency_name.unwrap(),
        );
        
        match currency_name {
            KEY_SYMBOL | KEYS_SYMBOL => {
                keys = count_str.parse::<T>()
                    .map_err(|e| ParseError::ParseNumeric(e.to_string()))?;
            },
            METAL_SYMBOL => {
                metal = get_metal_from_float(count_str.parse::<f32>()?);
            },
            _ => {
                return Err(ParseError::Invalid);
            },
        }
    }
    
    if keys == T::default() && metal == 0 {
        return Err(ParseError::NoCurrencies);
    }
    
    Ok((keys, metal))
}

pub fn round_metal(metal: i32, rounding: &Rounding) -> i32 {
    if metal == 0 {
        return metal;
    }
    
    match *rounding {
        // No rounding needed if the metal value is an even number.
        Rounding::UpScrap if metal % 2 != 0 => {
            metal + 1
        },
        // No rounding needed if the metal value is an even number.
        Rounding::DownScrap if metal % 2 != 0 => {
            metal - 1
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
        _ => {
            metal
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrap;
    
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
        assert_eq!(scrap!(3), get_metal_from_float(0.33));
    }
    
    #[test]
    fn converts_to_metal_float() {
        assert_eq!(0.33, get_metal_float(6));
    }
}