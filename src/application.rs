use crate::config::GeneralOptions;
use crate::skill::doomsday_algorithm::CMD_DOOMSDAY_ALGORITHM;
use crate::skill::powers::CMD_POWERS;
use crate::skill::times_table::CMD_TIMES_TABLE;
use crate::args::prelude::*;
use crate::Config;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const COMMANDS: [help::Command; 3] = [
    help::Command::new(CMD_POWERS, "Practice powers (configurable base)."),
    help::Command::new(CMD_TIMES_TABLE, "Practice multiplication table."),
    help::Command::new(CMD_DOOMSDAY_ALGORITHM, "Practice the Doomsday algorithm."),
];

pub struct Application;

impl Application {
    pub fn run(config: &Config) -> Result<(), String> {
        if config.options.show_help {
            Application::print_help();
        } else if config.options.show_version {
            Application::print_version();
        } else {
            println!("{config:#?}");
        }

        Ok(())
    }

    pub(crate) fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... command [command_option]...")
    }

    pub(crate) fn help_prompt() -> String {
        format!("Try '{APP_NAME} --help' for more information.")
    }

    fn print_help() {
        let help_text = help::build(
            &Application::usage(),
            &GeneralOptions::get_arg_definitions(),
            &COMMANDS,
        );
        println!("{help_text}");
    }

    fn print_version() {
        println!("{APP_NAME} {VERSION}");
    }
}
