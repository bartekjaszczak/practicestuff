use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use crate::config::NumberOfQuestions;

const DURATION_ZERO: Duration = Duration::new(0, 0);

pub struct StatsLock {
    stats: RwLock<Stats>,
}

impl StatsLock {
    pub fn new() -> Self {
        Self {
            stats: RwLock::new(Stats {
                number_of_questions: NumberOfQuestions::Infinite,
                number_of_answered_questions: 0,
                number_of_correct_answers: 0,
                start_time: Instant::now(),
                current_question_start_time: Instant::now(),
                current_question_answered: false,
                time_per_question: vec![],
            }),
        }
    }

    pub fn start(&self, number_of_questions: NumberOfQuestions) {
        self.write().start(number_of_questions);
    }

    pub fn start_new_question(&self) {
        self.write().start_new_question();
    }

    pub fn answer_question(&self, correct: bool) {
        self.write().answer_question(correct);
    }

    pub fn get_summary(&self) -> String {
        self.read().get_summary()
    }

    pub fn get_number_of_correct_answers(&self) -> String {
        self.read().get_number_of_correct_answers()
    }

    pub fn get_number_of_remaining_questions(&self) -> u32 {
        self.read().get_number_of_remaining_questions()
    }

    /// Calculates accuracy. Takes into account total number of questions.
    /// Returns "0.00" for Infinite mode.
    pub fn get_total_accuracy(&self) -> String {
        self.read().get_total_accuracy()
    }

    /// Calculates accuracy. Takes into account questions answered so far.
    pub fn get_current_accuracy(&self) -> String {
        self.read().get_current_accuracy()
    }

    /// Preferably call this method first to stop the timer as early as possible
    pub fn get_total_time(&self) -> String {
        self.read().get_total_time()
    }

    pub fn get_last_question_time(&self) -> String {
        self.read().get_last_question_time()
    }

    pub fn get_min_question_time(&self) -> String {
        self.read().get_min_question_time()
    }

    pub fn get_max_question_time(&self) -> String {
        self.read().get_max_question_time()
    }

    pub fn get_avg_question_time(&self) -> String {
        self.read().get_avg_question_time()
    }

    fn write(&self) -> RwLockWriteGuard<Stats> {
        self.stats.write().expect("Stats are blocked")
    }

    fn read(&self) -> RwLockReadGuard<Stats> {
        self.stats.read().expect("Stats are blocked")
    }
}

struct Stats {
    number_of_questions: NumberOfQuestions,
    number_of_answered_questions: u32,
    number_of_correct_answers: u32,
    start_time: Instant,
    current_question_start_time: Instant,
    current_question_answered: bool,
    time_per_question: Vec<Duration>,
}

impl Stats {
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

    // Infinite:
    //   Summary: answered a total of {} questions
    //   current: correct/answers_so_far
    //
    // Limited:
    //   Summary: total: total, answered: answers_so_far, skipped: remaining questions
    //   current: correct/answers_so_far

    pub fn get_summary(&self) -> String {
        match self.number_of_questions {
            NumberOfQuestions::Infinite => {
                format!("Questions total: {}", self.number_of_answered_questions)
            }
            NumberOfQuestions::Limited(total) => format!(
                "Questions total: {}, answers: {}, skipped: {}",
                total,
                self.number_of_answered_questions,
                self.get_number_of_remaining_questions()
            ),
        }
    }

    pub fn get_number_of_correct_answers(&self) -> String {
        format!(
            "{}/{}",
            self.number_of_correct_answers, self.number_of_answered_questions
        )
    }

    pub fn get_number_of_remaining_questions(&self) -> u32 {
        match self.number_of_questions {
            NumberOfQuestions::Infinite => 0,
            NumberOfQuestions::Limited(total) => total - self.number_of_answered_questions,
        }
    }

    pub fn get_total_accuracy(&self) -> String {
        let divisor = match self.number_of_questions {
            NumberOfQuestions::Infinite => 0,
            NumberOfQuestions::Limited(num) => num,
        };
        self.get_accuracy(divisor)
    }

    pub fn get_current_accuracy(&self) -> String {
        let divisor = self.number_of_answered_questions;
        self.get_accuracy(divisor)
    }

    fn get_accuracy(&self, divisor: u32) -> String {
        let acc = if divisor == 0 {
            0.0
        } else {
            f64::from(self.number_of_correct_answers) / f64::from(divisor)
        } * 100.0;
        format!("{acc:.2}%")
    }

    pub fn get_total_time(&self) -> String {
        let total_time = self.start_time.elapsed();
        Self::format_duration(&total_time)
    }

    pub fn get_last_question_time(&self) -> String {
        let time = self
            .time_per_question
            .last()
            .expect("No questions answered so far");
        Self::format_duration(time)
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
