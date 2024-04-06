use crate::types::Currency;
use crate::constants::ONE_REF_FLOAT;
use serde::Deserialize;

/// Deserializes float weapon values as weapons.
pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: serde::Deserializer<'de>
{
    
    // get the metal value as a float e.g. 2.55 ref
    let metal_refined_float = f32::deserialize(deserializer)?;
    // will fit it into the nearest weapon value
    let metal = (metal_refined_float * ONE_REF_FLOAT).round() as Currency;
    
    Ok(metal)
}