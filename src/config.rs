use crate::application::{self, Application};
use crate::args::prelude::*;
use crate::skill::doomsday_algorithm;
use crate::skill::powers;
use crate::skill::times_table;
use crate::skill::{self, Skill};

use std::cmp;

const COMMANDS: [&str; 3] = [powers::CMD, times_table::CMD, doomsday_algorithm::CMD];

const ARG_ID_HELP: &str = "help";
const ARG_ID_VERSION: &str = "version";
const ARG_ID_NUMBER_OF_QUESTIONS: &str = "num_of_questions";
const ARG_ID_DISABLE_LIVE_STATISTICS: &str = "disable_live_stats";
const ARG_ID_BEHAVIOUR_ON_ERROR: &str = "behaviour_on_err";

const BEHAVIOUR_ON_ERROR_CONTINUE: &str = "continue";
const BEHAVIOUR_ON_ERROR_SHOW_CORRECT: &str = "showcorrect";
const BEHAVIOUR_ON_ERROR_REPEAT: &str = "repeat";

#[derive(Debug, Clone, Copy)]
pub enum NumberOfQuestions {
    Limited(u32),
    Infinite,
}

#[derive(Debug)]
pub struct Config {
    pub options: GeneralOptions,
    pub skill: Option<Box<dyn Skill>>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(Self::build_err_message(None));
        }
        let (options, command, command_options) = Config::split_args(&args[1..]);

        let options = GeneralOptions::build(options);
        if let Ok(options) = options {
            if options.show_help || options.show_version {
                return Ok(Self {
                    options,
                    skill: None,
                });
            }
        }

        let options = options.map_err(|err| Self::build_err_message(Some(err)))?;
        let Some(command) = command else {
            return Err(Self::build_err_message(Some("missing command".to_string())));
        };
        let skill = skill::build(&command, command_options)?;

        Ok(Self {
            options,
            skill: Some(skill),
        })
    }

    fn split_args(args: &[String]) -> (&[String], Option<String>, &[String]) {
        let mut command = None;
        let mut command_pos = args.len();
        for (i, arg) in args.iter().enumerate() {
            if COMMANDS.contains(&arg.as_str()) {
                command = Some(arg.clone());
                command_pos = i;
                break;
            }
        }
        let args_pos = cmp::min(command_pos + 1, args.len());
        (&args[..command_pos], command, &args[args_pos..])
    }

    fn build_err_message(msg: Option<String>) -> String {
        if let Some(msg) = msg {
            format!(
                "{}: {}\n{}\n{}",
                application::APP_NAME,
                msg,
                Application::usage(),
                Application::help_prompt()
            )
        } else {
            format!("{}\n{}", Application::usage(), Application::help_prompt())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BehaviourOnError {
    NextQuestion,
    ShowCorrect,
    Repeat,
}

impl BehaviourOnError {
    fn from_string(value: &str) -> BehaviourOnError {
        match value {
            BEHAVIOUR_ON_ERROR_CONTINUE => BehaviourOnError::NextQuestion,
            BEHAVIOUR_ON_ERROR_SHOW_CORRECT => BehaviourOnError::ShowCorrect,
            BEHAVIOUR_ON_ERROR_REPEAT => BehaviourOnError::Repeat,
            _ => panic!("incorrect value for BehaviourOnError"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GeneralOptions {
    pub show_help: bool,
    pub show_version: bool,

    pub number_of_questions: NumberOfQuestions,
    pub disable_live_statistics: bool,
    pub behaviour_on_error: BehaviourOnError,
}

impl GeneralOptions {
    fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::get_arg_definitions();
        let parsed_args = parser::parse_and_validate_arg_list(args, &arg_definitions)?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let show_version =
            bool::set_value_from_arg_or_default(ARG_ID_VERSION, &parsed_args, &arg_definitions);
        let number_of_questions = u32::set_value_from_arg_or_default(
            ARG_ID_NUMBER_OF_QUESTIONS,
            &parsed_args,
            &arg_definitions,
        );
        let number_of_questions = if number_of_questions == 0 {
            NumberOfQuestions::Infinite
        } else {
            NumberOfQuestions::Limited(number_of_questions)
        };
        let disable_live_statistics = bool::set_value_from_arg_or_default(
            ARG_ID_DISABLE_LIVE_STATISTICS,
            &parsed_args,
            &arg_definitions,
        );
        let behaviour_on_error = String::set_value_from_arg_or_default(
            ARG_ID_BEHAVIOUR_ON_ERROR,
            &parsed_args,
            &arg_definitions,
        );
        let behaviour_on_error = BehaviourOnError::from_string(&behaviour_on_error);

        Ok(Self {
            show_help,
            show_version,
            number_of_questions,
            disable_live_statistics,
            behaviour_on_error,
        })
    }

    pub fn get_arg_definitions() -> Vec<ArgDefinition> {
        vec![
            ArgDefinition::builder()
                .id(ARG_ID_HELP)
                .short_name('h')
                .long_name("help")
                .description(vec!["Display this help message.".to_string()])
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_VERSION)
                .short_name('v')
                .long_name("version")
                .description(vec!["Show version information.".to_string()])
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_NUMBER_OF_QUESTIONS)
                .short_name('n')
                .long_name("number-of-questions")
                .description(vec![
                    "Specify the number of questions to ask (0 for infinite, default: 20)"
                        .to_string(),
                ])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(20))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_DISABLE_LIVE_STATISTICS)
                .short_name('d')
                .long_name("disable-live-statistics")
                .description(vec![
                    "Disable live statistics; statistics will not display between questions."
                        .to_string(),
                ])
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_BEHAVIOUR_ON_ERROR)
                .short_name('b')
                .long_name("behaviour-on-error")
                .description(vec![
                    "Define behaviour on incorrect answer (default: showcorrect):".to_string(),
                    "  - continue: proceed to the next question.".to_string(),
                    "  - showcorrect: proceed to the next question and display the correct answer."
                        .to_string(),
                    "  - repeat: ask the question again until the correct answer is provided."
                        .to_string(),
                ])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(
                    vec![
                        BEHAVIOUR_ON_ERROR_CONTINUE.to_string(),
                        BEHAVIOUR_ON_ERROR_SHOW_CORRECT.to_string(),
                        BEHAVIOUR_ON_ERROR_REPEAT.to_string(),
                    ],
                )))
                .stop_parsing(false)
                .default_value(ArgValue::Str("showcorrect".to_string()))
                .build(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Config:
    //  - build: args < 2 triggers error msg
    //  - build: show help / show version returns no skill but options are set
    //  - build: missing command
    //  - build: correct build with skill
    //  - split args tests
    //  - build err message tests
    // Behaviour on error:
    //  - translation string -> enum (from_string)
    // General options:
    //  - build: 1 or 2 successful builds with varying options
}
