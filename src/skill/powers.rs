use super::{QuestionAndAnswer, SkillBase};
use crate::application::APP_NAME;

pub const CMD_POWERS: &str = "powers";

const DEFAULT_ARGUMENT_BASE: u32 = 2;
const DEFAULT_ARGUMENT_LOWER_BOUNDARY: u32 = 1;
const DEFAULT_ARGUMENT_UPPER_BOUNDARY: u32 = 10;

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
        let mut show_help = false;
        let mut base = DEFAULT_ARGUMENT_BASE;
        let mut lower_boundary = DEFAULT_ARGUMENT_LOWER_BOUNDARY;
        let mut upper_boundary = DEFAULT_ARGUMENT_UPPER_BOUNDARY;

        let mut option_type = None;
        let mut expecting_value = false;

        let mut last_option = String::new();

        for arg in args {
            if expecting_value {
                if let Some(option) = option_type {
                    match option {
                        OptionType::Base => {
                            base = Powers::parse_u32_argument(arg)?;
                        }
                        OptionType::LowerBoundary => {
                            lower_boundary = Powers::parse_u32_argument(arg)?;
                        }
                        OptionType::UpperBoundary => {
                            upper_boundary = Powers::parse_u32_argument(arg)?;
                        }
                        _ => (),
                    }
                }
                expecting_value = false;
                option_type = None;
            } else {
                last_option.clone_from(arg);
                let (option, value_required) = Powers::parse_option_type(arg)?;
                match option {
                    OptionType::ShowHelp => {
                        show_help = true;
                        break;
                    }
                    _ => option_type = Some(option),
                }
                expecting_value = value_required;
            }
        }

        if expecting_value {
            return Err(build_err_message(Some(format!(
                "'{last_option}' option requires an argument"
            ))));
        }

        if lower_boundary > upper_boundary {
            return Err(build_err_message(Some(
                "lower boundary cannot be greater than upper boundary".to_string(),
            )));
        }

        Ok(Powers {
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

    fn parse_option_type(option: &str) -> Result<(OptionType, RequiresValue), String> {
        match option {
            "--help" | "-h" => Ok((OptionType::ShowHelp, false)),
            "--base" | "-b" => Ok((OptionType::Base, true)),
            "--lower-boundary" | "-l" => Ok((OptionType::LowerBoundary, true)),
            "--upper-boundary" | "-u" => Ok((OptionType::UpperBoundary, true)),
            _ => Err(build_err_message(Some(format!(
                "unrecognised option: '{option}'"
            )))),
        }
    }

    fn parse_u32_argument(value: &str) -> Result<u32, String> {
        match value.parse::<u32>() {
            Ok(number) => Ok(number),
            _ => Err(build_err_message(Some(format!(
                "incorrect option argument: '{value}'"
            )))),
        }
    }
}

impl SkillBase for Powers {
    fn generate_questions_and_answers(&self, count: u32) -> Vec<QuestionAndAnswer> {
        todo!()
    }
}

fn build_err_message(msg: Option<String>) -> String {
    if let Some(msg) = msg {
        format!(
            "{}: {}: {}\n{}\n{}",
            APP_NAME,
            CMD_POWERS,
            msg,
            Powers::usage(),
            Powers::help_prompt()
        )
    } else {
        format!("{}\n{}", Powers::usage(), Powers::help_prompt())
    }
}
