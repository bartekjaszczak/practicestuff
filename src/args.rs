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

use core::panic;

pub mod prelude {
    pub use crate::args::{
        ArgDefinition, ArgKindDefinition, ArgValue, SetFromArg, ValueKindDefinition,
    };
}

type IsLongName = bool;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueKindDefinition {
    UnsignedInt,
    OneOfStr(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub enum ArgKindDefinition {
    Flag,
    Value(ValueKindDefinition),
}

pub struct ArgDefinition {
    id: String,
    short_name: Option<char>,
    long_name: Option<String>,
    description: Vec<String>,
    kind: ArgKindDefinition,
    stop_parsing: bool,
    default_value: ArgValue,
}

impl ArgDefinition {
    pub fn builder() -> ArgDefinitionBuilder {
        ArgDefinitionBuilder::default()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn short_name(&self) -> Option<char> {
        self.short_name
    }

    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_deref()
    }

    pub fn description(&self) -> &[String] {
        &self.description
    }

    pub fn kind(&self) -> &ArgKindDefinition {
        &self.kind
    }

    pub fn stop_parsing(&self) -> bool {
        self.stop_parsing
    }

    pub fn default_value(&self) -> &ArgValue {
        &self.default_value
    }
}

#[derive(Default)]
pub struct ArgDefinitionBuilder {
    id: Option<String>,
    short_name: Option<char>,
    long_name: Option<String>,
    description: Vec<String>,
    kind: Option<ArgKindDefinition>,
    stop_parsing: Option<bool>,
    default_value: Option<ArgValue>,
}

impl ArgDefinitionBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn short_name(mut self, short_name: char) -> Self {
        self.short_name = Some(short_name);
        self
    }

    pub fn long_name(mut self, long_name: &str) -> Self {
        self.long_name = Some(long_name.to_string());
        self
    }

    pub fn description(mut self, description: Vec<String>) -> Self {
        self.description = description;
        self
    }

    pub fn kind(mut self, kind: ArgKindDefinition) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn stop_parsing(mut self, stop_parsing: bool) -> Self {
        self.stop_parsing = Some(stop_parsing);
        self
    }

    pub fn default_value(mut self, default_value: ArgValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn build(self) -> ArgDefinition {
        assert!(
            !(self.short_name.is_none() && self.long_name.is_none()),
            "either short or long name is required"
        );
        self.validate_arg_type();
        ArgDefinition {
            id: self.id.expect("id is required"),
            short_name: self.short_name,
            long_name: self.long_name,
            description: self.description,
            kind: self.kind.expect("kind is required"),
            stop_parsing: self.stop_parsing.unwrap_or(false),
            default_value: self.default_value.expect("default value is required"),
        }
    }

    fn validate_arg_type(&self) {
        match &self.kind {
            Some(ArgKindDefinition::Flag) => {
                assert!(
                    self.default_value.is_none()
                        || matches!(self.default_value, Some(ArgValue::Bool(_))),
                    "default value must be of type bool"
                );
            }
            Some(ArgKindDefinition::Value(kind)) => match kind {
                ValueKindDefinition::UnsignedInt => {
                    assert!(
                        self.default_value.is_none()
                            || matches!(self.default_value, Some(ArgValue::UnsignedInt(_))),
                        "default value must be of type u32"
                    );
                }
                ValueKindDefinition::OneOfStr(_) => {
                    assert!(
                        self.default_value.is_none()
                            || matches!(self.default_value, Some(ArgValue::Str(_))),
                        "default value must be of type String"
                    );
                }
            },
            None => panic!("kind is required"),
        }
    }
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
    let arg_definition =
        match_arg_to_definition(next_arg, arg_name, is_long_name, arg_definition_list)?;
    if let ArgKindDefinition::Value(_) = arg_definition.kind {
        if (is_long_name && arg_value.is_none())
            || (!is_long_name && arg_list.len() < consumed_args + 1)
        {
            return Err(format!("option '{next_arg}' requires an argument"));
        }

        if !is_long_name {
            arg_value = Some(arg_list[consumed_args].as_str()); // Already validated above
            consumed_args += 1;
        }
    }

    if arg_definition.stop_parsing {
        consumed_args += 69420;
    }

    Ok((
        validate_and_create_arg(next_arg, arg_value, arg_definition)?,
        consumed_args,
    ))
}

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
        if eq_index + 1 == arg.len() {
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

    Ok(ArgValuePair::new(&arg_definition.id, value))
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
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_arg_definition() {
        let arg_definition = ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .long_name("some-arg")
            .description(vec!["some description".to_string()])
            .kind(ArgKindDefinition::Flag)
            .stop_parsing(true)
            .default_value(ArgValue::Bool(false))
            .build();

        assert_eq!(arg_definition.id, "some_arg");
        assert_eq!(arg_definition.short_name, Some('s'));
        assert_eq!(arg_definition.long_name, Some("some-arg".to_string()));
        assert_eq!(
            arg_definition.description,
            vec!["some description".to_string()]
        );
        assert_eq!(arg_definition.kind, ArgKindDefinition::Flag);
        assert!(arg_definition.stop_parsing);
        assert_eq!(arg_definition.default_value, ArgValue::Bool(false));
    }

    #[test]
    #[should_panic(expected = "id is required")]
    fn arg_definition_requires_id() {
        ArgDefinition::builder()
            .short_name('s')
            .long_name("some-arg")
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "either short or long name is required")]
    fn arg_definition_requires_short_or_long_name() {
        ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build(); // passes

        ArgDefinition::builder()
            .id("some_arg")
            .long_name("long")
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build(); // passes

        ArgDefinition::builder()
            .id("some_arg")
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "kind is required")]
    fn arg_definition_requires_kind() {
        ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value is required")]
    fn arg_definition_requires_default_value() {
        ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type bool")]
    fn flag_arg_definition_requires_bool_default_value() {
        ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::UnsignedInt(42))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type u32")]
    fn u32_arg_definition_requires_u32_default_value() {
        ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type String")]
    fn string_arg_definition_requires_string_default_value() {
        ArgDefinition::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                vec![],
            )))
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    fn find_arg_test() {
        let arg_id = "some arg";
        let arg_to_be_found = ArgValuePair::new(arg_id, ArgValue::Bool(false));
        let mut arg_list = vec![
            ArgValuePair::new("different arg", ArgValue::Bool(false)),
            ArgValuePair::new("another arg", ArgValue::Bool(false)),
        ];

        assert_eq!(find_arg(arg_id, &arg_list), None);

        arg_list.push(arg_to_be_found.clone());

        assert_eq!(find_arg(arg_id, &arg_list), Some(&arg_to_be_found));

        arg_list.clear();

        assert_eq!(find_arg(arg_id, &arg_list), None);
    }

    #[test]
    fn bool_assign_value_from_arg_list() {
        let expected = true;
        let arg_id = "some_arg";
        let arg_list = [ArgValuePair::new("some_arg", ArgValue::Bool(expected))];
        let arg_definitions = [ArgDefinition::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];
        let val = bool::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);

        assert_eq!(val, expected);
    }

    #[test]
    fn u32_assign_value_from_arg_list() {
        let expected = 42;
        let arg_id = "some_arg";
        let arg_list = [ArgValuePair::new(
            "some_arg",
            ArgValue::UnsignedInt(expected),
        )];
        let arg_definitions = [ArgDefinition::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(0))
            .build()];
        let val = u32::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);

        assert_eq!(val, expected);
    }

    #[test]
    fn string_assign_value_from_arg_list() {
        let expected = "some string".to_string();
        let arg_id = "some_arg";
        let arg_list = [ArgValuePair::new(
            "some_arg",
            ArgValue::Str(expected.clone()),
        )];

        let arg_definitions = [ArgDefinition::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                vec![],
            )))
            .default_value(ArgValue::Str(String::new()))
            .build()];
        let val = String::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);

        assert_eq!(val, expected);
    }

    #[test]
    #[should_panic(expected = "invalid type for option")]
    fn arg_found_but_no_value() {
        let arg_id = "arg";
        let arg_list = [ArgValuePair::new(arg_id, ArgValue::Bool(true))];
        let arg_definitions = [];

        String::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);
    }

    #[test]
    fn arg_not_found_but_default_is_present() {
        let arg_id = "arg";
        let arg_default_value = 42;
        let arg_list = [];
        let arg_definitions = [ArgDefinition::builder()
            .id(arg_id)
            .short_name('a')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(arg_default_value))
            .build()];

        let val = u32::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);

        assert_eq!(val, arg_default_value);
    }

    #[test]
    #[should_panic(expected = "missing argument definition for option")]
    fn arg_not_found_anywhere() {
        let arg_id = "arg";
        let arg_list = [];
        let arg_definitions = [];
        String::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);
    }

    #[test]
    fn u32_parsing() {
        let u32_max = u32::MAX.to_string();
        let correct = [
            (u32_max.as_str(), u32::MAX),
            ("0", 0),
            ("42", 42),
            ("1234567", 1_234_567),
        ];

        for (value, expected) in &correct {
            assert_eq!(parse_u32(value).expect("test failed"), *expected);
        }

        let u32_max_plus_1 = (u64::from(u32::MAX) + 1).to_string();
        let incorrect = [
            u32_max_plus_1.as_str(),
            "-1",
            "qwerty",
            "a123a",
            "-123456",
            "12a",
            "a12",
        ];

        for value in incorrect {
            assert!(parse_u32(value).is_err());
        }
    }

    #[test]
    fn one_of_strings() {
        let value = "hehe";
        let possible_values = ["hihi".to_string(), "hehe".to_string(), "haha".to_string()];

        let res = validate_one_of_str(value, &possible_values);
        assert_eq!(res.expect("test failed"), value);

        let possible_values = ["hihi".to_string(), "haha".to_string()];
        let res = validate_one_of_str(value, &possible_values);
        assert!(res.is_err());

        let possible_values = [];
        let res = validate_one_of_str(value, &possible_values);
        assert!(res.is_err());
    }

    #[test]
    #[should_panic(expected = "doesn't allow an argument")]
    fn found_value_in_flag() {
        let arg = "--some-flag";
        let arg_value = Some("someval");
        let arg_definition = ArgDefinition::builder()
            .id(arg)
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build();

        validate_and_create_arg(arg, arg_value, &arg_definition).expect("test failed");
    }

    #[test]
    #[should_panic(expected = "requires an argument")]
    fn found_no_value_when_expected() {
        let arg = "--some-arg";
        let arg_value = None;
        let arg_definition = ArgDefinition::builder()
            .id(arg)
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(42))
            .build();

        validate_and_create_arg(arg, arg_value, &arg_definition).expect("test failed");
    }

    #[test]
    fn validate_and_create_bool_arg() {
        let arg = "--some_flag";
        let arg_value = None;
        let arg_definition = ArgDefinition::builder()
            .id(arg)
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build();

        let expected = ArgValuePair::new(arg, ArgValue::Bool(true));
        let arg_value_pair =
            validate_and_create_arg(arg, arg_value, &arg_definition).expect("test failed");
        assert_eq!(arg_value_pair, expected);
    }

    #[test]
    fn validate_and_create_u32_arg() {
        let arg = "--some_arg";
        let value = 42;
        let arg_value = Some(42.to_string());
        let arg_definition = ArgDefinition::builder()
            .id(arg)
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(0))
            .build();

        let expected = ArgValuePair::new(arg, ArgValue::UnsignedInt(value));
        let arg_value_pair = validate_and_create_arg(arg, arg_value.as_deref(), &arg_definition)
            .expect("test failed");
        assert_eq!(arg_value_pair, expected);
    }

    #[test]
    fn validate_and_create_string_arg() {
        let arg = "--some_arg";
        let value = "val";
        let arg_value = Some(value);
        let arg_definition = ArgDefinition::builder()
            .id(arg)
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                vec!["val".to_string(), "another_val".to_string()],
            )))
            .default_value(ArgValue::Str("default".to_string()))
            .build();

        let expected = ArgValuePair::new(arg, ArgValue::Str(value.to_string()));
        let arg_value_pair =
            validate_and_create_arg(arg, arg_value, &arg_definition).expect("test failed");
        assert_eq!(arg_value_pair, expected);
    }

    #[test]
    fn arg_not_matching_any_definition() {
        let arg = "my_arg";
        let arg_name = "my_arg_name";
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("another_arg")
                .short_name('s')
                .long_name("different_name")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("different_arg")
                .short_name('a')
                .long_name("another_name")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .default_value(ArgValue::UnsignedInt(42))
                .build(),
        ];

        let result = match_arg_to_definition(arg, arg_name, false, &arg_definition_list);
        assert!(result.is_err());
    }

    #[test]
    fn arg_matching_by_short_name() {
        let arg = "my_arg";
        let arg_name = "a";
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("another_arg")
                .short_name('s')
                .long_name("different_name")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("different_arg")
                .short_name('a')
                .long_name("another_name")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .default_value(ArgValue::UnsignedInt(42))
                .build(),
        ];

        let result = match_arg_to_definition(arg, arg_name, false, &arg_definition_list);
        assert!(result.is_ok());
    }

    #[test]
    fn arg_matching_by_long_name() {
        let arg = "my_arg";
        let arg_name = "long_name";
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("another_arg")
                .short_name('s')
                .long_name("long_name")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("different_arg")
                .short_name('a')
                .long_name("another_name")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .default_value(ArgValue::UnsignedInt(42))
                .build(),
        ];

        let result = match_arg_to_definition(arg, arg_name, true, &arg_definition_list);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "invalid option")]
    fn short_args_cant_have_values_after_equal_sign() {
        decompose_and_validate_arg_structure("-a=42").unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid option")]
    fn arg_should_start_with_hyphen() {
        decompose_and_validate_arg_structure("a=42").unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid option")]
    fn short_args_must_have_one_letter() {
        decompose_and_validate_arg_structure("-short").unwrap();
    }

    #[test]
    #[should_panic(expected = "requires an argument")]
    fn value_must_be_provided_after_equal_sign() {
        decompose_and_validate_arg_structure("--long=").unwrap();
    }

    #[test]
    fn short_arg_decomposed() {
        let result = decompose_and_validate_arg_structure("-a").expect("test failed");
        assert!(!result.0); // false == short arg
        assert_eq!(result.1, "a");
        assert!(result.2.is_none()); // no value
    }

    #[test]
    fn long_arg_decomposed() {
        let result = decompose_and_validate_arg_structure("--long-arg=value").expect("test failed");
        assert!(result.0); // true == long arg
        assert_eq!(result.1, "long-arg");
        assert_eq!(result.2.expect("test failed"), "value");
    }

    #[test]
    #[should_panic(expected = "list length should be validated before calling this function")]
    fn validate_empty_list_of_args() {
        parse_and_validate_arg(&[], &[]).unwrap();
    }

    #[test]
    #[should_panic(expected = "unrecognised option")]
    fn validate_arg_not_in_the_definition_list() {
        let arg_list = ["-a".to_string(), "-b".to_string(), "-c".to_string()];
        parse_and_validate_arg(&arg_list, &[]).unwrap();
    }

    #[test]
    #[should_panic(expected = "requires an argument")]
    fn validate_long_arg_with_no_value() {
        let arg_list = ["--long".to_string()];
        let arg_definition_list = [ArgDefinition::builder()
            .id("long")
            .long_name("long")
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(42))
            .build()];

        parse_and_validate_arg(&arg_list, &arg_definition_list).unwrap();
    }

    #[test]
    fn validate_short_arg_correctly() {
        let arg_value = 65;
        let arg_list = ["-s".to_string(), arg_value.to_string()];
        let arg_definition_list = [ArgDefinition::builder()
            .id("short")
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(42))
            .build()];

        let parsed_arg = parse_and_validate_arg(&arg_list, &arg_definition_list).unwrap();
        let expected_arg = ArgValuePair::new("short", ArgValue::UnsignedInt(arg_value));

        assert_eq!(parsed_arg.0, expected_arg);
        assert_eq!(parsed_arg.1, 2); // consumed args
    }

    #[test]
    fn validate_long_arg_with_value_correctly() {
        let arg_list = ["--long=65".to_string()];
        let arg_definition_list = [ArgDefinition::builder()
            .id("long")
            .long_name("long")
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(42))
            .build()];

        let parsed_arg = parse_and_validate_arg(&arg_list, &arg_definition_list).unwrap();
        let expected_arg = ArgValuePair::new("long", ArgValue::UnsignedInt(65));

        assert_eq!(parsed_arg.0, expected_arg);
        assert_eq!(parsed_arg.1, 1); // consumed args
    }

    #[test]
    fn validate_long_arg_without_value_correctly() {
        let arg_list = ["--long".to_string()];
        let arg_definition_list = [ArgDefinition::builder()
            .id("long")
            .long_name("long")
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        let parsed_arg = parse_and_validate_arg(&arg_list, &arg_definition_list).unwrap();
        let expected_arg = ArgValuePair::new("long", ArgValue::Bool(true));

        assert_eq!(parsed_arg.0, expected_arg);
        assert_eq!(parsed_arg.1, 1); // consumed args
    }

    #[test]
    #[should_panic(expected = "requires an argument")]
    fn validate_short_arg_with_missing_value() {
        let arg_list = ["-s".to_string()];
        let arg_definition_list = [ArgDefinition::builder()
            .id("short")
            .short_name('s')
            .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
            .default_value(ArgValue::UnsignedInt(42))
            .build()];

        parse_and_validate_arg(&arg_list, &arg_definition_list).unwrap();
    }

    #[test]
    fn parse_full_arg_list_correctly() {
        let arg_list = [
            "--long-u32=42".to_string(),
            "--long-flag".to_string(),
            "-s".to_string(),
            "valid_string".to_string(),
            "-f".to_string(),
        ];
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("long_u32")
                .short_name('u')
                .long_name("long-u32")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .default_value(ArgValue::UnsignedInt(666))
                .build(),
            ArgDefinition::builder()
                .id("long_flag")
                .short_name('l')
                .long_name("long-flag")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("short_with_value")
                .short_name('s')
                .long_name("short-with-value")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                    vec![
                        "valid_string".to_string(),
                        "another_valid_string".to_string(),
                    ],
                )))
                .default_value(ArgValue::Str("default".to_string()))
                .build(),
            ArgDefinition::builder()
                .id("short_flag")
                .short_name('f')
                .long_name("short-flag")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
        ];

        let parsed_args =
            parse_and_validate_arg_list(&arg_list, &arg_definition_list).expect("test failed");
        let expected_args = vec![
            ArgValuePair::new("long_u32", ArgValue::UnsignedInt(42)),
            ArgValuePair::new("long_flag", ArgValue::Bool(true)),
            ArgValuePair::new(
                "short_with_value",
                ArgValue::Str("valid_string".to_string()),
            ),
            ArgValuePair::new("short_flag", ArgValue::Bool(true)),
        ];
        assert_eq!(parsed_args, expected_args);
    }

    #[test]
    fn stop_parsing_at_certain_arg() {
        let arg_list = [
            "--long-u32=42".to_string(),
            "--long-flag".to_string(),
            "-s".to_string(),
            "valid_string".to_string(),
            "-f".to_string(),
            "some_invalid_garbage".to_string(),
            "--another_invalid=".to_string(),
        ];
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("long_u32")
                .short_name('u')
                .long_name("long-u32")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(666))
                .build(),
            ArgDefinition::builder()
                .id("long_flag")
                .short_name('l')
                .long_name("long-flag")
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("short_with_value")
                .short_name('s')
                .long_name("short-with-value")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                    vec![
                        "valid_string".to_string(),
                        "another_valid_string".to_string(),
                    ],
                )))
                .stop_parsing(false)
                .default_value(ArgValue::Str("default".to_string()))
                .build(),
            ArgDefinition::builder()
                .id("short_flag")
                .short_name('f')
                .long_name("short-flag")
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
        ];

        let parsed_args =
            parse_and_validate_arg_list(&arg_list, &arg_definition_list).expect("test failed");
        let expected_args = vec![
            ArgValuePair::new("long_u32", ArgValue::UnsignedInt(42)),
            ArgValuePair::new("long_flag", ArgValue::Bool(true)),
        ];
        assert_eq!(parsed_args, expected_args);
    }

    #[test]
    fn duplicated_arg_but_the_value_matches() {
        let arg_list = [
            "--long-u32=42".to_string(),
            "--long-flag".to_string(),
            "-s".to_string(),
            "valid_string".to_string(),
            "-f".to_string(),
            "-u".to_string(), // repeated argument...
            "42".to_string(), // ...but with the same value
        ];
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("long_u32")
                .short_name('u')
                .long_name("long-u32")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(666))
                .build(),
            ArgDefinition::builder()
                .id("long_flag")
                .short_name('l')
                .long_name("long-flag")
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("short_with_value")
                .short_name('s')
                .long_name("short-with-value")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                    vec![
                        "valid_string".to_string(),
                        "another_valid_string".to_string(),
                    ],
                )))
                .stop_parsing(false)
                .default_value(ArgValue::Str("default".to_string()))
                .build(),
            ArgDefinition::builder()
                .id("short_flag")
                .short_name('f')
                .long_name("short-flag")
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
        ];

        let parsed_args =
            parse_and_validate_arg_list(&arg_list, &arg_definition_list).expect("test failed");
        let expected_args = vec![
            ArgValuePair::new("long_u32", ArgValue::UnsignedInt(42)),
            ArgValuePair::new("long_flag", ArgValue::Bool(true)),
            ArgValuePair::new(
                "short_with_value",
                ArgValue::Str("valid_string".to_string()),
            ),
            ArgValuePair::new("short_flag", ArgValue::Bool(true)),
        ];
        assert_eq!(parsed_args, expected_args);
    }

    #[test]
    #[should_panic(expected = "conflicting arguments for duplicated option")]
    fn conflicting_arguments() {
        let arg_list = [
            "--long-u32=42".to_string(),
            "--long-flag".to_string(),
            "-s".to_string(),
            "valid_string".to_string(),
            "-f".to_string(),
            "-u".to_string(), // duplicated argument...
            "43".to_string(), // with different value (42 vs 43)
        ];
        let arg_definition_list = [
            ArgDefinition::builder()
                .id("long_u32")
                .short_name('u')
                .long_name("long-u32")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(666))
                .build(),
            ArgDefinition::builder()
                .id("long_flag")
                .short_name('l')
                .long_name("long-flag")
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("short_with_value")
                .short_name('s')
                .long_name("short-with-value")
                .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                    vec![
                        "valid_string".to_string(),
                        "another_valid_string".to_string(),
                    ],
                )))
                .stop_parsing(false)
                .default_value(ArgValue::Str("default".to_string()))
                .build(),
            ArgDefinition::builder()
                .id("short_flag")
                .short_name('f')
                .long_name("short-flag")
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
        ];

        let parsed_args =
            parse_and_validate_arg_list(&arg_list, &arg_definition_list).expect("test failed");
        let expected_args = vec![
            ArgValuePair::new("long_u32", ArgValue::UnsignedInt(42)),
            ArgValuePair::new("long_flag", ArgValue::Bool(true)),
            ArgValuePair::new(
                "short_with_value",
                ArgValue::Str("valid_string".to_string()),
            ),
            ArgValuePair::new("short_flag", ArgValue::Bool(true)),
        ];
        assert_eq!(parsed_args, expected_args);
    }
}
