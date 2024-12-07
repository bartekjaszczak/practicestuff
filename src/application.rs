use std::borrow::Borrow;
use std::io::{self, Write};
use std::process;
use std::sync::Arc;

use crossterm::style::Color;
use rand::Rng;

use crate::args::prelude::*;
use crate::colour;
use crate::config::{BehaviourOnError, Config, NumberOfQuestions};
use crate::question::{Generator, Question};
use crate::skill::doomsday_algorithm;
use crate::skill::powers;
use crate::skill::times_table;
use crate::skill::Skill;
use crate::stats;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const COMMANDS: [help::Command; 3] = [
    help::Command::new(powers::CMD, "Practise powers (configurable base)."),
    help::Command::new(times_table::CMD, "Practise multiplication table."),
    help::Command::new(doomsday_algorithm::CMD, "Practise the Doomsday algorithm."),
];

pub struct Application;

impl Application {
    /// Runs the application with options specified in `config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use practicestuff::{Application, Config};
    ///
    /// let args: Vec<_> = env::args().collect();
    /// if let Ok(config) = Config::build(&args) {
    ///     Application::run(config);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if ctrlc crate fails to set the ctrl-c handler.
    /// Panics if any internal error happens. It shouldn't, but it might.
    pub fn run(config: Config) {
        let app = Arc::new(AppImpl {
            config,
            stats: stats::Lock::new(),
        });

        let app_ref = app.clone();
        ctrlc::set_handler(move || {
            app_ref.handle_interrupt();
        })
        .expect("Error setting Ctrl-C handler");

        app.run();
    }

    pub(crate) fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... command [command_option]...")
    }

    pub(crate) fn help_prompt() -> String {
        format!("Try '{APP_NAME} --help' for more information.")
    }

    fn additional_info() -> String {
        format!(
            "To display options and details for a specific command,\nrun '{APP_NAME} <command> -h'."
        )
    }

    fn print_version() {
        println!("{APP_NAME} {VERSION}");
    }
}

struct AppImpl {
    config: Config,
    stats: stats::Lock,
}

impl AppImpl {
    pub fn run(&self) {
        if self.config.options.show_help {
            self.print_help();
            return;
        } else if self.config.options.show_version {
            Application::print_version();
            return;
        } else if let Some(skill) = &self.config.skill {
            if skill.wants_to_print_help() {
                println!("{}", skill.get_help_text());
                return;
            }
        }

        self.play();
    }

    fn print_help(&self) {
        let definitions = &self.config.options.arg_definitions;
        let options = help::Options::new("General options", definitions);
        let help_text = help::build(
            &Application::usage(),
            Some(&Application::additional_info()),
            &options,
            &COMMANDS,
        );
        println!("{help_text}");
    }

    fn handle_interrupt(&self) {
        println!();
        self.print_stats_post_game();
        process::exit(1);
    }

    fn play(&self) {
        let generator = Generator::new(self.number_of_questions(), self.get_skill());

        self.before_game();

        while generator.has_next_question() {
            self.handle_question(
                &generator
                    .next_question()
                    .expect("next_question called even though there were no questions left"),
            );
        }

        self.print_stats_post_game();
    }

    fn before_game(&self) {
        self.print_intro();

        self.stats.start(self.number_of_questions());
    }

