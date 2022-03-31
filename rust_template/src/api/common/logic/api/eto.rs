use serde::Serialize;
use serde::de::DeserializeOwned;

/*
    This is the logic layer version of the Entity
*/
pub trait EntityETO: Serialize + DeserializeOwned {}