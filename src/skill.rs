pub mod doomsday_algorithm;
pub mod powers;
pub mod times_table;

use std::fmt::Debug;

use powers::Powers;
use times_table::TimesTable;
use super::question::Question;

pub trait Base {
    fn wants_to_print_help(&self) -> bool;
    fn get_help_text(&self) -> String;
    fn generate_questions(&self, count: u32) -> Vec<Question>;
}

pub trait Skill: Base + Debug + Sync + Send {}
impl<T: Base + Debug + Sync + Send> Skill for T {}

pub fn build(command: &str, args: &[String]) -> Result<Box<dyn Skill>, String> {
    match command {
        powers::CMD => Ok(Box::new(Powers::build(args)?)),
        times_table::CMD => Ok(Box::new(TimesTable::build(args)?)),
        doomsday_algorithm::CMD => todo!(),
        _ => unreachable!("all commands should be added here"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "all commands should be added")]
    fn build_incorrect_command() {
        let command = "unknown";
        let args = [];
        build(command, &args).unwrap();
    }

    #[test]
    fn build_powers() {
        let command = powers::CMD;
        let args = [];
        build(command, &args).unwrap();
    }

    #[test]
    fn build_times_table() {
        let command = times_table::CMD;
        let args = [];
        build(command, &args).unwrap();
    }
}
