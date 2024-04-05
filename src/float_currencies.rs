use crate::helpers;
use crate::types::Currency;
use crate::error::ParseError;
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL};
use crate::Currencies;
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops::{self, AddAssign, SubAssign, MulAssign, DivAssign};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use serde::ser::SerializeStruct;

/// For storing floating point values of currencies. This is useful for retaining the original 
/// values from responses. Convert to [`Currencies`] to perform precise arithmetical operations or 
/// comparisons.
/// 
/// # Examples
/// ```
/// use tf2_price::{FloatCurrencies, Currencies, metal, refined};
/// 
/// let float_currencies = FloatCurrencies {
///     keys: 1.0,
///     metal: 1.33,
/// };
/// let mut currencies = Currencies::try_from(float_currencies).unwrap();
/// 
/// assert_eq!(currencies.keys, 1);
/// assert_eq!(currencies.weapons, metal!(1.33));
/// 
/// // For precision, arithmetical operations should be done with Currencies, not FloatCurrencies.
/// currencies.weapons += refined!(1);
/// 
/// assert_eq!(currencies.weapons, metal!(2.33));
/// ```
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(remote = "Self")]
pub struct FloatCurrencies {
    /// Amount of keys.
    #[serde(default)]
    pub keys: f32,
    /// Amount of metal expressed as a float e.g. "1.33 ref". Unlike [`Currencies`], this 
    /// **is not** represented as weapons. This is meant to retain the original values from 
    /// responses.
    #[serde(default)]
    pub metal: f32,
}

impl PartialOrd for FloatCurrencies {
    fn partial_cmp(&self, other: &FloatCurrencies) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for FloatCurrencies {
    fn cmp(&self, other:&Self) -> Ordering {
        if self.keys > other.keys {
            Ordering::Greater
        } else if self.keys < other.keys {
            Ordering::Less
        } else if self.metal > other.metal {
            Ordering::Greater
        } else if self.metal < other.metal {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Eq for FloatCurrencies {}

impl FloatCurrencies {
    /// Creates a new [`FloatCurrencies`] with `0` keys and `0` metal. Same as 
    /// `FloatCurrencies::default()`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::FloatCurrencies;
    /// 
    /// let currencies = FloatCurrencies::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Checks if the `keys` value is a fractional value.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::FloatCurrencies;
    /// 
    /// let currencies = FloatCurrencies {
    ///     keys: 1.5,
    ///     metal: 0.0,
    /// };
    /// 
    /// assert!(currencies.is_fract());
    /// ```
    pub fn is_fract(&self) -> bool {
        self.keys.fract() != 0.0
    }
    
    /// Converts currencies to a value in weapons using the given key price (represented as 
    /// weapons). Rounds float conversions.
    /// 
    /// This method is [saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic).
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{FloatCurrencies, refined};
    /// 
    /// let key_price_weapons = refined!(50);
    /// let currencies = FloatCurrencies {
    ///     keys: 1.0,
    ///     metal: 5.0,
    /// };
    /// 
    /// // 1.0 * 50 refined + 5 refined = 55 refined
    /// assert_eq!(currencies.to_weapons(key_price_weapons), refined!(55));
    /// ```
    pub fn to_weapons(
        &self,
        key_price_weapons: Currency,
    ) -> Currency {
        let keys_weapons = (self.keys * key_price_weapons as f32).round() as Currency;
        
        helpers::get_weapons_from_metal_float(self.metal).saturating_add(keys_weapons)
    }
    
    /// Converts currencies to a value in weapons using the given key price (represented as 
    /// weapons).
    /// 
    /// Checks for safe conversion.
    /// 
    /// In cases where the result overflows or underflows beyond the limit for 
    /// [`Currency`], `None` is returned. Currencies containing NaN or Infinity values will also
    /// return `None`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currency, FloatCurrencies, refined};
    /// 
    /// let key_price_weapons = refined!(50);
    /// 
    /// assert_eq!(
    ///     FloatCurrencies {
    ///         keys: Currency::MAX as f32 + 1.0,
    ///         metal: 0.0,
    ///     }.checked_to_weapons(key_price_weapons),
    ///     None,
    /// );
    /// ```
    pub fn checked_to_weapons(
        &self,
        key_price_weapons: Currency,
    ) -> Option<Currency> {
        let keys_weapons_float = (self.keys * key_price_weapons as f32).round();
        let keys_weapons = helpers::strict_f32_to_currency(keys_weapons_float)?;
        
        helpers::checked_get_weapons_from_metal_float(self.metal)?.checked_add(keys_weapons)
    }
    
    /// Checks if the currencies do not contain any value.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::FloatCurrencies;
    /// 
    /// let mut currencies = FloatCurrencies::default();
    /// assert!(currencies.is_empty());
    /// 
    /// // Keys now has a value other than 0.0.
    /// currencies.keys = 1.0;
    /// assert!(!currencies.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keys == 0.0 && self.metal == 0.0
    }
    
    /// Checks whether the currencies have enough keys and metal to afford the `other` currencies.
    /// This is simply `self.keys >= other.keys && self.metal >= other.metal`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::FloatCurrencies;
    /// 
    /// let currencies = FloatCurrencies {
    ///     keys: 100.0,
    ///     metal: 30.0,
    /// };
    /// 
    /// // We have at least 50 keys and 30 metal.
    /// assert!(currencies.can_afford(&FloatCurrencies {
    ///     keys: 50.0,
    ///     metal: 30.0,
    /// }));
    /// // Not enough metal - we can't afford this.
    /// assert!(!currencies.can_afford(&FloatCurrencies {
    ///     keys: 50.0,
    ///     metal: 100.0,
    /// }));
    /// ```
    pub fn can_afford(&self, other: &Self) -> bool {
        self.keys >= other.keys && self.metal >= other.metal
    }
}

impl PartialEq<Currencies> for FloatCurrencies {
    fn eq(&self, other: &Currencies) -> bool {
        self.keys.fract() == 0.0 &&
        self.keys == other.keys as f32 &&
        helpers::get_weapons_from_metal_float(self.metal) == other.weapons
    }
}

impl_op_ex!(+ |a: &FloatCurrencies, b: &FloatCurrencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys + b.keys,
        metal: a.metal + b.metal,
    } 
});

impl_op_ex!(+ |a: &FloatCurrencies, b: &Currencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys + b.keys as f32,
        // Convert the value in weapons to a value in refined before applying the operation.
        metal: a.metal + helpers::get_metal_float_from_weapons(b.weapons),
    } 
});

