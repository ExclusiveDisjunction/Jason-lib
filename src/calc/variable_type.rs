use std::fmt::{Debug, Display};
use std::convert::{Into, TryFrom};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use crate::core::io::BinaryUnit;

pub trait VariableType : Display + Debug + PartialEq + Clone + Into<Vec<BinaryUnit>> + TryFrom<Vec<BinaryUnit>> {
    fn required_units(&self) -> usize;
} 

pub trait SimpleNumerical : VariableType + AddAssign + AddAssign<f64> + SubAssign + SubAssign<f64> + MulAssign + MulAssign<f64> + DivAssign + DivAssign<f64> {

}