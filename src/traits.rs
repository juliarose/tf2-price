//! Traits for currency types.

use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Trait for serializing and deserializing currencies.
pub trait SerializeCurrencies: Sized + Debug + Serialize + DeserializeOwned {}