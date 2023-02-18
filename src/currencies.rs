use crate::helpers;
use crate::types::Currency;
use crate::traits::SerializeCurrencies;
use crate::error::{TryFromListingCurrenciesError, ParseError};
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL, EMPTY_SYMBOL};
use crate::{ListingCurrencies, Rounding};
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops::{self, AddAssign, SubAssign, MulAssign, DivAssign};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use serde::ser::SerializeStruct;

/// For storing item currencies values.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(remote = "Self")]
pub struct Currencies {
    /// Amount of keys.
    #[serde(default)]
    pub keys: Currency,
    /// Amount of metal expressed as weapons. A metal value of 6 would be equivalent to 3 scrap. 
    /// It's recommended to use the `ONE_REF`, `ONE_REC`, `ONE_SCRAP`, and `ONE_WEAPON` constants 
    /// to perform arithmatic.
    #[serde(deserialize_with = "helpers::metal_deserializer", default)]
    pub metal: Currency,
}

impl PartialOrd for Currencies {
    fn partial_cmp(&self, other: &Currencies) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for Currencies {
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

impl SerializeCurrencies for Currencies {}

impl Default for Currencies {
    fn default() -> Self {
        Self::new()
    }
}

impl Currencies {
    /// Creates a new [`Currencies`] with `0` keys and `0` metal.
    pub fn new() -> Self {
        Self {
            keys: 0,
            metal: 0,
        }
    }
    
    /// Converts a metal value into the appropriate number of keys using the given key price 
    /// (represented as weapons).
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price = refined!(60);
    /// let currencies = Currencies::from_metal(refined!(80), key_price);
    /// 
    /// assert_eq!(currencies, Currencies { keys: 1, metal: refined!(20) });
    /// ```
    pub fn from_metal(metal: Currency, key_price: Currency) -> Self {
        Self {
            // Will be 0 if metal is 30 and key price is 32 (rounds down)
            keys: metal / key_price,
            metal: metal % key_price,
        }
    }
    
    /// Converts from [`ListingCurrencies`] using the given key price (represented as weapons).
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, ListingCurrencies, refined};
    /// 
    /// let key_price = refined!(60);
    /// let listing_currencies = ListingCurrencies { keys: 1.5, metal: 0 };
    /// let currencies = Currencies::from_listing_currencies(listing_currencies, key_price);
    /// 
    /// assert_eq!(currencies.keys, 1);
    /// assert_eq!(currencies.metal, refined!(30));
    /// ```
    pub fn from_listing_currencies(
        currencies: ListingCurrencies,
        key_price: Currency,
    ) -> Self {
        let keys_metal = ((currencies.keys % 1.0) * key_price as f32).round() as Currency;
        
        Self {
            keys: currencies.keys as Currency,
            metal: keys_metal.saturating_add(currencies.metal),
        }
    }
    
    /// Converts an f32 key value into `Currencies` using the given key price represented as 
    /// weapons.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, ListingCurrencies, refined};
    /// 
    /// let key_price = refined!(60);
    /// let currencies = Currencies::from_keys_f32(1.5, key_price);
    /// 
    /// assert_eq!(currencies.keys, 1);
    /// assert_eq!(currencies.metal, refined!(30));
    /// ```
    pub fn from_keys_f32(keys: f32, key_price: Currency) -> Self {
        Self {
            keys: keys as Currency,
            metal: ((keys % 1.0) * key_price as f32) as Currency
        }
    }
    
    /// Converts currencies to a metal value using the given key price (represented as weapons).
    /// In cases where the result overflows or underflows beyond the limit for [`i64`], the max or 
    /// min i64 will be returned. In most cases values this high are not useful.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price = refined!(50);
    /// let currencies = Currencies { keys: 1, metal: refined!(10) };
    /// 
    /// assert_eq!(currencies.to_metal(key_price), refined!(60));
    /// ```
    pub fn to_metal(&self, key_price: Currency) -> Currency {
        helpers::to_metal(self.metal, self.keys, key_price)
    }
    
    /// Converts currencies to a metal value using the given key price (represented as weapons).
    /// In cases where the result overflows or underflows beyond the limit for [`i64`], `None` will
    /// be returned.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price = refined!(50);
    /// let currencies = Currencies { keys: i64::MAX, metal: refined!(10) };
    /// 
    /// assert!(currencies.checked_to_metal(key_price).is_none());
    /// ```
    pub fn checked_to_metal(&self, key_price: Currency) -> Option<Currency> {
        helpers::checked_to_metal(self.metal, self.keys, key_price)
    }
    
    /// Checks if the currencies do contain any value.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::Currencies;
    /// 
    /// assert!(Currencies { keys: 0, metal: 0 }.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keys == 0 && self.metal == 0
    }
    
