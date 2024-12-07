use super::{ArgValue, ArgValuePair};
use super::definition::Arg;

pub trait SetFromArg {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[Arg],
    ) -> Self;
}

pub fn find_arg<'a>(arg_id: &str, arg_list: &'a [ArgValuePair]) -> Option<&'a ArgValuePair> {
    arg_list.iter().find(|elem| elem.id == arg_id)
}

impl SetFromArg for i32 {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[Arg],
    ) -> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::Int(val) = arg.value {
                val
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id() == arg_id) {
            if let ArgValue::Int(val) = &arg_definition.default_value() {
                *val
            } else {
                panic!("invalid type for default value of option: '{arg_id}'");
            }
        } else {
            panic!("missing argument definition for option: '{arg_id}'");
        }
    }
}

impl SetFromArg for u32 {
    fn set_value_from_arg_or_default(
        arg_id: &str,
        arg_list: &[ArgValuePair],
        arg_definitions: &[Arg],
    ) -> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::UnsignedInt(val) = arg.value {
                val
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id() == arg_id) {
            if let ArgValue::UnsignedInt(val) = &arg_definition.default_value() {
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
        arg_definitions: &[Arg],
    ) -> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::Bool(val) = arg.value {
                val
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id() == arg_id) {
            if let ArgValue::Bool(val) = &arg_definition.default_value() {
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
        arg_definitions: &[Arg],
    ) -> Self {
        if let Some(arg) = find_arg(arg_id, arg_list) {
            if let ArgValue::Str(val) = &arg.value {
                val.clone()
            } else {
                panic!("invalid type for option: '{arg_id}'");
            }
        } else if let Some(arg_definition) = arg_definitions.iter().find(|elem| elem.id() == arg_id)
        {
            if let ArgValue::Str(val) = &arg_definition.default_value() {
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
    use crate::args::definition::{ArgKind, ValueKind};

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
        let arg_definitions = [Arg::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKind::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];
        let val = bool::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);

        assert_eq!(val, expected);
    }

    #[test]
    fn i32_assign_value_from_arg_list() {
        let expected = 42;
        let arg_id = "some_arg";
        let arg_list = [ArgValuePair::new(
            "some_arg",
            ArgValue::Int(expected),
        )];
        let arg_definitions = [Arg::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKind::Value(ValueKind::Int))
            .default_value(ArgValue::Int(0))
            .build()];
        let val = i32::set_value_from_arg_or_default(arg_id, &arg_list, &arg_definitions);

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
        let arg_definitions = [Arg::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKind::Value(ValueKind::UnsignedInt))
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

        let arg_definitions = [Arg::builder()
            .id(arg_id)
            .short_name('s')
            .kind(ArgKind::Value(ValueKind::OneOfStr(
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
        let arg_definitions = [Arg::builder()
            .id(arg_id)
            .short_name('a')
            .kind(ArgKind::Value(ValueKind::UnsignedInt))
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
}
