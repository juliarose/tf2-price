use crate::helpers;
use crate::types::Currency;
use crate::traits::SerializeCurrencies;
use crate::error::ParseError;
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL, EMPTY_SYMBOL};
use crate::Currencies;
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops::{self, AddAssign, SubAssign, MulAssign, DivAssign};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use serde::ser::SerializeStruct;

/// The `keys` and `metal` fields for [`FloatCurrencies`] are defined as an [`f32`]. Use this 
/// anywhere you may need values which include decimal places.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(remote = "Self")]
pub struct FloatCurrencies {
    /// Amount of keys.
    #[serde(default)]
    pub keys: f32,
    /// Amount of metal expressed as a float e.g. "1.33 ref".
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

impl SerializeCurrencies for FloatCurrencies {}

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
    
    /// Converts currencies to a metal value using the given key price (represented as weapons).
    /// Rounds float conversions and saturates at integer bounds.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{FloatCurrencies, refined};
    /// 
    /// let key_price = refined!(50);
    /// let currencies = FloatCurrencies {
    ///     keys: 1.0,
    ///     metal: 5.0,
    /// };
    /// 
    /// // 1.0 * 50 refined + 5 refined = 55 refined
    /// assert_eq!(currencies.to_metal(key_price), refined!(55));
    /// ```
    pub fn to_metal(&self, key_price: Currency) -> Currency {
        helpers::get_metal_from_float(self.metal).saturating_add((self.keys * key_price as f32).round() as Currency)
    }
    
    /// Converts currencies to a metal value using the given key price (represented as weapons).
    /// In cases where the result overflows or underflows beyond the limit for [`Currency`], `None` 
    /// is returned.
    pub fn checked_to_metal(&self, key_price: Currency) -> Option<Currency> {
        let result = (self.keys * key_price as f32).round();
        let result_metal = helpers::strict_f32_to_currency(result)?;
        
        // Check for overflow by seeing if conversions produce unequal results
        if result != result_metal as f32 {
            return None;
        }
        
        helpers::checked_get_metal_from_float(self.metal)?.checked_add(result_metal)
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
        self.keys == 0.0 && !self.keys.is_nan() &&
        self.metal == 0.0 && !self.metal.is_nan()
    }
    
    /// Checks whether the currencies have enough keys and metal to afford the `other` currencies.
    /// This is simply `self.keys >= other.keys && self.metal >= other.metal`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::Currencies;
    /// 
    /// let currencies = Currencies { keys: 100, metal: 30 };
    /// 
    /// // We have at least 50 keys and 30 metal.
    /// assert!(currencies.can_afford(&Currencies { keys: 50, metal: 30 }));
    /// // Not enough metal - we can't afford this.
    /// assert!(!currencies.can_afford(&Currencies { keys: 50, metal: 100 }));
    /// ```
    pub fn can_afford(&self, other: &Self) -> bool {
        self.keys >= other.keys && self.metal >= other.metal
    }
}

impl PartialEq<Currencies> for FloatCurrencies {
    fn eq(&self, other: &Currencies) -> bool {
        self.keys.fract() == 0.0 &&
        self.keys == other.keys as f32 &&
        self.metal.fract() == 0.0 &&
        helpers::get_metal_from_float(self.metal) == other.metal
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
        metal: a.metal + b.metal as f32,
    } 
});

impl_op_ex!(+ |a: &Currencies, b: &FloatCurrencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys as f32 + b.keys,
        metal: a.metal as f32 + b.metal,
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
        metal: a.metal - b.metal as f32,
    } 
});

