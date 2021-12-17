use std::fmt::{Debug, Formatter};
use std::ops::{Div, Mul};

/// Represents that this object has an associated ID
pub trait Id<Output = usize> {
    fn get_id(&self) -> Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Fraction<T> {
    pub numerator: T,
    pub denominator: T
}
impl<T> Fraction<T> {
    pub const fn new(numerator: T, denominator: T) -> Fraction<T>{
        Fraction { numerator, denominator }
    }
}

impl<T> Fraction<T> {
    pub fn into<U>(self) -> Fraction<U>
        where T: Into<U> {
        Fraction {
            numerator: self.numerator.into(),
            denominator: self.denominator.into()
        }
    }
}

impl<T, U> Mul<T> for Fraction<T>
    where T: Mul<T, Output=U> + Into<U> {
    type Output = Fraction<U>;

    fn mul(self, rhs: T) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs,
            denominator: self.denominator.into()
        }
    }
}
impl<T, U> Fraction<T>
    where T: Div<T, Output=U>
{
    pub fn resolve(self) -> U {
        self.numerator.div(self.denominator)
    }
}