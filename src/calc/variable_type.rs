use std::fmt::{Debug, Display};
use std::convert::{Into, TryFrom};
use crate::{log_debug, log_info, log_error, operator_error, argument_error, binary_unit, core::{io::BinaryUnit, errors::Error}};

#[derive(PartialEq, Clone, Copy)]
pub enum VariableTypes {
    Scalar,
    Vector, 
    Matrix,
    Complex
}
impl Debug for VariableTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Scalar => "SCA",
                Self::Vector => "VEC",
                Self::Matrix => "MAT",
                Self::Complex => "CMP",
            }
        )
    }
}
impl Display for VariableTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Scalar => "Scalar",
                Self::Vector => "Vector",
                Self::Matrix => "Matrix",
                Self::Complex => "Complex",
            }
        )
    }
}

pub trait VariableType : Display + Debug {
    fn get_type(&self) -> VariableTypes;
    fn clone_box(&self) -> Box<dyn VariableType>;

    fn required_units(&self) -> usize;
    fn to_binary(&self) -> Vec<BinaryUnit>;
    fn from_binary(&mut self, data: Vec<BinaryUnit>) -> Result<(), Error>;
    fn eq(&self, other: &Box<dyn VariableType>) -> bool;

}

pub fn apply_operation(oper: char, one: impl VariableType, two: impl VariableType) -> Box<dyn VariableType> {
    todo!()
}
pub fn from_binary(data: Vec<BinaryUnit>, target_type: &VariableTypes) -> Result<Box<dyn VariableType>, Error> {
    let mut result: Box<dyn VariableType> = Box::new(
            match target_type {
            VariableTypes::Scalar => ,
            VariableTypes::Vector => todo!(),
            VariableTypes::Matrix => todo!(),
            VariableTypes::Complex => todo!()
        }
    );

    result.as_mut().from_binary()?;
    Ok(result)
}