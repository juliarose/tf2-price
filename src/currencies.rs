use crate::{
    Rounding,
    ListingCurrencies,
    helpers,
    constants::{
        KEYS_SYMBOL,
        KEY_SYMBOL,
        METAL_SYMBOL,
        EMPTY_SYMBOL,
    },
};
use std::{fmt, ops::{self, AddAssign, SubAssign, MulAssign, DivAssign}};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error, ser::SerializeStruct};

/// For storing item currencies values.
/// 
/// Metal values are stored as their lowest denomination, 1 weapon. A metal value of 6 would 
/// be equivalent to 3 scrap. You may use the `ONE_REF`, `ONE_REC`, `ONE_SCRAP`, and `ONE_WEAPON`
/// constants to perform arithmatic.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(remote = "Self")]
pub struct Currencies {
    #[serde(default)]
    pub keys: i32,
    #[serde(deserialize_with = "helpers::metal_deserializer", default)]
    pub metal: i32,
}

impl Default for Currencies {
    
    fn default() -> Self {
        Self::new()
    }
}

impl Currencies {
    
    pub fn new() -> Self {
        Self {
            keys: 0,
            metal: 0,
        }
    }
    
    /// Creates currencies from a metal value using the given key price.
    pub fn from_metal(metal: i32, key_price: i32) -> Self {
        Self {
            // Will be 0 if metal is 30 and key price is 32 (rounds down)
            keys: metal / key_price,
            metal: metal % key_price,
        }
    }
    
    /// Converts from `ListingCurrencies` using the given key price.
    pub fn from_listing_currencies(currencies: ListingCurrencies, key_price: i32) -> Self {
        let keys = currencies.keys;
        let metal = currencies.metal;
        
        Self {
            keys: keys as i32,
            metal: ((keys % 1.0) * key_price as f32).round() as i32 + metal
        }
    }
    
    /// Converts an f32 key value into `Currencies` using the given key price.
    pub fn from_keys_f32(keys: f32, key_price: i32) -> Self {
        Self {
            keys: keys as i32,
            metal: ((keys % 1.0) * key_price as f32) as i32
        }
    }
    
    /// Converts `Currencies` into a metal value using the given key price.
    pub fn to_metal(&self, key_price: i32) -> i32 {
        self.metal + (self.keys * key_price)
    }
    
    /// Checks if the currencies contain any value.
    pub fn is_empty(&self) -> bool {
        self.keys == 0 && self.metal == 0
    }
    
    /// Rounds the metal value using the given rounding method.
    pub fn round(&mut self, rounding: &Rounding) {
        self.metal = helpers::round_metal(self.metal, rounding);
    }
}

/// Comparison with `ListingCurrencies` will fail if `ListingCurrencies` has a fractional key value.
impl PartialEq<ListingCurrencies> for Currencies {
    
    fn eq(&self, other: &ListingCurrencies) -> bool {
        !other.is_fract() &&
        self.keys == other.keys as i32 &&
        self.metal == other.metal
    }
}

impl_op_ex!(+ |a: &Currencies, b: &Currencies| -> Currencies { 
    Currencies {
        keys: a.keys + b.keys,
        metal: a.metal + b.metal
    } 
});

impl_op_ex!(- |a: &Currencies, b: &Currencies| -> Currencies { 
    Currencies {
        keys: a.keys - b.keys,
        metal: a.metal - b.metal,
    }
});

impl_op_ex!(* |currencies: &Currencies, num: i32| -> Currencies {
    Currencies {
        keys: currencies.keys * num,
        metal: currencies.metal * num,
    }
});

impl_op_ex!(/ |currencies: &Currencies, num: i32| -> Currencies {
    Currencies {
        keys: currencies.keys / num,
        metal: currencies.metal / num,
    }
});

impl_op_ex!(* |currencies: &Currencies, num: f32| -> Currencies {
    Currencies { 
        keys: (currencies.keys as f32 * num).round() as i32,
        metal: (currencies.metal as f32 * num).round() as i32,
    }
});

impl_op_ex!(/ |currencies: &Currencies, num: f32| -> Currencies {
    Currencies {
        keys: (currencies.keys as f32 / num).round() as i32,
        metal: (currencies.metal as f32 / num).round() as i32,
    }
});

impl AddAssign<Currencies> for Currencies {
    
    fn add_assign(&mut self, other: Self) {
        self.keys += other.keys;
        self.metal += other.metal;
    }
}

impl AddAssign<&Currencies> for Currencies {
    
