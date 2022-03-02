mod helpers;

pub use helpers::{get_metal_from_float, get_metal_float};

pub const ONE_WEAPON: i32 = 1;
pub const ONE_SCRAP: i32 = ONE_WEAPON * 2;
pub const ONE_REC: i32 = ONE_SCRAP * 3;
pub const ONE_REF: i32 = ONE_REC * 3;

const KEY_SYMBOL: &str = "key";
const KEYS_SYMBOL: &str = "keys";
const METAL_SYMBOL: &str = "ref";
const INVALID_CURRENCIES_FORMAT: &str = "Invalid currencies format";

use std::{fmt, ops::{Add, Sub, Mul, Div, AddAssign, SubAssign}};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error, ser::SerializeStruct};

// Generate value for refined metal
#[macro_export]
macro_rules! refined {
    ($a:expr) => {
        {
            $a * 18
        }
    }
}

// Generate value for reclaimed metal
#[macro_export]
macro_rules! reclaimed {
    ($a:expr) => {
        {
            $a * 6
        }
    }
}

// Generate value for scrap metal
#[macro_export]
macro_rules! scrap {
    ($a:expr) => {
        {
            $a * 2
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Rounding {
    Up,
    Down,
    Refined,
    UpRefined,
    DownRefined,
    None,
}

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
    
    pub fn from_metal(metal: i32, key_price: u32) -> Self {
        Self {
            // Will be 0 if metal is 30 and 32 (rounds down)
            keys: metal / key_price as i32,
            metal: metal % key_price as i32,
        }
    }
    
    pub fn to_metal(&self, key_price: u32) -> i32 {
        self.metal + (self.keys * key_price as i32)
    }
    
    pub fn is_empty(&self) -> bool {
        self.keys == 0 && self.metal == 0
    }
    
    pub fn round(&mut self, rounding: &Rounding) {
        if self.metal == 0 {
            return;
        }
        
        match *rounding {
            // No rounding needed if the metal value is an even number.
            Rounding::Up if self.metal % 2 != 0 => {
                self.metal += 1;
            },
            // No rounding needed if the metal value is an even number.
            Rounding::Down if self.metal % 2 != 0 => {
                self.metal -= 1;
            },
            Rounding::Refined => {
                let value = self.metal + ONE_REF / 2;
                
                self.metal = value - (value % ONE_REF);
            },
            Rounding::UpRefined => {
                let remainder = self.metal % ONE_REF;
                
                if remainder != 0 {
                    if self.metal > 0 {
                        self.metal -= remainder + -ONE_REF;
                    } else {
                        self.metal -= remainder;
                    }
                }
            },
            Rounding::DownRefined => {
                let remainder = self.metal % ONE_REF;
                
                if remainder != 0 {
                    if self.metal > 0 {
                        self.metal -= remainder;
                    } else {
                        self.metal -= remainder + ONE_REF;
                    }
                }
            },
            _ => {},
        }
    }
}

impl Add<Currencies> for Currencies {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            keys: self.keys + other.keys,
            metal: self.metal + other.metal,
        }
    }
}

impl Add<&Currencies> for Currencies {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            keys: self.keys + other.keys,
            metal: self.metal + other.metal,
        }
    }
}

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

impl Sub<Currencies> for Currencies {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            keys: self.keys - other.keys,
            metal: self.metal - other.metal,
        }
    }
}

impl Sub<&Currencies> for Currencies {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self {
            keys: self.keys - other.keys,
            metal: self.metal - other.metal,
        }
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

impl Div<i32> for Currencies {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self {
            keys: self.keys / other,
            metal: self.metal / other,
        }
    }
}

impl Mul<i32> for Currencies {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            keys: self.keys * other,
            metal: self.metal * other,
        }
    }
}

impl<'a> TryFrom<&'a str> for Currencies {
    type Error = &'static str;
    
