use super::ArgValue;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueKind {
    Int,
    UnsignedInt,
    OneOfStr(Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgKind {
    Flag,
    Value(ValueKind),
}

#[derive(Debug, Clone)]
pub struct Arg {
    id: String,
    short_name: Option<char>,
    long_name: Option<String>,
    description: Vec<String>,
    kind: ArgKind,
    stop_parsing: bool,
    default_value: ArgValue,
}

impl Arg {
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

    pub fn kind(&self) -> &ArgKind {
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
    id: String,
    short_name: Option<char>,
    long_name: Option<String>,
    description: Vec<String>,
    kind: Option<ArgKind>,
    stop_parsing: bool,
    default_value: Option<ArgValue>,
}

impl ArgDefinitionBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
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

    pub fn kind(mut self, kind: ArgKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn stop_parsing(mut self, stop_parsing: bool) -> Self {
        self.stop_parsing = stop_parsing;
        self
    }

    pub fn default_value(mut self, default_value: ArgValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn build(self) -> Arg {
        assert!(!self.id.is_empty(), "id is required");
        assert!(
            !(self.short_name.is_none() && self.long_name.is_none()),
            "either short or long name is required"
        );
        self.validate_arg_type();
        Arg {
            id: self.id,
            short_name: self.short_name,
            long_name: self.long_name,
            description: self.description,
            kind: self.kind.expect("kind is required"),
            stop_parsing: self.stop_parsing,
            default_value: self.default_value.expect("default value is required"),
        }
    }

    fn validate_arg_type(&self) {
        match &self.kind {
            Some(ArgKind::Flag) => {
                assert!(
                    self.default_value.is_none()
                        || matches!(self.default_value, Some(ArgValue::Bool(_))),
                    "default value must be of type bool"
                );
            }
            Some(ArgKind::Value(kind)) => match kind {
                ValueKind::Int => {
                    assert!(
                        self.default_value.is_none()
                            || matches!(self.default_value, Some(ArgValue::Int(_))),
                        "default value must be of type i32"
                    );
                }
                ValueKind::UnsignedInt => {
                    assert!(
                        self.default_value.is_none()
                            || matches!(self.default_value, Some(ArgValue::UnsignedInt(_))),
                        "default value must be of type u32"
                    );
                }
                ValueKind::OneOfStr(_) => {
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

pub fn match_arg<'a>(
    arg: &str,
    arg_name: &str,
    is_long_name: bool,
    arg_definition_list: &'a [Arg],
) -> Result<&'a Arg, String> {
    if let Some(arg_definition) = arg_definition_list.iter().find(|elem| {
        if is_long_name {
            if let Some(long_name) = &elem.long_name() {
                return *long_name == arg_name;
            }
            false
        } else {
            if let Some(short_name) = elem.short_name() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_arg_definition() {
        let arg_definition = Arg::builder()
            .id("some_arg")
            .short_name('s')
            .long_name("some-arg")
            .description(vec!["some description".to_string()])
            .kind(ArgKind::Flag)
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
        assert_eq!(arg_definition.kind, ArgKind::Flag);
        assert!(arg_definition.stop_parsing);
        assert_eq!(arg_definition.default_value, ArgValue::Bool(false));
    }

    #[test]
    #[should_panic(expected = "id is required")]
    fn arg_definition_requires_id() {
        Arg::builder()
            .short_name('s')
            .long_name("some-arg")
            .kind(ArgKind::Flag)
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "either short or long name is required")]
    fn arg_definition_requires_short_or_long_name() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKind::Flag)
            .default_value(ArgValue::Bool(false))
            .build(); // passes

        Arg::builder()
            .id("some_arg")
            .long_name("long")
            .kind(ArgKind::Flag)
            .default_value(ArgValue::Bool(false))
            .build(); // passes

        Arg::builder()
            .id("some_arg")
            .kind(ArgKind::Flag)
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "kind is required")]
    fn arg_definition_requires_kind() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value is required")]
    fn arg_definition_requires_default_value() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKind::Flag)
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type bool")]
    fn flag_arg_definition_requires_bool_default_value() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKind::Flag)
            .default_value(ArgValue::UnsignedInt(42))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type u32")]
    fn u32_arg_definition_requires_u32_default_value() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKind::Value(ValueKind::UnsignedInt))
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type i32")]
    fn i32_arg_definition_requiresi_32_default_value() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKind::Value(ValueKind::Int))
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    #[should_panic(expected = "default value must be of type String")]
    fn string_arg_definition_requires_string_default_value() {
        Arg::builder()
            .id("some_arg")
            .short_name('s')
            .kind(ArgKind::Value(ValueKind::OneOfStr(vec![])))
            .default_value(ArgValue::Bool(false))
            .build();
    }

    #[test]
    fn arg_not_matching_any_definition() {
        let arg = "my_arg";
        let arg_name = "my_arg_name";
        let arg_definition_list = [
            Arg::builder()
                .id("another_arg")
                .short_name('s')
                .long_name("different_name")
                .kind(ArgKind::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id("different_arg")
                .short_name('a')
                .long_name("another_name")
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .default_value(ArgValue::UnsignedInt(42))
                .build(),
        ];

        let result = match_arg(arg, arg_name, false, &arg_definition_list);
        assert!(result.is_err());
    }

    #[test]
    fn arg_matching_by_short_name() {
        let arg = "my_arg";
        let arg_name = "a";
        let arg_definition_list = [
            Arg::builder()
                .id("another_arg")
                .short_name('s')
                .long_name("different_name")
                .kind(ArgKind::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id("different_arg")
                .short_name('a')
                .long_name("another_name")
                .kind(ArgKind::Value(ValueKind::Int))
                .default_value(ArgValue::Int(42))
                .build(),
        ];

        let result = match_arg(arg, arg_name, false, &arg_definition_list);
        assert!(result.is_ok());
    }

    #[test]
    fn arg_matching_by_long_name() {
        let arg = "my_arg";
        let arg_name = "long_name";
        let arg_definition_list = [
            Arg::builder()
                .id("another_arg")
                .short_name('s')
                .long_name("long_name")
                .kind(ArgKind::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id("different_arg")
                .short_name('a')
                .long_name("another_name")
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .default_value(ArgValue::UnsignedInt(42))
                .build(),
        ];

        let result = match_arg(arg, arg_name, true, &arg_definition_list);
        assert!(result.is_ok());
    }
}
