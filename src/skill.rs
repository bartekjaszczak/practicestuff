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
    pub fn builder() -> QuestionBuilder {
        QuestionBuilder::default()
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

#[derive(Default)]
pub struct QuestionBuilder {
    question: String,
    answer: String,
    alternative_answers: Vec<String>,
    allow_any_case: bool,
}

impl QuestionBuilder {
    pub fn question(mut self, question: &str) -> Self {
        self.question = question.to_string();
        self
    }

    pub fn answer(mut self, answer: &str) -> Self {
        self.answer = answer.to_string();
        self
    }

    pub fn alternative_answers(mut self, alternative_answers: &[String]) -> Self {
        self.alternative_answers = alternative_answers.to_vec();
        self
    }

    pub fn allow_any_case(mut self, allow_any_case: bool) -> Self {
        self.allow_any_case = allow_any_case;
        self
    }

    pub fn build(self) -> Question {
        assert!(!self.question.is_empty(), "Question cannot be empty");
        assert!(!self.answer.is_empty(), "Answer cannot be empty");
        Question {
            question: self.question,
            answer: self.answer,
            alternative_answers: self.alternative_answers,
            allow_any_case: self.allow_any_case,
        }
    }
}

pub trait SkillBase {
    fn show_help_and_exit(&self) -> bool;
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