impl_op_ex!(+ |a: &Currencies, b: &FloatCurrencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys as f32 + b.keys,
        // Convert the value in weapons to a value in refined before applying the operation.
        metal: helpers::get_metal_float_from_weapons(a.weapons) + b.metal,
    } 
});

impl_op_ex!(- |a: &FloatCurrencies, b: &FloatCurrencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys - b.keys,
        metal: a.metal - b.metal,
    }
});

impl_op_ex!(- |a: &FloatCurrencies, b: &Currencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys - b.keys as f32,
        // Convert the value in weapons to a value in refined before applying the operation.
        metal: a.metal - helpers::get_metal_float_from_weapons(b.weapons),
    } 
});

impl_op_ex!(- |a: &Currencies, b: &FloatCurrencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys as f32 - b.keys,
        // Convert the value in weapons to a value in refined before applying the operation.
        metal: helpers::get_metal_float_from_weapons(a.weapons) - b.metal,
    } 
});

impl_op_ex!(* |currencies: &FloatCurrencies, num: Currency| -> FloatCurrencies {
    FloatCurrencies {
        keys: currencies.keys * num as f32,
        metal: currencies.metal * num as f32,
    }
});

impl_op_ex!(/ |currencies: &FloatCurrencies, num: Currency| -> FloatCurrencies {
    FloatCurrencies {
        keys: currencies.keys / num as f32,
        metal: currencies.metal / num as f32,
    }
});

impl_op_ex!(* |currencies: &FloatCurrencies, num: f32| -> FloatCurrencies {
    FloatCurrencies { 
        keys: currencies.keys * num,
        metal: currencies.metal * num,
    }
});

impl_op_ex!(/ |currencies: &FloatCurrencies, num: f32| -> FloatCurrencies {
    FloatCurrencies {
        keys: currencies.keys / num,
        metal: currencies.metal / num,
    }
});

impl AddAssign<FloatCurrencies> for FloatCurrencies {
    fn add_assign(&mut self, other: Self) {
        self.keys += other.keys;
        self.metal += other.metal;
    }
}

impl AddAssign<&FloatCurrencies> for FloatCurrencies {
    fn add_assign(&mut self, other: &Self) {
        self.keys += other.keys;
        self.metal += other.metal;
    }
}

impl SubAssign<FloatCurrencies> for FloatCurrencies {
    fn sub_assign(&mut self, other: Self) {
        self.keys -= other.keys;
        self.metal -= other.metal;
    }
}