    /// Rounds the metal value using the given rounding method.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, Rounding, refined, scrap};
    /// 
    /// let currencies = Currencies { keys: 0, metal: refined!(1) + scrap!(3) };
    /// 
    /// assert_eq!(currencies.round(&Rounding::Refined).metal, refined!(1));
    /// assert_eq!(currencies.round(&Rounding::UpRefined).metal, refined!(2));
    /// ```
    pub fn round(mut self, rounding: &Rounding) -> Self {
        self.metal = helpers::round_metal(self.metal, rounding);
        self
    }
    
    /// Neatens currencies. If the `metal` value is over `key_price`, the `metal` value will be 
    /// converted to `keys`, with the remainder remaining as `metal`. This method is saturating.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price = refined!(50);
    /// // The amount of metal is 10 refined over the key price.
    /// let currencies = Currencies { keys: 1, metal: refined!(60) }.neaten(key_price);
    /// 
    /// assert_eq!(currencies, Currencies { keys: 2,  metal: refined!(10) });
    /// ```
    pub fn neaten(&self, key_price: Currency) -> Self {
        Self::from_metal(self.to_metal(key_price), key_price)
    }
    
    /// Checks whether the currencies have enough keys and metal to afford the `other` currencies.
    /// This is simply `self.keys >= other.keys && self.metal >= other.metal`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let currencies = Currencies { keys: 100, metal: refined!(30) };
    /// 
    /// // We have at least 50 keys and 30 refined.
    /// assert!(currencies.can_afford(&Currencies { keys: 50, metal: refined!(30) }));
    /// // Not enough metal - we can't afford this.
    /// assert!(!currencies.can_afford(&Currencies { keys: 50, metal: refined!(100) }));
    /// ```
    pub fn can_afford(&self, other: &Self) -> bool {
        self.keys >= other.keys && self.metal >= other.metal
    }
    
