use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use crate::config::NumberOfQuestions;

const DURATION_ZERO: Duration = Duration::new(0, 0);

pub struct Lock {
    stats: RwLock<Stats>,
}

impl Lock {
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
        self.current_question_answered = false;
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
            self.current_question_answered = true;
        }
    }

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

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn build_and_verify_stats_limited_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Limited(10));
        assert_eq!(stats.get_number_of_correct_answers(), "0/0");
        assert_eq!(stats.get_number_of_remaining_questions(), 10);
        assert_eq!(stats.get_total_accuracy(), "0.00%");
        assert_eq!(stats.get_current_accuracy(), "0.00%");
    }

    #[test]
    fn build_and_verify_stats_unlimited_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Infinite);
        assert_eq!(stats.get_number_of_correct_answers(), "0/0");
        assert_eq!(
            stats.get_number_of_remaining_questions(),
            0,
            "Should always return 0 for infinite mode"
        );
        assert_eq!(stats.get_total_accuracy(), "0.00%");
        assert_eq!(stats.get_current_accuracy(), "0.00%");
    }

    #[test]
    fn start_and_answer_some_questions_limited_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Limited(10));

        // 3 correct answers
        for _ in 0..3 {
            stats.start_new_question();
            stats.answer_question(true);
        }

        assert_eq!(stats.read().time_per_question.len(), 3);

        // 3 incorrect answers
        for _ in 0..3 {
            stats.start_new_question();
            stats.answer_question(false);
        }

        assert_eq!(stats.read().time_per_question.len(), 6);

        assert_eq!(stats.get_number_of_correct_answers(), "3/6");
        assert_eq!(stats.get_number_of_remaining_questions(), 4);
        assert_eq!(stats.get_total_accuracy(), "30.00%");
        assert_eq!(stats.get_current_accuracy(), "50.00%");

        let summary = stats.get_summary();
        assert!(summary.contains("Questions total: 10"));
        assert!(summary.contains("answers: 6"));
        assert!(summary.contains("skipped: 4"));
    }

    #[test]
    fn start_and_answer_some_questions_unlimited_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Infinite);

        // 3 correct answers
        for _ in 0..3 {
            stats.start_new_question();
            stats.answer_question(true);
        }

        assert_eq!(stats.read().time_per_question.len(), 3);

        // 3 incorrect answers
        for _ in 0..3 {
            stats.start_new_question();
            stats.answer_question(false);
        }

        assert_eq!(stats.read().time_per_question.len(), 6);

        assert_eq!(stats.get_number_of_correct_answers(), "3/6");
        assert_eq!(
            stats.get_number_of_remaining_questions(),
            0,
            "Should always return 0 for infinite mode"
        );
        assert_eq!(
            stats.get_total_accuracy(),
            "0.00%",
            "Should always return 0.00% for infinite mode"
        );
        assert_eq!(stats.get_current_accuracy(), "50.00%");

        let summary = stats.get_summary();
        assert!(summary.contains("Questions total: 6"));
    }

    #[test]
    fn repeating_wrong_answer_limited_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Limited(10));

        // 3 correct answers
        for _ in 0..3 {
            stats.start_new_question();
            stats.answer_question(true);
        }

        assert_eq!(stats.read().time_per_question.len(), 3);

        // Answering incorrectly first (Repeat mode)
        stats.start_new_question();
        stats.answer_question(false);
        assert_eq!(stats.read().time_per_question.len(), 4);
        stats.answer_question(false);
        assert_eq!(stats.read().time_per_question.len(), 4);
        stats.answer_question(false);
        assert_eq!(stats.read().time_per_question.len(), 4);
        stats.answer_question(true);
        assert_eq!(stats.read().time_per_question.len(), 4);

        assert_eq!(
            stats.get_number_of_correct_answers(),
            "3/4",
            "Answering incorrectly for the first time == incorrect answer"
        );
        assert_eq!(stats.get_number_of_remaining_questions(), 6);
        assert_eq!(stats.get_total_accuracy(), "30.00%");
        assert_eq!(stats.get_current_accuracy(), "75.00%");

        let summary = stats.get_summary();
        assert!(summary.contains("Questions total: 10"));
        assert!(summary.contains("answers: 4"));
        assert!(summary.contains("skipped: 6"));
    }

    #[test]
    fn repeating_wrong_answer_unlimited_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Infinite);

        // 3 correct answers
        for _ in 0..3 {
            stats.start_new_question();
            stats.answer_question(true);
        }

        assert_eq!(stats.read().time_per_question.len(), 3);

        // Answering incorrectly first (Repeat mode)
        stats.start_new_question();
        stats.answer_question(false);
        assert_eq!(stats.read().time_per_question.len(), 4);
        stats.answer_question(false);
        assert_eq!(stats.read().time_per_question.len(), 4);
        stats.answer_question(false);
        assert_eq!(stats.read().time_per_question.len(), 4);
        stats.answer_question(true);
        assert_eq!(stats.read().time_per_question.len(), 4);

        assert_eq!(
            stats.get_number_of_correct_answers(),
            "3/4",
            "Answering incorrectly for the first time == incorrect answer"
        );
        assert_eq!(stats.get_current_accuracy(), "75.00%");

        let summary = stats.get_summary();
        assert!(summary.contains("Questions total: 4"));
    }

    #[test]
    fn time_stats_no_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Infinite);

        sleep(Duration::from_millis(200));

        let max_time = stats.get_max_question_time();
        let min_time = stats.get_min_question_time();
        let avg_time = stats.get_avg_question_time();

        assert_eq!(max_time, "0.0s");
        assert_eq!(min_time, "0.0s");
        assert_eq!(avg_time, "0.0s");

        let total_time = stats.get_total_time();
        assert_ne!(total_time, "0.0s");
    }

    #[test]
    fn time_stats_one_question() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Infinite);

        stats.start_new_question();
        sleep(Duration::from_millis(200));
        stats.answer_question(true);

        let max_time = stats.get_max_question_time();
        let min_time = stats.get_min_question_time();
        let avg_time = stats.get_avg_question_time();

        assert_eq!(max_time, min_time);
        assert_eq!(min_time, avg_time);

        assert_ne!(max_time, "0.0s");
    }

    /// This test is flaky and might fail in some unforeseen scenarios
    #[test]
    fn time_stats_two_questions() {
        let stats = Lock::new();
        stats.start(NumberOfQuestions::Infinite);

        stats.start_new_question();
        sleep(Duration::from_millis(50));
        stats.answer_question(true);

        stats.start_new_question();
        sleep(Duration::from_millis(400));
        stats.answer_question(true);

        let max_time = stats.get_max_question_time();
        let min_time = stats.get_min_question_time();
        let avg_time = stats.get_avg_question_time();

        assert_ne!(max_time, min_time);
        assert_ne!(min_time, avg_time);
        assert_ne!(max_time, avg_time);

        assert_ne!(max_time, "0.0s", "Should be **more or less** 0.4s");
        assert_ne!(min_time, "0.0s", "Should be **more or less** 0.05s");
        assert_ne!(avg_time, "0.0s", "Should be **more or less** 0.225s");
    }
}
