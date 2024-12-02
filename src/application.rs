use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::args::prelude::*;
use crate::config::Config;
use crate::config::GeneralOptions;
use crate::skill::doomsday_algorithm;
use crate::skill::powers;
use crate::skill::times_table;
use crate::skill::Question;

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
            score: AtomicU32::new(0),
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
}

struct AppImpl {
    config: Config,
    score: AtomicU32,
}

impl AppImpl {
    pub fn run(&self) {
        if self.config.options.show_help {
            Self::print_help();
        } else if self.config.options.show_version {
            Self::print_version();
        } else if let Some(skill) = &self.config.skill {
            if skill.show_help_and_exit() {
                return;
            }
        }

        self.play();
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

    fn play(&self) {
        let num_of_questions = self.config.options.number_of_questions;
        let skill = self
            .config
            .skill
            .as_ref()
            .expect("Skill expected at this point");
        let questions = skill.generate_questions(num_of_questions);
        assert_eq!(questions.len(), num_of_questions as usize);

        Self::print_intro(num_of_questions);

        let infinite = num_of_questions == 0;
        let mut question_number: u32 = 0;

        loop {
            let question = if infinite {
                skill.generate_questions(1).first().unwrap().clone()
            } else {
                questions.get(question_number as usize).unwrap().clone()
            };

            let correct = Self::handle_question(&question);
            if correct {
                println!("Correct!");
                self.score.fetch_add(1, Ordering::Relaxed);
            } else {
                println!("Incorrect!");
            }
            question_number += 1;
        }
    }

    fn get_input() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn print_intro(num_of_questions: u32) {
        let infinite = num_of_questions == 0;
        let s = if num_of_questions > 1 || infinite {
            "s"
        } else {
            ""
        };
        let num_of_questions = if infinite {
            "Infinite".to_string()
        } else {
            num_of_questions.to_string()
        };

        println!("{num_of_questions} question{s}. Use Ctrl+C to exit.");
    }

    fn handle_question(question: &Question) -> bool {
        println!("\nQ: {}", question.question());
        print!("A: ");
        let _ = io::stdout().flush();

        let answer = Self::get_input();
        question.is_answer_correct(&answer)
    }

    fn handle_interrupt(&self) {
        // println!(
        //     "\nHandling interrupt, score: {}",
        //     self.score.load(Ordering::Relaxed)
        // );
        println!("\nExiting...");
        process::exit(1);
    }
}
