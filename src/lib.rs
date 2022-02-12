use std::fmt;
use serde::{Serialize, Deserialize};
use lazy_regex::regex_captures;

pub const ONE_WEAPON: u32 = 1;
pub const ONE_SCRAP: u32 = ONE_WEAPON * 2;
pub const ONE_REC: u32 = ONE_SCRAP * 3;
pub const ONE_REF: u32 = ONE_REC * 3;

// Generate value for refined metal
#[macro_export]
macro_rules! refined {
    ($a:expr) => {
        {
            $a * 18
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

pub enum Rounding {
    Up,
    Down,
    None,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Currencies {
    pub keys: u32,
    pub metal: u32,
}

impl Currencies {
    
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn default() -> Self {
        Self {
            keys: 0,
            metal: 0
        }
    }
    
    pub fn to_value(&self, key_price: u32) -> u32 {
        self.metal + (self.keys * key_price)
    }
    
    pub fn round(&mut self, rounding: Rounding) {
        if self.metal > 0 && self.metal % 2 != 0 {
            match rounding {
                Rounding::Up => {
                    self.metal = self.metal + 1;
                },
                Rounding::Down => {
                    // if we have keys, this is allowed to be 0
                    // otherwise this needs to be above 1,
                    // otherwise we will have none on both fields (this shouldn't happen)
                    if self.metal > 1 || self.keys > 0 {
                        self.metal = self.metal - 1;
                    }
                },
                _ => {
                    // do nothing
                },
            };
        }
    }
    
    pub fn empty(&self) -> bool {
        self.keys == 0 && self.metal == 0
    }
}

impl From<CurrenciesForm> for Currencies {
    
    fn from(form: CurrenciesForm) -> Currencies {
        let mut currencies = Currencies {
            keys: 0,
            metal: 0,
        };
        
        if let Some(keys) = form.keys {
            currencies.keys = keys;
        }
        
        if let Some(metal) = form.metal {
            let value: u32 = (metal * (ONE_REF as f32)).round() as u32;
            
            currencies.metal = value;
        }
        
        currencies
    }
}

impl TryFrom<String> for Currencies {
    type Error = &'static str;
    
    fn try_from(text: String) -> Result<Self, Self::Error>  {
        let mut currencies = Currencies::default();
        
        for element in text.split(", ") {
            if let Some((_, count_str, currency_name)) = regex_captures!(r"^([0-9]+\.?[0-9]*) (keys?|ref)$", element) {
                match currency_name {
                    "key" | "keys" => {
                        if let Ok(count) = count_str.parse::<u32>() {
                            currencies.keys = count;
                        } else {
                            return Err("Error parsing key count");
                        }
                    },
                    "ref" => {
                        if let Ok(count) = count_str.parse::<f32>() {
                            let value: u32 = (count * (ONE_REF as f32)).round() as u32;
                            
                            currencies.metal = value;
                        } else {
                            return Err("Error parsing metal count");
                        }
                    },
                    _ => {
                        // this should never be reached
                        return Err("Unknown error");
                    },
                }
            } else {
                return Err("Invalid currencies string");
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
        let mut strings: Vec<String> = Vec::new();
        
        if self.keys > 0 {
            strings.push(format!("{} {}", self.keys, pluralize("key", self.keys as usize)));
        }
        
        if self.metal > 0 {
            strings.push(format!("{} {}", get_metal_float(self.metal), "ref"));
        }
        
        write!(f, "{}", strings.join(", "))
    }
}

// For use in requests. 
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CurrenciesForm {
    pub keys: Option<u32>,
    pub metal: Option<f32>,
}

impl From<Currencies> for CurrenciesForm {
    
    fn from(currencies: Currencies) -> CurrenciesForm {
        let mut form = CurrenciesForm {
            keys: None,
            metal: None,
        };
        
        if currencies.keys > 0 {
            form.keys = Some(currencies.keys);
        }
        
        if currencies.metal > 0 {
            form.metal = Some(get_metal_float(currencies.metal));
        }
        
        form
    }
}

fn pluralize(symbol: &str, amount: usize) -> String {
    if amount == 1 {
        symbol.to_string()
    } else {
        format!("{}s", symbol)
    }
}

fn get_metal_float(value: u32) -> f32 {
    f32::trunc((value as f32 / (ONE_REF as f32)) * 100.0) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_currencies_from_string() {
        let currencies_str = String::from("2 keys, 23.44 ref");
        let currencies = Currencies::try_from(currencies_str).unwrap();
        
        assert_eq!(currencies.keys, 2);
        assert_eq!(currencies.metal, 422);
    }
    
    #[test]
    fn formats_currencies() {
        let currencies = Currencies {
            keys: 2,
            metal: 422
        };
        
        assert_eq!(format!("{}", currencies), String::from("2 keys, 23.44 ref"));
    }
    
    #[test]
    fn formats_currencies_singular() {
        let currencies = Currencies {
            keys: 1,
            metal: 422
        };
        
        assert_eq!(format!("{}", currencies), String::from("1 key, 23.44 ref"));
    }
    
    #[test]
    fn converts_to_value() {
        let currencies = Currencies {
            keys: 1,
            metal: 422
        };
        let key_price = 422;
        let value = currencies.to_value(key_price);
        
        assert_eq!(value, 844);
    }
    
    #[test]
    fn rounds_metal_down() {
        let mut currencies = Currencies {
            keys: 1,
            metal: 423
        };
        
        currencies.round(Rounding::Down);
        
        assert_eq!(currencies.metal, 422);
    }
    
    #[test]
    fn rounds_metal_up() {
        let mut currencies = Currencies {
            keys: 1,
            metal: 423
        };
        
        currencies.round(Rounding::Up);
        
        assert_eq!(currencies.metal, 424);
    }
    
    #[test]
    fn converts_to_currencies_form() {
        let currencies = Currencies {
            keys: 1,
            metal: 422
        };
        let currencies_form: CurrenciesForm = currencies.into();
        
        assert_eq!(currencies_form, CurrenciesForm {
            keys: Some(1),
            metal: Some(23.44),
        });
    }
}