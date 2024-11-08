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

#[derive(Clone)]
pub enum ValueKindDefinition {
    UnsignedInt,
    OneOfStr(Vec<String>),
}

pub enum ArgKindDefinition {
    Flag,
    Value(ValueKindDefinition),
}

pub struct ArgDefinition {
    pub id: String,
    pub short_name: Option<char>,
    pub long_name: Option<String>,
    pub kind: ArgKindDefinition,
    pub stop_parsing: bool,
    pub default_value: ArgValue,
}

#[derive(PartialEq, Clone)]
pub enum ArgValue {
    UnsignedInt(u32),
    Str(String),
    Bool(bool),
}

#[derive(PartialEq)]
pub struct ArgValuePair {
    id: String,
    value: ArgValue,
}

pub fn parse_and_validate_arg_list(
    arg_list: &[String],
    arg_definition_list: &[ArgDefinition],
) -> Result<Vec<ArgValuePair>, String> {
    let mut current_index = 0;
    let mut parsed_args: Vec<ArgValuePair> = Vec::new();
    while current_index < arg_list.len() {
        let (parsed_arg, args_consumed) =
            parse_and_validate_arg(&arg_list[current_index..], arg_definition_list)?;

        if let Some(arg) = parsed_args.iter().find(|elem| elem.id == parsed_arg.id) {
            if arg != &parsed_arg {
                return Err(format!(
                    "conflicting arguments for duplicated option: '{}'",
                    &arg_list[current_index]
                ));
            }
        } else {
            parsed_args.push(parsed_arg);
        }

        current_index += args_consumed;
    }

    Ok(parsed_args)
}

fn parse_and_validate_arg(
    arg_list: &[String],
    arg_definition_list: &[ArgDefinition],
) -> Result<(ArgValuePair, usize), String> {
    let mut consumed_args = 0;
    let next_arg = arg_list
        .first()
        .expect("list length should be validated before calling this function");
    consumed_args += 1;
    let (is_long_name, arg_name, mut arg_value) = decompose_and_validate_arg_structure(next_arg)?;
    let arg_definition = match_arg_to_definition(next_arg, arg_name, is_long_name, arg_definition_list)?;
    if let ArgKindDefinition::Value(_) = arg_definition.kind {
        if (is_long_name && arg_value.is_none())
            || (!is_long_name && arg_list.len() < consumed_args + 1)
        {
            return Err(format!("option '{next_arg}' requires an argument"));
        }

        if !is_long_name {
            arg_value = Some(
                arg_list
                    .get(consumed_args)
                    .expect("list length should be validated before getting the element"),
            );
            consumed_args += 1;
        }
    }

    if arg_definition.stop_parsing {
        consumed_args += 69420;
    }

    Ok((
        validate_and_create_arg(next_arg, arg_name, arg_value, arg_definition)?,
        consumed_args,
    ))
}

type IsLongName = bool;

fn decompose_and_validate_arg_structure(
    arg: &str,
) -> Result<(IsLongName, &str, Option<&str>), String> {
    let eq_index = arg.find('=');
    let is_long_name = arg.starts_with("--");

    if !is_long_name && (eq_index.is_some() || !arg.starts_with('-') || arg.len() != 2) {
        return Err(format!("invalid option: '{arg}'"));
    }

    let value;
    let name;

    let start_index = if is_long_name { 2 } else { 1 };

    if let Some(eq_index) = eq_index {
        if eq_index == arg.len() {
            return Err(format!("option '{arg}' requires an argument"));
        }

        name = &arg[start_index..eq_index];
        value = Some(&arg[eq_index + 1..]);
    } else {
        name = &arg[start_index..];
        value = None;
    }

    Ok((is_long_name, name, value))
}

fn match_arg_to_definition<'a>(
    arg: &str,
    arg_name: &str,
    is_long_name: bool,
    arg_definition_list: &'a [ArgDefinition],
) -> Result<&'a ArgDefinition, String> {
    if let Some(arg_definition) = arg_definition_list.iter().find(|elem| {
        if is_long_name {
            if let Some(long_name) = &elem.long_name {
                return long_name.as_str() == arg_name;
            }
            false
        } else {
            if let Some(short_name) = elem.short_name {
                return short_name == arg_name.chars().next().unwrap();
            }
            false
        }
    }) {
        Ok(arg_definition)
    } else {
        Err(format!("unrecognised option: '{arg}'"))
    }
}

fn validate_and_create_arg(
    arg: &str,
    arg_name: &str,
    arg_value: Option<&str>,
    arg_definition: &ArgDefinition,
) -> Result<ArgValuePair, String> {
    let value = match &arg_definition.kind {
        ArgKindDefinition::Flag => {
            if arg_value.is_some() {
                return Err(format!("option '{arg}' doesn't allow an argument"));
            }
            ArgValue::Bool(true)
        }
        ArgKindDefinition::Value(value_kind) => {
            let Some(arg_value) = arg_value else {
                return Err(format!("option '{arg}' requires an argument"));
            };
            match &value_kind {
                ValueKindDefinition::UnsignedInt => ArgValue::UnsignedInt(parse_u32(arg_value)?),
                ValueKindDefinition::OneOfStr(possible_values) => {
                    ArgValue::Str(validate_one_of_str(arg_value, possible_values)?.to_string())
                }
            }
        }
    };

    Ok(ArgValuePair {
        id: arg_definition.id.clone(),
        value,
    })
}

fn parse_u32(value: &str) -> Result<u32, String> {
    match value.parse::<u32>() {
        Ok(number) => Ok(number),
        _ => Err(format!("invalid option argument: '{value}'")),
    }
}

fn validate_one_of_str<'a>(value: &'a str, possible_values: &[String]) -> Result<&'a str, String> {
    if possible_values.contains(&value.to_string()) {
        Ok(value)
    } else {
        Err(format!(
            "invalid option argument: '{value}'. Valid arguments are: {}",
            possible_values.join(", ")
        ))
    }
}

pub trait SetFromArg {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[ArgDefinition],
    ) -> Self;
}

fn find_arg<'a>(arg_id: &str, arg_list: &'a [ArgValuePair]) -> Option<&'a ArgValuePair> {
    arg_list.iter().find(|elem| elem.id == arg_id)
}

impl SetFromArg for u32 {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[ArgDefinition],
    )-> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::UnsignedInt(val) = arg.value {
                val
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id == arg_id) {
            if let ArgValue::UnsignedInt(val) = &arg_definition.default_value {
                *val
            } else {
                panic!("invalid type for default value of option: '{arg_id}'");
            }
        } else {
            panic!("missing argument definition for option: '{arg_id}'");
        }
    }
}

impl SetFromArg for bool {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[ArgDefinition],
    ) -> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::Bool(val) = arg.value {
                val
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id == arg_id) {
            if let ArgValue::Bool(val) = &arg_definition.default_value {
                *val
            } else {
                panic!("invalid type for default value of option: '{arg_id}'");
            }
        } else {
            panic!("missing argument definition for option: '{arg_id}'");
        }
    }
}

impl SetFromArg for String {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[ArgDefinition],
    ) -> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::Str(val) = &arg.value {
                val.clone()
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id == arg_id) {
            if let ArgValue::Str(val) = &arg_definition.default_value {
                val.clone()
            } else {
                panic!("invalid type for default value of option: '{arg_id}'");
            }
        } else {
            panic!("missing argument definition for option: '{arg_id}'");
        }
    }
}
