//! # tf2-price
//! 
//! Utilities for Team Fortress 2 item pricing.
//! 
//! ## Usage
//!
//! ```
//! use tf2_price::{Currencies, ONE_REF, scrap};
//! 
//! let mut currencies = Currencies { keys: 5, weapons: scrap!(5) };
//! 
//! // add another currencies
//! currencies += Currencies { keys: 2, weapons: 0 };
//! assert_eq!(currencies, Currencies { keys: 7, weapons: 10 });
//! 
//! // add keys
//! currencies.keys += 5;
//! assert_eq!(currencies, Currencies { keys: 12, weapons: 10 });
//! 
//! // add metal - this value is represented as the number of weapons
//! currencies.weapons += ONE_REF * 5;
//! assert_eq!(currencies, Currencies { keys: 12, weapons: 100 });
//! ```
//! 
//! ## Conventions
//! 
//! With the exception of [`FloatCurrencies`], all `metal` values are represented as the number of 
//! weapons. For 1 refined, this would be 18. The macros and constant values should be used to 
//! avoid any errors in accounting. For example: if adding one scrap, add `ONE_SCRAP` to the 
//! `metal` field. `scrap!(1)` will also create the same value. The `metal!` macro can be used to 
//! convert floating point refined values into weapons e.g. `metal!(1.0)` will convert to 18.
//! 
//! In addition, all key values in methods are represented as values in weapons.
//! 
//! Arithmatic uses saturating operations. Adding two currencies that both contain values of 
//! [`i64::MAX`] will result in [`i64::MAX`] rather than rolling over. While values are stored as 
//! 64-bit integers and usually won't overflow if you're using reasonable numbers, if you need to 
//! check for overflows some checked methods are included.

#![warn(missing_docs)]
#[macro_use] extern crate impl_ops;

pub mod error;

mod types;
mod helpers;
mod currencies;
mod float_currencies;
mod rounding;
mod constants;
mod usd_currencies;

pub use usd_currencies::USDCurrencies;
pub use currencies::Currencies;
pub use float_currencies::FloatCurrencies;
pub use types::Currency;
pub use rounding::Rounding;
pub use helpers::{
    get_weapons_from_metal_float,
    checked_get_weapons_from_metal_float,
    get_metal_float_from_weapons,
};
pub use constants::{ONE_REF, ONE_REC, ONE_SCRAP, ONE_WEAPON};

#[cfg(not(feature = "b32"))]
/// Generates value for metal.
#[macro_export]
macro_rules! metal {
    ( $a:expr ) => {
        ($a * 18.0_f32).round() as i64
    }
}

#[cfg(feature = "b32")]
/// Generates value for metal.
#[macro_export]
macro_rules! metal {
    ( $a:expr ) => {
        ($a * 18.00_f32).round() as i32
    }
}

/// Generates value for refined metal.
#[macro_export]
macro_rules! refined {
    ( $a:expr ) => {
        $a * 18
    }
}

/// Generates value for reclaimed metal.
#[macro_export]
macro_rules! reclaimed {
    ( $a:expr ) => {
        $a * 6
    }
}

/// Generates value for scrap metal.
#[macro_export]
macro_rules! scrap {
    ( $a:expr ) => {
        $a * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn metal_macro() {
        assert_eq!(metal!(1.0), 18);
        assert_eq!(metal!(1.05), 19);
        assert_eq!(metal!(1.11), 20);
        assert_eq!(metal!(1.77), 32);
        assert_eq!(metal!(1.99), 36);
        assert_eq!(metal!(50.66), 912);
    }
}