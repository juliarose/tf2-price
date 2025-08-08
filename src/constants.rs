use crate::types::Currency;

/// Value for one weapon.
pub const ONE_WEAPON: Currency = 1;
/// Value for one scrap metal (represented as weapons).
pub const ONE_SCRAP: Currency = 2;
/// Value for one reclaimed metal (represented as weapons).
pub const ONE_REC: Currency = 6;
/// Value for one refined metal (represented as weapons).
pub const ONE_REF: Currency = 18;
/// Value for one refined metal as a float.
pub const ONE_REF_FLOAT: f32 = 18.0;
/// Value for one refined metal as a float (64 bits).
#[cfg(feature = "serde")]
pub const ONE_REF_FLOAT_64: f64 = 18.0;

/// Symbol for one key.
pub const KEY_SYMBOL: &str = "key";
/// Symbol for multiple keys.
pub const KEYS_SYMBOL: &str = "keys";
/// Symbol for metal.
pub const METAL_SYMBOL: &str = "ref";
