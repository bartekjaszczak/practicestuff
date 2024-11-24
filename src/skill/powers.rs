use super::{Question, SkillBase};
use crate::application::APP_NAME;
use crate::args::prelude::*;

pub const CMD_POWERS: &str = "powers";

const ARG_ID_HELP: &str = "help";
const ARG_ID_BASE: &str = "base";
const ARG_ID_LOWER_BOUNDARY: &str = "lower_boundary";
const ARG_ID_UPPER_BOUNDARY: &str = "upper_boundary";

#[derive(Debug)]
pub struct Powers {
    show_help: bool,

    base: u32,
    lower_boundary: u32,
    upper_boundary: u32,
}

enum OptionType {
    ShowHelp,
    Base,
    LowerBoundary,
    UpperBoundary,
}

impl Powers {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::get_arg_definitions();
        let parsed_args = parser::parse_and_validate_arg_list(args, &arg_definitions)
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
        let definitions = &Powers::get_arg_definitions();
        let options = help::Options::new("Powers options", definitions);
        let help_text = help::build(&Powers::usage(), &options, &[]);
        println!("{help_text}");
    }

    fn get_arg_definitions() -> Vec<ArgDefinition> {
        vec![
            ArgDefinition::builder()
                .id(ARG_ID_HELP)
                .short_name('h')
                .long_name("help")
                .description(vec!["Display help for powers command.".to_string()])
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_BASE)
                .short_name('b')
                .long_name("base")
                .description(vec!["Set the base for powers (default: 2).".to_string()])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(2))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_LOWER_BOUNDARY)
                .short_name('l')
                .long_name("lower-boundary")
                .description(vec![
                    "Set the minimum exponent to use in questions (default: 1).".to_string(),
                ])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(1))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_UPPER_BOUNDARY)
                .short_name('u')
                .long_name("upper-boundary")
                .description(vec![
                    "Set the maximum exponent to use in questions (default: 16).".to_string(),
                ])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(16))
                .build(),
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
    fn generate_questions(&self, count: u32) -> Vec<Question> {
        todo!()
    }

    fn show_help_and_exit(&self) -> bool {
        if self.show_help {
            Self::print_help();
            return true;
        }

        false
    }
}
