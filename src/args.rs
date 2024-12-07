//! Simple argument parser. Supports:
//! - short (-x) and long (--long-name) arguments, including flags and valued arguments
//!     - values are accepted in separate argument for short args and after '=' sign for long args
//! - validation against argument definitions
//! - assigning argument from list to variable of certain type by argument id
//! - stopping parsing on special arguments (--help, --version)
//! - default values
//!
//! Does not support:
//! - commands or subcommands
//! - short argument concatenation (-x, -y, -z => -xyz)

mod definition;
pub mod help;
pub mod parser;
mod set_value;

pub mod prelude {
    pub use super::definition::{Arg, ArgKind, ValueKind};
    pub use super::help;
    pub use super::parser;
    pub use super::set_value::SetFromArg;
    pub use super::ArgValue;
}

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum ArgValue {
    Int(i32),
    UnsignedInt(u32),
    Str(String),
    Bool(bool),
}

impl Display for ArgValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(value) => write!(f, "{value}"),
            Self::UnsignedInt(value) => write!(f, "{value}"),
            Self::Str(value) => write!(f, "{value}"),
            Self::Bool(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgValuePair {
    id: String,
    value: ArgValue,
}

impl ArgValuePair {
    fn new(id: &str, value: ArgValue) -> Self {
        Self {
            id: id.to_string(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_value_to_string() {
        let value = ArgValue::Int(-42);
        assert_eq!(value.to_string(), "-42");

        let value = ArgValue::UnsignedInt(42);
        assert_eq!(value.to_string(), "42");

        let value = ArgValue::Str("value".to_string());
        assert_eq!(value.to_string(), "value");

        let value = ArgValue::Bool(true);
        assert_eq!(value.to_string(), "true");
    }
}
