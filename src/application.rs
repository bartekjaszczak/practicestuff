use std::borrow::Borrow;
use std::io::{self, Write};
use std::process;
use std::sync::{Arc, RwLock};

use crate::args::prelude::*;
use crate::config::Config;
use crate::config::GeneralOptions;
use crate::question::{Question, QuestionGenerator};
use crate::skill::doomsday_algorithm;
use crate::skill::powers;
use crate::skill::times_table;
use crate::skill::Skill;
use crate::stats::Stats;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const COMMANDS: [help::Command; 3] = [
    help::Command::new(powers::CMD, "Practice powers (configurable base)."),
    help::Command::new(times_table::CMD, "Practice multiplication table."),
    help::Command::new(doomsday_algorithm::CMD, "Practice the Doomsday algorithm."),
];

pub struct Application;

impl Application {
    pub fn run(config: Config) {
        let app = Arc::new(AppImpl {
            config,
            stats: RwLock::new(Stats::new()),
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

    fn print_help() {
        let definitions = &GeneralOptions::get_arg_definitions();
        let options = help::Options::new("General options", definitions);
        let help_text = help::build(&Application::usage(), &options, &COMMANDS);
        println!("{help_text}");
    }

    fn print_version() {
        println!("{APP_NAME} {VERSION}");
    }
}

struct AppImpl {
    config: Config,
    stats: RwLock<Stats>,
}

impl AppImpl {
    pub fn run(&self) {
        if self.config.options.show_help {
            Application::print_help();
        } else if self.config.options.show_version {
            Application::print_version();
        } else if let Some(skill) = &self.config.skill {
            if skill.only_show_help_and_exit() {
                return;
            }
        }

        self.play();
    }

    fn handle_interrupt(&self) {
        self.print_post_game_stats();
        process::exit(1);
    }

    fn play(&self) {
        let generator = QuestionGenerator::new(self.number_of_questions(), self.get_skill());

        self.before_game();

        while generator.has_next_question() {
            self.handle_question(&generator.next_question());
        }

        self.print_post_game_stats();
    }

    fn before_game(&self) {
        let number_of_questions = self.config.options.number_of_questions;
        Self::print_intro(number_of_questions);

        self.stats
            .write()
            .expect("Stats are blocked")
            .start(number_of_questions);
    }

    fn handle_question(&self, question: &Question) {
        println!("\nQ: {}", question.question());
        print!("A: ");
        io::stdout().flush().expect("IO operation failed (flush)");

        self.stats
            .write()
            .expect("Stats are blocked")
            .start_new_question();

        let answer = Self::get_input();
        let correct = question.is_answer_correct(&answer);

        self.stats
            .write()
            .expect("Stats are blocked")
            .answer_question(correct);

        Self::print_answer_feedback(correct);
    }

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("IO operation failed (stdin)");
        input.trim().to_string()
    }

    fn print_intro(number_of_questions: u32) {
        let infinite = number_of_questions == 0;
        let s = if number_of_questions > 1 || infinite {
            "s"
        } else {
            ""
        };
        let number_of_questions = if infinite {
            "Infinite".to_string()
        } else {
            number_of_questions.to_string()
        };

        println!("{number_of_questions} question{s}. Use Ctrl+C to exit.");
    }

    fn print_post_game_stats(&self) {
        if let Ok(stats) = self.stats.try_read() {
            println!(
                "\nQuestions answered: {}",
                stats.get_number_of_answered_questions()
            );
            println!("Correct answers: {}", stats.get_number_of_correct_answers());
            println!("Accuracy: {}", stats.get_accuracy());
            println!("Total time: {}", stats.get_total_time());
            println!("Time taken per question:");
            println!("  min: {}", stats.get_min_question_time());
            println!("  max: {}", stats.get_max_question_time());
            println!("  avg: {}", stats.get_avg_question_time());
        } else {
            println!("\nUnable to show statistics");
        }
    }

    fn print_answer_feedback(correct: bool) {
        if correct {
            println!("Correct!");
        } else {
            println!("Incorrect!");
        }
    }

    fn get_skill(&self) -> &dyn Skill {
        self.config
            .skill
            .as_ref()
            .expect("Skill expected at this point")
            .borrow()
    }

    fn number_of_questions(&self) -> u32 {
        self.config.options.number_of_questions
    }
}
