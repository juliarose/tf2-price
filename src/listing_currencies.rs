use crate::helpers;
use crate::traits::SerializeCurrencies;
use crate::error::ParseError;
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL, EMPTY_SYMBOL};
use crate::{Currencies, Rounding};
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops::{self, AddAssign, SubAssign, MulAssign, DivAssign};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use serde::ser::SerializeStruct;

/// The `keys` field for [`ListingCurrencies`] is defined as an [`f32`]. Use this anywhere you may
/// need key values which include decimal places.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(remote = "Self")]
pub struct ListingCurrencies {
    /// Amount of keys.
    #[serde(default)]
    pub keys: f32,
    /// Amount of metal expressed as weapons. A metal value of 6 would be equivalent to 3 scrap. 
    /// It's recommended to use the [`ONE_REF`], [`ONE_REC`], [`ONE_SCRAP`], and [`ONE_WEAPON`] 
    /// constants to perform arithmatic.
    #[serde(deserialize_with = "helpers::metal_deserializer", default)]
    pub metal: i32,
}

impl PartialOrd for ListingCurrencies {
    fn partial_cmp(&self, other: &ListingCurrencies) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for ListingCurrencies {
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

impl Eq for ListingCurrencies {}

impl SerializeCurrencies for ListingCurrencies {}

impl Default for ListingCurrencies {
    fn default() -> Self {
        Self::new()
    }
}

impl ListingCurrencies {
    /// Creates a new [`ListingCurrencies`] with `0` keys and `0` metal.
    pub fn new() -> Self {
        Self {
            keys: 0.0,
            metal: 0,
        }
    }
    
    /// Checks if the `keys` value is a fractional value.
    pub fn is_fract(&self) -> bool {
        self.keys.fract() != 0.0
    }
    
    /// Converts currencies to a metal value using the given key price (represented as weapons).
    /// Rounds float conversions and saturates at integer bounds.
    pub fn to_metal(&self, key_price: i32) -> i32 {
        self.metal.saturating_add((self.keys * key_price as f32).round() as i32)
    }
    
    /// Converts currencies to a metal value using the given key price (represented as weapons).
    /// In cases where the result overflows or underflows beyond the limit for i32, `None` is 
    /// returned.
    pub fn checked_to_metal(&self, key_price: i32) -> Option<i32> {
        let result = (self.keys * key_price as f32).round();
        let result_i32 = result as i32;
        
        // Check for overflow by seeing if conversions produce unequal results
        if result != result_i32 as f32 {
            return None;
        }
        
        self.metal.checked_add(result_i32)
    }
    
    /// Checks if the currencies contain any value.
    pub fn is_empty(&self) -> bool {
        self.keys == 0.0 && self.metal == 0
    }
    
    /// Rounds the metal value using the given rounding method.
    pub fn round(mut self, rounding: &Rounding) -> Self {
        self.metal = helpers::round_metal(self.metal, rounding);
        self
    }
    
    /// Checks whether the currencies have enough keys and metal to afford the `other` currencies.
    /// This is simply `self.keys >= other.keys && self.metal >= other.metal`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::Currencies;
    /// 
    /// let currencies = Currencies { keys: 100, metal: 30 };
    /// 
    /// // We have at least 50 keys and 30 metal.
    /// assert!(currencies.can_afford(&Currencies {
    ///     keys: 50,
    ///     metal: 30,
    /// }));
    /// // Not enough metal - we can't afford this.
    /// assert!(!currencies.can_afford(&Currencies {
    ///     keys: 50,
    ///     metal: 100,
    /// }));
    /// ```
    pub fn can_afford(&self, other: &Self) -> bool {
        self.keys >= other.keys && self.metal >= other.metal
    }
}

impl PartialEq<Currencies> for ListingCurrencies {
    fn eq(&self, other: &Currencies) -> bool {
        self.keys.fract() == 0.0 &&
        self.keys == other.keys as f32 &&
        self.metal == other.metal
    }
}

impl_op_ex!(+ |a: &ListingCurrencies, b: &ListingCurrencies| -> ListingCurrencies { 
    ListingCurrencies {
        keys: a.keys + b.keys,
        metal: a.metal.saturating_add(b.metal),
    } 
});

impl_op_ex!(+ |a: &ListingCurrencies, b: &Currencies| -> ListingCurrencies { 
    ListingCurrencies {
        keys: a.keys + b.keys as f32,
        metal: a.metal.saturating_add(b.metal),
    } 
});

impl_op_ex!(+ |a: &Currencies, b: &ListingCurrencies| -> ListingCurrencies { 
    ListingCurrencies {
        keys: a.keys as f32 + b.keys,
        metal: a.metal.saturating_add(b.metal),
    } 
});

impl_op_ex!(- |a: &ListingCurrencies, b: &ListingCurrencies| -> ListingCurrencies { 
    ListingCurrencies {
        keys: a.keys - b.keys,
        metal: a.metal.saturating_sub(b.metal),
    }
});

impl_op_ex!(- |a: &ListingCurrencies, b: &Currencies| -> ListingCurrencies { 
    ListingCurrencies {
        keys: a.keys - b.keys as f32,
        metal: a.metal.saturating_sub(b.metal),
    } 
});

impl_op_ex!(- |a: &Currencies, b: &ListingCurrencies| -> ListingCurrencies { 
    ListingCurrencies {
        keys: a.keys as f32 - b.keys,
        metal: a.metal.saturating_sub(b.metal),
    } 
});

impl_op_ex!(* |currencies: &ListingCurrencies, num: i32| -> ListingCurrencies {
    ListingCurrencies {
        keys: currencies.keys * num as f32,
        metal: currencies.metal.saturating_mul(num),
    }
});

impl_op_ex!(/ |currencies: &ListingCurrencies, num: i32| -> ListingCurrencies {
    ListingCurrencies {
        keys: currencies.keys / num as f32,
        metal: currencies.metal.saturating_div(num),
    }
});

impl_op_ex!(* |currencies: &ListingCurrencies, num: f32| -> ListingCurrencies {
    ListingCurrencies { 
        keys: currencies.keys * num,
        metal: (currencies.metal as f32 * num).round() as i32,
    }
});

impl_op_ex!(/ |currencies: &ListingCurrencies, num: f32| -> ListingCurrencies {
    ListingCurrencies {
        keys: currencies.keys / num,
        metal: (currencies.metal as f32 / num).round() as i32,
    }
});

impl AddAssign<ListingCurrencies> for ListingCurrencies {
    fn add_assign(&mut self, other: Self) {
        self.keys += other.keys;
        self.metal = self.metal.saturating_add(other.metal);
    }
}

impl AddAssign<&ListingCurrencies> for ListingCurrencies {
    fn add_assign(&mut self, other: &Self) {
        self.keys += other.keys;
        self.metal = self.metal.saturating_add(other.metal);
    }
}

impl SubAssign<ListingCurrencies> for ListingCurrencies {
    fn sub_assign(&mut self, other: Self) {
        self.keys -= other.keys;
        self.metal = self.metal.saturating_sub(other.metal);
    }
}

impl SubAssign<&ListingCurrencies> for ListingCurrencies {
    fn sub_assign(&mut self, other: &Self) {
        self.keys -= other.keys;
        self.metal = self.metal.saturating_sub(other.metal);
    }
}

// Operations for non-float currencies

impl AddAssign<Currencies> for ListingCurrencies {
    fn add_assign(&mut self, other: Currencies) {
        self.keys += other.keys as f32;
        self.metal = self.metal.saturating_add(other.metal);
    }
}

impl AddAssign<&Currencies> for ListingCurrencies {
    fn add_assign(&mut self, other: &Currencies) {
        self.keys += other.keys as f32;
        self.metal = self.metal.saturating_add(other.metal);
    }
}

impl SubAssign<Currencies> for ListingCurrencies {
    fn sub_assign(&mut self, other: Currencies) {
        self.keys -= other.keys as f32;
        self.metal = self.metal.saturating_sub(other.metal);
    }
}

impl SubAssign<&Currencies> for ListingCurrencies {
    fn sub_assign(&mut self, other: &Currencies) {
        self.keys -= other.keys as f32;
        self.metal = self.metal.saturating_sub(other.metal);
    }
}

impl MulAssign<i32> for ListingCurrencies {
    fn mul_assign(&mut self, other: i32) {
        self.keys *= other as f32;
        self.metal = self.metal.saturating_mul(other);
    }
}

impl MulAssign<f32> for ListingCurrencies {
    fn mul_assign(&mut self, other: f32) {
        self.keys *= other;
        self.metal = (self.metal as f32 * other).round() as i32;
    }
}

impl DivAssign<i32> for ListingCurrencies {
    fn div_assign(&mut self, other: i32) {
        self.keys /= other as f32;
        self.metal = self.metal.saturating_div(other);
    }
}

impl DivAssign<f32> for ListingCurrencies {
    fn div_assign(&mut self, other: f32) {
        self.keys /= other;
        self.metal = (self.metal as f32 / other).round() as i32;
    }
}

impl<'a> TryFrom<&'a str> for ListingCurrencies {
    type Error = ParseError;
    
    fn try_from(string: &'a str) -> Result<Self, Self::Error>  {
        let (keys, metal) = helpers::parse_from_string::<f32>(string)?;
        
        Ok(ListingCurrencies {
            keys,
            metal,
        })
    }
}

impl From<Currencies> for ListingCurrencies {
    fn from(currencies: Currencies) -> ListingCurrencies {
        ListingCurrencies {
            keys: currencies.keys as f32,
            metal: currencies.metal,
        }
    }
}

impl From<&Currencies> for ListingCurrencies {
    fn from(currencies: &Currencies) -> ListingCurrencies {
        ListingCurrencies {
            keys: currencies.keys as f32,
            metal: currencies.metal,
        }
    }
}

impl fmt::Display for ListingCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keys != 0.0 && self.metal != 0 {
            write!(
                f,
                "{} {}, {} {}",
                helpers::print_float(self.keys),
                helpers::pluralize_float(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
                helpers::get_metal_float(self.metal),
                METAL_SYMBOL,
            )
        } else if self.keys != 0.0 {
            write!(
                f,
                "{} {}",
                helpers::print_float(self.keys),
                helpers::pluralize_float(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
            )
        } else if self.metal != 0 {
            write!(
                f,
                "{} {}",
                helpers::get_metal_float(self.metal),
                METAL_SYMBOL,
            )
        } else {
            write!(f, "{}", EMPTY_SYMBOL)
        }
    }
}

impl<'de> Deserialize<'de> for ListingCurrencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let currencies = Self::deserialize(deserializer)?;
        
        if currencies.keys == 0.0 && currencies.metal == 0 {
            return Err(D::Error::custom("Does not contain values for keys or metal"));
        }
        
        Ok(currencies)
    }
}