    fn add_assign(&mut self, other: &Self) {
        self.keys += other.keys;
        self.metal += other.metal;
    }
}

impl SubAssign<Currencies> for Currencies {
    
    fn sub_assign(&mut self, other: Self) {
        self.keys -= other.keys;
        self.metal -= other.metal;
    }
}

impl SubAssign<&Currencies> for Currencies {
    
    fn sub_assign(&mut self, other: &Self) {
        self.keys -= other.keys;
        self.metal -= other.metal;
    }
}

impl MulAssign<i32> for Currencies {
    
    fn mul_assign(&mut self, other: i32) {
        self.keys *= other;
        self.metal *= other;
    }
}

impl MulAssign<f32> for Currencies {
    
    fn mul_assign(&mut self, other: f32) {
        self.keys = (self.keys as f32 * other).round() as i32;
        self.metal = (self.metal as f32 * other).round() as i32;
    }
}

impl DivAssign<i32> for Currencies {
    
    fn div_assign(&mut self, other: i32) {
        self.keys /= other;
        self.metal /= other;
    }
}

impl DivAssign<f32> for Currencies {
    
    fn div_assign(&mut self, other: f32) {
        self.keys = (self.keys as f32 / other).round() as i32;
        self.metal = (self.metal as f32 / other).round() as i32;
    }
}

impl<'a> TryFrom<&'a str> for Currencies {
    type Error = &'static str;
    
    fn try_from(string: &'a str) -> Result<Self, Self::Error>  {
        let (keys, metal) = helpers::parse_from_string::<i32>(string)?;
        
        Ok(Currencies {
            keys,
            metal,
        })
    }
}

/// Results in error if `ListingCurrencies` contains a fractional key value.
impl TryFrom<ListingCurrencies> for Currencies {
    type Error = &'static str;
    
    fn try_from(currencies: ListingCurrencies) -> Result<Self, Self::Error> {
        if currencies.is_fract() {
            return Err("Currencies contain fractional key value");
        }
        
        Ok(Currencies {
            keys: currencies.keys as i32,
            metal: currencies.metal,
        })
    }
}

/// Results in error if `ListingCurrencies` contains a fractional key value.
impl TryFrom<&ListingCurrencies> for Currencies {
    type Error = &'static str;
    
    fn try_from(currencies: &ListingCurrencies) -> Result<Self, Self::Error> {
        if currencies.is_fract() {
            return Err("Currencies contain fractional key value");
        }
        
        Ok(Currencies {
            keys: currencies.keys as i32,
            metal: currencies.metal,
        })
    }
}

