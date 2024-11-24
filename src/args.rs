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
    pub use super::definition::{ArgDefinition, ArgKindDefinition, ValueKindDefinition};
    pub use super::help;
    pub use super::parser;
    pub use super::set_value::SetFromArg;
    pub use super::ArgValue;
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgValue {
    UnsignedInt(u32),
    Str(String),
    Bool(bool),
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