impl Serialize for ListingCurrencies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut currencies = serializer.serialize_struct("ListingCurrencies", 2)?;
        
        if self.keys == 0.0 {
            currencies.skip_field("keys")?;
        } else if self.keys.fract() == 0.0 {
            currencies.serialize_field("keys", &(self.keys as i32))?;
        } else {
            currencies.serialize_field("keys", &self.keys)?;
        }
        
        if self.metal == 0 {
            currencies.skip_field("metal")?;
        } else {
            let float = helpers::get_metal_float(self.metal);
            
            if float.fract() == 0.0 {
                currencies.serialize_field("metal", &(float as i32))?;
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
    use crate::{refined, scrap};
    use assert_json_diff::assert_json_eq;
    use serde_json::{self, json, Value};

    #[test]
    fn currencies_equal() {
        assert_eq!(ListingCurrencies {
            keys: 2.0,
            metal: refined!(23) + scrap!(4),
        }, ListingCurrencies {
            keys: 2.0,
            metal: refined!(23) + scrap!(4),
        });
    }
    
    #[test]
    fn to_metal_correct() {
        let key_price = 10;
        
        assert_eq!(ListingCurrencies {
            keys: 0.19,
            metal: 0,
        }.to_metal(key_price), 2);
    }
    
    #[test]
    fn currencies_not_equal() {
        assert_ne!(ListingCurrencies {
            keys: 2.0,
            metal: refined!(23) + scrap!(4),
        }, ListingCurrencies {
            keys: 2.0,
            metal: refined!(23),
        });
    }
    
    #[test]
    fn currencies_added() {
        assert_eq!(ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        } + ListingCurrencies {
            keys: 5.0,
            metal: refined!(5),
        }, ListingCurrencies {
            keys: 15.0,
            metal: refined!(15),
        });
    }
    
    #[test]
    fn currencies_subtracted() {
        assert_eq!(ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        } - ListingCurrencies {
            keys: 5.0,
            metal: refined!(5),
        }, ListingCurrencies {
            keys: 5.0,
            metal: refined!(5),
        });
    }
    
    #[test]
    fn currencies_multiplied_by_i32() {
        assert_eq!(ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        } * 5, ListingCurrencies {
            keys: 50.0,
            metal: refined!(50),
        });
    }
    
    #[test]
    fn currencies_divided_by_i32() {
        assert_eq!(ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        } / 5, ListingCurrencies {
            keys: 2.0,
            metal: refined!(2),
        });
    }
    
    #[test]
    fn currencies_multiplied_by_f32() {
        assert_eq!(ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        } * 2.5, ListingCurrencies {
            keys: 25.0,
            metal: refined!(25),
        });
    }
    
    #[test]
    fn currencies_divided_by_f32() {
        assert_eq!(ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        } / 2.5, ListingCurrencies {
            keys: 4.0,
            metal: refined!(4),
        });
    }
    
    #[test]
    fn converts_into_currencies() {
        let currencies: Currencies = ListingCurrencies {
            keys: 10.0,
            metal: refined!(10),
        }.try_into().unwrap();
        
        assert_eq!(currencies, Currencies {
            keys: 10,
            metal: refined!(10),
        });
    }
    
    #[test]
    fn subtracts_non_float_currencies() {
        assert_eq!(ListingCurrencies {
            keys: 1.5,
            metal: 0,
        } - Currencies { keys: 1, metal: 0 }, ListingCurrencies { keys: 0.5, metal: 0 });
    }
    
    #[test]
    fn adds_non_float_currencies() {
        assert_eq!(ListingCurrencies {
            keys: 1.5,
            metal: 0,
        } + Currencies { keys: 1, metal: 0 }, ListingCurrencies { keys: 2.5, metal: 0 });
    }
    
    #[test]
    fn converts_into_currencies_with_key_price() {
        let currencies = Currencies::from_listing_currencies(ListingCurrencies {
            keys: 2.5,
            metal: refined!(10),
        }, refined!(10));
        
        assert_eq!(currencies, Currencies {
            keys: 2,
            metal: refined!(15),
        });
    }
    
    #[test]
    fn converts_into_currencies_with_key_price_negative_values() {
        let currencies = Currencies::from_listing_currencies(ListingCurrencies {
            keys: 2.5,
            metal: refined!(-10),
        }, refined!(10));
        
        assert_eq!(currencies, Currencies {
            keys: 2,
            metal: refined!(-5),
        });
    }
    
    #[test]
    fn fails_to_convert_into_currencies_when_fractional() {
        let currencies = Currencies::try_from(ListingCurrencies {
            keys: 10.5,
            metal: refined!(10),
        });
        
        assert!(currencies.is_err());
    }
    
    #[test]
    fn formats_currencies() {
        assert_eq!(&format!("{}", ListingCurrencies {
            keys: 2.0,
            metal: refined!(23),
        }), "2 keys, 23 ref");
    }
    
    #[test]
    fn formats_currencies_with_decimal_places() {
        assert_eq!(&format!("{}", ListingCurrencies {
            keys: 2.2555,
            metal: refined!(23),
        }), "2.26 keys, 23 ref");
    }
    
    #[test]
    fn deserializes_currencies() {
        let currencies: ListingCurrencies = serde_json::from_str(r#"{"keys":1,"metal": 23.44}"#).unwrap();
        
        assert_eq!(ListingCurrencies {
            keys: 1.0,
            metal: refined!(23) + scrap!(4),
        }, currencies);
    }
    
    #[test]
    fn deserializes_currencies_with_no_keys() {
        let currencies: Currencies = serde_json::from_str(r#"{"metal": 23.44}"#).unwrap();
        
        assert_eq!(ListingCurrencies {
            keys: 0.0,
            metal: refined!(23) + scrap!(4),
        }, currencies);
    }
    
    #[test]
    fn serializes_currencies() {
        let currencies = ListingCurrencies {
            keys: 1.0,
            metal: refined!(23) + scrap!(4)
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
        let currencies = ListingCurrencies {
            keys: 1.5,
            metal: refined!(23) + scrap!(4)
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
        assert!(ListingCurrencies { keys: 1.0, metal: 5 } > ListingCurrencies { keys: 0.0, metal: 10});
    }
    
    #[test]
    fn less_than() {
        assert!(ListingCurrencies { keys: 0.0, metal: 1 } < ListingCurrencies { keys: 0.0, metal: 4});
    }
    
    #[test]
    fn sorts() {
        let mut currencies = vec![
            ListingCurrencies { keys: 2.0, metal: 4 },
            ListingCurrencies { keys: 0.0, metal: 2 },
            ListingCurrencies { keys: 10.0, metal: 4 },
        ];
        
        // lowest to highest
        currencies.sort();
        
        assert_eq!(*currencies.iter().rev().next().unwrap(), ListingCurrencies { keys: 10.0, metal: 4});
    }
    
    #[test]
    fn checked_to_metal() {
        assert_eq!(
            ListingCurrencies { keys: i32::MAX as f32, metal: 4 }.checked_to_metal(i32::MAX),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal_correct_value() {
        assert_eq!(
            ListingCurrencies { keys: 10.0, metal: 5 }.checked_to_metal(10),
            Some(105),
        );
    }
}