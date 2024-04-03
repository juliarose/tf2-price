use crate::types::Currency;

/// Value for one weapon.
pub const ONE_WEAPON: Currency = 1;
/// Value for one scrap metal.
pub const ONE_SCRAP: Currency = ONE_WEAPON * 2;
/// Value for one reclaimed metal.
pub const ONE_REC: Currency = ONE_SCRAP * 3;
/// Value for one refined metal.
pub const ONE_REF: Currency = ONE_REC * 3;
/// Value for one refined metal as a float.
pub const ONE_REF_FLOAT: f32 = ONE_REF as f32;

/// Symbol for one key.
pub const KEY_SYMBOL: &str = "key";
/// Symbol for multiple keys.
pub const KEYS_SYMBOL: &str = "keys";
/// Symbol for metal.
pub const METAL_SYMBOL: &str = "ref";