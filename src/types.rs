#[cfg(not(feature = "b32"))]
/// The integer type used for currencies.
pub type Currency = i64;

#[cfg(feature = "b32")]
/// The integer type used for currencies.
pub type Currency = i32;