impl fmt::Display for Currencies {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keys != 0 && self.metal != 0 {
            write!(
                f,
                "{} {}, {} {}",
                self.keys,
                helpers::pluralize(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
                helpers::get_metal_float(self.metal),
                METAL_SYMBOL,
            )
        } else if self.keys != 0 {
            write!(
                f,
                "{} {}",
                self.keys,
                helpers::pluralize(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
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

impl<'de> Deserialize<'de> for Currencies {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let currencies = Self::deserialize(deserializer)?;
        
        if currencies.keys == 0 && currencies.metal == 0 {
            return Err(D::Error::custom("Does not contain values for keys or metal"));
        }
        
        Ok(currencies)
    }
}

impl Serialize for Currencies {
    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut currencies = serializer.serialize_struct("Currencies", 2)?;
        
        if self.keys == 0 {
            currencies.skip_field("keys")?;
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
        assert_eq!(Currencies {
            keys: 2,
            metal: refined!(23) + scrap!(4),
        }, Currencies {
            keys: 2,
            metal: refined!(23) + scrap!(4),
        });
    }
    
    #[test]
    fn currencies_not_equal() {
        assert_ne!(Currencies {
            keys: 2,
            metal: refined!(23) + scrap!(4),
        }, Currencies {
            keys: 2,
            metal: refined!(23),
        });
    }
    
    #[test]
    fn currencies_added() {
        assert_eq!(Currencies {
            keys: 10,
            metal: refined!(10),
        } + Currencies {
            keys: 5,
            metal: refined!(5),
        }, Currencies {
            keys: 15,
            metal: refined!(15),
        });
    }
    
    #[test]
    fn currencies_subtracted() {
        assert_eq!(Currencies {
            keys: 10,
            metal: refined!(10),
        } - Currencies {
            keys: 5,
            metal: refined!(5),
        }, Currencies {
            keys: 5,
            metal: refined!(5),
        });
    }
    
    #[test]
    fn currencies_multiplied_by_i32() {
        assert_eq!(Currencies {
            keys: 10,
            metal: refined!(10),
        } * 5, Currencies {
            keys: 50,
            metal: refined!(50),
        });
    }
    
    #[test]
    fn currencies_divided_by_f32() {
        assert_eq!(Currencies {
            keys: 10,
            metal: refined!(10),
        } / 2.5, Currencies {
            keys: 4,
            metal: refined!(4),
        });
    }
    
    #[test]
    fn currencies_divided_by_i32() {
        assert_eq!(Currencies {
            keys: 10,
            metal: refined!(10),
        } / 5, Currencies {
            keys: 2,
            metal: refined!(2),
        });
    }
    
    #[test]
    fn currencies_multiplied_by_f32() {
        assert_eq!(Currencies {
            keys: 10,
            metal: refined!(10),
        } * 2.5, Currencies {
            keys: 25,
            metal: refined!(25),
        });
    }
    
    #[test]
    fn currencies_mul_assign_i32() {
        let mut currencies = Currencies {
            keys: 10,
            metal: refined!(10),
        };
        
        currencies *= 2;
        
        assert_eq!(currencies, Currencies {
            keys: 20,
            metal: refined!(20),
        });
    }
    
    #[test]
    fn currencies_mul_assign_f32() {
        let mut currencies = Currencies {
            keys: 10,
            metal: refined!(10),
        };
        
        currencies *= 2.5;
        
        assert_eq!(currencies, Currencies {
            keys: 25,
            metal: refined!(25),
        });
    }
    
    #[test]
    fn currencies_div_assign_i32() {
        let mut currencies = Currencies {
            keys: 10,
            metal: refined!(10),
        };
        
        currencies /= 2;
        
        assert_eq!(currencies, Currencies {
            keys: 5,
            metal: refined!(5),
        });
    }
    
    #[test]
    fn currencies_div_assign_f32() {
        let mut currencies = Currencies {
            keys: 10,
            metal: refined!(10),
        };
        
        currencies /= 2.5;
        
        assert_eq!(currencies, Currencies {
            keys: 4,
            metal: refined!(4),
        });
    }
    
    #[test]
    fn parses_currencies_from_string() {
        let currencies = Currencies::try_from("2 keys, 23.44 ref").unwrap();
        
        assert_eq!(currencies.keys, 2);
        assert_eq!(currencies.metal, 422);
    }
    
    #[test]
    fn parses_currencies_from_string_only_keys() {
        let currencies = Currencies::try_from("1 key").unwrap();
        
        assert_eq!(currencies.keys, 1);
        assert_eq!(currencies.metal, 0);
    }
    
    #[test]
    fn parses_currencies_from_string_only_metal() {
        let currencies = Currencies::try_from("2 ref").unwrap();
        
        assert_eq!(currencies.keys, 0);
        assert_eq!(currencies.metal, refined!(2));
    }
    
    #[test]
    fn parses_currencies_from_string_invalid_currencies() {
        assert_eq!(Currencies::try_from("what").is_err(), true);
    }
    
    #[test]
    fn parses_currencies_from_string_invalid_currencies_extra() {
        assert_eq!(Currencies::try_from("2 keys, 3 what").is_err(), true);
    }
    
    #[test]
    fn gets_correct_value_from_metal() {
        assert_eq!(Currencies::from_metal(9, 10), Currencies {
            keys: 0,
            metal: 9,
        });
    }
    
    #[test]
    fn gets_correct_value_from_metal_with_keys() {
        assert_eq!(Currencies::from_metal(10, 10), Currencies {
            keys: 1,
            metal: 0,
        });
    }
    
    #[test]
    fn gets_correct_value_from_metal_with_keys_and_metal() {
        assert_eq!(Currencies::from_metal(11, 10), Currencies {
            keys: 1,
            metal: 1,
        });
    }
    
    #[test]
    fn gets_correct_value_from_keys_f32() {
        assert_eq!(Currencies::from_keys_f32(1.5, 10), Currencies {
            keys: 1,
            metal: 5,
        });
    }
    
    #[test]
    fn formats_currencies() {
        assert_eq!(format!("{}", Currencies {
            keys: 2,
            metal: refined!(23) + scrap!(4),
        }), "2 keys, 23.44 ref");
    }
    
    #[test]
    fn formats_currencies_singular() {
        assert_eq!(format!("{}", Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        }), "1 key, 23.44 ref");
    }
    
    #[test]
    fn formats_currencies_with_no_trailing_decimal_places() {
        assert_eq!(&format!("{}", Currencies {
            keys: 2,
            metal: refined!(23),
        }), "2 keys, 23 ref");
    }
    
    #[test]
    fn formats_currencies_with_no_metal() {
        assert_eq!(&format!("{}", Currencies {
            keys: 2,
            metal: 0,
        }), "2 keys");
    }
    
    #[test]
    fn converts_to_metal() {
        let currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        };
        let key_price = 422;
        let value = currencies.to_metal(key_price);
        
        assert_eq!(value, 844);
    }
    
    #[test]
    fn rounds_metal_down() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4) + 1,
        };
        
