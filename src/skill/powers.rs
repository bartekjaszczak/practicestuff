use super::{QuestionAndAnswer, SkillBase};
use crate::application::APP_NAME;
use crate::args::{self, ArgDefinition, SetFromArg};

pub const CMD_POWERS: &str = "powers";

const ARG_ID_HELP: &str = "help";
const ARG_ID_BASE: &str = "base";
const ARG_ID_LOWER_BOUNDARY: &str = "lower_boundary";
const ARG_ID_UPPER_BOUNDARY: &str = "upper_boundary";

type RequiresValue = bool;

#[derive(Debug)]
pub struct Powers {
    show_help: bool,

    base: u32,
    lower_boundary: u32,
    upper_boundary: u32,
}

// -b, --base
// -l, --lower-boundary
// -u, --upper-boundary

enum OptionType {
    ShowHelp,
    Base,
    LowerBoundary,
    UpperBoundary,
}

impl Powers {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::get_arg_definitions();
        let parsed_args = args::parse_and_validate_arg_list(args, &arg_definitions)
            .map_err(|err| Self::build_err_message(Some(err)))?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let base = u32::set_value_from_arg_or_default(ARG_ID_BASE, &parsed_args, &arg_definitions);
        let lower_boundary = u32::set_value_from_arg_or_default(
            ARG_ID_LOWER_BOUNDARY,
            &parsed_args,
            &arg_definitions,
        );
        let upper_boundary = u32::set_value_from_arg_or_default(
            ARG_ID_UPPER_BOUNDARY,
            &parsed_args,
            &arg_definitions,
        );

        if lower_boundary > upper_boundary {
            return Err(Self::build_err_message(Some(
                "lower boundary must be less than or equal to upper boundary".to_string(),
            )));
        }

        Ok(Self {
            show_help,
            base,
            lower_boundary,
            upper_boundary,
        })
    }

    fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... powers [powers_option]")
    }

    fn help_prompt() -> String {
        format!("Try '{APP_NAME} powers --help' for more information.")
    }

    fn print_help() {
        todo!()
    }

    fn get_arg_definitions() -> Vec<ArgDefinition> {
        vec![
            ArgDefinition {
                id: ARG_ID_HELP.to_string(),
                short_name: Some('h'),
                long_name: Some("help".to_string()),
                kind: args::ArgKindDefinition::Flag,
                stop_parsing: true,
                default_value: args::ArgValue::Bool(false),
            },
            ArgDefinition {
                id: ARG_ID_BASE.to_string(),
                short_name: Some('b'),
                long_name: Some("base".to_string()),
                kind: args::ArgKindDefinition::Value(args::ValueKindDefinition::UnsignedInt),
                stop_parsing: false,
                default_value: args::ArgValue::UnsignedInt(2),
            },
            ArgDefinition {
                id: ARG_ID_LOWER_BOUNDARY.to_string(),
                short_name: Some('l'),
                long_name: Some("lower-boundary".to_string()),
                kind: args::ArgKindDefinition::Value(args::ValueKindDefinition::UnsignedInt),
                stop_parsing: false,
                default_value: args::ArgValue::UnsignedInt(1),
            },
            ArgDefinition {
                id: ARG_ID_UPPER_BOUNDARY.to_string(),
                short_name: Some('u'),
                long_name: Some("upper-boundary".to_string()),
                kind: args::ArgKindDefinition::Value(args::ValueKindDefinition::UnsignedInt),
                stop_parsing: false,
                default_value: args::ArgValue::UnsignedInt(10),
            },
        ]
    }

    fn build_err_message(msg: Option<String>) -> String {
        if let Some(msg) = msg {
            format!(
                "{}: {}: {}\n{}\n{}",
                APP_NAME,
                CMD_POWERS,
                msg,
                Self::usage(),
                Self::help_prompt()
            )
        } else {
            format!("{}\n{}", Self::usage(), Self::help_prompt())
        }
    }
}

impl SkillBase for Powers {
    fn generate_questions_and_answers(&self, count: u32) -> Vec<QuestionAndAnswer> {
        todo!()
    }
}
