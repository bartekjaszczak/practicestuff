use std::time::{Duration, Instant};

use crate::config::NumberOfQuestions;

const DURATION_ZERO: Duration = Duration::new(0, 0);

pub struct Stats {
    number_of_questions: NumberOfQuestions,
    number_of_answered_questions: u32,
    number_of_correct_answers: u32,
    start_time: Instant,
    current_question_start_time: Instant,
    current_question_answered: bool,
    time_per_question: Vec<Duration>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            number_of_questions: NumberOfQuestions::Infinite,
            number_of_answered_questions: 0,
            number_of_correct_answers: 0,
            start_time: Instant::now(),
            current_question_start_time: Instant::now(),
            current_question_answered: false,
            time_per_question: vec![],
        }
    }

    pub fn start(&mut self, number_of_questions: NumberOfQuestions) {
        self.number_of_questions = number_of_questions;
        self.start_time = Instant::now();
    }

    pub fn start_new_question(&mut self) {
        self.current_question_start_time = Instant::now();
    }

    pub fn answer_question(&mut self, correct: bool) {
        if self.current_question_answered {
            // Question has already been answered, so it's a second attempt
            // Update total time, but don't mark as correct even if current answer matches
            *self
                .time_per_question
                .last_mut()
                .expect("answer_question incorrectly called") =
                self.current_question_start_time.elapsed();
        } else {
            self.time_per_question
                .push(self.current_question_start_time.elapsed());
            self.number_of_answered_questions += 1;
            if correct {
                self.number_of_correct_answers += 1;
            }
        }
    }

    pub fn get_number_of_answered_questions(&self) -> String {
        match self.number_of_questions {
            NumberOfQuestions::Infinite => format!("{}", self.number_of_answered_questions),
            NumberOfQuestions::Limited(num) => {
                format!("{}/{}", self.number_of_answered_questions, num)
            }
        }
    }

    pub fn get_number_of_correct_answers(&self) -> String {
        format!("{}", self.number_of_correct_answers)
    }

    pub fn get_accuracy(&self) -> String {
        let acc = match self.number_of_questions {
            NumberOfQuestions::Infinite => {
                let divisor = self.number_of_answered_questions;
                if divisor == 0 {
                    0.0
                } else {
                    f64::from(self.number_of_correct_answers) / f64::from(divisor)
                }
            }
            NumberOfQuestions::Limited(num) => {
                if num == 0 {
                    0.0
                } else {
                    f64::from(self.number_of_correct_answers) / f64::from(num)
                }
            }
        } * 100.0;
        format!("{acc:.2}%")
    }

    /// Preferably call this method first to stop the timer as early as possible
    pub fn get_total_time(&self) -> String {
        let total_time = self.start_time.elapsed();
        Self::format_duration(&total_time)
    }

    pub fn get_min_question_time(&self) -> String {
        let min_time = self
            .time_per_question
            .iter()
            .min()
            .unwrap_or(&DURATION_ZERO);
        Self::format_duration(min_time)
    }

    pub fn get_max_question_time(&self) -> String {
        let max_time = self
            .time_per_question
            .iter()
            .max()
            .unwrap_or(&DURATION_ZERO);
        Self::format_duration(max_time)
    }

    pub fn get_avg_question_time(&self) -> String {
        let total_time = self.time_per_question.iter().sum::<Duration>();
        let answered_questions = u32::try_from(self.time_per_question.len())
            .expect("Time per question vector len > u32::MAX");
        if answered_questions == 0 {
            Self::format_duration(&DURATION_ZERO)
        } else {
            Self::format_duration(&(total_time / answered_questions))
        }
    }

    fn format_duration(duration: &Duration) -> String {
        let hours = (duration.as_secs() / 60) / 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let seconds = duration.as_secs() % 60;
        let milliseconds = Self::truncate_trailing_zeros(duration.subsec_millis());

        let mut time = String::new();
        if hours > 0 {
            time.push_str(&format!("{hours}h "));
        }
        if minutes > 0 {
            time.push_str(&format!("{minutes}m "));
        }
        time.push_str(&format!("{seconds}.{milliseconds}s"));

        time
    }

    fn truncate_trailing_zeros(number: u32) -> String {
        let mut number = number.to_string();
        while number.ends_with('0') && number.len() > 1 {
            number.pop();
        }
        number
    }
}
