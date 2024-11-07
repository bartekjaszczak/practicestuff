use crate::Config;

pub const APP_NAME: &str = "practicestuff";

pub struct Application;

impl Application {
    pub fn run(config: &Config) -> Result<(), String> {
        println!("{config:#?}");

        Ok(())
    }

    pub(crate) fn usage() -> String {
        format!("Usage: {APP_NAME} [OPTION]... COMMAND [ARGS]...")
    }

    pub(crate) fn help_prompt() -> String {
        format!("Try '{APP_NAME} --help' for more information.")
    }

    fn print_help() {

    }
}
