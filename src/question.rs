use std::cell::{Cell, RefCell};

use crate::config::NumberOfQuestions;
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

    pub fn correct_answer(&self) -> &String {
        &self.answer
    }

    pub fn is_answer_correct(&self, answer: &str) -> bool {
        if self.allow_any_case {
            answer.to_ascii_lowercase() == self.answer.to_ascii_lowercase()
                || self
                    .alternative_answers
                    .iter()
                    .any(|elem| answer.to_ascii_lowercase() == elem.to_ascii_lowercase())
        } else {
            answer == self.answer || self.alternative_answers.contains(&answer.to_string())
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
    number_of_questions: NumberOfQuestions,
    current_question: Cell<u32>,
    skill: &'a dyn Skill,
    cache: RefCell<Option<Vec<Question>>>,
}

impl<'a> QuestionGenerator<'a> {
    pub fn new(
        number_of_questions: NumberOfQuestions,
        skill: &'a dyn Skill,
    ) -> QuestionGenerator<'a> {
        QuestionGenerator {
            number_of_questions,
            current_question: Cell::new(0),
            skill,
            cache: RefCell::new(None),
        }
    }

    pub fn next_question(&self) -> Result<Question, String> {
        match self.number_of_questions {
            NumberOfQuestions::Infinite => Ok(self
                .skill
                .generate_questions(1)
                .first()
                .expect("Question could not be generated")
                .clone()),
            NumberOfQuestions::Limited(num) => {
                self.current_question.set(self.current_question.get() + 1);
                let mut cache = self.cache.borrow_mut();
                if cache.is_none() {
                    let questions = self.skill.generate_questions(num);
                    assert_eq!(
                        questions.len(),
                        num as usize,
                        "Skill did not generate correct amount of questions"
                    );
                    cache.replace(questions);
                }
                if let Some(questions) = cache.as_ref() {
                    if self.current_question.get() as usize > questions.len() {
                        Err("No questions left".to_string())
                    } else {
                        Ok(questions[(self.current_question.get() - 1) as usize].clone())
                    }
                } else {
                    panic!("Questions could not be generated")
                }
            }
        }
    }

    pub fn has_next_question(&self) -> bool {
        match self.number_of_questions {
            NumberOfQuestions::Infinite => true,
            NumberOfQuestions::Limited(num) => self.current_question.get() < num,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, sync::RwLock};

    use crate::skill::SkillBase;

    use super::*;

    #[test]
    #[should_panic(expected = "cannot be empty")]
    fn question_must_have_question_and_answer() {
        Question::builder().build();
    }

    #[test]
    #[should_panic(expected = "Answer cannot be empty")]
    fn question_must_have_answer() {
        Question::builder().question("Question").build();
    }

    #[test]
    #[should_panic(expected = "Question cannot be empty")]
    fn question_must_have_question() {
        Question::builder().answer("Answer").build();
    }

    #[test]
    fn question_builder() {
        let question = Question::builder()
            .question("Question")
            .answer("Answer")
            .alternative_answers(&["Alt1".to_string(), "Alt2".to_string()])
            .allow_any_case(true)
            .build();

        assert_eq!(question.question, "Question");
        assert_eq!(question.answer, "Answer");
        assert_eq!(
            question.alternative_answers,
            vec!["Alt1".to_string(), "Alt2".to_string()]
        );
        assert!(question.allow_any_case);
    }

    #[test]
    fn returns_question_and_answer() {
        let question = Question::builder()
            .question("Question")
            .answer("Answer")
            .build();

        assert_eq!(question.question(), "Question");
        assert_eq!(question.correct_answer(), "Answer");
    }

    #[test]
    fn answer_verification() {
        let question = Question::builder()
            .question("Question")
            .answer("Answer")
            .build();

        assert!(question.is_answer_correct("Answer"));
        assert!(!question.is_answer_correct("Wrong"));

        let question = Question::builder()
            .question("Question")
            .answer("Answer")
            .alternative_answers(&["Alt1".to_string(), "Alt2".to_string()])
            .build();

        assert!(question.is_answer_correct("Answer"));
        assert!(question.is_answer_correct("Alt1"));
        assert!(question.is_answer_correct("Alt2"));
        assert!(!question.is_answer_correct("Alt3"));
        assert!(!question.is_answer_correct("alt1"));
        assert!(!question.is_answer_correct("answer"));

        let question = Question::builder()
            .question("Question")
            .answer("Answer")
            .alternative_answers(&["Alt1".to_string(), "Alt2".to_string()])
            .allow_any_case(true)
            .build();

        assert!(question.is_answer_correct("Answer"));
        assert!(question.is_answer_correct("Alt1"));
        assert!(question.is_answer_correct("Alt2"));
        assert!(!question.is_answer_correct("Alt3"));
        assert!(question.is_answer_correct("alt1"));
        assert!(question.is_answer_correct("answer"));
    }

    #[derive(Debug)]
    struct SkillMock {
        generate_questions_calls: RwLock<u32>,
    }

    impl SkillBase for SkillMock {
        fn wants_to_print_help(&self) -> bool {
            false
        }

        fn get_help_text(&self) -> String {
            String::new()
        }

        fn generate_questions(&self, count: u32) -> Vec<Question> {
            *self
                .generate_questions_calls
                .write()
                .expect("Test: poisoned lock (write)") += 1;
            vec![
                Question::builder()
                    .question("Question")
                    .answer("Answer")
                    .build();
                count as usize
            ]
        }
    }

    impl SkillMock {
        pub fn new() -> Self {
            Self {
                generate_questions_calls: RwLock::new(0),
            }
        }

        pub fn generate_questions_calls(&self) -> u32 {
            *self
                .generate_questions_calls
                .read()
                .expect("Test: poisoned lock (read)")
        }
    }

    #[test]
    fn generator_pregeneration() {
        let number_of_questions = 5;
        let skill_mock = SkillMock::new();
        let generator =
            QuestionGenerator::new(NumberOfQuestions::Limited(number_of_questions), &skill_mock);
        for _ in 0..number_of_questions {
            assert!(generator.has_next_question());
            let result = generator.next_question();
            assert!(result.is_ok());
        }
        assert_eq!(skill_mock.generate_questions_calls(), 1);

        assert!(!generator.has_next_question());
        let result = generator.next_question();
        assert!(result.is_err());
    }

    #[test]
    fn generator_infinite_mode() {
        let number_of_questions = 10;
        let skill_mock = SkillMock::new();
        let generator = QuestionGenerator::new(NumberOfQuestions::Infinite, &skill_mock);
        for i in 0..number_of_questions {
            assert!(generator.has_next_question());
            let result = generator.next_question();
            assert!(result.is_ok());
            assert_eq!(skill_mock.generate_questions_calls(), i + 1);
        }
        assert!(generator.has_next_question());
    }

    #[derive(Debug)]
    struct FaultySkillMock;

    impl SkillBase for FaultySkillMock {
        fn wants_to_print_help(&self) -> bool {
            false
        }

        fn get_help_text(&self) -> String {
            String::new()
        }

        fn generate_questions(&self, _count: u32) -> Vec<Question> {
            // Always generates 2 questions
            vec![
                Question::builder()
                    .question("Question")
                    .answer("Answer")
                    .build();
                2
            ]
        }
    }

    #[test]
    #[should_panic(expected = "Skill did not generate correct amount of questions")]
    fn not_enough_questions_generated() {
        let number_of_questions = 5;
        let generator = QuestionGenerator::new(
            NumberOfQuestions::Limited(number_of_questions),
            &FaultySkillMock,
        );
        generator.next_question().unwrap();
    }
}
