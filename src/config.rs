use std::cmp;

use crate::application::{self, Application};
use crate::args::prelude::*;
use crate::skill::doomsday_algorithm;
use crate::skill::powers;
use crate::skill::times_table;
use crate::skill::{self, Skill};

const COMMANDS: [&str; 3] = [powers::CMD, times_table::CMD, doomsday_algorithm::CMD];

const ARG_ID_HELP: &str = "help";
const ARG_ID_VERSION: &str = "version";
const ARG_ID_NUMBER_OF_QUESTIONS: &str = "num_of_questions";
const ARG_ID_DISABLE_LIVE_STATISTICS: &str = "disable_live_stats";
const ARG_ID_BEHAVIOUR_ON_ERROR: &str = "behaviour_on_err";

const BEHAVIOUR_ON_ERROR_CONTINUE: &str = "continue";
const BEHAVIOUR_ON_ERROR_SHOW_CORRECT: &str = "showcorrect";
const BEHAVIOUR_ON_ERROR_REPEAT: &str = "repeat";

#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Builds a config from a list of `args`.
    ///
    /// # Examples
    ///
    /// ```
    /// use practicestuff::Config;
    ///
    /// let args = ["cmd".to_string(), "-h".to_string()];
    /// let config = Config::build(&args).unwrap();
    /// ```
    /// # Errors
    ///
    /// This function will return an error if any part of parsing goes wrong.
    /// That include, but is not limited to:
    ///
    /// - unrecognised option/command option
    /// - missing or incorrect option argument
    /// - missing command
    /// - ambiguous option/command options arguments
    /// - impossible combination of options/command options
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(Self::build_err_message(None));
        }
        let (options, command, command_options) = Config::split_args(&args[1..]);

        let options = GeneralOptions::build(options);
        if let Ok(options) = &options {
            if options.show_help || options.show_version {
                return Ok(Self {
                    options: options.clone(),
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct GeneralOptions {
    pub arg_definitions: Vec<Arg>,
    pub show_help: bool,
    pub show_version: bool,

    pub number_of_questions: NumberOfQuestions,
    pub disable_live_statistics: bool,
    pub behaviour_on_error: BehaviourOnError,
}

impl GeneralOptions {
    fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::build_arg_definitions();
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
            arg_definitions,
            show_help,
            show_version,
            number_of_questions,
            disable_live_statistics,
            behaviour_on_error,
        })
    }

    fn build_arg_definitions() -> Vec<Arg> {
        vec![
            Arg::builder()
                .id(ARG_ID_HELP)
                .short_name('h')
                .long_name("help")
                .description(vec!["Display this help message.".to_string()])
                .kind(ArgKind::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id(ARG_ID_VERSION)
                .short_name('v')
                .long_name("version")
                .description(vec!["Show version information.".to_string()])
                .kind(ArgKind::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id(ARG_ID_NUMBER_OF_QUESTIONS)
                .short_name('n')
                .long_name("number-of-questions")
                .description(vec![
                    "Specify the number of questions to ask".to_string(),
                    "(0 for infinite, default: 20).".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(20))
                .build(),
            Arg::builder()
                .id(ARG_ID_DISABLE_LIVE_STATISTICS)
                .short_name('d')
                .long_name("disable-live-statistics")
                .description(vec![
                    "Disable live statistics; statistics will not".to_string(),
                    "display between questions.".to_string(),
                ])
                .kind(ArgKind::Flag)
                .stop_parsing(false)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id(ARG_ID_BEHAVIOUR_ON_ERROR)
                .short_name('b')
                .long_name("behavior-on-error")
                .description(vec![
                    "Define behaviour on incorrect answer:".to_string(),
                    "(default: showcorrect):".to_string(),
                    "  - continue: proceed to the next question.".to_string(),
                    "  - showcorrect: proceed to the next".to_string(),
                    "    question and display the correct answer.".to_string(),
                    "  - repeat: ask the question again until".to_string(),
                    "    the correct answer is provided.".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::OneOfStr(vec![
                    BEHAVIOUR_ON_ERROR_CONTINUE.to_string(),
                    BEHAVIOUR_ON_ERROR_SHOW_CORRECT.to_string(),
                    BEHAVIOUR_ON_ERROR_REPEAT.to_string(),
                ])))
                .stop_parsing(false)
                .default_value(ArgValue::Str("showcorrect".to_string()))
                .build(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Usage")]
    fn build_not_enough_args_0() {
        let args = [];
        Config::build(&args).expect("should fail due to missing args");
    }

    #[test]
    #[should_panic(expected = "Usage")]
    fn build_not_enough_args_1() {
        let args = ["some_arg".to_string()];
        Config::build(&args).expect("should fail due to missing args");
    }

    #[test]
    fn build_show_help() {
        let args = ["command".to_string(), "--help".to_string()];
        let config = Config::build(&args).expect("should build successfully");
        assert!(config.options.show_help);
        assert!(!config.options.show_version);
        assert!(config.skill.is_none());
    }

    #[test]
    fn build_show_version() {
        let args = ["command".to_string(), "--version".to_string()];
        let config = Config::build(&args).expect("should build successfully");
        assert!(!config.options.show_help);
        assert!(config.options.show_version);
        assert!(config.skill.is_none());
    }

    #[test]
    #[should_panic(expected = "missing command")]
    fn build_missing_command() {
        let args = [
            "command".to_string(),
            "--number-of-questions=10".to_string(),
        ];
        Config::build(&args).expect("should fail due to missing command");
    }

    #[test]
    #[should_panic(expected = "invalid option")]
    fn build_unrecognised_command() {
        let args = ["command".to_string(), "unrecognised".to_string()];
        Config::build(&args).expect("should fail due to unrecognised command");
    }

    #[test]
    fn build_successful_no_args() {
        let args = ["command".to_string(), "powers".to_string()];
        let config = Config::build(&args).expect("should build successfully");
        assert!(!config.options.show_help);
        assert!(!config.options.show_version);
        assert_eq!(
            config.options.number_of_questions,
            NumberOfQuestions::Limited(20)
        );
        assert!(!config.options.disable_live_statistics);
        assert_eq!(
            config.options.behaviour_on_error,
            BehaviourOnError::ShowCorrect
        );
        assert!(config.skill.is_some());
    }

    #[test]
    fn build_successful_all_args_used() {
        let args = [
            "command".to_string(),
            "--number-of-questions=10".to_string(),
            "--disable-live-statistics".to_string(),
            "--behavior-on-error=repeat".to_string(),
            "powers".to_string(),
        ];
        let config = Config::build(&args).expect("should build successfully");
        assert!(!config.options.show_help);
        assert!(!config.options.show_version);
        assert_eq!(
            config.options.number_of_questions,
            NumberOfQuestions::Limited(10)
        );
        assert!(config.options.disable_live_statistics);
        assert_eq!(config.options.behaviour_on_error, BehaviourOnError::Repeat);
        assert!(config.skill.is_some());

        // Different set of args
        let args = [
            "command".to_string(),
            "-n".to_string(),
            "0".to_string(),
            "-d".to_string(),
            "-b".to_string(),
            "continue".to_string(),
            "powers".to_string(),
        ];
        let config = Config::build(&args).expect("should build successfully");
        assert!(!config.options.show_help);
        assert!(!config.options.show_version);
        assert_eq!(
            config.options.number_of_questions,
            NumberOfQuestions::Infinite
        );
        assert!(config.options.disable_live_statistics);
        assert_eq!(
            config.options.behaviour_on_error,
            BehaviourOnError::NextQuestion
        );
        assert!(config.skill.is_some());
    }

    #[test]
    fn args_split() {
        let args = [
            "--number-of-questions=10".to_string(),
            "-d".to_string(),            // Disable live stats
            "-b".to_string(),            // Set behaviour on error to...
            "repeat".to_string(),        // ...repeat
            "powers".to_string(),        // some command
            "-a".to_string(),            // some command args
            "--some-arg=42".to_string(), // another command arg
        ];
        let (options, command, command_options) = Config::split_args(&args);

        assert_eq!(
            options,
            [
                "--number-of-questions=10".to_string(),
                "-d".to_string(),
                "-b".to_string(),
                "repeat".to_string()
            ]
        );
        assert_eq!(command, Some("powers".to_string()));
        assert_eq!(
            command_options,
            ["-a".to_string(), "--some-arg=42".to_string()]
        );
    }

    #[test]
    fn args_split_unrecognised_command() {
        let args = [
            "--number-of-questions=10".to_string(),
            "-d".to_string(),            // Disable live stats
            "-b".to_string(),            // Set behaviour on error to...
            "somecommand".to_string(),   // ...repeat
            "-a".to_string(),            // some command args
            "--some-arg=42".to_string(), // another command arg
        ];
        let (options, command, command_options) = Config::split_args(&args);

        assert_eq!(
            options, args,
            "command not recognised, hence all args are treated as general args"
        );
        assert_eq!(command, None);
        assert!(command_options.is_empty());
    }

    #[test]
    fn args_split_no_general_options() {
        let args = [
            "powers".to_string(),
            "-a".to_string(),
            "--some-arg=42".to_string(),
        ];
        let (options, command, command_options) = Config::split_args(&args);

        assert!(options.is_empty());
        assert_eq!(command, Some("powers".to_string()));
        assert_eq!(
            command_options,
            ["-a".to_string(), "--some-arg=42".to_string()]
        );
    }

    #[test]
    fn build_error_message() {
        let msg = "some error message";
        let error_message = Config::build_err_message(Some(msg.to_string()));
        assert!(error_message.contains(msg));
        assert!(error_message.contains(&Application::usage()));
        assert!(error_message.contains(&Application::help_prompt()));

        let error_message = Config::build_err_message(None);
        assert!(error_message.contains(&Application::usage()));
        assert!(error_message.contains(&Application::help_prompt()));
    }

    #[test]
    fn parse_behaviour_on_error() {
        assert_eq!(
            BehaviourOnError::from_string(BEHAVIOUR_ON_ERROR_CONTINUE),
            BehaviourOnError::NextQuestion
        );
        assert_eq!(
            BehaviourOnError::from_string(BEHAVIOUR_ON_ERROR_SHOW_CORRECT),
            BehaviourOnError::ShowCorrect
        );
        assert_eq!(
            BehaviourOnError::from_string(BEHAVIOUR_ON_ERROR_REPEAT),
            BehaviourOnError::Repeat
        );
    }
}
