use crate::{
    Rounding,
    Currencies,
    helpers,
    constants::{
        KEYS_SYMBOL,
        KEY_SYMBOL,
        METAL_SYMBOL,
    },
};
use std::{fmt, ops::{Add, Sub, Mul, Div, AddAssign, SubAssign}};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error, ser::SerializeStruct};

/// Currencies for listings.
/// 
/// The `keys` field for `ListingCurrencies` is defined as an f32. Use this anywhere you
/// may need key values which include decimal places.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(remote = "Self")]
pub struct ListingCurrencies {
    #[serde(default)]
    pub keys: f32,
    #[serde(deserialize_with = "helpers::metal_deserializer", default)]
    pub metal: i32,
}

impl Default for ListingCurrencies {
    
    fn default() -> Self {
        Self::new()
    }
}

impl ListingCurrencies {
    
    pub fn new() -> Self {
        Self {
            keys: 0.0,
            metal: 0,
        }
    }
    
    /// Checks if the `keys` value has a fractional value.
    pub fn is_fract(&self) -> bool {
        self.keys.fract() != 0.0
    }
    
    /// Creates currencies from a metal value using the given key price.
    pub fn to_metal(&self, key_price: i32) -> i32 {
        self.metal + (self.keys * key_price as f32) as i32
    }
    
    /// Checks if the currencies contain any value.
    pub fn is_empty(&self) -> bool {
        self.keys == 0.0 && self.metal == 0
    }
    
    pub fn round(&mut self, rounding: &Rounding) {
        self.metal = helpers::round_metal(self.metal, rounding);
    }
}

impl PartialEq<Currencies> for ListingCurrencies {
    
    fn eq(&self, other: &Currencies) -> bool {
        self.keys.fract() == 0.0 &&
        self.keys == other.keys as f32 &&
        self.metal == other.metal
    }
}

impl Add<ListingCurrencies> for ListingCurrencies {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            keys: self.keys + other.keys,
            metal: self.metal + other.metal,
        }
    }
}

impl Add<&ListingCurrencies> for ListingCurrencies {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            keys: self.keys + other.keys,
            metal: self.metal + other.metal,
        }
    }
}

impl AddAssign<ListingCurrencies> for ListingCurrencies {
    
    fn add_assign(&mut self, other: Self) {
        self.keys += other.keys;
        self.metal += other.metal;
    }
}

impl AddAssign<&ListingCurrencies> for ListingCurrencies {
    
    fn add_assign(&mut self, other: &Self) {
        self.keys += other.keys;
        self.metal += other.metal;
    }
}

impl Sub<ListingCurrencies> for ListingCurrencies {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            keys: self.keys - other.keys,
            metal: self.metal - other.metal,
        }
    }
}

impl Sub<&ListingCurrencies> for ListingCurrencies {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self {
            keys: self.keys - other.keys,
            metal: self.metal - other.metal,
        }
    }
}

impl SubAssign<ListingCurrencies> for ListingCurrencies {
    
    fn sub_assign(&mut self, other: Self) {
        self.keys -= other.keys;
        self.metal -= other.metal;
    }
}

impl SubAssign<&ListingCurrencies> for ListingCurrencies {
    
    fn sub_assign(&mut self, other: &Self) {
        self.keys -= other.keys;
        self.metal -= other.metal;
    }
}

// Operations for non-float currencies

impl Add<Currencies> for ListingCurrencies {
    type Output = Self;

    fn add(self, other: Currencies) -> Self {
        Self {
            keys: self.keys + other.keys as f32,
            metal: self.metal + other.metal,
        }
    }
}

impl Add<&Currencies> for ListingCurrencies {
    type Output = Self;

    fn add(self, other: &Currencies) -> Self {
        Self {
            keys: self.keys + other.keys as f32,
            metal: self.metal + other.metal,
        }
    }
}

impl AddAssign<Currencies> for ListingCurrencies {
    
    fn add_assign(&mut self, other: Currencies) {
        self.keys += other.keys as f32;
        self.metal += other.metal;
    }
}

impl AddAssign<&Currencies> for ListingCurrencies {
    
    fn add_assign(&mut self, other: &Currencies) {
        self.keys += other.keys as f32;
        self.metal += other.metal;
    }
}

impl Sub<Currencies> for ListingCurrencies {
    type Output = Self;

    fn sub(self, other: Currencies) -> Self {
        Self {
            keys: self.keys - other.keys as f32,
            metal: self.metal - other.metal,
        }
    }
}

impl Sub<&Currencies> for ListingCurrencies {
    type Output = Self;

    fn sub(self, other: &Currencies) -> Self {
        Self {
            keys: self.keys - other.keys as f32,
            metal: self.metal - other.metal,
        }
    }
}

impl SubAssign<Currencies> for ListingCurrencies {
    
    fn sub_assign(&mut self, other: Currencies) {
        self.keys -= other.keys as f32;
        self.metal -= other.metal;
    }
}

impl SubAssign<&Currencies> for ListingCurrencies {
    
    fn sub_assign(&mut self, other: &Currencies) {
        self.keys -= other.keys as f32;
        self.metal -= other.metal;
    }
}

// Operations for integers

impl Div<i32> for ListingCurrencies {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self {
            keys: self.keys / other as f32,
            metal: self.metal / other,
        }
    }
}

impl Mul<i32> for ListingCurrencies {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            keys: self.keys * other as f32,
            metal: self.metal * other,
        }
    }
}

// Operations for floats

impl Div<f32> for ListingCurrencies {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            keys: self.keys / other,
            metal: (self.metal as f32 / other).round() as i32,
        }
    }
}

impl Mul<f32> for ListingCurrencies {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            keys: self.keys * other,
            metal: (self.metal as f32 * other).round() as i32,
        }
    }
}

impl<'a> TryFrom<&'a str> for ListingCurrencies {
    type Error = &'static str;
    
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
        if self.keys > 0.0 && self.metal > 0 {
            write!(
                f,
                "{} {}, {} {}",
                helpers::print_float(self.keys),
                helpers::pluralize_float(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
                helpers::get_metal_float(self.metal),
                METAL_SYMBOL,
            )
        } else if self.keys > 0.0 {
            write!(
                f,
                "{} {}",
                helpers::print_float(self.keys),
                helpers::pluralize_float(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
            )
        } else if self.metal > 0 {
            write!(
                f,
                "{} {}",
                helpers::get_metal_float(self.metal),
                METAL_SYMBOL,
            )
        } else {
            write!(f, "")
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
        
        assert_eq!(currencies.is_err(), true);
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
}