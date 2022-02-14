use serde::{Deserialize, Deserializer};
use crate::ONE_REF;

pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>
{
    let float = f32::deserialize(deserializer)?;
    let metal = (float * (ONE_REF as f32)).round() as u32;
    
    Ok(metal)
}

pub fn pluralize(amount: u32, singular: &'static str, plural: &'static str) -> &'static str {
    if amount == 1 {
        singular
    } else {
        plural
    }
}

pub fn get_metal_float(value: u32) -> f32 {
    f32::trunc((value as f32 / (ONE_REF as f32)) * 100.0) / 100.0
}