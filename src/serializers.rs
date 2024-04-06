use crate::types::Currency;
use crate::constants::ONE_REF_FLOAT;
use crate::helpers::cents_to_dollars;
use serde::{Serializer, Deserialize, Deserializer};

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

/// Serializes and deserializes cents.
pub mod cents {
    use super::*;
    
    pub fn serialize<S>(value: &Currency, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_f32(cents_to_dollars(*value))
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Currency, D::Error>
    where
        D: Deserializer<'de>
    {
        let usd = f32::deserialize(deserializer)?;
        let cents = (usd * 100.0).round() as Currency;
        
        Ok(cents)
    }
}