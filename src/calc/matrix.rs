use std::fmt::{Display, Debug};
use std::ops::{Add, Sub, Div, Mul};

use crate::{binary_unit, conversion_error, operator_error, core::{io::BinaryUnit, errors::Error}};
use super::{variable_type::VariableType, scalar::Scalar, vector::MVector};

#[derive(Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>
}
impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Into<Vec<BinaryUnit>> for Matrix {
    fn into(self) -> Vec<BinaryUnit> {
        todo!()
    }
}
impl TryFrom<Vec<BinaryUnit>> for Matrix {
    type Error = Error;
    fn try_from(value: Vec<BinaryUnit>) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl Default for Matrix {
    fn default() -> Self {
        Self {
            data: vec![]
        }
    }
}
impl VariableType for Matrix {
    fn required_units(&self) -> usize {
        self.rows() * self.cols() + 2
    }
}
impl Add for Matrix {
    type Output = Result<Matrix, Error>;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Sub for Matrix {
    type Output = Result<Matrix, Error>;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Mul for Matrix {
    type Output = Result<Matrix, Error>;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Mul<MVector<Scalar>> for Matrix {
    type Output = Result<MVector<Scalar>, Error>;
    fn mul(self, rhs: MVector<Scalar>) -> Self::Output {
        todo!()
    }
}
impl Mul<Scalar> for Matrix {
    type Output = Result<Matrix, Error>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        todo!()
    }
}
impl Mul<f64> for Matrix {
    type Output = Result<Matrix, Error>;
    fn mul(self, rhs: f64) -> Self::Output {
        todo!()
    }
}
impl Div<Scalar> for Matrix {
    type Output = Result<Matrix, Error>;
    fn div(self, rhs: Scalar) -> Self::Output {
        todo!()
    }
}
impl Div<f64> for Matrix {
    type Output = Result<Matrix, Error>;
    fn div(self, rhs: f64) -> Self::Output {
        todo!()   
    }
}
impl Matrix {
    pub fn rows(&self) -> usize {
        self.data.len()
    }
    pub fn cols(&self) -> usize {
        if self.data.is_empty() {
            0
        } 
        else {
            self.data[0].len()
        }
    }
}