    fn try_from(text: &'a str) -> Result<Self, Self::Error>  {
        let mut currencies = Currencies::default();
        
        for element in text.split(", ") {
            let mut element_split = element.split(' ');
            let (
                count_str,
                currency_name,
            ) = (
                element_split.next(),
                element_split.next(),
            );
            
            if count_str.is_none() || currency_name.is_none() || element_split.next().is_some() {
                return Err(INVALID_CURRENCIES_FORMAT);
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
                    if let Ok(count) = count_str.parse::<i32>() {
                        currencies.keys = count;
                    } else {
                        return Err("Error parsing key count");
                    }
                },
                METAL_SYMBOL => {
                    if let Ok(count) = count_str.parse::<f32>() {
                        let value = helpers::get_metal_from_float(count);
                        
                        currencies.metal = value;
                    } else {
                        return Err("Error parsing metal count");
                    }
                },
                _ => {
                    return Err(INVALID_CURRENCIES_FORMAT);
                },
            }
        }
        
        if currencies.keys == 0 && currencies.metal == 0 {
            return Err("No currencies could be parsed from string");
        }
        
        Ok(currencies)
    }
}

impl fmt::Display for Currencies {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keys > 0 && self.metal > 0 {
            write!(
                f,
                "{} {}, {} {}",
                self.keys,
                helpers::pluralize(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
                helpers::get_metal_float(self.metal),
                METAL_SYMBOL,
            )
        } else if self.keys > 0 {
            write!(
                f,
                "{} {}",
                self.keys,
                helpers::pluralize(self.keys, KEY_SYMBOL, KEYS_SYMBOL),
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
        let currencies = {
            let a = Currencies {
                keys: 10,
                metal: refined!(10),
            };
            let b = Currencies {
                keys: 5,
                metal: refined!(5),
            };
            
            a + b
        };
        
        assert_eq!(currencies, Currencies {
            keys: 15,
            metal: refined!(15),
        });
    }
    
    #[test]
    fn currencies_subtracted() {
        let currencies = {
            let a = Currencies {
                keys: 10,
                metal: refined!(10),
            };
            let b = Currencies {
                keys: 5,
                metal: refined!(5),
            };
            
            a - b
        };
        
        assert_eq!(currencies, Currencies {
            keys: 5,
            metal: refined!(5),
        });
    }
    
    #[test]
    fn currencies_multiplied_by_i32() {
        let currencies = Currencies {
            keys: 10,
            metal: refined!(10),
        };
        
        assert_eq!(currencies * 5, Currencies {
            keys: 50,
            metal: refined!(50),
        });
    }
    
    #[test]
    fn currencies_divided_by_i32() {
        let currencies = Currencies {
            keys: 10,
            metal: refined!(10),
        };
        
        assert_eq!(currencies / 5, Currencies {
            keys: 2,
            metal: refined!(2),
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
        let currencies = Currencies::try_from("what");
        
        assert_eq!(currencies.is_err(), true);
    }
    
    #[test]
    fn parses_currencies_from_string_invalid_currencies_extra() {
        let currencies = Currencies::try_from("2 keys, 3 what");
        
        assert_eq!(currencies.is_err(), true);
    }
    
    #[test]
    fn formats_currencies() {
        let currencies = Currencies {
            keys: 2,
            metal: refined!(23) + scrap!(4),
        };
        
        assert_eq!(format!("{}", currencies), "2 keys, 23.44 ref");
    }
    
    #[test]
    fn formats_currencies_singular() {
        let currencies = Currencies {
            keys: 1,
            metal: refined!(23) + scrap!(4),
        };
        
        assert_eq!(format!("{}", currencies), "1 key, 23.44 ref");
    }
    
    #[test]
    fn formats_currencies_with_no_trailing_decimal_places() {
        let currencies = Currencies {
            keys: 2,
            metal: refined!(23),
        };
        
        assert_eq!(&format!("{}", currencies), "2 keys, 23 ref");
    }
    
    #[test]
    fn formats_currencies_with_no_metal() {
        let currencies = Currencies {
            keys: 2,
            metal: 0,
        };
        
        assert_eq!(&format!("{}", currencies), "2 keys");
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
        
        currencies.round(&Rounding::Down);
        
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
        
        currencies.round(&Rounding::Up);
        
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