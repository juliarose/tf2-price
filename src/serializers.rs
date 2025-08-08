use crate::types::Currency;
use crate::constants::ONE_REF_FLOAT_64;
use serde::Deserialize;

/// Deserializes float weapon values as weapons.
pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: serde::Deserializer<'de>
{
    // Get the metal value as a float e.g. 2.55 ref.
    let metal_refined_float = f64::deserialize(deserializer)?;
    let metal = (metal_refined_float * ONE_REF_FLOAT_64)
        .round()
        .clamp(Currency::MIN as f64, Currency::MAX as f64) as Currency;
    
    Ok(metal)
}
