use super::ArgValue;

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
    id: String,
    short_name: Option<char>,
    long_name: Option<String>,
    description: Vec<String>,
    kind: Option<ArgKindDefinition>,
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

    pub fn kind(mut self, kind: ArgKindDefinition) -> Self {
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

    pub fn build(self) -> ArgDefinition {
        assert!(!self.id.is_empty(), "id is required");
        assert!(
            !(self.short_name.is_none() && self.long_name.is_none()),
            "either short or long name is required"
        );
        self.validate_arg_type();
        ArgDefinition {
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

pub fn match_arg_to_definition<'a>(
    arg: &str,
    arg_name: &str,
    is_long_name: bool,
    arg_definition_list: &'a [ArgDefinition],
) -> Result<&'a ArgDefinition, String> {
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
}
