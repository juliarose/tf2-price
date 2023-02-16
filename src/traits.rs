use std::fmt::Debug;
use serde::{Serialize, de::DeserializeOwned};

pub trait SerializeCurrencies: Sized + Debug + Serialize + DeserializeOwned {}