use std::fmt::{Display, Debug};
use std::ops::{Add, Sub, Div, Mul};

use crate::{binary_unit, operation_error, conversion_error, operator_error, core::{io::BinaryUnit, errors::Error}};
use super::{variable_type::{VariableType, SimpleNumerical}, scalar::Scalar};


#[derive(Clone)]
pub struct MVector<T> where T: SimpleNumerical{
    data: Vec<T>
}
impl<T: SimpleNumerical> Debug for MVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl<T: SimpleNumerical> Display for MVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl<T: SimpleNumerical> PartialEq for MVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<T: SimpleNumerical> Into<Vec<BinaryUnit>> for MVector<T> {
    fn into(self) -> Vec<BinaryUnit> {
        let mut result = vec![binary_unit!(self.dim())];
        if self.dim() != 0 {
            for item in self.data {
                result.push(binary_unit!(item))
            }
        }

        result
    }
}
impl<T: SimpleNumerical> TryFrom<Vec<BinaryUnit>> for MVector<T> {
    type Error = Error;
    fn try_from(value: Vec<BinaryUnit>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(conversion_error!("cannot construct MVector from zero elements"));
        }

        let mut iter = value.into_iter();
        let dim: usize = iter.next().unwrap().try_into()?; //We can unwrap because there is at least one element

        if dim == 0 {
            Ok(Self::default())
        }
        else {
            let mut data: Vec<T> = vec![];
            for item in iter {
                let converted = match T::try_from(vec![item]) {
                    Ok(t) => t,
                    Err(e) => return Err(e)
                }
                data.push(converted);
            }

            Ok(
                Self {
                    data
                }
            )
        }
    }
}
impl<T: SimpleNumerical> Default for MVector<T> {
    fn default() -> Self {
        Self {
            data: vec![]
        }
    }
}
impl<T: SimpleNumerical> VariableType for MVector<T> {
    fn required_units(&self) -> usize {
        self.dim() + 1
    }
}
impl<T: SimpleNumerical> Add for MVector<T> {
    type Output = Result<Self, Error>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_error() || rhs.is_error() {
            Err(operator_error!('+', self, rhs))
        }
        else {
            let mut result: MVector<T>;
            let minor: MVector<T>;
            if self.dim() >= rhs.dim() { //Ours is larger
                result = self;
                minor = rhs;
            } else {
                result = rhs;
                minor = self;
            }

            //If we would have (i + 2j) + (i - 3j + 4k), we would expect this to be (2i - j + 4k), not fail. This is why we add to the larger one, and return it.
            for (i, e) in minor.data.into_iter().enumerate() {
                result.data[i] -= e;
            }

            Ok(result)
        }
    }
}
impl<T: SimpleNumerical> Sub for MVector<T> {
    type Output = Result<Self, Error>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_error() || rhs.is_error() {
            Err(operator_error!('-', self, rhs))
        }
        else {
            let mut result: MVector<T>;
            let minor: MVector<T>;
            if self.dim() >= rhs.dim() { //Ours is larger
                result = self;
                minor = rhs;
            } else {
                result = rhs;
                minor = self;
            }

            //If we would have (i + 2j) + (i - 3j + 4k), we would expect this to be (2i - j + 4k), not fail. This is why we add to the larger one, and return it.
            for (i, e) in minor.data.into_iter().enumerate() {
                result.data[i] -= e;
            }

            Ok(result)
        }
    }
}
impl<T: SimpleNumerical> Mul<Scalar> for MVector<T> {
    type Output = Result<Self, Error>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.mul(data)
    }
}
impl<T: SimpleNumerical> Mul<f64> for MVector<T> {
    type Output = Result<Self, Error>;
    fn mul(self, rhs: f64) -> Self::Output {
        if self.is_error() {
            Err(operator_error!('*', self, rhs))
        }
        else {
            let mut result = self;
            for element in result.data.iter_mut() {
                (*element) *= rhs;
            }

            Ok(result)
        }
    }
}
impl<T: SimpleNumerical> Div<Scalar> for MVector<T> {
    type Output = Result<Self, Error>;
    fn div(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.div(data)
    }
}
impl<T: SimpleNumerical> Div<f64> for MVector<T> {
    type Output = Result<Self, Error>;
    fn div(self, rhs: f64) -> Self::Output {
        if self.is_error() {
            Err(operator_error!('/', self, rhs))
        }
        else {
            let mut result = self;
            for element in result.data.iter_mut() {
                (*element) /= rhs;
            }

            Ok(result)
        }
    }
}
impl<T: SimpleNumerical> MVector<T> {
    pub fn dim(&self) -> usize {
        self.data.len()
    }
    pub fn is_error(&self) -> bool {
        self.data.is_empty()
    }

    pub fn magnitude(&self) -> Option<f64> {
        if self.is_error() {
            return None;
        }

        let mut result: f64  = 0.00;
        for item in &self.data {
            result += item.powi(2);
        }
        result = result.sqrt();

        Some(result)
    }
    pub fn angle(&self) -> Result<f64, Error> {
        if self.is_error() {
            return Err(operation_error!('θ', "no data loaded (error state)"));
        }
        else if self.dim() != 2 {
            return Err(operation_error!('θ', "can only find angle for dim = 2, got dim = {}", self.dim()));
        }

        Ok( self.data[1].atan2(self.data[0]) )
    }

    pub fn dot(self, rhs: Self) -> Result<Self, Error> {
        todo!()
    }
    pub fn cross(self, rhs: Self) -> Result<Self, Error> {
        todo!()
    }
    pub fn to_unit(self) -> Self {
        todo!()
    }
}
