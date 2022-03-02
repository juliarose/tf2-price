use serde::{Deserialize, Deserializer};
use crate::ONE_REF;

pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>
{
    let float = f32::deserialize(deserializer)?;
    let metal = (float * (ONE_REF as f32)).round() as i32;
    
    Ok(metal)
}

pub fn pluralize(amount: i32, singular: &'static str, plural: &'static str) -> &'static str {
    if amount == 1 {
        singular
    } else {
        plural
    }
}

pub fn get_metal_float(value: i32) -> f32 {
    f32::trunc((value as f32 / (ONE_REF as f32)) * 100.0) / 100.0
}

pub fn get_metal_from_float(value: f32) -> i32 {
    (value * (ONE_REF as f32)).round() as i32
}