impl_op_ex!(- |a: &Currencies, b: &FloatCurrencies| -> FloatCurrencies { 
    FloatCurrencies {
        keys: a.keys as f32 - b.keys,
        metal: a.metal as f32 - b.metal,
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
        self.metal += helpers::get_metal_float(other.metal);
    }
}

impl AddAssign<&Currencies> for FloatCurrencies {
    fn add_assign(&mut self, other: &Currencies) {
        self.keys += other.keys as f32;
        self.metal += helpers::get_metal_float(other.metal);
    }
}

impl SubAssign<Currencies> for FloatCurrencies {
    fn sub_assign(&mut self, other: Currencies) {
        self.keys -= other.keys as f32;
        self.metal -= helpers::get_metal_float(other.metal);
    }
}

impl SubAssign<&Currencies> for FloatCurrencies {
    fn sub_assign(&mut self, other: &Currencies) {
        self.keys -= other.keys as f32;
        self.metal -= helpers::get_metal_float(other.metal);
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

impl<'a> TryFrom<&'a str> for FloatCurrencies {
    type Error = ParseError;
    
    fn try_from(string: &'a str) -> Result<Self, Self::Error>  {
        let (keys, metal) = helpers::parse_from_string_with_float_metal(string)?;
        
        Ok(FloatCurrencies {
            keys,
            metal,
        })
    }
}

impl From<Currencies> for FloatCurrencies {
    fn from(currencies: Currencies) -> FloatCurrencies {
        FloatCurrencies {
            keys: currencies.keys as f32,
            metal: helpers::get_metal_float(currencies.metal),
        }
    }
}

impl From<&Currencies> for FloatCurrencies {
    fn from(currencies: &Currencies) -> FloatCurrencies {
        FloatCurrencies {
            keys: currencies.keys as f32,
            metal: helpers::get_metal_float(currencies.metal),
        }
    }
}

impl fmt::Display for FloatCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keys != 0.0 && self.metal != 0.0 {
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
        } else if self.metal != 0.0 {
            write!(
                f,
                "{} {}",
                helpers::print_float(self.metal),
                METAL_SYMBOL,
            )
        } else {
            write!(f, "{}", EMPTY_SYMBOL)
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
        } else {
            let float = self.metal;
            
            if float.fract() == 0.0 {
                currencies.serialize_field("metal", &(float as Currency))?;
            } else {
                currencies.serialize_field("metal", &float)?;
            }
        }
        
        currencies.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::refined;
    use assert_json_diff::assert_json_eq;
    use serde_json::{self, json, Value};

    #[test]
    fn currencies_equal() {
        assert_eq!(FloatCurrencies {
            keys: 2.0,
            metal: 23.44,
        }, FloatCurrencies {
            keys: 2.0,
            metal: 23.44,
        });
    }
    
    #[test]
    fn to_metal_correct() {
        let key_price = 10;
        
        assert_eq!(FloatCurrencies {
            keys: 0.19,
            metal: 0.0,
        }.to_metal(key_price), 2);
    }
    
    #[test]
    fn currencies_not_equal() {
        assert_ne!(FloatCurrencies {
            keys: 2.0,
            metal: 23.44,
        }, FloatCurrencies {
            keys: 2.0,
            metal: 23.0,
        });
    }
    
    #[test]
    fn currencies_added() {
        assert_eq!(FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        } + FloatCurrencies {
            keys: 5.0,
            metal: 5.0,
        }, FloatCurrencies {
            keys: 15.0,
            metal: 15.0,
        });
    }
    
    #[test]
    fn currencies_subtracted() {
        assert_eq!(FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        } - FloatCurrencies {
            keys: 5.0,
            metal: 5.0,
        }, FloatCurrencies {
            keys: 5.0,
            metal: 5.0,
        });
    }
    
    #[test]
    fn currencies_multiplied_by_metal() {
        assert_eq!(FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        } * 5, FloatCurrencies {
            keys: 50.0,
            metal: 50.0,
        });
    }
    
    #[test]
    fn currencies_divided_by_metal() {
        assert_eq!(FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        } / 5, FloatCurrencies {
            keys: 2.0,
            metal: 2.0,
        });
    }
    
    #[test]
    fn currencies_multiplied_by_f32() {
        assert_eq!(FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        } * 2.5, FloatCurrencies {
            keys: 25.0,
            metal: 25.0,
        });
    }
    
    #[test]
    fn currencies_divided_by_f32() {
        assert_eq!(FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        } / 2.5, FloatCurrencies {
            keys: 4.0,
            metal: 4.0,
        });
    }
    
    #[test]
    fn converts_into_currencies() {
        let currencies: Currencies = FloatCurrencies {
            keys: 10.0,
            metal: 10.0,
        }.try_into().unwrap();
        
        assert_eq!(currencies, Currencies {
            keys: 10,
            metal: refined!(10),
        });
    }
    
    #[test]
    fn subtracts_non_float_currencies() {
        assert_eq!(FloatCurrencies {
            keys: 1.5,
            metal: 0.0,
        } - Currencies { keys: 1, metal: 0 }, FloatCurrencies { keys: 0.5, metal: 0.0 });
    }
    
    #[test]
    fn adds_non_float_currencies() {
        assert_eq!(FloatCurrencies {
            keys: 1.5,
            metal: 0.0,
        } + Currencies { keys: 1, metal: 0 }, FloatCurrencies { keys: 2.5, metal: 0.0 });
    }
    
    #[test]
    fn converts_into_currencies_with_key_price() {
        let currencies = Currencies::from_float_currencies(FloatCurrencies {
            keys: 2.5,
            metal: 10.0,
        }, refined!(10));
        
        assert_eq!(currencies, Currencies {
            keys: 2,
            metal: refined!(15),
        });
    }
    
    #[test]
    fn converts_into_currencies_with_key_price_negative_values() {
        let currencies = Currencies::from_float_currencies(FloatCurrencies {
            keys: 2.5,
            metal: -10.0,
        }, refined!(10));
        
        assert_eq!(currencies, Currencies {
            keys: 2,
            metal: refined!(-5),
        });
    }
    
    #[test]
    fn fails_to_convert_into_currencies_when_fractional() {
        let currencies = Currencies::try_from(FloatCurrencies {
            keys: 10.5,
            metal: 10.0,
        });
        
        assert!(currencies.is_err());
    }
    
    #[test]
    fn formats_currencies() {
        assert_eq!(&format!("{}", FloatCurrencies {
            keys: 2.0,
            metal: 23.0,
        }), "2 keys, 23 ref");
    }
    
    #[test]
    fn formats_currencies_with_decimal_places() {
        assert_eq!(&format!("{}", FloatCurrencies {
            keys: 2.2555,
            metal: 23.0,
        }), "2.26 keys, 23 ref");
    }
    
    #[test]
    fn deserializes_currencies() {
        let currencies: FloatCurrencies = serde_json::from_str(r#"{"keys":1,"metal": 23.44}"#).unwrap();
        
        assert_eq!(FloatCurrencies {
            keys: 1.0,
            metal: 23.44,
        }, currencies);
    }
    
    #[test]
    fn deserializes_currencies_with_no_keys() {
        let currencies: FloatCurrencies = serde_json::from_str(r#"{"metal": 23.44}"#).unwrap();
        
        assert_eq!(FloatCurrencies {
            keys: 0.0,
            metal: 23.44,
        }, currencies);
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
        
        assert_json_eq!(
            actual,
            expected,
        );
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
        
        assert_json_eq!(
            actual,
            expected,
        );
    }
    
    #[test]
    fn greater_than() {
        assert!(FloatCurrencies { keys: 1.0, metal: 5.0 } > FloatCurrencies { keys: 0.0, metal: 10.0 });
    }
    
    #[test]
    fn less_than() {
        assert!(FloatCurrencies { keys: 0.0, metal: 1.0 } < FloatCurrencies { keys: 0.0, metal: 4.0 });
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
        
        assert_eq!(*currencies.iter().rev().next().unwrap(), FloatCurrencies { keys: 10.0, metal: 4.0 });
    }
    
    #[test]
    fn checked_to_metal() {
        assert_eq!(
            FloatCurrencies { keys: Currency::MAX as f32, metal: 4.0 }.checked_to_metal(Currency::MAX),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal_correct_value() {
        assert_eq!(
            FloatCurrencies { keys: 10.0, metal: 5.0 }.checked_to_metal(10),
            Some(100 + refined!(5)),
        );
    }
}