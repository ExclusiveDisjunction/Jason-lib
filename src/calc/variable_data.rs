pub use super::scalar::Scalar;
pub use super::vector::MVector;
pub use super::matrix::Matrix;
pub use super::complex::Complex;

use std::fmt::{Display, Debug};

#[derive(PartialEq, Clone)]
pub enum VariableData {
    Scalar(Scalar),
    Complex(Complex),
    Vector(MVector<Scalar>), 
    CVector(MVector<Complex>),
    Matrix(Matrix),
}
impl Debug for VariableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(s) => (s as &dyn Debug).fmt(f),
            Self::Vector(v) => (v as &dyn Debug).fmt(f),
            Self::CVector(v) => (v as &dyn Debug).fmt(f),
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
            Self::CVector(v) => (v as &dyn Display).fmt(f),
            Self::Matrix(m) => (m as &dyn Display).fmt(f),
            Self::Complex(c) => (c as &dyn Display).fmt(f)
        }
    }
}