impl SubAssign<&FloatCurrencies> for FloatCurrencies {
    fn sub_assign(&mut self, other: &Self) {
        self.keys -= other.keys;
        self.metal-= other.metal;
    }
}

// Operations for non-float currencies

impl AddAssign<Currencies> for FloatCurrencies {
    fn add_assign(&mut self, other: Currencies) {
        self.keys += other.keys as f32;
        // The float value is a value in weapons, but we need to convert it to a float value in
        // refined before applying the operation.
        self.metal += helpers::get_metal_float_from_weapons(other.weapons);
    }
}

impl AddAssign<&Currencies> for FloatCurrencies {
    fn add_assign(&mut self, other: &Currencies) {
        self.keys += other.keys as f32;
        // The float value is a value in weapons, but we need to convert it to a float value in
        // refined before applying the operation.
        self.metal += helpers::get_metal_float_from_weapons(other.weapons);
    }
}

impl SubAssign<Currencies> for FloatCurrencies {
    fn sub_assign(&mut self, other: Currencies) {
        self.keys -= other.keys as f32;
        // The float value is a value in weapons, but we need to convert it to a float value in
        // refined before applying the operation.
        self.metal -= helpers::get_metal_float_from_weapons(other.weapons);
    }
}

impl SubAssign<&Currencies> for FloatCurrencies {
    fn sub_assign(&mut self, other: &Currencies) {
        self.keys -= other.keys as f32;
        // The float value is a value in weapons, but we need to convert it to a float value in
        // refined before applying the operation.
        self.metal -= helpers::get_metal_float_from_weapons(other.weapons);
    }
}

impl MulAssign<Currency> for FloatCurrencies {
    fn mul_assign(&mut self, other: Currency) {
        self.keys *= other as f32;
        self.metal *= other as f32;
    }
}

impl MulAssign<f32> for FloatCurrencies {
    fn mul_assign(&mut self, other: f32) {
        self.keys *= other;
        self.metal *= other;
    }
}

impl DivAssign<Currency> for FloatCurrencies {
    fn div_assign(&mut self, other: Currency) {
        self.keys /= other as f32;
        self.metal /= other as f32;
    }
}

impl DivAssign<f32> for FloatCurrencies {
    fn div_assign(&mut self, other: f32) {
        self.keys /= other;
        self.metal /= other;
    }
}

impl TryFrom<&str> for FloatCurrencies {
    type Error = ParseError;
    
    fn try_from(string: &str) -> Result<Self, Self::Error>  {
        string.parse::<Self>()
    }
}

impl TryFrom<&String> for FloatCurrencies {
    type Error = ParseError;
    
    fn try_from(string: &String) -> Result<Self, Self::Error> {
        string.parse::<Self>()
    }
}

impl TryFrom<String> for FloatCurrencies {
    type Error = ParseError;
    
    fn try_from(string: String) -> Result<Self, Self::Error> {
        string.parse::<Self>()
    }
}

impl std::str::FromStr for FloatCurrencies {
    type Err = ParseError;
    
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (keys, metal) = helpers::parse_float_from_string(string)?;
        
        Ok(Self {
            keys,
            metal,
        })
    }
}

impl From<Currencies> for FloatCurrencies {
    fn from(currencies: Currencies) -> Self {
        Self {
            keys: currencies.keys as f32,
            metal: helpers::get_metal_float_from_weapons(currencies.weapons),
        }
    }
}

impl From<&Currencies> for FloatCurrencies {
    fn from(currencies: &Currencies) -> Self {
        Self {
            keys: currencies.keys as f32,
            metal: helpers::get_metal_float_from_weapons(currencies.weapons),
        }
    }
}

impl fmt::Display for FloatCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Either both keys and metal are non-zero or both are zero.
        if (self.keys != 0.0 && self.metal != 0.0) || self.is_empty() {
            write!(
                f,
                "{} {}, {} {}",
                helpers::print_float(self.keys),
                helpers::pluralize_float(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
                helpers::print_float(self.metal),
                METAL_SYMBOL,
            )
        } else if self.keys != 0.0 {
            write!(
                f,
                "{} {}",
                helpers::print_float(self.keys),
                helpers::pluralize_float(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
            )
        } else {
            // It can be assumed that metal is not zero.
            write!(
                f,
                "{} {}",
                helpers::print_float(self.metal),
                METAL_SYMBOL,
            )
        }
    }
}

