use crate::Config;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        todo!()
    }

    fn print_version() {
        println!("{APP_NAME} {VERSION}");
    }
}