    fn handle_question(&self, question: &Question) {
        println!(
            "\n{}{}",
            colour::format_text("Q: ", self.use_colour(), Color::DarkYellow),
            question.prompt()
        );
        print!(
            "{}",
            colour::format_text("A: ", self.use_colour(), Color::DarkYellow)
        );
        io::stdout().flush().expect("IO operation failed (flush)");

        self.stats.start_new_question();

        let mut answer = Self::get_input();
        let mut correct = question.is_answer_correct(&answer);

        self.stats.answer_question(correct);
        self.print_answer_feedback(correct, question.correct_answer());

        if let BehaviourOnError::Repeat = self.config.options.behaviour_on_error {
            while !correct {
                print!("A: ");
                io::stdout().flush().expect("IO operation failed (flush)");
                answer = Self::get_input();
                correct = question.is_answer_correct(&answer);
                self.stats.answer_question(correct);
                self.print_answer_feedback(correct, question.correct_answer());
            }
        }

        if !self.config.options.disable_live_statistics {
            self.print_stats_in_between();
        }
    }

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("IO operation failed (stdin)");
        input.trim().to_string()
    }

    fn print_intro(&self) {
        let number_of_questions = match self.number_of_questions() {
            NumberOfQuestions::Infinite => "Infinite questions",
            NumberOfQuestions::Limited(num) => {
                if num > 1 {
                    &format!("{num} questions")
                } else {
                    "1 question"
                }
            }
        };

        let number_of_questions =
            colour::format_text(number_of_questions, self.use_colour(), Color::DarkYellow);
        let ctrl_c = colour::format_text("Ctrl+C", self.use_colour(), Color::Yellow);

        println!("{number_of_questions}. Use {ctrl_c} to exit.");
    }

    fn print_stats_in_between(&self) {
        let text = match self.number_of_questions() {
            NumberOfQuestions::Infinite => colour::format_text(
                &format!(
                    "Time taken: {}, current accuracy: {} ({})",
                    self.stats.get_last_question_time(),
                    self.stats.get_current_accuracy(),
                    self.stats.get_number_of_correct_answers()
                ),
                self.use_colour(),
                Color::Grey,
            ),
            NumberOfQuestions::Limited(_) => colour::format_text(
                &format!(
                    "Time taken: {}, current accuracy: {} ({}), questions left: {}",
                    self.stats.get_last_question_time(),
                    self.stats.get_current_accuracy(),
                    self.stats.get_number_of_correct_answers(),
                    self.stats.get_number_of_remaining_questions(),
                ),
                self.use_colour(),
                Color::Grey,
            ),
        };
        println!("{text}");
    }

    fn print_stats_post_game(&self) {
        self.print_summary();
        self.print_time_stats();
    }

    fn print_summary(&self) {
        println!("\n{}", self.stats.get_summary());
        println!(
            "Correct answers: {}",
            self.stats.get_number_of_correct_answers()
        );
        match self.number_of_questions() {
            NumberOfQuestions::Infinite => {
                println!("Accuracy: {}", self.stats.get_current_accuracy());
            }
            NumberOfQuestions::Limited(_) => {
                if self.stats.get_number_of_remaining_questions() == 0 {
                    println!("Accuracy: {}", self.stats.get_total_accuracy());
                } else {
                    println!(
                        "Accuracy: {} (total accuracy: {})",
                        self.stats.get_current_accuracy(),
                        self.stats.get_total_accuracy()
                    );
                }
            }
        }
    }

    fn print_time_stats(&self) {
        println!("Total time: {}", self.stats.get_total_time());
        println!("Time taken per question:");
        println!("  min: {}", self.stats.get_min_question_time());
        println!("  max: {}", self.stats.get_max_question_time());
        println!("  avg: {}", self.stats.get_avg_question_time());
    }

    fn print_answer_feedback(&self, correct: bool, correct_answer: &str) {
        let mut feedback = String::new();
        if correct {
            feedback.push_str(&colour::format_text(
                &Self::random_feedback_correct(),
                self.use_colour(),
                Color::Green,
            ));
        } else {
            feedback.push_str(&colour::format_text(
                &Self::random_feedback_incorrect(),
                self.use_colour(),
                Color::Red,
            ));
            match self.config.options.behaviour_on_error {
                BehaviourOnError::ShowCorrect => {
                    feedback.push_str(&format!(" Correct answer: {correct_answer}"));
                }
                BehaviourOnError::Repeat => feedback.push_str(" Try again:"),
                BehaviourOnError::NextQuestion => (),
            }
        }
        println!("{feedback}");
    }

    fn random_feedback_correct() -> String {
        let options = [
            "Correct!",
            "Correct",
            "Good job.",
            "Correct answer.",
            "Ok.",
            "Positive.",
            "You got it!",
            "Nice!",
            "Great!",
            "Awesome!",
            "Well done!",
            "Good work!",
            "Nice work!",
            "You're right!",
            "You're correct!",
            "You're doing great!",
            "You're on fire!",
            "You're on a roll!",
            "You're on a streak!",
            "Definitely!",
            "Absolutely!",
            "Absolutely correct!",
            "Yes.",
            "Yes!",
            "That's right.",
            "That's correct.",
            "That's it!",
            "That's the one!",
            "That's the answer!",
            "How'd you know that?",
            "How'd you know?",
            "How'd you get that?",
            "Affirmative.",
            "Affirmative!",
            "Very good.",
            "Good.",
            "Good!",
            "Good answer.",
            "Great answer.",
            "Great job!",
            "Great work!",
            "Yup.",
            "Yup!",
            "Yup, that's it.",
            "Correctamundo!",
            "Yeah!",
            "Yeah.",
            "Yeah, that's it.",
            "Yeah, that's right.",
            "Yeah, that's correct.",
        ];
        options[rand::thread_rng().gen_range(0..options.len())].to_string()
    }

    fn random_feedback_incorrect() -> String {
        let options = [
            "Incorrect!",
            "Incorrect.",
            "Incorrect answer.",
            "Wrong answer.",
            "Wrong.",
            "Nope.",
            "Negative.",
            "Maybe next time!",
            "Not quite.",
            "No.",
            "...What?",
            "Are you even trying?",
            "Almost...",
            "Not quite.",
            "Not quite there...",
            "Not quite right.",
            "Not quite correct.",
            "Definitely not that.",
            "Definitely not...",
            "Absolutely not.",
            "Absolutely not!",
            "Wroooong.",
            "You're not very good at this, are you?",
            "You're not very good at this.",
            "Need more practice!",
            "My grandma could do better.",
            "Nooooope",
            "Nope!",
            "That's not it.",
            "That's not the answer.",
            "That's not the one.",
            "Not the brightest bulb, are you?",
            "Not the sharpest tool in the shed, are you?",
            "Way off.",
            "Way off!",
            "Way off the mark.",
            "Waaaaay off.",
            "Nah.",
            "Nah mate.",
            "Very much no.",
            "That's absurd.",
            "That's ridiculous.",
            "Preposterous.",
            "Well, that's just silly.",
            "Well...",
            "Well, that's not right.",
            "Well, no.",
            "Well, nope.",
            "Well, that's not it.",
            "No, that's not it.",
        ];
        options[rand::thread_rng().gen_range(0..options.len())].to_string()
    }

    fn get_skill(&self) -> &dyn Skill {
        self.config
            .skill
            .as_ref()
            .expect("Skill expected at this point")
            .borrow()
    }

    fn number_of_questions(&self) -> NumberOfQuestions {
        self.config.options.number_of_questions
    }

    fn use_colour(&self) -> bool {
        self.config.options.use_colour
    }
}
