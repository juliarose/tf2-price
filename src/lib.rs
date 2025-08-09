//! Provides utilities for Team Fortress 2 item pricing.
//! 
//! # Usage
//!
//! ```
//! use tf2_price::{Currencies, ref_to_weps};
//! 
//! let mut currencies = Currencies {
//!     keys: 5,
//!     weapons: ref_to_weps!(1.33), // 1.33 refined metal
//! };
//! 
//! // add keys
//! currencies.keys += 5;
//! assert_eq!(currencies, Currencies { keys: 10, weapons: 24 });
//! 
//! // add metal - this value is represented as the number of weapons
//! currencies.weapons += ref_to_weps!(0.33);
//! assert_eq!(currencies, Currencies { keys: 10, weapons: 30 });
//! 
//! // add another currencies
//! currencies += Currencies {
//!     keys: 2,
//!     weapons: 0,
//! };
//! assert_eq!(currencies, Currencies { keys: 12, weapons: 30 });
//! ```
//! 
//! # Conventions
//! 
//! Metal values are represented as weapons, the smallest unit of currency. To ensure accurate
//! accounting, utilize the provided constants. To convert floating point refined values into
//! weapons, use the [`ref_to_weps`] macro e.g. `ref_to_weps!(1.33)` converts into 24 weapons.
//! 
//! Arithmetic operations employ
//! [saturating operations](https://en.wikipedia.org/wiki/Saturation_arithmetic),
//! preventing overflow. Adding two currencies that exceed [i64::MAX] will yield [i64::MAX] instead
//! of wrapping around. You can also utilize the `checked_*` methods if checking for overflow is
//! needed.

#![warn(missing_docs)]

pub mod error;

mod types;
mod helpers;
mod currencies;
mod float_currencies;
mod rounding;
mod constants;
#[cfg(feature = "serde")]
mod serializers;

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
/// Converts a refined metal value into weapons. While this method is convenient, keep in mind that
/// macros in Rust can increase compilation time and binary size, so don't overuse them. Prefer
/// using the constants provided from this crate.
/// 
/// The algorithm for this macro is simply `($a as f32 * 18.0_f32).round() as i64`.
/// 
/// # Examples
/// ```
/// use tf2_price::ref_to_weps;
/// 
/// assert_eq!(ref_to_weps!(1.0), 18);
/// assert_eq!(ref_to_weps!(1), 18);
/// assert_eq!(ref_to_weps!(1.05), 19);
/// assert_eq!(ref_to_weps!(1.11), 20);
/// assert_eq!(ref_to_weps!(1.77), 32);
/// ```
#[macro_export]
macro_rules! ref_to_weps {
    ( $a:expr ) => {
        ($a as f32 * 18.0_f32).round() as i64
    }
}

#[cfg(feature = "b32")]
/// Converts a refined metal value into weapons. While this method is convenient, keep in mind that
/// macros in Rust can increase compilation time and binary size, so don't overuse them. Prefer
/// using the constants provided from this crate.
/// 
/// The algorithm for this macro is simply `($a as f32 * 18.0_f32).round() as i32`.
/// 
/// # Examples
/// ```
/// use tf2_price::ref_to_weps;
/// 
/// assert_eq!(ref_to_weps!(1.0), 18);
/// assert_eq!(ref_to_weps!(1), 18);
/// assert_eq!(ref_to_weps!(1.05), 19);
/// assert_eq!(ref_to_weps!(1.11), 20);
/// assert_eq!(ref_to_weps!(1.77), 32);
/// ```
#[macro_export]
macro_rules! ref_to_weps {
    ( $a:expr ) => {
        ($a as f32 * 18.0_f32).round() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn metal_macro() {
        assert_eq!(ref_to_weps!(1.0), 18);
        assert_eq!(ref_to_weps!(1), 18);
        assert_eq!(ref_to_weps!(1.05), 19);
        assert_eq!(ref_to_weps!(1.11), 20);
        assert_eq!(ref_to_weps!(1.77), 32);
        assert_eq!(ref_to_weps!(1.99), 36);
        assert_eq!(ref_to_weps!(50.66), 912);
    }
}
