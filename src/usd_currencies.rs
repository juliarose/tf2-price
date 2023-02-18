use crate::helpers;
use crate::types::Currency;
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops::{self, AddAssign, SubAssign, MulAssign, DivAssign};
use serde::{Serialize, Deserialize};

/// For storing cash values.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct USDCurrencies {
    /// Cash value in cents.
    #[serde(with = "helpers::cents", default)]
    pub usd: Currency,
}

impl PartialOrd for USDCurrencies {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for USDCurrencies {
    fn cmp(&self, other: &Self) -> Ordering {
        self.usd.cmp(&other.usd)
    }
}

impl Eq for USDCurrencies {}

impl Default for USDCurrencies {
    fn default() -> Self {
        Self::new()
    }
}

impl USDCurrencies {
    /// Creates a new [`USDCurrencies`] with `0` usd.
    pub fn new() -> Self {
        Self {
            usd: 0,
        }
    }
    
    /// Converts currencies to a key value using the given key price (represented as weapons).
    pub fn to_keys(&self, usd_key_price: Currency) -> f32 {
        self.usd as f32 / usd_key_price as f32
    }
    
    /// Converts currencies to a metal value using the key prices.
    /// 
    /// # Examples
    ///
    /// ```
    /// assert_eq!(tf2_price::USDCurrencies { usd: 100 }.to_metal(100, 10), 10);
    /// ```
    pub fn to_metal(
        &self,
        usd_key_price: Currency,
        metal_key_price: Currency,
    ) -> Currency {
        ((self.usd as f32 / usd_key_price as f32) * metal_key_price as f32).round() as Currency
    }
    
    /// Checks if the currencies contain any value.
    pub fn is_empty(&self) -> bool {
        self.usd == 0
    }
    
    /// Converts to dollars.
    /// 
    /// # Examples
    ///
    /// ```
    /// assert_eq!(tf2_price::USDCurrencies { usd: 99 }.to_dollars(), 0.99);
    /// ```
    pub fn to_dollars(&self) -> f32 {
        helpers::cents_to_dollars(self.usd)
    }
    
