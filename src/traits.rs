use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait SerializeCurrencies: Sized + Debug + Serialize + DeserializeOwned {}