use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait EntityETO: Serialize + DeserializeOwned {}