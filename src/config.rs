use crate::application::{self, Application};
use crate::skill::doomsday_algorithm::CMD_DOOMSDAY_ALGORITHM;
use crate::skill::powers::CMD_POWERS;
use crate::skill::times_table::CMD_TIMES_TABLE;
use crate::skill::{self, Skill};

use std::cmp;

const COMMANDS: [&str; 3] = [CMD_POWERS, CMD_TIMES_TABLE, CMD_DOOMSDAY_ALGORITHM];

const DEFAULT_OPTION_NUMBER_OF_QUESTION: u32 = 20;
const DEFAULT_OPTION_SHOW_LIVE_STATISTICS: bool = true;
const DEFAULT_OPTION_BEHAVIOUR_ON_ERROR: BehaviourOnError = BehaviourOnError::ShowCorrect;

#[derive(Debug)]
pub struct Config {
    general_options: GeneralOptions,
    skill: Box<dyn Skill>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(build_err_message(None));
        }
        let (general_options, command, args) = Config::split_args(&args[1..]);

        let general_options = GeneralOptions::build(general_options)?;
        let Some(command) = command else {
            return Err(build_err_message(Some("missing command".to_string())));
        };

        let skill = skill::build_skill(&command, args)?;

        Ok(Config {
            general_options,
            skill,
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
}

#[derive(Debug)]
enum BehaviourOnError {
    NextQuestion,
    ShowCorrect,
    Repeat,
}

enum OptionType {
    ShowHelp,
    ShowVersion,

    NumberOfQuestions,
    ShowLiveStatistics,
    BehaviourOnError,
}

#[derive(Debug)]
struct GeneralOptions {
    show_help: bool,
    show_version: bool,

    number_of_questions: u32,
    show_live_statistics: bool,
    behaviour_on_error: BehaviourOnError,
}

impl GeneralOptions {
    fn build(args: &[String]) -> Result<GeneralOptions, String> {
        let mut show_help = false;
        let mut show_version = false;
        let mut number_of_questions = DEFAULT_OPTION_NUMBER_OF_QUESTION;
        let mut show_live_statistics = DEFAULT_OPTION_SHOW_LIVE_STATISTICS;
        let mut behaviour_on_error = DEFAULT_OPTION_BEHAVIOUR_ON_ERROR;

        let mut option_type = None;
        let mut expecting_value = false;

        let mut last_option = String::new();

        for arg in args {
            if expecting_value {
                // Read option value
                if let Some(option) = option_type {
                    match option {
                        OptionType::NumberOfQuestions => {
                            number_of_questions = GeneralOptions::parse_number_of_questions(arg)?;
                        }
                        OptionType::BehaviourOnError => {
                            behaviour_on_error = GeneralOptions::parse_behaviour_on_error(arg)?;
                        }
                        _ => (),
                    }
                }
                expecting_value = false;
                option_type = None;
            } else {
                // Read option
                last_option.clone_from(arg);
                let (option, value_required) = GeneralOptions::parse_option_type(arg)?;
                match option {
                    OptionType::ShowHelp => {
                        show_help = true;
                        break;
                    }
                    OptionType::ShowVersion => {
                        show_version = true;
                        break;
                    }
                    OptionType::ShowLiveStatistics => show_live_statistics = true,
                    _ => option_type = Some(option),
                }
                expecting_value = value_required;
            }
        }

        if expecting_value {
            return Err(build_err_message(Some(format!(
                "'{last_option}' option requires a value"
            ))));
        }

        Ok(GeneralOptions {
            show_help,
            show_version,
            number_of_questions,
            show_live_statistics,
            behaviour_on_error,
        })
    }

    fn parse_option_type(option: &str) -> Result<(OptionType, bool), String> {
        match option {
            "--help" | "-h" => Ok((OptionType::ShowHelp, false)),
            "--version" | "-v" => Ok((OptionType::ShowVersion, false)),
            "--number-of-questions" | "-n" => Ok((OptionType::NumberOfQuestions, true)),
            "--show-live-statistics" | "-s" => Ok((OptionType::ShowLiveStatistics, false)),
            "--behaviour_on_error" | "-b" => Ok((OptionType::BehaviourOnError, true)),
            _ => Err(build_err_message(Some(format!(
                "unrecognised option: '{option}'"
            )))),
        }
    }

    fn parse_number_of_questions(value: &str) -> Result<u32, String> {
        match value.parse::<u32>() {
            Ok(number) => Ok(number),
            _ => Err(build_err_message(Some(format!(
                "incorrect option value: '{value}'"
            )))),
        }
    }

    fn parse_behaviour_on_error(value: &str) -> Result<BehaviourOnError, String> {
        match value {
            "continue" => Ok(BehaviourOnError::NextQuestion),
            "showcorrect" => Ok(BehaviourOnError::ShowCorrect),
            "repeat" => Ok(BehaviourOnError::Repeat),
            _ => Err(build_err_message(Some(format!(
                "incorrect option value: '{value}'"
            )))),
        }
    }
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