    /// Checked integer multiplication. Computes `self * rhs` for each field, returning `None` if 
    /// overflow occurred
    pub fn checked_mul(&self, rhs: Currency) -> Option<Self> {
        let keys = self.keys.checked_mul(rhs)?;
        let metal = self.metal.checked_mul(rhs)?;
        
        Some(Self { keys, metal })
    }
    
    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0` or the 
    /// division results in overflow.
    pub fn checked_div(&self, rhs: Currency) -> Option<Self> {
        let keys = self.keys.checked_div(rhs)?;
        let metal = self.metal.checked_div(rhs)?;
        
        Some(Self { keys, metal })
    }
    
    /// Adds currencies. `None` if the result overflows integer bounds.
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let keys = self.keys.checked_add(other.keys)?;
        let metal = self.metal.checked_add(other.metal)?;
        
        Some(Self { keys, metal })
    }
    
    /// Subtracts currencies. `None` if the result overflows integer bounds.
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        let keys = self.keys.checked_sub(other.keys)?;
        let metal = self.metal.checked_sub(other.metal)?;
        
        Some(Self { keys, metal })
    }
}

/// Comparison with [`ListingCurrencies`] will fail if [`ListingCurrencies`] has a fractional key 
/// value.
impl PartialEq<ListingCurrencies> for Currencies {
    fn eq(&self, other: &ListingCurrencies) -> bool {
        !other.is_fract() &&
        self.keys == other.keys as Currency &&
        self.metal == other.metal
    }
}

impl_op_ex!(+ |a: &Currencies, b: &Currencies| -> Currencies { 
    Currencies {
        keys: a.keys.saturating_add(b.keys),
        metal: a.metal.saturating_add(b.metal),
    } 
});

impl_op_ex!(- |a: &Currencies, b: &Currencies| -> Currencies { 
    Currencies {
        keys: a.keys.saturating_sub(b.keys),
        metal: a.metal.saturating_sub(b.metal),
    }
});

impl_op_ex!(* |currencies: &Currencies, num: Currency| -> Currencies {
    Currencies {
        keys: currencies.keys.saturating_mul(num),
        metal: currencies.metal.saturating_mul(num),
    }
});

impl_op_ex!(/ |currencies: &Currencies, num: Currency| -> Currencies {
    Currencies {
        keys: currencies.keys.saturating_div(num),
        metal: currencies.metal.saturating_div(num),
    }
});

impl_op_ex!(* |currencies: &Currencies, num: f32| -> Currencies {
    Currencies { 
        keys: (currencies.keys as f32 * num).round() as Currency,
        metal: (currencies.metal as f32 * num).round() as Currency,
    }
});

impl_op_ex!(/ |currencies: &Currencies, num: f32| -> Currencies {
    Currencies {
        keys: (currencies.keys as f32 / num).round() as Currency,
        metal: (currencies.metal as f32 / num).round() as Currency,
    }
});

impl AddAssign<Currencies> for Currencies {
    fn add_assign(&mut self, other: Self) {
        self.keys = self.keys.saturating_add(other.keys);
        self.metal = self.metal.saturating_add(other.metal);
    }
}

impl AddAssign<&Currencies> for Currencies {
    fn add_assign(&mut self, other: &Self) {
        self.keys = self.keys.saturating_add(other.keys);
        self.metal = self.metal.saturating_add(other.metal);
    }
}

impl SubAssign<Currencies> for Currencies {
    fn sub_assign(&mut self, other: Self) {
        self.keys = self.keys.saturating_sub(other.keys);
        self.metal = self.metal.saturating_sub(other.metal);
    }
}

impl SubAssign<&Currencies> for Currencies {
    fn sub_assign(&mut self, other: &Self) {
        self.keys = self.keys.saturating_sub(other.keys);
        self.metal = self.metal.saturating_sub(other.metal);
    }
}

impl MulAssign<Currency> for Currencies {
    fn mul_assign(&mut self, other: Currency) {
        self.keys = self.keys.saturating_mul(other);
        self.metal = self.metal.saturating_mul(other);
    }
}

impl MulAssign<f32> for Currencies {
    fn mul_assign(&mut self, other: f32) {
        self.keys = (self.keys as f32 * other).round() as Currency;
        self.metal = (self.metal as f32 * other).round() as Currency;
    }
}

impl DivAssign<Currency> for Currencies {
    fn div_assign(&mut self, other: Currency) {
        self.keys = self.keys.saturating_div(other);
        self.metal = self.metal.saturating_div(other);
    }
}

impl DivAssign<f32> for Currencies {
    fn div_assign(&mut self, other: f32) {
        self.keys = (self.keys as f32 / other).round() as Currency;
        self.metal = (self.metal as f32 / other).round() as Currency;
    }
}

impl<'a> TryFrom<&'a str> for Currencies {
    type Error = ParseError;
    
    fn try_from(string: &'a str) -> Result<Self, Self::Error>  {
        let (keys, metal) = helpers::parse_from_string::<Currency>(string)?;
        
        Ok(Currencies {
            keys,
            metal,
        })
    }
}

/// Results in error if [`ListingCurrencies`] contains a fractional key value.
impl TryFrom<ListingCurrencies> for Currencies {
    type Error = TryFromListingCurrenciesError;
    
    fn try_from(currencies: ListingCurrencies) -> Result<Self, Self::Error> {
        if currencies.is_fract() {
            return Err(TryFromListingCurrenciesError { fract: currencies.keys.fract() });
        }
        
        Ok(Currencies {
            keys: currencies.keys as Currency,
            metal: currencies.metal,
        })
    }
}

/// Results in error if [`ListingCurrencies`] contains a fractional key value.
impl TryFrom<&ListingCurrencies> for Currencies {
    type Error = &'static str;
    
    fn try_from(currencies: &ListingCurrencies) -> Result<Self, Self::Error> {
        if currencies.is_fract() {
            return Err("Currencies contain fractional key value");
        }
        
        Ok(Currencies {
            keys: currencies.keys as Currency,
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
    fn currencies_multiplied_by_metal() {
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
    fn currencies_divided_by_metal() {
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
    fn currencies_mul_assign_metal() {
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
    fn currencies_div_assign_metal() {
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
        assert!(Currencies::try_from("what").is_err());
    }
    
    #[test]
    fn parses_currencies_from_string_invalid_currencies_extra() {
        assert!(Currencies::try_from("2 keys, 3 what").is_err());
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
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4) + 1,
        }.round(&Rounding::DownScrap).metal, 422);
    }
    
    #[test]
    fn rounds_metal_down_refined() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        }.round(&Rounding::DownRefined).metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_up_refined_negative() {
        assert_eq!(Currencies {
            keys: 1,
            metal: -refined!(23) + scrap!(1),
        }.round(&Rounding::UpRefined).metal, -refined!(22));
    }
    
    #[test]
    fn rounds_metal_up_refined_negative_whole_value() {
        assert_eq!(Currencies {
            keys: 1,
            metal: -refined!(23),
        }.round(&Rounding::UpRefined).metal, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_negative() {
        assert_eq!(Currencies {
            keys: 1,
            metal: -refined!(23) + scrap!(8),
        }.round(&Rounding::DownRefined).metal, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_negative_whole_value() {
        assert_eq!(Currencies {
            keys: 1,
            metal: -refined!(23),
        }.round(&Rounding::DownRefined).metal, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_whole_value() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23),
        }.round(&Rounding::DownRefined).metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_up_refined() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        }.round(&Rounding::UpRefined).metal, refined!(24));
    }
    
    #[test]
    fn rounds_metal_up_refined_whole_value() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23),
        }.round(&Rounding::UpRefined).metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_down_correctly() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(3),
        }.round(&Rounding::Refined).metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_down_correctly_whole_value() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23),
        }.round(&Rounding::Refined).metal, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_up_correctly() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(5),
        }.round(&Rounding::Refined).metal, refined!(24));
    }
    
    #[test]
    fn rounds_metal_up() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4) + 1,
        }.round(&Rounding::UpScrap).metal, 424);
    }
    
    #[test]
    fn neatens() {
        assert_eq!(Currencies {
            keys: 1,
            metal: refined!(110),
        }.neaten(refined!(50)), Currencies {
            keys: 3,
            metal: refined!(10),
        });
    }
    
    #[test]
    fn neatens_negative() {
        assert_eq!(Currencies {
            keys: 1,
            metal: -refined!(110),
        }.neaten(refined!(50)), Currencies {
            keys: -1,
            metal: -refined!(10),
        });
    }
    
    #[test]
    fn neatens_negative_result_should_be_positive() {
        assert_eq!(Currencies {
            keys: 2,
            metal: -refined!(60),
        }.neaten(refined!(50)), Currencies {
            keys: 0,
            metal: refined!(40),
        });
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
    
    #[test]
    fn accepts_trait_currencies() {
        fn get_keys<T>(currencies: &T) -> String
        where T: SerializeCurrencies {
            serde_json::to_string(currencies).unwrap()
        }
        
        let currencies = Currencies { keys: 1, metal: 1 };
        let serialized = get_keys(&currencies);
        
        assert_eq!(serialized, r#"{"keys":1,"metal":0.05}"#);
    }
    
    #[test]
    fn greater_than() {
        assert!(Currencies { keys: 1, metal: 5 } > Currencies { keys: 0, metal: 10});
    }
    
    #[test]
    fn less_than() {
        assert!(Currencies { keys: 0, metal: 1 } < Currencies { keys: 0, metal: 4});
    }
    
    #[test]
    fn sorts() {
        let mut currencies = vec![
            Currencies { keys: 2, metal: 4},
            Currencies { keys: 0, metal: 2},
            Currencies { keys: 10, metal: 4},
        ];
        
        // lowest to highest
        currencies.sort();
        
        assert_eq!(*currencies.iter().rev().next().unwrap(), Currencies { keys: 10, metal: 4});
    }
    
    #[test]
    fn to_metal_saturating_integer_bounds() {
        let key_price = refined!(50);
        
        assert_eq!(Currencies { keys: Currency::MAX - 100, metal: 0 }.to_metal(key_price), Currency::MAX);
        assert_eq!(Currencies { keys: Currency::MAX - 100, metal: 0 }.to_metal(-key_price), Currency::MIN);
        assert_eq!(Currencies { keys: 1, metal: Currency::MAX }.to_metal(key_price), Currency::MAX);
        assert_eq!(Currencies { keys: -1, metal: Currency::MIN }.to_metal(key_price), Currency::MIN);
        assert_eq!(Currencies { keys: 1, metal: Currency::MIN }.to_metal(key_price), Currency::MIN + key_price);
    }
    
    #[test]
    fn checked_mul() {
        assert_eq!(Currencies { keys: 2, metal: 0 }.checked_mul(Currency::MAX), None);
    }
    
    #[test]
    fn checked_add() {
        assert_eq!(
            Currencies { keys: 2, metal: 0 }.checked_add(&Currencies { keys: Currency::MAX, metal: 0 }),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal() {
        assert_eq!(
            Currencies { keys: Currency::MAX, metal: 0 }.checked_to_metal(Currency::MAX),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal_correct_value() {
        assert_eq!(
            Currencies { keys: 10, metal: 5 }.checked_to_metal(10),
            Some(105),
        );
    }
}