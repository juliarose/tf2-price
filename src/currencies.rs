use crate::helpers;
use crate::types::Currency;
use crate::error::{ParseError, TryFromFloatCurrenciesError};
use crate::constants::{KEYS_SYMBOL, KEY_SYMBOL, METAL_SYMBOL};
use crate::{FloatCurrencies, Rounding};
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use auto_ops::impl_op_ex;

/// For storing item currencies values.
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(remote = "Self"))]
pub struct Currencies {
    /// Amount of keys.
    #[cfg_attr(feature = "serde", serde(default))]
    pub keys: Currency,
    /// Amount of metal expressed as weapons. It's recommended to use the `ONE_REF`, `ONE_REC`, 
    /// `ONE_SCRAP`, and `ONE_WEAPON` constants to perform arithmatic.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(rename = "metal"))]
    #[cfg_attr(feature = "serde", serde(deserialize_with = "helpers::metal_deserializer"))]
    pub weapons: Currency,
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
        } else if self.weapons > other.weapons {
            Ordering::Greater
        } else if self.weapons < other.weapons {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Currencies {
    /// Creates a new [`Currencies`] with `0` keys and `0` weapons. Same as `Currencies::default()`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::Currencies;
    /// 
    /// let currencies = Currencies::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Converts a weapon value into the appropriate number of keys and weapons using the given 
    /// key price (represented as weapons).
    /// 
    /// This method is [saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic).
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price = refined!(60);
    /// let currencies = Currencies::from_weapons(refined!(80), key_price);
    /// 
    /// assert_eq!(currencies, Currencies { keys: 1, weapons: refined!(20) });
    /// ```
    pub fn from_weapons(
        weapons: Currency,
        key_price_weapons: Currency,
    ) -> Self {
        Self {
            // Will be 0 if weapons is 30 and key price is 32 (rounds down)
            keys: weapons.saturating_div(key_price_weapons),
            weapons: weapons % key_price_weapons,
        }
    }
    
    /// Converts from [`FloatCurrencies`] using the given key price (represented as weapons).
    /// 
    /// This method is [saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic).
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, FloatCurrencies, refined};
    /// 
    /// let key_price_weapons = refined!(60);
    /// let float_currencies = FloatCurrencies { keys: 1.5, metal: 0.0 };
    /// let currencies = Currencies::from_float_currencies_with(
    ///     float_currencies,
    ///     key_price_weapons,
    /// );
    /// 
    /// assert_eq!(currencies.keys, 1);
    /// assert_eq!(currencies.weapons, refined!(30));
    /// ```
    pub fn from_float_currencies_with(
        currencies: FloatCurrencies,
        key_price_weapons: Currency,
    ) -> Self {
        let keys_weapons = (
            (currencies.keys.fract()) * key_price_weapons as f32
        ).round() as Currency;
        let weapons = helpers::get_weapons_from_metal_float(currencies.metal);
        
        Self {
            keys: currencies.keys as Currency,
            weapons: weapons.saturating_add(keys_weapons),
        }
    }
    
    /// Converts from [`FloatCurrencies`] using the given key price (represented as weapons).
    /// 
    /// Checks for safe conversion.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, FloatCurrencies, Currency, refined};
    /// 
    /// let key_price_weapons = refined!(60);
    /// let float_currencies = FloatCurrencies {
    ///     keys: 1.5,
    ///     metal: 0.0,
    /// };
    /// let currencies = Currencies::try_from_float_currencies_with(
    ///     float_currencies,
    ///     key_price_weapons,
    /// ).unwrap();
    /// 
    /// assert_eq!(currencies.keys, 1);
    /// assert_eq!(currencies.weapons, refined!(30));
    /// 
    /// let float_currencies = FloatCurrencies {
    ///     keys: Currency::MAX as f32 * 2.0,
    ///     metal: 0.0,
    /// };
    /// let currencies = Currencies::try_from_float_currencies_with(
    ///     float_currencies,
    ///     key_price_weapons,
    /// );
    /// 
    /// assert!(currencies.is_none());
    /// ```
    pub fn try_from_float_currencies_with(
        currencies: FloatCurrencies,
        key_price_weapons: Currency,
    ) -> Option<Self> {
        // Convert the integer part of the keys value.
        // Using trunc() is OK here in the event that keys is Infinity or NaN, the output will be 
        // the same value.
        let keys = helpers::strict_f32_to_currency(currencies.keys.trunc())?;
        // Take the remainder of the keys value.
        let keys_weapons_float = (currencies.keys.fract() * key_price_weapons as f32).round();
        let keys_weapons = helpers::strict_f32_to_currency(keys_weapons_float)?;
        // Convert the metal value to weapon, add the weapons from the remainder.
        let weapons = helpers::checked_get_weapons_from_metal_float(currencies.metal)?.checked_add(keys_weapons)?;
        
        Some(Self {
            keys,
            weapons,
        })
    }
    
    /// Converts an f32 key value into `Currencies` using the given key price (represented as 
    /// weapons).
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, FloatCurrencies, refined};
    /// 
    /// let key_price = refined!(60);
    /// let currencies = Currencies::from_keys_f32(1.5, key_price);
    /// 
    /// assert_eq!(currencies.keys, 1);
    /// assert_eq!(currencies.weapons, refined!(30));
    /// ```
    pub fn from_keys_f32(
        keys: f32,
        key_price_weapons: Currency,
    ) -> Self {
        Self {
            keys: keys as Currency,
            weapons: ((keys.fract()) * key_price_weapons as f32) as Currency
        }
    }
    
    /// Converts currencies to a weapon value using the given key price (represented as weapons).
    /// 
    /// This method is [saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic).
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price = refined!(50);
    /// let currencies = Currencies {
    ///     keys: 1,
    ///     weapons: refined!(10),
    /// };
    /// 
    /// assert_eq!(currencies.to_weapons(key_price), refined!(60));
    /// ```
    pub fn to_weapons(&self, key_price: Currency) -> Currency {
        helpers::to_metal(self.weapons, self.keys, key_price)
    }
    
    /// Converts currencies to a weapon value using the given key price (represented as weapons).
    /// In cases where the result overflows or underflows beyond the limit for [`Currency`], 
    /// `None` will be returned.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, Currency, refined};
    /// 
    /// let key_price_weapons = refined!(50);
    /// let currencies = Currencies {
    ///     keys: Currency::MAX,
    ///     weapons: refined!(10),
    /// };
    /// 
    /// assert!(currencies.checked_to_weapons(key_price_weapons).is_none());
    /// ```
    pub fn checked_to_weapons(&self, key_price: Currency) -> Option<Currency> {
        helpers::checked_to_metal(self.weapons, self.keys, key_price)
    }
    
    /// Checks if the currencies do contain any value.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::Currencies;
    /// 
    /// assert!(Currencies {
    ///     keys: 0,
    ///     weapons: 0,
    /// }.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keys == 0 && self.weapons == 0
    }
    
    /// Rounds the weapon value using the given rounding method. Returns a new `Currencies` 
    /// rather than mutating the original in-place.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, Rounding, refined, scrap};
    /// 
    /// let currencies = Currencies {
    ///     keys: 0,
    ///     weapons: refined!(1) + scrap!(3),
    /// };
    /// 
    /// assert_eq!(currencies.round(&Rounding::Refined).weapons, refined!(1));
    /// assert_eq!(currencies.round(&Rounding::UpRefined).weapons, refined!(2));
    /// ```
    pub fn round(mut self, rounding: &Rounding) -> Self {
        self.weapons = helpers::round_metal(self.weapons, rounding);
        self
    }
    
    /// Neatens currencies. If the `weapons` value is over `key_price_weapons`, the `weapons` 
    /// value will be converted to `keys`, with the remainder remaining as `weapons`.
    /// 
    /// This method is [saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic).
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let key_price_weapons = refined!(50);
    /// let currencies = Currencies {
    ///     keys: 1,
    ///     weapons: refined!(60),
    /// }.neaten(key_price_weapons);
    /// 
    /// assert_eq!(
    ///     currencies,
    ///     Currencies {
    ///         keys: 2,
    ///         weapons: refined!(10),
    ///     },
    /// );
    /// ```
    pub fn neaten(&self, key_price_weapons: Currency) -> Self {
        Self::from_weapons(self.to_weapons(key_price_weapons), key_price_weapons)
    }
    
    /// Checks whether the currencies have enough `keys` and `weapons` to afford the `other` 
    /// currencies. This is simply `self.keys >= other.keys && self.weapons >= other.weapons`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, refined};
    /// 
    /// let currencies = Currencies {
    ///     keys: 100,
    ///     weapons: refined!(30),
    /// };
    /// 
    /// // We have at least 50 keys and 30 refined.
    /// assert!(currencies.can_afford(&Currencies {
    ///     keys: 50,
    ///     weapons: refined!(30),
    /// }));
    /// // Not enough metal - we can't afford this.
    /// assert!(!currencies.can_afford(&Currencies {
    ///     keys: 50,
    ///     weapons: refined!(100)
    /// }));
    /// ```
    pub fn can_afford(&self, other: &Self) -> bool {
        self.keys >= other.keys && self.weapons >= other.weapons
    }
    
    /// Checked integer multiplication. Computes `self * rhs` for each field, returning `None` if 
    /// overflow occurred.
    /// 
    /// # Examples
    /// ```
    /// use tf2_price::{Currencies, Currency};
    /// 
    /// let currencies = Currencies {
    ///     keys: Currency::MAX,
    ///     weapons: 0,
    /// };
    /// 
    /// // Overflows, returns None.
    /// assert!(currencies.checked_mul(5).is_none());
    /// ```
    pub fn checked_mul(&self, rhs: Currency) -> Option<Self> {
        let keys = self.keys.checked_mul(rhs)?;
        let weapons = self.weapons.checked_mul(rhs)?;
        
        Some(Self { keys, weapons })
    }
    
    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0` or the 
    /// division results in overflow.
    pub fn checked_div(&self, rhs: Currency) -> Option<Self> {
        let keys = self.keys.checked_div(rhs)?;
        let weapons = self.weapons.checked_div(rhs)?;
        
        Some(Self { keys, weapons })
    }
    
    /// Adds currencies. `None` if the result overflows integer bounds.
    pub fn checked_add(&self, other: Self) -> Option<Self> {
        let keys = self.keys.checked_add(other.keys)?;
        let weapons = self.weapons.checked_add(other.weapons)?;
        
        Some(Self { keys, weapons })
    }
    
    /// Subtracts currencies. `None` if the result overflows integer bounds.
    pub fn checked_sub(&self, other: Self) -> Option<Self> {
        let keys = self.keys.checked_sub(other.keys)?;
        let weapons = self.weapons.checked_sub(other.weapons)?;
        
        Some(Self { keys, weapons })
    }
}

/// Comparison with [`FloatCurrencies`] will fail if [`FloatCurrencies`] has a fractional key 
/// value.
impl PartialEq<FloatCurrencies> for Currencies {
    fn eq(&self, other: &FloatCurrencies) -> bool {
        if let Some(weapons) = helpers::checked_get_weapons_from_metal_float(other.metal) {
            !other.is_fract() &&
            self.keys == other.keys as Currency &&
            self.weapons == weapons
        } else {
            false
        }
    }
}

impl_op_ex!(+ |a: &Currencies, b: &Currencies| -> Currencies { 
    Currencies {
        keys: a.keys.saturating_add(b.keys),
        weapons: a.weapons.saturating_add(b.weapons),
    } 
});

impl_op_ex!(- |a: &Currencies, b: &Currencies| -> Currencies { 
    Currencies {
        keys: a.keys.saturating_sub(b.keys),
        weapons: a.weapons.saturating_sub(b.weapons),
    }
});

impl_op_ex!(* |currencies: &Currencies, num: Currency| -> Currencies {
    Currencies {
        keys: currencies.keys.saturating_mul(num),
        weapons: currencies.weapons.saturating_mul(num),
    }
});

impl_op_ex!(/ |currencies: &Currencies, num: Currency| -> Currencies {
    Currencies {
        keys: currencies.keys.saturating_div(num),
        weapons: currencies.weapons.saturating_div(num),
    }
});

impl_op_ex!(* |currencies: &Currencies, num: f32| -> Currencies {
    Currencies { 
        keys: (currencies.keys as f32 * num).round() as Currency,
        weapons: (currencies.weapons as f32 * num).round() as Currency,
    }
});

impl_op_ex!(/ |currencies: &Currencies, num: f32| -> Currencies {
    Currencies {
        keys: (currencies.keys as f32 / num).round() as Currency,
        weapons: (currencies.weapons as f32 / num).round() as Currency,
    }
});

impl AddAssign<Currencies> for Currencies {
    fn add_assign(&mut self, other: Self) {
        self.keys = self.keys.saturating_add(other.keys);
        self.weapons = self.weapons.saturating_add(other.weapons);
    }
}

impl AddAssign<&Currencies> for Currencies {
    fn add_assign(&mut self, other: &Self) {
        self.keys = self.keys.saturating_add(other.keys);
        self.weapons = self.weapons.saturating_add(other.weapons);
    }
}

impl SubAssign<Currencies> for Currencies {
    fn sub_assign(&mut self, other: Self) {
        self.keys = self.keys.saturating_sub(other.keys);
        self.weapons = self.weapons.saturating_sub(other.weapons);
    }
}

impl SubAssign<&Currencies> for Currencies {
    fn sub_assign(&mut self, other: &Self) {
        self.keys = self.keys.saturating_sub(other.keys);
        self.weapons = self.weapons.saturating_sub(other.weapons);
    }
}

impl MulAssign<Currency> for Currencies {
    fn mul_assign(&mut self, other: Currency) {
        self.keys = self.keys.saturating_mul(other);
        self.weapons = self.weapons.saturating_mul(other);
    }
}

impl MulAssign<f32> for Currencies {
    fn mul_assign(&mut self, other: f32) {
        self.keys = (self.keys as f32 * other).round() as Currency;
        self.weapons = (self.weapons as f32 * other).round() as Currency;
    }
}

impl DivAssign<Currency> for Currencies {
    fn div_assign(&mut self, other: Currency) {
        self.keys = self.keys.saturating_div(other);
        self.weapons = self.weapons.saturating_div(other);
    }
}

impl DivAssign<f32> for Currencies {
    fn div_assign(&mut self, other: f32) {
        self.keys = (self.keys as f32 / other).round() as Currency;
        self.weapons = (self.weapons as f32 / other).round() as Currency;
    }
}

impl TryFrom<&str> for Currencies {
    type Error = ParseError;
    
    fn try_from(string: &str) -> Result<Self, Self::Error>  {
        string.parse::<Self>()
    }
}

impl TryFrom<&String> for Currencies {
    type Error = ParseError;
    
    fn try_from(string: &String) -> Result<Self, Self::Error> {
        string.parse::<Self>()
    }
}

impl TryFrom<String> for Currencies {
    type Error = ParseError;
    
    fn try_from(string: String) -> Result<Self, Self::Error> {
        string.parse::<Self>()
    }
}

impl std::str::FromStr for Currencies {
    type Err = ParseError;
    
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (
            keys,
            weapons,
        ) = helpers::parse_currency_from_string(string)?;
        
        Ok(Self {
            keys,
            weapons,
        })
    }
}

/// Results in error if [`FloatCurrencies`] contains a fractional key value.
impl TryFrom<FloatCurrencies> for Currencies {
    type Error = TryFromFloatCurrenciesError;
    
    fn try_from(currencies: FloatCurrencies) -> Result<Self, Self::Error> {
        if currencies.keys.fract() != 0.0 {
            return Err(TryFromFloatCurrenciesError::Fractional {
                fract: currencies.keys.fract(),
            });
        }
        
        let keys = helpers::strict_f32_to_currency(currencies.keys)
            .ok_or(TryFromFloatCurrenciesError::OutOfBounds {
                value: currencies.keys,
            })?;
        let weapons = helpers::checked_get_weapons_from_metal_float(currencies.metal)
            .ok_or(TryFromFloatCurrenciesError::OutOfBounds {
                value: currencies.metal,
            })?;
        
        Ok(Self {
            keys,
            weapons,
        })
    }
}

/// Results in error if [`FloatCurrencies`] contains a fractional key value.
impl TryFrom<&FloatCurrencies> for Currencies {
    type Error = TryFromFloatCurrenciesError;
    
    fn try_from(currencies: &FloatCurrencies) -> Result<Self, Self::Error> {
        Self::try_from(*currencies)
    }
}

impl fmt::Display for Currencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Either both keys and metal are non-zero or both are zero.
        if (self.keys != 0 && self.weapons != 0) || self.is_empty() {
            write!(
                f,
                "{} {}, {} {}",
                self.keys,
                helpers::pluralize(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
                helpers::get_metal_float_from_weapons(self.weapons),
                METAL_SYMBOL,
            )
        } else if self.keys != 0 {
            write!(
                f,
                "{} {}",
                self.keys,
                helpers::pluralize(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
            )
        } else {
            // It can be assumed that metal is not zero.
            write!(
                f,
                "{} {}",
                helpers::get_metal_float_from_weapons(self.weapons),
                METAL_SYMBOL,
            )
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Currencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        
        let currencies = Self::deserialize(deserializer)?;
        
        if currencies.keys == 0 && currencies.weapons == 0 {
            return Err(D::Error::custom("Does not contain values for keys or metal"));
        }
        
        Ok(currencies)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Currencies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        
        let mut currencies = serializer.serialize_struct("Currencies", 2)?;
        
        if self.keys == 0 {
            currencies.skip_field("keys")?;
        } else {
            currencies.serialize_field("keys", &self.keys)?;
        }
        
        if self.weapons == 0 {
            currencies.skip_field("metal")?;
        } else {
            let float = helpers::get_metal_float_from_weapons(self.weapons);
            
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

    #[test]
    fn currencies_equal() {
        assert_eq!(
            Currencies {
                keys: 2,
                weapons: refined!(23) + scrap!(4),
            },
            Currencies {
                keys: 2,
                weapons: refined!(23) + scrap!(4),
            },
        );
    }
    
    #[test]
    fn currencies_not_equal() {
        assert_ne!(
            Currencies {
                keys: 2,
                weapons: refined!(23) + scrap!(4),
            },
            Currencies {
                keys: 2,
                weapons: refined!(23),
            },
        );
    }
    
    #[test]
    fn currencies_added() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: refined!(10),
            } + Currencies {
                keys: 5,
                weapons: refined!(5),
            },
            Currencies {
                keys: 15,
                weapons: refined!(15),
            },
        );
    }
    
    #[test]
    fn currencies_subtracted() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: refined!(10),
            } - Currencies {
                keys: 5,
                weapons: refined!(5),
            },
            Currencies {
                keys: 5,
                weapons: refined!(5),
            },
        );
    }
    
    #[test]
    fn currencies_multiplied_by_metal() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: refined!(10),
            } * 5,
            Currencies {
                keys: 50,
                weapons: refined!(50),
            },
        );
    }
    
    #[test]
    fn currencies_divided_by_f32() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: refined!(10),
            } / 2.5,
            Currencies {
                keys: 4,
                weapons: refined!(4),
            },
        );
    }
    
    #[test]
    fn currencies_divided_by_metal() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: refined!(10),
            } / 5,
            Currencies {
                keys: 2,
                weapons: refined!(2),
            },
        );
    }
    
    #[test]
    fn currencies_multiplied_by_f32() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: refined!(10),
            } * 2.5,
            Currencies {
                keys: 25,
                weapons: refined!(25),
            },
        );
    }
    
    #[test]
    fn currencies_mul_assign_metal() {
        let mut currencies = Currencies {
            keys: 10,
            weapons: refined!(10),
        };
        
        currencies *= 2;
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 20,
                weapons: refined!(20),
            },
        );
    }
    
    #[test]
    fn currencies_mul_assign_f32() {
        let mut currencies = Currencies {
            keys: 10,
            weapons: refined!(10),
        };
        
        currencies *= 2.5;
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 25,
                weapons: refined!(25),
            },
        );
    }
    
    #[test]
    fn currencies_div_assign_metal() {
        let mut currencies = Currencies {
            keys: 10,
            weapons: refined!(10),
        };
        
        currencies /= 2;
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 5,
                weapons: refined!(5),
            },
        );
    }
    
    #[test]
    fn currencies_div_assign_f32() {
        let mut currencies = Currencies {
            keys: 10,
            weapons: refined!(10),
        };
        
        currencies /= 2.5;
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 4,
                weapons: refined!(4),
            },
        );
    }
    
    #[test]
    fn parses_currencies_from_string() {
        let currencies = Currencies::try_from("2 keys, 23.44 ref").unwrap();
        
        assert_eq!(currencies.keys, 2);
        assert_eq!(currencies.weapons, 422);
    }
    
    #[test]
    fn parses_currencies_from_string_only_keys() {
        let currencies = Currencies::try_from("1 key").unwrap();
        
        assert_eq!(currencies.keys, 1);
        assert_eq!(currencies.weapons, 0);
    }
    
    #[test]
    fn parses_currencies_from_string_only_metal() {
        let currencies = Currencies::try_from("2 ref").unwrap();
        
        assert_eq!(currencies.keys, 0);
        assert_eq!(currencies.weapons, refined!(2));
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
    fn prints_empty_currencies() {
        assert_eq!(Currencies::default().to_string(), "0 keys, 0 ref");
    }
    
    #[test]
    fn gets_correct_value_from_metal() {
        assert_eq!(
            Currencies::from_weapons(9, 10),
            Currencies {
                keys: 0,
                weapons: 9,
            },
        );
    }
    
    #[test]
    fn gets_correct_value_from_metal_with_keys() {
        assert_eq!(
            Currencies::from_weapons(10, 10),
            Currencies {
                keys: 1,
                weapons: 0,
            },
        );
    }
    
    #[test]
    fn gets_correct_value_from_metal_with_keys_and_metal() {
        assert_eq!(
            Currencies::from_weapons(11, 10),
            Currencies {
                keys: 1,
                weapons: 1,
            },
        );
    }
    
    #[test]
    fn gets_correct_value_from_keys_f32() {
        assert_eq!(
            Currencies::from_keys_f32(1.5, 10),
            Currencies {
                keys: 1,
                weapons: 5,
            },
        );
    }
    
    #[test]
    fn formats_currencies() {
        let currencies = Currencies {
            keys: 2,
            weapons: refined!(23) + scrap!(4),
        };
        
        assert_eq!(format!("{currencies}"), "2 keys, 23.44 ref");
    }
    
    #[test]
    fn formats_currencies_singular() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4),
        };
        
        assert_eq!(format!("{currencies}"), "1 key, 23.44 ref");
    }
    
    #[test]
    fn formats_currencies_with_no_trailing_decimal_places() {
        let currencies = Currencies {
            keys: 2,
            weapons: refined!(23),
        };
        
        assert_eq!(format!("{currencies}"), "2 keys, 23 ref");
    }
    
    #[test]
    fn formats_currencies_with_no_metal() {
        let currencies = Currencies {
            keys: 2,
            weapons: 0,
        };
        
        assert_eq!(format!("{currencies}"), "2 keys");
    }
    
    #[test]
    fn converts_to_metal() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4),
        };
        let value = currencies.to_weapons(422);
        
        assert_eq!(value, 844);
    }
    
    #[test]
    fn rounds_metal_down() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4) + 1,
        };
        
        assert_eq!(currencies.round(&Rounding::DownScrap).weapons, 422);
    }
    
    #[test]
    fn rounds_metal_down_refined() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4),
        };
        
        assert_eq!(currencies.round(&Rounding::DownRefined).weapons, refined!(23));
    }
    
    #[test]
    fn rounds_metal_up_refined_negative() {
        let currencies = Currencies {
            keys: 1,
            weapons: -refined!(23) + scrap!(1),
        };
        
        assert_eq!(currencies.round(&Rounding::UpRefined).weapons, -refined!(22));
    }
    
    #[test]
    fn rounds_metal_up_refined_negative_whole_value() {
        let currencies = Currencies {
            keys: 1,
            weapons: -refined!(23),
        };
        
        assert_eq!(currencies.round(&Rounding::UpRefined).weapons, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_negative() {
        let currencies = Currencies {
            keys: 1,
            weapons: -refined!(23) + scrap!(8),
        };
        
        assert_eq!(currencies.round(&Rounding::DownRefined).weapons, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_negative_whole_value() {
        let currencies = Currencies {
            keys: 1,
            weapons: -refined!(23),
        };
        
        assert_eq!(currencies.round(&Rounding::DownRefined).weapons, -refined!(23));
    }
    
    #[test]
    fn rounds_metal_down_refined_whole_value() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23),
        };
        
        assert_eq!(currencies.round(&Rounding::DownRefined).weapons, refined!(23));
    }
    
    #[test]
    fn rounds_metal_up_refined() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4),
        };
        
        assert_eq!(currencies.round(&Rounding::UpRefined).weapons, refined!(24));
    }
    
    #[test]
    fn rounds_metal_up_refined_whole_value() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23),
        };
        
        assert_eq!(currencies.round(&Rounding::UpRefined).weapons, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_down_correctly() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(3),
        };
        
        assert_eq!(currencies.round(&Rounding::Refined).weapons, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_down_correctly_whole_value() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23),
        };
        
        assert_eq!(currencies.round(&Rounding::Refined).weapons, refined!(23));
    }
    
    #[test]
    fn rounds_metal_refined_up_correctly() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(5),
        };
        
        assert_eq!(currencies.round(&Rounding::Refined).weapons, refined!(24));
    }
    
    #[test]
    fn rounds_metal_up() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4) + 1,
        };
        
        assert_eq!(currencies.round(&Rounding::UpScrap).weapons, 424);
    }
    
    #[test]
    fn neatens() {
        let currenices = Currencies {
            keys: 1,
            weapons: refined!(110),
        };
        
        assert_eq!(
            currenices.neaten(refined!(50)),
            Currencies {
                keys: 3,
                weapons: refined!(10),
            },
        );
    }
    
    #[test]
    fn neatens_negative() {
        let currencies = Currencies {
            keys: 1,
            weapons: -refined!(110),
        };
        
        assert_eq!(
            currencies.neaten(refined!(50)),
            Currencies {
                keys: -1,
                weapons: -refined!(10),
            },
        );
    }
    
    #[test]
    fn neatens_negative_result_should_be_positive() {
        let currencies = Currencies {
            keys: 2,
            weapons: -refined!(60),
        };
        
        assert_eq!(
            currencies.neaten(refined!(50)),
            Currencies {
                keys: 0,
                weapons: refined!(40),
            },
        );
    }
    
    #[test]
    fn to_metal_with_negative_keys() {
        let key_price_weapons = refined!(10);
        let currencies = Currencies {
            keys: -10,
            // 2 keys of metal, so the total should be -8 keys
            weapons: key_price_weapons * 2,
        };
        
        assert_eq!(currencies.to_weapons(key_price_weapons), -(key_price_weapons * 8));
    }
    
    #[test]
    fn greater_than() {
        let a = Currencies { keys: 1, weapons: 5 };
        let b = Currencies { keys: 0, weapons: 10 };
        
        assert!(a > b);
    }
    
    #[test]
    fn less_than() {
        let a = Currencies { keys: 0, weapons: 1 };
        let b = Currencies { keys: 0, weapons: 4 };
        
        assert!(a < b);
    }
    
    #[test]
    fn sorts() {
        let mut currencies = vec![
            Currencies { keys: 2, weapons: 4 },
            Currencies { keys: 0, weapons: 2 },
            Currencies { keys: 10, weapons: 4 },
        ];
        
        // lowest to highest
        currencies.sort();
        
        assert_eq!(
            *currencies.iter().rev().next().unwrap(),
            Currencies { keys: 10, weapons: 4 },
        );
    }
    
    #[test]
    fn to_metal_saturating_integer_bounds() {
        let key_price_weapons = refined!(50);
        
        assert_eq!(
            Currencies {
                keys: Currency::MAX - 100,
                weapons: 0,
            }.to_weapons(key_price_weapons),
            Currency::MAX,
        );
        assert_eq!(
            Currencies {
                keys: Currency::MAX - 100,
                weapons: 0,
            }.to_weapons(-key_price_weapons),
            Currency::MIN,
        );
        assert_eq!(
            Currencies {
                keys: 1,
                weapons: Currency::MAX,
            }.to_weapons(key_price_weapons),
            Currency::MAX,
        );
        assert_eq!(
            Currencies {
                keys: -1,
                weapons: Currency::MIN,
            }.to_weapons(key_price_weapons),
            Currency::MIN,
        );
        assert_eq!(
            Currencies {
                keys: 1,
                weapons: Currency::MIN,
            }.to_weapons(key_price_weapons),
            Currency::MIN + key_price_weapons,
        );
    }
    
    #[test]
    fn checked_mul() {
        assert_eq!(
            Currencies {
                keys: 2,
                weapons: 0,
            }.checked_mul(Currency::MAX),
            None,
        );
    }
    
    #[test]
    fn checked_add() {
        assert_eq!(
            Currencies {
                keys: 2,
                weapons: 0,
            }.checked_add(Currencies {
                keys: Currency::MAX,
                weapons: 0,
            }),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal() {
        assert_eq!(
            Currencies {
                keys: Currency::MAX,
                weapons: 0,
            }.checked_to_weapons(Currency::MAX),
            None,
        );
    }
    
    #[test]
    fn checked_to_metal_correct_value() {
        assert_eq!(
            Currencies {
                keys: 10,
                weapons: 5,
            }.checked_to_weapons(10),
            Some(105),
        );
    }
    
    #[test]
    fn from_float_currencies() {
        let float_currencies = FloatCurrencies {
            keys: 1.0,
            metal: 1.33,
        };
        let currencies = Currencies::try_from(float_currencies).unwrap();
        
        assert_eq!(currencies.weapons, refined!(1) + scrap!(3));
    }
    
    #[test]
    fn from_float_currencies_infinity() {
        assert!(Currencies::try_from(FloatCurrencies {
            keys: f32::INFINITY,
            metal: 1.33,
        }).is_err());
        assert!(Currencies::try_from(FloatCurrencies {
            keys: f32::NEG_INFINITY,
            metal: 1.33,
        }).is_err());
    }
    
    #[test]
    fn from_float_currencies_nan() {
        assert!(Currencies::try_from(FloatCurrencies {
            keys: f32::NAN,
            metal: 1.33,
        }).is_err());
        assert!(Currencies::try_from(FloatCurrencies {
            keys: f32::NAN,
            metal: 1.33,
        }).is_err());
    }
    
    #[test]
    fn from_float_currencies_does_not_overflow_bounds() {
        assert!(Currencies::try_from(FloatCurrencies {
            keys: Currency::MAX as f32 * 2.0,
            metal: 1.33,
        }).is_err());
    }
    
    #[test]
    fn can_hash() {
        let mut hash = std::collections::HashMap::<Currencies, i32>::new();
        
        hash.insert(Currencies {
            keys: 1,
            weapons: 1,
        }, 1);
        hash.insert(Currencies {
            keys: 1,
            weapons: 2,
        }, 1);
        
        if let Some(value) = hash.get_mut(&Currencies {
            keys: 1,
            weapons: 1,
        }) {
            *value += 1;
        }
        
        assert_eq!(
            hash.get(&Currencies {
                keys: 1,
                weapons: 1,
            }),
            Some(&2),
        );
        assert_eq!(
            hash.get(&Currencies {
                keys: 1,
                weapons: 2,
            }),
            Some(&1),
        );
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests_serde {
    use super::*;
    use crate::{refined, scrap};
    use serde_json::{self, json, Value};
    use assert_json_diff::assert_json_eq;
    
    #[test]
    fn correct_json_format() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4),
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
    fn deserializes_currencies() {
        let currencies: Currencies = serde_json::from_str(r#"{"keys":1,"metal": 23.44}"#).unwrap();
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 1,
                weapons: refined!(23) + scrap!(4),
            },
        );
    }
    
    #[test]
    fn deserializes_currencies_with_no_keys() {
        let currencies: Currencies = serde_json::from_str(r#"{"metal": 23.44}"#).unwrap();
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 0,
                weapons: refined!(23) + scrap!(4),
            },
        );
    }
    
    #[test]
    fn deserializes_currencies_with_no_metal() {
        let currencies: Currencies = serde_json::from_str(r#"{"keys":5}"#).unwrap();
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 5,
                weapons: 0,
            },
        );
    }
    
    #[test]
    fn deserializes_currencies_with_weapon_value() {
        let currencies: Currencies = serde_json::from_str(r#"{"keys":1,"metal": 23.16}"#).unwrap();
        
        assert_eq!(
            currencies,
            Currencies {
                keys: 1,
                weapons: refined!(23) + 3,
            },
        );
    }
    
    #[test]
    fn serializes_currencies() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23) + scrap!(4)
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
    fn serializes_currencies_whole_numbers_have_no_decimals() {
        let currencies = Currencies {
            keys: 1,
            weapons: refined!(23)
        };
        let currencies_json = serde_json::to_string(&currencies).unwrap();
        let actual: Value = serde_json::from_str(&currencies_json).unwrap();
        let expected: Value = json!({
            "keys": 1,
            "metal": 23
        });
        
        assert_json_eq!(actual, expected);
    }
}