    /// Checked integer multiplication. Computes `self * rhs` for each field, returning `None` if 
    /// overflow occurred
    pub fn checked_mul(&self, rhs: Currency) -> Option<Self> {
        Some(Self { usd: self.usd.checked_mul(rhs)? })
    }
    
    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0` or the 
    /// division results in overflow.
    pub fn checked_div(&self, rhs: Currency) -> Option<Self> {
        Some(Self { usd: self.usd.checked_div(rhs)? })
    }
    
    /// Adds currencies. `None` if the result overflows integer bounds.
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        Some(Self { usd: self.usd.checked_add(other.usd)? })
    }
    
    /// Subtracts currencies. `None` if the result overflows integer bounds.
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(Self { usd: self.usd.checked_sub(other.usd)? })
    }
}

impl fmt::Display for USDCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", helpers::thousands(format!("{:.2}", self.to_dollars())))
    }
}

impl_op_ex!(+ |a: &USDCurrencies, b: &USDCurrencies| -> USDCurrencies { 
    USDCurrencies {
        usd: a.usd.saturating_add(b.usd),
    } 
});

impl_op_ex!(- |a: &USDCurrencies, b: &USDCurrencies| -> USDCurrencies { 
    USDCurrencies {
        usd: a.usd.saturating_sub(b.usd),
    }
});

impl_op_ex!(* |currencies: &USDCurrencies, num: Currency| -> USDCurrencies {
    USDCurrencies {
        usd: currencies.usd.saturating_mul(num),
    }
});

impl_op_ex!(/ |currencies: &USDCurrencies, num: Currency| -> USDCurrencies {
    USDCurrencies {
        usd: currencies.usd.saturating_div(num),
    }
});

impl_op_ex!(* |currencies: &USDCurrencies, num: f32| -> USDCurrencies {
    USDCurrencies { 
        usd: (currencies.usd as f32 * num).round() as Currency,
    }
});

impl_op_ex!(/ |currencies: &USDCurrencies, num: f32| -> USDCurrencies {
    USDCurrencies {
        usd: (currencies.usd as f32 / num).round() as Currency,
    }
});

impl AddAssign<USDCurrencies> for USDCurrencies {
    fn add_assign(&mut self, other: Self) {
        self.usd = self.usd.saturating_add(other.usd);
    }
}

impl AddAssign<&USDCurrencies> for USDCurrencies {
    fn add_assign(&mut self, other: &Self) {
        self.usd = self.usd.saturating_add(other.usd);
    }
}

impl SubAssign<USDCurrencies> for USDCurrencies {
    fn sub_assign(&mut self, other: Self) {
        self.usd = self.usd.saturating_sub(other.usd);
    }
}

impl SubAssign<&USDCurrencies> for USDCurrencies {
    fn sub_assign(&mut self, other: &Self) {
        self.usd = self.usd.saturating_sub(other.usd);
    }
}

impl MulAssign<Currency> for USDCurrencies {
    fn mul_assign(&mut self, other: Currency) {
        self.usd = self.usd.saturating_mul(other);
    }
}

impl MulAssign<f32> for USDCurrencies {
    fn mul_assign(&mut self, other: f32) {
        self.usd = (self.usd as f32 * other).round() as Currency;
    }
}

impl DivAssign<Currency> for USDCurrencies {
    fn div_assign(&mut self, other: Currency) {
        self.usd = self.usd.saturating_div(other);
    }
}

impl DivAssign<f32> for USDCurrencies {
    fn div_assign(&mut self, other: f32) {
        self.usd = (self.usd as f32 / other).round() as Currency;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, json, Value};
    use assert_json_diff::assert_json_eq;

    #[test]
    fn currencies_equal() {
        assert_eq!(USDCurrencies {
            usd: 10,
        }, USDCurrencies {
            usd: 10,
        });
    }
    
    #[test]
    fn currencies_not_equal() {
        assert_ne!(USDCurrencies {
            usd: 10,
        }, USDCurrencies {
            usd: 2,
        });
    }
    
    #[test]
    fn currencies_added() {
        assert_eq!(USDCurrencies {
            usd: 10,
        } + USDCurrencies {
            usd: 5,
        }, USDCurrencies {
            usd: 15,
        });
    }
    
    #[test]
    fn currencies_subtracted() {
        assert_eq!(USDCurrencies {
            usd: 10,
        } - USDCurrencies {
            usd: 5,
        }, USDCurrencies {
            usd: 5,
        });
    }
    
    #[test]
    fn currencies_multiplied_by_metal() {
        assert_eq!(USDCurrencies {
            usd: 10,
        } * 5, USDCurrencies {
            usd: 50,
        });
    }
    
    #[test]
    fn currencies_divided_by_f32() {
        assert_eq!(USDCurrencies {
            usd: 10,
        } / 2.5, USDCurrencies {
            usd: 4,
        });
    }
    
    #[test]
    fn currencies_divided_by_metal() {
        assert_eq!(USDCurrencies {
            usd: 10,
        } / 5, USDCurrencies {
            usd: 2,
        });
    }
    
    #[test]
    fn currencies_multiplied_by_f32() {
        assert_eq!(USDCurrencies {
            usd: 10,
        } * 2.5, USDCurrencies {
            usd: 25,
        });
    }
    
    #[test]
    fn currencies_mul_assign_metal() {
        let mut currencies = USDCurrencies {
            usd: 10,
        };
        
        currencies *= 2;
        
        assert_eq!(currencies, USDCurrencies {
            usd: 20,
        });
    }
    
    #[test]
    fn currencies_mul_assign_f32() {
        let mut currencies = USDCurrencies {
            usd: 10,
        };
        
        currencies *= 2.5;
        
        assert_eq!(currencies, USDCurrencies {
            usd: 25,
        });
    }
    
    #[test]
    fn currencies_div_assign_metal() {
        let mut currencies = USDCurrencies {
            usd: 10,
        };
        
        currencies /= 2;
        
        assert_eq!(currencies, USDCurrencies {
            usd: 5,
        });
    }
    
    #[test]
    fn currencies_div_assign_f32() {
        let mut currencies = USDCurrencies {
            usd: 10,
        };
        
        currencies /= 2.5;
        
        assert_eq!(currencies, USDCurrencies {
            usd: 4,
        });
    }
    
    #[test]
    fn to_string() {
        assert_eq!(USDCurrencies {
            usd: 320,
        }.to_string(), "$3.20");
    }
    
    #[test]
    fn to_string_with_thosands() {
        assert_eq!(USDCurrencies {
            usd: 123456,
        }.to_string(), "$1,234.56");
    }
    
    #[test]
    fn correct_json_format() {
        let currencies = USDCurrencies {
            usd: 123,
        };
        let currencies_json = serde_json::to_string(&currencies).unwrap();
        let actual: Value = serde_json::from_str(&currencies_json).unwrap();
        let expected: Value = json!({
            "usd": 1.23
        });
        
        assert_json_eq!(
            actual,
            expected,
        );
    }
    
    #[test]
    fn deserializes_currencies() {
        let currencies: USDCurrencies = serde_json::from_str(r#"{"usd":1234.56}"#).unwrap();
        
        assert_eq!(USDCurrencies {
            usd: 123456,
        }, currencies);
    }
}