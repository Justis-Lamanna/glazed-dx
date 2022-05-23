use serde::{Deserialize, Deserializer};
use serde::de::Error;

#[derive(Debug, Clone, Copy)]
pub enum OneOrTwo<T> {
    One(T),
    Two(T, T)
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for OneOrTwo<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut s = Vec::<T>::deserialize(deserializer)?;
        if s.len() == 1 {
            Ok(Self::One(s.pop().unwrap()))
        } else if s.len() == 2 {
            Ok(Self::Two(s.pop().unwrap(), s.pop().unwrap()))
        } else {
            Err(D::Error::invalid_length(s.len(), &"One or two types only"))
        }
    }
}