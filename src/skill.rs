pub mod doomsday_algorithm;
pub mod powers;
pub mod times_table;

use std::fmt;

use powers::Powers;

#[derive(Clone)]
pub struct Question {
    question: String,
    answer: String,
    alternative_answers: Vec<String>,
    allow_any_case: bool,
}

impl Question {
    pub fn new(
        question: &str,
        answer: &str,
        alternative_answers: &[String],
        allow_any_case: bool,
    ) -> Self {
        Self {
            question: question.to_string(),
            answer: answer.to_string(),
            alternative_answers: alternative_answers.to_vec(),
            allow_any_case,
        }
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn is_answer_correct(&self, answer: &String) -> bool {
        if self.allow_any_case {
            answer.to_ascii_lowercase() == self.answer.to_ascii_lowercase()
                || self
                    .alternative_answers
                    .iter()
                    .any(|elem| answer.to_ascii_lowercase() == elem.to_ascii_lowercase())
        } else {
            answer.as_str() == self.answer || self.alternative_answers.contains(answer)
        }
    }
}

pub trait SkillBase {
    fn show_help_and_exit(&self) -> bool;
    fn generate_questions(&self, count: u32) -> Vec<Question>;
}

pub trait Skill: SkillBase + fmt::Debug {}
impl<T: SkillBase + fmt::Debug> Skill for T {}

pub fn build(command: &str, args: &[String]) -> Result<Box<dyn Skill>, String> {
    match command {
        powers::CMD => Ok(Box::new(Powers::build(args)?)),
        times_table::CMD => todo!(),
        doomsday_algorithm::CMD => todo!(),
        _ => unreachable!("all commands should be added here"),
    }
}
