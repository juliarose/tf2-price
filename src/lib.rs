#![warn(missing_docs)]

//! # tf2-price
//! 
//! Utilities for Team Fortress 2 item pricing.
//! 
//! ## Usage
//!
//! ```
//! use tf2_price::{Currencies, ONE_REF, scrap};
//! 
//! let mut currencies = Currencies { keys: 5, metal: scrap!(5) };
//! 
//! // add another currencies
//! currencies += Currencies { keys: 2, metal: 0 };
//! assert_eq!(currencies, Currencies { keys: 7, metal: 10 });
//! 
//! // add keys
//! currencies.keys += 5;
//! assert_eq!(currencies, Currencies { keys: 12, metal: 10 });
//! 
//! // add metal - this value is represented as the number of weapons
//! currencies.metal += ONE_REF * 5;
//! assert_eq!(currencies, Currencies { keys: 12, metal: 100 });
//! ```
//! 
//! ## Conventions
//! 
//! All `metal` values are represented as the number of weapons. For 1 refined, this would be 18. 
//! The macros and constant values should be used to avoid any errors in accounting. For example: 
//! if adding one scrap, add `ONE_SCRAP` to the `metal` field. `scrap!(1)` will also create the 
//! same value.
//! 
//! In addition, all key values in methods are represented as values in weapons. If you need to 
//! use a floating point key price e.g. 70.22, you may use [`helpers::get_metal_from_float`] which 
//! will convert it into the closest appropriate value e.g. `(70.22 * 18 as f32).round() as i64`.
//! 
//! Arithmatic uses saturating operations. Adding two currencies that both contain values of 
//! [`i64::MAX`] will result in [`i64::MAX`] rather than rolling over. While values are stored as 
//! 64-bit integers and usually won't overflow if you're using reasonable numbers, if you need to 
//! check for overflows some checked methods are included.
#[macro_use] extern crate impl_ops;

pub mod traits;
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
pub use helpers::{get_metal_from_float, checked_get_metal_from_float, get_metal_float};
pub use constants::{ONE_REF, ONE_REC, ONE_SCRAP, ONE_WEAPON};

/// Generates value for refined metal.
#[macro_export]
macro_rules! refined {
    ( $a:expr ) => {
        {
            $a * 18
        }
    }
}

/// Generates value for reclaimed metal.
#[macro_export]
macro_rules! reclaimed {
    ( $a:expr ) => {
        {
            $a * 6
        }
    }
}

/// Generates value for scrap metal.
#[macro_export]
macro_rules! scrap {
    ( $a:expr ) => {
        {
            $a * 2
        }
    }
}