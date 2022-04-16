use serde::{Serialize, de::DeserializeOwned};

pub trait SerializeCurrencies: Sized + std::fmt::Debug + Serialize + DeserializeOwned {}