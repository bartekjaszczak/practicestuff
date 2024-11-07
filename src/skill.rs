pub mod doomsday_algorithm;
pub mod powers;
pub mod times_table;

use std::fmt;

use doomsday_algorithm::CMD_DOOMSDAY_ALGORITHM;
use powers::{Powers, CMD_POWERS};
use times_table::CMD_TIMES_TABLE;

struct QuestionAndAnswer {
    question: String,
    answer: String,
}

trait SkillBase {
    fn generate_questions_and_answers(&self, count: u32) -> Vec<QuestionAndAnswer>;
}

pub trait Skill: SkillBase + fmt::Debug {}
impl<T: SkillBase + fmt::Debug> Skill for T {}

pub fn build_skill(command: &str, args: &[String]) -> Result<Box<dyn Skill>, String> {
    match command {
        CMD_POWERS => Ok(Box::new(Powers::build(args)?)),
        CMD_TIMES_TABLE => todo!(),
        CMD_DOOMSDAY_ALGORITHM => todo!(),
        _ => unreachable!("all commands should be added here"),
    }
}
