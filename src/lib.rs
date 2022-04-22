#[macro_use] extern crate impl_ops;

mod helpers;
mod currencies;
mod listing_currencies;
mod rounding;
mod constants;
mod usd_currencies;

pub mod traits;
pub mod error;

pub use usd_currencies::USDCurrencies;
pub use currencies::Currencies;
pub use listing_currencies::ListingCurrencies;
pub use rounding::Rounding;
pub use helpers::{get_metal_from_float, get_metal_float};
pub use constants::{ONE_REF, ONE_REC, ONE_SCRAP, ONE_WEAPON};

/// Generate value for refined metal
#[macro_export]
macro_rules! refined {
    ($a:expr) => {
        {
            $a * 18
        }
    }
}

/// Generate value for reclaimed metal
#[macro_export]
macro_rules! reclaimed {
    ($a:expr) => {
        {
            $a * 6
        }
    }
}

/// Generate value for scrap metal
#[macro_export]
macro_rules! scrap {
    ($a:expr) => {
        {
            $a * 2
        }
    }
}