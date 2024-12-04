use core::panic;
use std::cell::{Cell, RefCell};

use crate::skill::Skill;

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

pub struct QuestionGenerator<'a> {
    number_of_questions: u32,
    current_question: Cell<u32>,
    skill: &'a dyn Skill,
    cache: RefCell<Option<Vec<Question>>>,
}

impl<'a> QuestionGenerator<'a> {
    pub fn new(number_of_questions: u32, skill: &'a dyn Skill) -> QuestionGenerator<'a> {
        QuestionGenerator {
            number_of_questions,
            current_question: Cell::new(0),
            skill,
            cache: RefCell::new(None),
        }
    }

    pub fn next_question(&self) -> Question {
        if self.number_of_questions == 0 {
            self.skill
                .generate_questions(1)
                .first()
                .expect("Question could not be generated")
                .clone()
        } else {
            self.current_question.set(self.current_question.get() + 1);
            let mut cache = self.cache.borrow_mut();
            if cache.is_none() {
                let questions = self.skill.generate_questions(self.number_of_questions);
                assert_eq!(
                    questions.len(),
                    self.number_of_questions as usize,
                    "Skill did not generate correct amount of questions"
                );
                cache.replace(questions);
            }
            if let Some(questions) = cache.as_ref() {
                questions[(self.current_question.get() - 1) as usize].clone()
            } else {
                panic!("Questions could not be generated")
            }
        }
    }

    pub fn has_next_question(&self) -> bool {
        self.number_of_questions == 0 || self.current_question.get() < self.number_of_questions
    }
}
