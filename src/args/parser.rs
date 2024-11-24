use super::definition::*;
use super::*;

type IsLongName = bool;

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
        definition::match_arg_to_definition(next_arg, arg_name, is_long_name, arg_definition_list)?;
    if let ArgKindDefinition::Value(_) = arg_definition.kind() {
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

    if arg_definition.stop_parsing() {
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


fn validate_and_create_arg(
    arg: &str,
    arg_value: Option<&str>,
    arg_definition: &ArgDefinition,
) -> Result<ArgValuePair, String> {
    let value = match &arg_definition.kind() {
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

    Ok(ArgValuePair::new(&arg_definition.id(), value))
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

#[cfg(test)]
mod tests {
    use super::*;

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
