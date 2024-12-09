use std::fmt::{Binary, Debug, Display};
use std::convert::{Into, TryFrom};
use std::ops::{Add, Sub, Mul, Div};
use crate::operation_error;
use crate::{operator_error, conversion_error, binary_unit, core::{io::BinaryUnit, errors::Error}};

#[derive(PartialEq, Clone)]
pub enum VariableData {
    Scalar(Scalar),
    Vector(MVector), 
    Matrix(Matrix),
    Complex(Complex)
}
impl Debug for VariableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(s) => (s as &dyn Debug).fmt(f),
            Self::Vector(v) => (v as &dyn Debug).fmt(f),
            Self::Matrix(m) => (m as &dyn Debug).fmt(f),
            Self::Complex(c) => (c as &dyn Debug).fmt(f)
        }
    }
}
impl Display for VariableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(s) => (s as &dyn Display).fmt(f),
            Self::Vector(v) => (v as &dyn Display).fmt(f),
            Self::Matrix(m) => (m as &dyn Display).fmt(f),
            Self::Complex(c) => (c as &dyn Display).fmt(f)
        }
    }
}

pub trait VariableType : Display + Debug + PartialEq + Clone + Into<Vec<BinaryUnit>> + TryFrom<Vec<BinaryUnit>> {
    fn required_units(&self) -> usize;
}

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
impl Add for Scalar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let data: f64 = rhs.into();
        self.add(data)
    }
}
impl Add<f64> for Scalar {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.data += rhs;

        result
    }
}
impl Add<Complex> for Scalar {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        let mut result = rhs;
        let us: f64 = self.into();
        result.a += us;

        result
    }
}
impl Sub for Scalar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let data: f64 = rhs.into();
        self.sub(data)
    }
}
impl Sub<f64> for Scalar {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.data -= rhs;

        result
    }
}
impl Sub<Complex> for Scalar {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        let mut result = rhs;
        let us: f64 = self.into();
        result.a -= us;

        result
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
        let mut result = rhs;
        result.a *= self;
        result.b *= self;

        result
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
    type Output = Result<Self, Erorr>;
    fn div(self, rhs: f64) -> Self::Output {
        todo!()
    }
}
impl Div<Complex> for Scalar {
    type Output = Result<Self, Error>;
    fn div(self, rhs: Complex) -> Self::Output {
        todo!()
    }
}
impl Scalar {

}

#[derive(Clone)]
pub struct MVector {
    data: Vec<f64>
}
impl Debug for MVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for MVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl PartialEq for MVector {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Into<Vec<BinaryUnit>> for MVector {
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
impl TryFrom<Vec<BinaryUnit>> for MVector {
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
            let mut data: Vec<f64> = vec![];
            for item in iter {
                let converted: f64 = item.try_into()?;
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
impl Default for MVector {
    fn default() -> Self {
        Self {
            data: vec![]
        }
    }
}
impl VariableType for MVector {
    fn required_units(&self) -> usize {
        self.dim() + 1
    }
}
impl Add for MVector {
    type Output = Result<Self, Error>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_error() || rhs.is_error() {
            Err(operator_error!('+', self, rhs))
        }
        else {
            let mut result: MVector;
            let minor: MVector;
            if self.dim() >= rhs.dim() { //Ours is larger
                result = self;
                minor = rhs;
            } else {
                result = rhs;
                minor = self;
            }

            //If we would have (i + 2j) + (i - 3j + 4k), we would expect this to be (2i - j + 4k), not fail. This is why we add to the larger one, and return it.
            for i in 0..minor.dim() {
                result.data[i] += minor.data[i];
            }

            Ok(result)
        }
    }
}
impl Sub for MVector {
    type Output = Result<Self, Error>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_error() || rhs.is_error() {
            Err(operator_error!('-', self, rhs))
        }
        else {
            let mut result: MVector;
            let minor: MVector;
            if self.dim() >= rhs.dim() { //Ours is larger
                result = self;
                minor = rhs;
            } else {
                result = rhs;
                minor = self;
            }

            //If we would have (i + 2j) + (i - 3j + 4k), we would expect this to be (2i - j + 4k), not fail. This is why we add to the larger one, and return it.
            for i in 0..minor.dim() {
                result.data[i] -= minor.data[i];
            }

            Ok(result)
        }
    }
}
impl Mul<Scalar> for MVector {
    type Output = Result<Self, Error>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.mul(data)
    }
}
impl Mul<f64> for MVector {
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
impl Div<Scalar> for MVector {
    type Output = Result<Self, Error>;
    fn div(self, rhs: Scalar) -> Self::Output {
        let data: f64 = rhs.into();
        self.div(data)
    }
}
impl Div<f64> for MVector {
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
impl MVector {
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
impl Mul<MVector> for Matrix {
    type Output = Result<MVector, Error>;
    fn mul(self, rhs: MVector) -> Self::Output {
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
impl Complex {
    fn new(a: f64, b: f64) -> Self {
        Self {
            a,
            b
        }
    }

    fn polar(&self) -> (f64, f64) {
        todo!()
    }
}