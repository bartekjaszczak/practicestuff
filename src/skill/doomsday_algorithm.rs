use chrono::{Datelike, NaiveDate, Weekday};
use std::iter;

use rand::Rng;

use super::Base;
use crate::application::APP_NAME;
use crate::args::prelude::*;
use crate::question::Question;

pub const CMD: &str = "doomsday";

const ARG_ID_HELP: &str = "help";
const ARG_ID_LOWER_BOUNDARY: &str = "lower_boundary";
const ARG_ID_UPPER_BOUNDARY: &str = "upper_boundary";

const GREGORIAN_CALENDAR_INTRODUCTION: i32 = 1582;

#[derive(Debug)]
pub struct Doomsday {
    arg_definitions: Vec<Arg>,
    show_help: bool,

    lower_boundary: i32,
    upper_boundary: i32,

    default_boundaries: bool,
}

impl Doomsday {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::build_arg_definitions();
        let parsed_args = parser::parse_and_validate_arg_list(args, &arg_definitions)
            .map_err(|err| Self::build_err_message(Some(err)))?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let lower_boundary = i32::set_value_from_arg_or_default(
            ARG_ID_LOWER_BOUNDARY,
            &parsed_args,
            &arg_definitions,
        );
        let upper_boundary = i32::set_value_from_arg_or_default(
            ARG_ID_UPPER_BOUNDARY,
            &parsed_args,
            &arg_definitions,
        );

        let default_boundaries =
            Self::check_boundaries(lower_boundary, upper_boundary, &arg_definitions)?;

        Ok(Self {
            arg_definitions,
            show_help,
            lower_boundary,
            upper_boundary,
            default_boundaries,
        })
    }

    fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... doomsday [doomsday_option]...")
    }

    fn help_prompt() -> String {
        format!("Try '{APP_NAME} doomsday --help' for more information.")
    }

    fn additional_info() -> String {
        let mut text = String::new();
        text.push_str(
            "Practise doomsday algorithm (https://en.wikipedia.org/wiki/Doomsday_rule).\n",
        );
        text.push_str(
            "By default, the dates range ± 100-140 years from now, with a slight chance\n",
        );
        text.push_str(
            "to go beyond that. Questions are presented in a form of YYYY-MM-DD, while\n",
        );
        text.push_str("answers are expected in English ('Monday', 'Mon', 'Mo') or as numbers\n");
        text.push_str("(Monday - 1, Tuesday - 2, etc).\n");
        text.push_str(
            "\nNote: the algorithm works only for Gregorian calendar introduced during\n",
        );
        text.push_str(
            "Gregorian reform in 1582. Some countries did not adopt even until 2006, so\n",
        );
        text.push_str(
            "depending on where you live, weekdays of dates between 1582 and 2006 might be\n",
        );
        text.push_str(
            "off (see https://en.wikipedia.org/wiki/Gregorian_calendar#Adoption_by_country).",
        );

        text
    }

    fn build_arg_definitions() -> Vec<Arg> {
        vec![
            Arg::builder()
                .id(ARG_ID_HELP)
                .short_name('h')
                .long_name("help")
                .description(vec!["Display help for powers command.".to_string()])
                .kind(ArgKind::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id(ARG_ID_LOWER_BOUNDARY)
                .short_name('l')
                .long_name("lower-boundary")
                .description(vec![
                    "Set the minimum year (default: 1880)".to_string(),
                    "If default boundaries are used, the range is dynamic".to_string(),
                    "with a most of the questions from years 1880-2115".to_string(),
                    "and a small probability to go beyond these years.".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::Int))
                .stop_parsing(false)
                .default_value(ArgValue::Int(1880))
                .build(),
            Arg::builder()
                .id(ARG_ID_UPPER_BOUNDARY)
                .short_name('u')
                .long_name("upper-boundary")
                .description(vec![
                    "Set the maximum year (default: 2115)".to_string(),
                    "If default boundaries are used, the range is dynamic".to_string(),
                    "with most of the questions from years 1880-2115".to_string(),
                    "and a small probability to go beyond these years.".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::Int))
                .stop_parsing(false)
                .default_value(ArgValue::Int(2115))
                .build(),
        ]
    }

    fn build_err_message(msg: Option<String>) -> String {
        if let Some(msg) = msg {
            format!(
                "{}: {}: {}\n{}\n{}",
                APP_NAME,
                CMD,
                msg,
                Self::usage(),
                Self::help_prompt()
            )
        } else {
            format!("{}\n{}", Self::usage(), Self::help_prompt())
        }
    }

    fn check_boundaries(
        lower_boundary: i32,
        upper_boundary: i32,
        arg_definitions: &Vec<Arg>,
    ) -> Result<bool, String> {
        if lower_boundary > upper_boundary {
            return Err(Self::build_err_message(Some(
                "lower boundary must be less than or equal to upper boundary".to_string(),
            )));
        }

        let default_lower_boundary = arg_definitions
            .iter()
            .find(|def| def.id() == ARG_ID_LOWER_BOUNDARY)
            .expect("lower boundary argument definition not found")
            .default_value()
            .to_string();
        let default_upper_boundary = arg_definitions
            .iter()
            .find(|def| def.id() == ARG_ID_UPPER_BOUNDARY)
            .expect("upper boundary argument definition not found")
            .default_value()
            .to_string();

        let default_boundaries = lower_boundary.to_string() == default_lower_boundary
            && upper_boundary.to_string() == default_upper_boundary;

        if lower_boundary <= GREGORIAN_CALENDAR_INTRODUCTION {
            return Err(Self::build_err_message(Some(
                format!("year boundary too low; Doomsday algorithm does not work for dates on {GREGORIAN_CALENDAR_INTRODUCTION} and before")
            )));
        }

        if NaiveDate::from_ymd_opt(upper_boundary, 12, 31).is_none() {
            return Err(Self::build_err_message(Some(
                "year boundaries cannot exceed 262143".to_string(), // Limitation of NaiveDate
            )));
        }

        Ok(default_boundaries)
    }

    fn generate_question(&self) -> Question {
        let (year_from, year_to) = self.calculate_year_range();

        let date_from = NaiveDate::from_ymd_opt(year_from, 1, 1)
            .unwrap() // checked in build()
            .num_days_from_ce();
        let date_to = NaiveDate::from_ymd_opt(year_to, 12, 31)
            .unwrap()
            .num_days_from_ce();

        let mut rng = rand::thread_rng();
        let date = rng.gen_range(date_from..=date_to);
        let date = NaiveDate::from_num_days_from_ce_opt(date).unwrap();

        Question::from_date(date)
    }

    fn calculate_year_range(&self) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        if self.default_boundaries && rng.gen_range(0..100) < 8 {
            (1753, 2617) // Arbitrary, from Gregorian calendar adoption to ~400 years into the future
        } else {
            (self.lower_boundary, self.upper_boundary)
        }
    }
}

