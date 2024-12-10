use std::fmt::{Debug, Display};
use std::ops::{Add, Sub, Mul, Div};

use crate::{conversion_error, operator_error, core::{io::BinaryUnit, errors::Error}};
use super::{variable_type::{VariableType, SimpleNumerical}, scalar::Scalar};

#[derive(Clone)]
pub struct Complex {
    a: f64,
    b: f64
}
impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for Complex  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Into<Vec<BinaryUnit>> for Complex {
    fn into(self) -> Vec<BinaryUnit> {
        todo!()
    }
}
impl TryFrom<Vec<BinaryUnit>> for Complex {
    type Error = Error;
    fn try_from(value: Vec<BinaryUnit>) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl Default for Complex {
    fn default() -> Self {
        Self {
            a: 0.00,
            b: 0.00
        }
    }
}
impl VariableType for Complex {
    fn required_units(&self) -> usize {
        2usize
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result.a += rhs.a;
        result.b += rhs.b;

        result
    }
}
impl Add<Scalar> for Complex {
    type Output = Self;
    fn add(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.add(data)
    }
}
impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        self.add(Complex::new(rhs, 0.0))
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result.a -= rhs.a;
        result.b -= rhs.b;

        result
    }
}
impl Sub<Scalar> for Complex {
    type Output = Self;
    fn sub(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.sub(data)
    }
}
impl Sub<f64> for Complex {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        self.sub(Complex::new(rhs, 0.0))
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Mul<Scalar> for Complex {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.mul(data)
    }
}
impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.a *= rhs;
        result.b *= rhs;

        result
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Div<Scalar> for Complex {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.div(data)
    }
}
impl Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.a /= rhs;
        result.b /= rhs;

        result
    }
}
impl SimpleNumerical for Complex {
    
}
impl Complex {
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            a,
            b
        }
    }

    pub fn polar(&self) -> (f64, f64) {
        todo!()
    }
}