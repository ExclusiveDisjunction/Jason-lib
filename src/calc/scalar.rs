use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::{binary_unit, conversion_error, operator_error, core::{io::BinaryUnit, errors::Error}};
use super::variable_type::{VariableType, SimpleNumerical};
use super::complex::Complex;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Scalar {
    data: f64
}
impl Debug for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Scalar:{})", self.data)
    }
}
impl Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
impl PartialEq<f64> for Scalar {
    fn eq(&self, other: &f64) -> bool {
        self.data == *other   
    }
}
impl Into<f64> for Scalar {
    fn into(self) -> f64 {
        self.data
    }
}
impl Into<Vec<BinaryUnit>> for Scalar {
    fn into(self) -> Vec<BinaryUnit> {
        vec![ binary_unit!(self.data) ]
    }
}
impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self {
            data: value
        }
    }
}
impl TryFrom<Vec<BinaryUnit>> for Scalar {
    type Error = Error;
    fn try_from(value: Vec<BinaryUnit>) -> Result<Self, Self::Error> {
        match value.into_iter().next() {
            None => return Err(conversion_error!("expected at least one unit, but got none")),
            Some(d) => {
                let as_float: f64 = d.try_into()?;
                Ok(Self::from(as_float))
            }
        }
    }
}
impl Default for Scalar {
    fn default() -> Self {
        Self {
            data: 0.00f64
        }
    }
}
impl VariableType for Scalar {
    fn required_units(&self) -> usize {
        1usize
    }
}
impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        let rhs: f64 = rhs.into();
        self.data += rhs;
    }
}
impl AddAssign<f64> for Scalar {
    fn add_assign(&mut self, rhs: f64) {
        self.data += rhs;
    }
}
impl Add<Complex> for Scalar {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        let lhs = Complex::new(self.data, 0.0);
        lhs.add(rhs)
    }
}
impl SubAssign for Scalar {
    fn sub_assign(&mut self, rhs: Self){
        let rhs: f64 = rhs.into();
        self.data -= rhs;
    }
}
impl SubAssign<f64> for Scalar {
    fn sub_assign(&mut self, rhs: f64) {
        self.data -= rhs;
    }
}
impl Sub<Complex> for Scalar {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        let lhs = Complex::new(self.data, 0.0);
        lhs.sub(rhs)
    }
}
impl Mul for Scalar {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let data: f64 = rhs.into();
        self.mul(data)
    }
}
impl Mul<f64> for Scalar {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.data *= rhs;

        result
    }
}
impl Mul<Complex> for Scalar {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        rhs.mul(self.data)
    }
}
impl Div for Scalar {
    type Output = Result<Self, Error>;
    fn div(self, rhs: Self) -> Self::Output {
        let data: f64 = rhs.into();
        self.div(data)
    }
}
impl Div<f64> for Scalar {
    type Output = Result<Self, Error>;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return Err(operator_error!('/', self, rhs));
        }

       let mut result = self;
       result.data /= rhs;

       Ok(result)
    }
}
impl Div<Complex> for Scalar {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        rhs.div(self.data)
    }
}
impl SimpleNumerical for Scalar {

}
impl Scalar {

}