impl Base for Doomsday {
    fn generate_questions(&self, count: u32) -> Vec<Question> {
        iter::repeat_with(|| self.generate_question())
            .take(count as usize)
            .collect()
    }

    fn wants_to_print_help(&self) -> bool {
        self.show_help
    }

    fn get_help_text(&self) -> String {
        let definitions = &self.arg_definitions;
        let options = help::Options::new("Powers options", definitions);
        help::build(
            &Self::usage(),
            Some(&Self::additional_info()),
            &options,
            &[],
        )
    }
}

impl Question {
    fn from_date(date: NaiveDate) -> Question {
        let (answer, alternative_answers) = match date.weekday() {
            Weekday::Mon => (
                "monday".to_string(),
                vec!["mo".to_string(), "mon".to_string(), 1.to_string()],
            ),
            Weekday::Tue => (
                "tuesday".to_string(),
                vec!["tu".to_string(), "tue".to_string(), 2.to_string()],
            ),
            Weekday::Wed => (
                "wednesday".to_string(),
                vec!["we".to_string(), "wed".to_string(), 3.to_string()],
            ),
            Weekday::Thu => (
                "thursday".to_string(),
                vec!["th".to_string(), "thu".to_string(), 4.to_string()],
            ),
            Weekday::Fri => (
                "friday".to_string(),
                vec!["fr".to_string(), "fri".to_string(), 5.to_string()],
            ),
            Weekday::Sat => (
                "saturday".to_string(),
                vec!["sa".to_string(), "sat".to_string(), 6.to_string()],
            ),
            Weekday::Sun => (
                "sunday".to_string(),
                vec![
                    "su".to_string(),
                    "sun".to_string(),
                    0.to_string(),
                    7.to_string(),
                ],
            ),
        };
        Question::builder()
            .question(&format!(
                "What is the weekday of {}?",
                date.format("%Y-%m-%d")
            ))
            .answer(&answer)
            .alternative_answers(&alternative_answers)
            .allow_any_case(true)
            .build()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn build_powers_defaults() {
//         let args = [];
//         let powers = Powers::build(&args).expect("Should build correctly with no args");
//         assert!(!powers.show_help);
//         assert_eq!(powers.base, 2);
//         assert_eq!(powers.lower_boundary, 1);
//         assert_eq!(powers.upper_boundary, 16);
//     }
//
//     #[test]
//     #[should_panic(expected = "invalid option argument")]
//     fn build_powers_incorrect_args() {
//         let args = ["-b".to_string(), "hehe".to_string(), "-what".to_string()];
//         Powers::build(&args).unwrap();
//     }
//
//     #[test]
//     fn build_powers_with_args() {
//         let args = [
//             "-b".to_string(),
//             "5".to_string(),
//             "--lower-boundary=4".to_string(),
//             "-u".to_string(),
//             "7".to_string(),
//         ];
//         let powers = Powers::build(&args).expect("Should build correctly with args");
//         assert!(!powers.show_help);
//         assert_eq!(powers.base, 5);
//         assert_eq!(powers.lower_boundary, 4);
//         assert_eq!(powers.upper_boundary, 7);
//     }
//
//     #[test]
//     #[should_panic(expected = "lower boundary must be less than or equal to upper boundary")]
//     fn build_powers_mismatched_boundaries() {
//         let args = [
//             "-l".to_string(),
//             "5".to_string(),
//             "-u".to_string(),
//             "4".to_string(),
//         ];
//         Powers::build(&args).unwrap();
//     }
//
//     #[test]
//     #[should_panic(expected = "exceeds maximum allowed value")]
//     fn build_powers_overflow() {
//         let args = [
//             "-b".to_string(),
//             "2".to_string(),
//             "-u".to_string(),
//             "64".to_string(),
//         ];
//         Powers::build(&args).unwrap();
//     }
//
//     #[test]
//     fn error_message() {
//         let err = Some("something extraordinarily wrong happened".to_string());
//         let message = Powers::build_err_message(err);
//         assert!(message.contains(APP_NAME));
//         assert!(message.contains(CMD));
//         assert!(message.contains("something extraordinarily wrong happened"));
//         assert!(message.contains("Usage"));
//         assert!(message.contains("for more information"));
//
//         let err = None;
//         let message = Powers::build_err_message(err);
//         assert!(message.contains(APP_NAME));
//         assert!(message.contains(CMD));
//         assert!(message.contains("Usage"));
//         assert!(message.contains("for more information"));
//     }
//
//     #[test]
//     fn max_exponent() {
//         assert_eq!(Powers::calculate_max_exponent(2, 64), 63);
//         assert_eq!(Powers::calculate_max_exponent(5, 40), 27);
//         assert_eq!(Powers::calculate_max_exponent(17, 100), 15);
//         assert_eq!(Powers::calculate_max_exponent(101, 21), 9);
//     }
//
//     #[test]
//     fn question_generation() {
//         let args = ["-u".to_string(), "1".to_string()];
//         let powers = Powers::build(&args).expect("Should build correctly");
//         let question = powers.generate_question();
//         assert_eq!(question.prompt(), "2^1");
//         assert_eq!(question.correct_answer(), "2");
//         assert!(question.is_answer_correct("2"));
//     }
//
//     #[test]
//     fn multiple_question_generation() {
//         let args = ["--base=3".to_string()];
//         let powers = Powers::build(&args).expect("Should build correctly");
//         let questions = powers.generate_questions(10);
//         assert_eq!(questions.len(), 10);
//         assert!(questions
//             .iter()
//             .all(|question| question.prompt().starts_with("3^")));
//     }
//
//     #[test]
//     fn print_help_only() {
//         let args = [];
//         let powers = Powers::build(&args).expect("Should build correctly");
//         assert!(!powers.wants_to_print_help());
//
//         let args = [
//             "-b".to_string(),
//             "4".to_string(),
//             "-h".to_string(),
//             "--upper-boundary=10".to_string(),
//         ];
//         let powers = Powers::build(&args).expect("Should build correctly");
//         assert!(powers.wants_to_print_help());
//     }
//
//     #[test]
//     fn help_text() {
//         let args = ["-h".to_string()];
//         let powers = Powers::build(&args).expect("Should build correctly");
//         let help_text = powers.get_help_text();
//         assert!(help_text.contains("Powers options"));
//         assert!(help_text.contains("Usage"));
//
//         // Ensure all flags are included
//         assert!(help_text.contains("-h, --help"));
//         assert!(help_text.contains("-b, --base"));
//         assert!(help_text.contains("-l, --lower-boundary"));
//         assert!(help_text.contains("-u, --upper-boundary"));
//     }
// }