        currencies.round(&Rounding::DownScrap);
        
        assert_eq!(currencies.metal, 422);
    }
    
    #[test]
    fn rounds_metal_down_refined() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        };
        
        currencies.round(&Rounding::DownRefined);
        
        assert_eq!(currencies.metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_up_refined_negative() {
        let mut currencies = Currencies {
            keys: 1,
            metal: -refined!(23) + scrap!(1),
        };
        
        currencies.round(&Rounding::UpRefined);
        
        assert_eq!(currencies.metal, -refined!(22));
    }
    
    #[test]
    fn rounds_metal_up_refined_negative_whole_value() {
        let mut currencies = Currencies {
            keys: 1,
            metal: -refined!(23),
        };
        
        currencies.round(&Rounding::UpRefined);
        
        assert_eq!(currencies.metal, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_negative() {
        let mut currencies = Currencies {
            keys: 1,
            metal: -refined!(23) + scrap!(8),
        };
        
        currencies.round(&Rounding::DownRefined);
        
        assert_eq!(currencies.metal, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_negative_whole_value() {
        let mut currencies = Currencies {
            keys: 1,
            metal: -refined!(23),
        };
        
        currencies.round(&Rounding::DownRefined);
        
        assert_eq!(currencies.metal, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_whole_value() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23),
        };
        
        currencies.round(&Rounding::DownRefined);
        
        assert_eq!(currencies.metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_up_refined() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        };
        
        currencies.round(&Rounding::UpRefined);
        
        assert_eq!(currencies.metal, refined!(24));
    }
    
    #[test]
    fn rounds_metal_up_refined_whole_value() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23),
        };
        
        currencies.round(&Rounding::UpRefined);
        
        assert_eq!(currencies.metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_down_correctly() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(3),
        };
        
        currencies.round(&Rounding::Refined);
        
        assert_eq!(currencies.metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_down_correctly_whole_value() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23),
        };
        
        currencies.round(&Rounding::Refined);
        
        assert_eq!(currencies.metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_up_correctly() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(5),
        };
        
        currencies.round(&Rounding::Refined);
        
        assert_eq!(currencies.metal, refined!(24));
    }
    
    #[test]
    fn rounds_metal_up() {
        let mut currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4) + 1,
        };
        
        currencies.round(&Rounding::UpScrap);
        
        assert_eq!(currencies.metal, 424);
    }
    
    #[test]
    fn correct_json_format() {
        let currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
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
    fn deserializes_currencies() {
        let currencies: Currencies = serde_json::from_str(r#"{"keys":1,"metal": 23.44}"#).unwrap();
        
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        }, currencies);
    }
    
    #[test]
    fn deserializes_currencies_with_no_keys() {
        let currencies: Currencies = serde_json::from_str(r#"{"metal": 23.44}"#).unwrap();
        
        assert_eq!(Currencies {
            keys: 0,
            metal: refined!(23) + scrap!(4),
        }, currencies);
    }
    
    #[test]
    fn deserializes_currencies_with_no_metal() {
        let currencies: Currencies = serde_json::from_str(r#"{"keys":5}"#).unwrap();
        
        assert_eq!(Currencies {
            keys: 5,
            metal: 0,
        }, currencies);
    }
    
    #[test]
    fn deserializes_currencies_with_weapon_value() {
        let currencies: Currencies = serde_json::from_str(r#"{"keys":1,"metal": 23.16}"#).unwrap();
        
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + 3,
        }, currencies);
    }
    
    #[test]
    fn serializes_currencies() {
        let currencies = Currencies {
            keys: 1,
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
    fn serializes_currencies_whole_numbers_have_no_decimals() {
        let currencies = Currencies {
            keys: 1,
            metal: refined!(23)
        };
        let currencies_json = serde_json::to_string(&currencies).unwrap();
        let actual: Value = serde_json::from_str(&currencies_json).unwrap();
        let expected: Value = json!({
            "keys": 1,
            "metal": 23
        });
        
        assert_json_eq!(
            actual,
            expected,
        );
    }
}