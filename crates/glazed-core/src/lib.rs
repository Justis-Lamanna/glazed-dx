use std::fmt::{Debug, Formatter};
use std::ops::{Div, Mul};

/// Represents that this object has an associated ID
pub trait Id<Output = usize> {
    fn get_id(&self) -> Output;
}