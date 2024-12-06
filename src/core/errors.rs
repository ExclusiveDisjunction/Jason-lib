use std::fmt::{Debug, Display};

pub enum Error {
    ArgumentError(String, String), //Name, Value
    NullError(String), //name
    FormatError(String, String), //string, reason
    RangeError(String, String, String, String), //variable, value, range min, range max
    NotFoundError(String), //identifyer
    PermissionError,
    OperatorError(String, String, Option<String>), //operator, operand1, [operand2] (for math)
    UnexpectedError(String), //Reason
    OperationError(String, String), //Action, Reason (for activities)
    IOError(std::io::Error)
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentError(name, value) => write!(f, "argument '{}' held an invalid value of '{}'", name, value),
            Self::NullError(name) => write!(f, "'{}' is null", name),
            Self::FormatError(string, reason) => write!(f, "the value '{}' held an invalid format because of '{}'", string, reason),
            Self::RangeError(variable, value, range_min, range_max) => write!(f, "the variable '{variable}' held a value outside of the range {range_min} - {range_max} ({value})"),
            Self::NotFoundError(value) => write!(f, "the value '{}' was not found", value),
            Self::PermissionError => write!(f, "invalid permissions"),
            Self::OperatorError(operator, operand1, operand2 ) => {
                match operand2 {
                    None => write!(f, "operator '{operator}' cannot be applied to '{operand1}'"),
                    Some(s) => write!(f, "operator '{operator}' cannot be applied to '{operand1}' and '{s}'")
                }
            },
            Self::UnexpectedError(s) => write!(f, "unexpected error: '{s}'"),
            Self::OperationError(action, reason) => write!(f, "operation '{action}' is not permitted because of '{reason}'"),
            Self::IOError(e) => (e as &dyn Debug).fmt(f)
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

pub fn argument_error(name: &str, value: &impl Debug) -> Error {
    Error::ArgumentError(name.to_string(), format!("{value:?}"))
}
pub fn null_error(name: &str) -> Error {
    Error::NullError(name.to_string())
}
pub fn format_error(string: &str, reason: &str) -> Error {
    Error::FormatError(string.to_string(), reason.to_string())
}
pub fn range_error(variable: &str, value: &impl Debug, min_value: impl Debug, max_value: impl Debug) -> Error {
    Error::RangeError(variable.to_string(), format!("{value:?}"), format!("{min_value:?}"), format!("{max_value:?}"))
}
pub fn not_found_error(identifyer: &str) -> Error {
    Error::NotFoundError(identifyer.to_string())
}
pub fn permission_error() -> Error {
    Error::PermissionError
}
pub fn operator_error(operator: &impl Debug, operand1: &impl Debug, operand2: Option<&impl Debug>) -> Error {
    match operand2 {
        None => Error::OperatorError(format!("{operator:?}"), format!("{operand1:?}"), None),
        Some(o2) => Error::OperatorError(format!("{operator:?}"), format!("{operand1:?}"), Some(format!("{o2:?}")))
    }
}
pub fn unexpected_error(reason: &str) -> Error {
    Error::UnexpectedError(reason.to_string())
}
pub fn operation_error(action: &str, reason: &str) -> Error {
    Error::OperationError(action.to_string(), reason.to_string())
}
pub fn io_error(error: std::io::Error) -> Error {
    Error::IOError(error)
}