impl<'de> Deserialize<'de> for FloatCurrencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let currencies = Self::deserialize(deserializer)?;
        
        if currencies.keys.is_nan() {
            return Err(D::Error::custom("Keys is NaN"));
        }
        
        if currencies.metal.is_nan() {
            return Err(D::Error::custom("Metal is NaN"));
        }
        
        if currencies.keys == 0.0 && currencies.metal == 0.0 {
            return Err(D::Error::custom("Does not contain values for keys or metal"));
        }
        
        Ok(currencies)
    }
}

impl Serialize for FloatCurrencies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut currencies = serializer.serialize_struct("FloatCurrencies", 2)?;
        
        if self.keys == 0.0 {
            currencies.skip_field("keys")?;
        } else if self.keys.fract() == 0.0 {
            currencies.serialize_field("keys", &(self.keys as Currency))?;
        } else {
            currencies.serialize_field("keys", &self.keys)?;
        }
        
        if self.metal == 0.0 {
            currencies.skip_field("metal")?;
        } else if self.metal.fract() == 0.0 {
            currencies.serialize_field("metal", &(self.metal as Currency))?;
        } else {
            currencies.serialize_field("metal", &((self.metal * 100.0) / 100.0))?;
        }
        
        currencies.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{refined, scrap};
    use assert_json_diff::assert_json_eq;
    use serde_json::{self, json, Value};
    
    #[test]
    fn to_weapons_correct() {
        let key_price = 10;
        
        assert_eq!(
            FloatCurrencies {
                keys: 0.19,
                metal: 0.0,
            }.to_weapons(key_price),
            2,
        );
    }

    #[test]
    fn currencies_equal() {
        assert_eq!(
            FloatCurrencies {
                keys: 2.0,
                metal: 23.44,
            },
            FloatCurrencies {
                keys: 2.0,
                metal: 23.44,
            },
        );
    }
    
    #[test]
    fn currencies_not_equal() {
        assert_ne!(
            FloatCurrencies {
                keys: 2.0,
                metal: 23.44,
            },
            FloatCurrencies {
                keys: 2.0,
                metal: 23.0,
            },
        );
    }
    
    #[test]
    fn currencies_added() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 10.0,
            } + FloatCurrencies {
                keys: 5.0,
                metal: 5.0,
            },
            FloatCurrencies {
                keys: 15.0,
                metal: 15.0,
            },
        );
    }
    
    #[test]
    fn currencies_subtracted() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 10.0,
            } - FloatCurrencies {
                keys: 5.0,
                metal: 5.0,
            },
            FloatCurrencies {
                keys: 5.0,
                metal: 5.0,
            },
        );
    }
    
    #[test]
    fn currencies_multiplied_by_metal() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 10.0,
            } * 5,
            FloatCurrencies {
                keys: 50.0,
                metal: 50.0,
            },
        );
    }
    
    #[test]
    fn currencies_divided_by_metal() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 10.0,
            } / 5,
            FloatCurrencies {
                keys: 2.0,
                metal: 2.0,
            },
        );
    }
    
    #[test]
    fn currencies_multiplied_by_f32() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 10.0,
            } * 2.5,
            FloatCurrencies {
                keys: 25.0,
                metal: 25.0,
            },
        );
    }
    
    #[test]
    fn currencies_divided_by_f32() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 10.0,
            } / 2.5,
            FloatCurrencies {
                keys: 4.0,
                metal: 4.0,
            },
        );
    }
    
    #[test]
    fn currencies_partial_eq() {
        assert_eq!(
            FloatCurrencies {
                keys: 1.0,
                metal: 1.33,
            },
            Currencies {
                keys: 1,
                weapons: refined!(1) + scrap!(3),
            },
        );
    }
    
    #[test]
    fn converts_into_currencies() {
        let currencies: Currencies = FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        }.try_into().unwrap();
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 10,
                weapons: refined!(10),
            },
        );
    }
    
    #[test]
    fn subtracts_non_float_currencies() {
        assert_eq!(
            FloatCurrencies {
                keys: 1.5,
                metal: 0.0,
            } - Currencies {
                keys: 1,
                weapons: 0,
            },
            FloatCurrencies {
                keys: 0.5,
                metal: 0.0,
            },
        );
    }
    
    #[test]
    fn adds_non_float_currencies() {
        assert_eq!(
            FloatCurrencies {
                keys: 1.5,
                metal: 0.0,
            } + Currencies {
                keys: 1,
                weapons: 0,
            },
            FloatCurrencies {
                keys: 2.5,
                metal: 0.0,
            },
        );
    }
    
    #[test]
    fn converts_into_currencies_with_key_price() {
        let currencies = Currencies::from_float_currencies_with(FloatCurrencies {
            keys: 2.5,
            metal: 10.0,
        }, refined!(10));
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 2,
                weapons: refined!(15),
            },
        );
    }
    
    #[test]
    fn converts_into_currencies_with_key_price_negative_values() {
        let currencies = Currencies::from_float_currencies_with(FloatCurrencies {
            keys: 2.5,
            metal: -10.0,
        }, refined!(10));
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 2,
                weapons: refined!(-5),
            },
        );
    }
    
    #[test]
    fn fails_to_convert_into_currencies_when_fractional() {
        assert!(Currencies::try_from(FloatCurrencies {
            keys: 10.5,
            metal: 10.0,
        }).is_err());
    }
    
    #[test]
    fn formats_currencies() {
        let currencies = FloatCurrencies {
            keys: 2.0,
            metal: 23.0,
        };
        
        assert_eq!(format!("{currencies}"), "2 keys, 23 ref");
    }
    
    #[test]
    fn formats_currencies_with_decimal_places() {
        let currencies = FloatCurrencies {
            keys: 2.2555,
            metal: 23.0,
        };
        
        assert_eq!(format!("{currencies}"), "2.26 keys, 23 ref");
    }
    
    #[test]
    fn prints_empty_currencies() {
        assert_eq!(FloatCurrencies::default().to_string(), "0 keys, 0 ref");
    }
    
    #[test]
    fn deserializes_currencies() {
        let currencies: FloatCurrencies = serde_json::from_str(
            r#"{"keys":1,"metal": 23.44}"#
        ).unwrap();
        
        assert_eq!(
            currencies,
            FloatCurrencies {
                keys: 1.0,
                metal: 23.44,
            },
        );
    }
    
    #[test]
    fn deserializes_currencies_with_no_keys() {
        let currencies: FloatCurrencies = serde_json::from_str(
            r#"{"metal": 23.44}"#
        ).unwrap();
        
        assert_eq!(
            currencies,
            FloatCurrencies {
                keys: 0.0,
                metal: 23.44,
            },
        );
    }
    
    #[test]
    fn serializes_currencies() {
        let currencies = FloatCurrencies {
            keys: 1.0,
            metal: 23.44,
        };
        let currencies_json = serde_json::to_string(&currencies).unwrap();
        let actual: Value = serde_json::from_str(&currencies_json).unwrap();
        let expected: Value = json!({
            "keys": 1,
            "metal": 23.44
        });
        
        assert_json_eq!(actual, expected);
    }
    
    #[test]
    fn serializes_currencies_with_float() {
        let currencies = FloatCurrencies {
            keys: 1.5,
            metal: 23.44,
        };
        let currencies_json = serde_json::to_string(&currencies).unwrap();
        let actual: Value = serde_json::from_str(&currencies_json).unwrap();
        let expected: Value = json!({
            "keys": 1.5,
            "metal": 23.44
        });
        
        assert_json_eq!(actual, expected);
    }
    
    #[test]
    fn greater_than() {
        let a = FloatCurrencies { keys: 1.0, metal: 5.0 };
        let b = FloatCurrencies { keys: 0.0, metal: 10.0 };
        
        assert!(a > b);
    }
    
    #[test]
    fn less_than() {
        let a = FloatCurrencies { keys: 0.0, metal: 1.0 };
        let b = FloatCurrencies { keys: 0.0, metal: 4.0 };
        
        assert!(a < b);
    }
    
    #[test]
    fn sorts() {
        let mut currencies = vec![
            FloatCurrencies { keys: 2.0, metal: 4.0 },
            FloatCurrencies { keys: 0.0, metal: 2.0 },
            FloatCurrencies { keys: 10.0, metal: 4.0 },
        ];
        
        // lowest to highest
        currencies.sort();
        
        assert_eq!(
            *currencies.iter().rev().next().unwrap(),
            FloatCurrencies {
                keys: 10.0,
                metal: 4.0,
            },
        );
    }
    
    #[test]
    fn checked_to_metal() {
        assert_eq!(
            FloatCurrencies {
                keys: Currency::MAX as f32,
                metal: 4.0,
            }.checked_to_weapons(Currency::MAX),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal_correct_value() {
        assert_eq!(
            FloatCurrencies {
                keys: 10.0,
                metal: 5.0,
            }.checked_to_weapons(10),
            Some(100 + refined!(5)),
        );
    }
}