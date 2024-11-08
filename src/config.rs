use crate::application::{self, Application};
use crate::args::{
    self, ArgDefinition, ArgKindDefinition, ArgValue, SetFromArg, ValueKindDefinition,
};
use crate::skill::doomsday_algorithm::CMD_DOOMSDAY_ALGORITHM;
use crate::skill::powers::CMD_POWERS;
use crate::skill::times_table::CMD_TIMES_TABLE;
use crate::skill::{self, Skill};

use std::cmp;

const COMMANDS: [&str; 3] = [CMD_POWERS, CMD_TIMES_TABLE, CMD_DOOMSDAY_ALGORITHM];

const ARG_ID_HELP: &str = "help";
const ARG_ID_VERSION: &str = "version";
const ARG_ID_NUMBER_OF_QUESTIONS: &str = "num_of_questions";
const ARG_ID_DISABLE_LIVE_STATISTICS: &str = "disable_live_stats";
const ARG_ID_BEHAVIOUR_ON_ERROR: &str = "behaviour_on_err";

const BEHAVIOUR_ON_ERROR_CONTINUE: &str = "continue";
const BEHAVIOUR_ON_ERROR_SHOW_CORRECT: &str = "showcorrect";
const BEHAVIOUR_ON_ERROR_REPEAT: &str = "repeat";

#[derive(Debug)]
pub struct Config {
    options: GeneralOptions,
    skill: Option<Box<dyn Skill>>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(Self::build_err_message(None));
        }
        let (options, command, command_options) = Config::split_args(&args[1..]);

        let options = GeneralOptions::build(options);
        if let Ok(options) = &options {
            if options.show_help || options.show_version {
                return Ok(Self {
                    options: *options,
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
enum BehaviourOnError {
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

#[derive(Debug, Copy, Clone)]
struct GeneralOptions {
    show_help: bool,
    show_version: bool,

    number_of_questions: u32,
    disable_live_statistics: bool,
    behaviour_on_error: BehaviourOnError,
}

impl GeneralOptions {
    fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::get_arg_definitions();
        let parsed_args = args::parse_and_validate_arg_list(args, &arg_definitions)?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let show_version =
            bool::set_value_from_arg_or_default(ARG_ID_VERSION, &parsed_args, &arg_definitions);
        let number_of_questions = u32::set_value_from_arg_or_default(
            ARG_ID_NUMBER_OF_QUESTIONS,
            &parsed_args,
            &arg_definitions,
        );
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

    fn get_arg_definitions() -> Vec<ArgDefinition> {
        vec![
            ArgDefinition {
                id: ARG_ID_HELP.to_string(),
                short_name: Some('h'),
                long_name: Some("help".to_string()),
                kind: ArgKindDefinition::Flag,
                stop_parsing: true,
                default_value: ArgValue::Bool(false),
            },
            ArgDefinition {
                id: ARG_ID_VERSION.to_string(),
                short_name: Some('v'),
                long_name: Some("version".to_string()),
                kind: ArgKindDefinition::Flag,
                stop_parsing: true,
                default_value: ArgValue::Bool(false),
            },
            ArgDefinition {
                id: ARG_ID_NUMBER_OF_QUESTIONS.to_string(),
                short_name: Some('n'),
                long_name: Some("number-of-questions".to_string()),
                kind: ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt),
                stop_parsing: false,
                default_value: ArgValue::UnsignedInt(20),
            },
            ArgDefinition {
                id: ARG_ID_DISABLE_LIVE_STATISTICS.to_string(),
                short_name: Some('d'),
                long_name: Some("disable-live-statistics".to_string()),
                kind: ArgKindDefinition::Flag,
                stop_parsing: false,
                default_value: ArgValue::Bool(false),
            },
            ArgDefinition {
                id: ARG_ID_BEHAVIOUR_ON_ERROR.to_string(),
                short_name: Some('b'),
                long_name: Some("behaviour-on-error".to_string()),
                kind: ArgKindDefinition::Value(ValueKindDefinition::OneOfStr(vec![
                    BEHAVIOUR_ON_ERROR_CONTINUE.to_string(),
                    BEHAVIOUR_ON_ERROR_SHOW_CORRECT.to_string(),
                    BEHAVIOUR_ON_ERROR_REPEAT.to_string(),
                ])),
                stop_parsing: false,
                default_value: ArgValue::Str("showcorrect".to_string()),
            },
        ]
    }
}
