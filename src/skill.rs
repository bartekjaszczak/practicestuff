pub mod doomsday_algorithm;
pub mod powers;
pub mod times_table;

use std::fmt;

use powers::Powers;
use super::question::Question;


pub trait SkillBase {
    fn wants_to_print_help(&self) -> bool;
    fn get_help_text(&self) -> String;
    fn generate_questions(&self, count: u32) -> Vec<Question>;
}

pub trait Skill: SkillBase + fmt::Debug + Sync + Send {}
impl<T: SkillBase + fmt::Debug + Sync + Send> Skill for T {}

pub fn build(command: &str, args: &[String]) -> Result<Box<dyn Skill>, String> {
    match command {
        powers::CMD => Ok(Box::new(Powers::build(args)?)),
        times_table::CMD => todo!(),
        doomsday_algorithm::CMD => todo!(),
        _ => unreachable!("all commands should be added here"),
    }
}
